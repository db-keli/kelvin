#![allow(unused)]
use kelvin_rs::{
    admin::{self, Admin},
    data::{list_decks, remove_vault},
    deck::Deck,
    deckdata::{delete_deck, update_deck, DeckData},
    password::generate_password,
    prompt::{
        clip, initialize_vault, prompt_deck, prompt_deck_open_sesame, prompt_env_var,
        prompt_logins, prompt_notes,
    },
};

use clap::{ArgAction, Arg, Command};
use std::process;

fn main() {
    initialize_vault().unwrap();
    let matches = Command::new("kelvin")
        .version("0.0.1")
        .author("Dompeh Kofi Bright, kekelidompeh@gmail.com")
        .about("A password management system, more of a vault admin")
        .subcommand(
            Command::new("generate")
                .about("Generates password and copies to clipboard")
                .arg(
                    Arg::new("length")
                        .short('l')
                        .long("length")
                        .required(false)
                        .help("Specify length of passcode to generate"),
                ),
        )
        .subcommand(Command::new("create-admin").about("Creates an admin account"))
        .subcommand(Command::new("deck").about("Adds a deck to the vault"))
        .subcommand(Command::new("list").about("Lists all stored domains"))
        .subcommand(Command::new("delete").about("Deletes a deck from the vault"))
        .subcommand(Command::new("update").about("Updates a deck's password in the vault"))
        .subcommand(Command::new("env").about("Outputs a password as an export statement"))
        .subcommand(Command::new("reset").about("Resets the vault"))
        .subcommand(
            Command::new("open-sesame")
                .about("Get password from vault")
                .arg(
                    Arg::new("stdout")
                        .long("stdout")
                        .action(ArgAction::SetTrue)
                        .help("Print to stdout instead of copying to clipboard"),
                ),
        )
        .get_matches();

    if let Some(_) = matches.subcommand_matches("create-admin") {
        let admin_credentials = prompt_logins().expect("Failed to get admin credentials");
        let mut admin = Admin::new(&admin_credentials.0, &admin_credentials.1);
        admin.hash_password();
        admin.save_to_json().expect("Failed to save admin to JSON");
    } else if let Some(matches) = matches.subcommand_matches("generate") {
        if matches.contains_id("length") {
            let length = matches
                .get_one::<String>("length")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let password = generate_password(length);
            clip(&password);
        } else {
            let password = generate_password(12);
            clip(&password);
        }
    } else if let Some(_) = matches.subcommand_matches("deck") {
        let (admin_username, admin_password) =
            prompt_logins().expect("Failed to get admin credentials");
        let admin = Admin::new(&admin_username, &admin_password);
        match admin.read_data_from_json() {
            Err(err) => print!("You're not an admin\n{}\n", err),
            Ok(admin) => {
                if admin.prompt_auth(admin_username, admin_password).unwrap() {
                    println!("Add a deck to the Vault.");
                    let (domain, password) = prompt_deck().unwrap();
                    let notes = prompt_notes().unwrap();
                    let deck = Deck::new(&domain, &password);
                    let encrypted_data = deck.encrypt();
                    let deck_data = DeckData::new(
                        admin,
                        deck.domain,
                        encrypted_data.0,
                        encrypted_data.1 .1,
                        encrypted_data.1 .0,
                        notes,
                    );
                    deck_data.save_to_json().unwrap();
                    println!("Deck saved.");
                } else {
                    println!("You're unauthorized");
                }
            }
        }
    } else if let Some(_) = matches.subcommand_matches("list") {
        let (admin_username, admin_password) =
            prompt_logins().expect("Failed to get admin credentials");
        let admin = Admin::new(&admin_username, &admin_password);
        match admin.read_data_from_json() {
            Err(_) => println!("You're not an admin"),
            Ok(admin) => {
                if admin.prompt_auth(admin_username, admin_password).unwrap() {
                    let decks = list_decks();
                    if decks.is_empty() {
                        println!("No entries in vault.");
                    } else {
                        for (domain, notes) in decks {
                            match notes {
                                Some(n) => println!("{}\t{}", domain, n),
                                None => println!("{}", domain),
                            }
                        }
                    }
                } else {
                    println!("You're unauthorized");
                }
            }
        }
    } else if let Some(_) = matches.subcommand_matches("delete") {
        let (admin_username, admin_password) =
            prompt_logins().expect("Failed to get admin credentials");
        let admin = Admin::new(&admin_username, &admin_password);
        match admin.read_data_from_json() {
            Err(_) => println!("You're not an admin"),
            Ok(admin) => {
                if admin.prompt_auth(admin_username, admin_password).unwrap() {
                    let domain = prompt_deck_open_sesame().unwrap();
                    delete_deck(&domain).unwrap();
                    println!("Deleted {}", domain);
                } else {
                    println!("You're unauthorized");
                }
            }
        }
    } else if let Some(_) = matches.subcommand_matches("update") {
        let (admin_username, admin_password) =
            prompt_logins().expect("Failed to get admin credentials");
        let admin = Admin::new(&admin_username, &admin_password);
        match admin.read_data_from_json() {
            Err(_) => println!("You're not an admin"),
            Ok(admin) => {
                if admin.prompt_auth(admin_username, admin_password).unwrap() {
                    let domain = prompt_deck_open_sesame().unwrap();
                    let (_, new_password) = prompt_deck().unwrap();
                    update_deck(&domain, &new_password, admin).unwrap();
                    println!("Updated {}", domain);
                } else {
                    println!("You're unauthorized");
                }
            }
        }
    } else if let Some(_) = matches.subcommand_matches("env") {
        let (admin_username, admin_password) =
            prompt_logins().expect("Failed to get admin credentials");
        let admin = Admin::new(&admin_username, &admin_password);
        match admin.read_data_from_json() {
            Err(_) => println!("You're not an admin"),
            Ok(admin) => {
                if admin.prompt_auth(admin_username, admin_password).unwrap() {
                    let domain = prompt_deck_open_sesame().unwrap();
                    let var_name = prompt_env_var().unwrap();
                    let deck = Deck::from_domain(&domain);
                    let data = deck.read_data_from_json().unwrap();
                    let password = String::from_utf8(data.decrypt()).unwrap();
                    println!("export {}={}", var_name, password);
                } else {
                    println!("You're unauthorized");
                }
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("open-sesame") {
        let (admin_username, admin_password) =
            prompt_logins().expect("Failed to get admin credentials");
        let admin = Admin::new(&admin_username, &admin_password);
        match admin.read_data_from_json() {
            Err(_) => println!("You're not an admin"),
            Ok(admin) => {
                if admin.prompt_auth(admin_username, admin_password).unwrap() {
                    let domain = prompt_deck_open_sesame().unwrap();
                    let deck = Deck::from_domain(&domain);
                    let data = deck.read_data_from_json().unwrap();
                    if let Some(notes) = &data.notes {
                        println!("Notes: {}", notes);
                    }
                    let password = String::from_utf8(data.decrypt()).unwrap();
                    let to_stdout = matches.get_flag("stdout");
                    if to_stdout {
                        println!("{}", password);
                    } else {
                        clip(&password);
                    }
                } else {
                    println!("You're unauthorized");
                }
            }
        }
    } else if let Some(_) = matches.subcommand_matches("reset") {
        let (admin_username, admin_password) =
            prompt_logins().expect("Failed to get admin credentials");
        let admin = Admin::new(&admin_username, &admin_password);
        match admin.read_data_from_json() {
            Err(_) => println!("You're not an admin"),
            Ok(admin) => {
                if admin.prompt_auth(admin_username, admin_password).unwrap() {
                    let _ = remove_vault().unwrap();
                } else {
                    println!("You're unauthorized");
                }
            }
        }
    }
}

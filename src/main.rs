#![allow(unused)]
use kelvin::{
    admin::Admin, 
    deck::Deck,
    deckdata::DeckData,
    password::generate_password,
    prompt::{clip, initialize_vault, prompt_deck, prompt_deck_open_sesame, prompt_logins},
    data,
};

use clap::{Arg, Command};
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
        .subcommand(Command::new("reset").about("Resets the vault"))
        .subcommand(Command::new("open-sesame").about("Get password from vault"))
        .get_matches();

    let mut status: Option<bool> = None;
    let admin: Option<Admin> = None;

    if let Some(matches) = matches.subcommand_matches("create-admin") {
        let admin_credentials = prompt_logins().expect("Failed to get admin credentials");
        let mut admin = Admin::new(&admin_credentials.0, &admin_credentials.1);
        admin.hash_password();
        admin.save_to_json().expect("Failed to save admin to JSON");
        status = Some(true);
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
            println!("Password copied to clipboard");
        }
    } else if let Some(matches) = matches.subcommand_matches("deck") {
        let (admin_username, admin_password) = prompt_logins().expect("Failed to get admin credentials");
        let admin = Admin::new(&admin_username, &admin_password);
        let status = admin.read_data_from_json();
        match status {
            Err(err) => {
                print!("You're not an admin\n{}\n", err);
            }
            Ok(admin) => {
                if admin.prompt_auth(admin_username, admin_password).unwrap() {
                    println!("Add a deck to the Vault.");
                    let deck_data = prompt_deck().unwrap();
                    let deck = Deck::new(&deck_data.0, &deck_data.1);
                    let encrypted_data = deck.encrypt();
                    let deck_data = DeckData::new(
                        admin,
                        deck.domain,
                        encrypted_data.0,
                        encrypted_data.1 .1,
                        encrypted_data.1 .0,
                    );
                    deck_data.save_to_json().unwrap();
                } else {
                    println!("You're unauthorized");
                }
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("open-sesame") {
        let (admin_username, admin_password) = prompt_logins().expect("Failed to get admin credentials");
        let admin = Admin::new(&admin_username, &admin_password);
        let status = admin.read_data_from_json();
        match status {
            Err(err) => {
                let err_msg = err.to_string();
                if err_msg.contains("No such file") {
                    print!("You're not an admin\n");
                }
            }
            Ok(admin) => {
                if admin.prompt_auth(admin_username, admin_password).unwrap() {
                    println!("Fill details to get password from the Vault.");
                    let deck = prompt_deck_open_sesame().unwrap();
                    let deck = Deck::new(&deck, "");
                    let data = deck.read_data_from_json().unwrap();
                    let password = String::from_utf8(data.decrypt()).unwrap();
                    clip(&password);
                    println!("Password copied to clipboard");
                } else {
                    println!("You're unauthorized");
                }
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("reset") {
        status = Some(true);
    }
}

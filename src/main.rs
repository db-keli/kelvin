mod admin;
mod deck;
mod deckdata;
use admin::Admin;
use clap::{Arg, Command};
use deck::Deck;
use deckdata::DeckData;
mod prompt;
mod password;

use password::generate_password;
use prompt::{prompt_deck, prompt_logins};

fn main() {
    let matches = Command::new("kelvin")
        .version("0.0.1")
        .author("Dompeh Kofi Bright, kekelidompeh@gmail.com")
        .about("A password managements system, more of a vault admin")
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
        .subcommand(Command::new("decrypt").about("Decrypts a deck"))
        .subcommand(Command::new("create-admin").about("Creates an admin account"))
        .subcommand(Command::new("deck").about("Adds a deck to the vault"))
        .subcommand(Command::new("reset").about("Resets the vault"))
        .subcommand(Command::new("open-sesame").about("Get password from vault"))
        .get_matches();

    let mut status: Option<bool> = None;
    let _admin: Option<Admin> = None;

    if let Some(_matches) = matches.subcommand_matches("create-admin") {
        let logins = prompt_logins().unwrap();
        let admin_password = logins.1;
        let mut admin = Admin::new(&logins.0, &admin_password);
        admin.hash_password();
        admin.save_to_json().unwrap();
        status = Some(true);
    } else if let Some(matches) = matches.subcommand_matches("generate") {
        if matches.contains_id("length") {
            let length = matches
                .get_one::<String>("length")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let password = generate_password(length);
            println!("{}", password);
        } else {
            let password = generate_password(12);
            println!("{}", password);
        }
    } else if let Some(_matches) = matches.subcommand_matches("deck") {
        let logins = prompt_logins().unwrap();
        let username = logins.0.trim().to_string();
        let password = logins.1.trim().to_string();
        let admin_logins = Admin::new(&username, &password);
        let status = admin_logins.read_data_from_json();
        if let Err(err) = &status {
            print!("You're not an admin\n{}\n", err);
        } else {
            let status = status.unwrap();
            if status.prompt_auth(username, password).unwrap() {
                println!("Add a deck to the Vault.");
                let deck_data = prompt_deck().unwrap();
                let deck = Deck::new(&deck_data.0, &deck_data.1);
                let encrypted_data = deck.encrypt();
                let deck_data = DeckData::new(
                    status,
                    deck.domain,
                    encrypted_data.0,
                    encrypted_data.1 .1,
                    encrypted_data.1 .0,
                );
                deck_data.save_to_json().unwrap();
            } else {
                println!("You're unathorized");
            }
        }
    } else if let Some(_matches) = matches.subcommand_matches("open-sesame") {
        let logins = prompt_logins().unwrap();
        let username = logins.0.trim().to_string();
        let password = logins.1.trim().to_string();
        let admin_logins = Admin::new(&username, &password);
        let status = admin_logins.read_data_from_json();
        if let Err(err) = &status {
            let err = err.to_string();
            if err.contains("No such file") {
                print!("You're not an admin\n");
            }
        } else {
            let status = status.unwrap();
            if status.prompt_auth(username, password).unwrap() {
                println!("Fill details to get password from the Vault.");
                let deck = prompt_deck().unwrap();
                let deck = Deck::new(&deck.0, &deck.1);
                let data = deck.read_data_from_json().unwrap();
                println!("Password: {:?}", String::from_utf8(data.decrypt()).unwrap());
            } else {
                println!("You're unathorized");
            }
        }
    } else if let Some(_matches) = matches.subcommand_matches("reset") {
        status = Some(true);
    }
}

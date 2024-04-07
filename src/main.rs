mod admin;
mod deck;
mod deckdata;

use std::io::{Error, ErrorKind};

use admin::admin::Admin;
use bcrypt::{hash, DEFAULT_COST};
use clap::Command;
use deck::deck::Deck;
use deckdata::deckdata::DeckData;
use kelvin::{check_file_exists1, generate_password, prompt_logins, read_user_data, prompt_deck};

fn main() {
    let matches = Command::new("kelvin")
        .version("0.0.1")
        .author("Dompeh Kofi Bright, kekelidompeh@gmail.com")
        .about("A password managements system, more of a vault admin")
        .subcommand(Command::new("create-admin").about("Creates an admin account"))
        .subcommand(Command::new("deck").about("Adds a deck to the vault"))
        .subcommand(Command::new("reset").about("Resets the vault"))
        .get_matches();

    let mut status: Option<bool> = None;
    let mut admin: Option<Admin> = None;

    if let Some(matches) = matches.subcommand_matches("create-admin") {
        let logins = prompt_logins().unwrap();
        let admin_password = logins.1;
        let mut admin = Admin::new(&logins.0, &admin_password);
        admin.hash_password();
        admin.save_to_json().unwrap();
        status = Some(true);
    } else if let Some(matches) = matches.subcommand_matches("deck") {
        let logins = prompt_logins().unwrap();
        let username = logins.0.trim().to_string();
        let password = logins.1.trim().to_string();
        let admin_logins = Admin::new(&username, &password);
        let status = admin_logins.read_data_from_json();
        if let Err(err) = &status {
            print!("{}", err);
        } else {
            let status = status.unwrap();
            if status.prompt_auth(username, password).unwrap() {
                println!("Add a deck to the Vault.");
                let deck_data = prompt_deck().unwrap();
                let deck = Deck::new(&deck_data.0, &deck_data.1);
                let encrypted_data = deck.encrypt();
                let deck_data = DeckData::new(status, deck.domain, encrypted_data.0, encrypted_data.1 .1, encrypted_data.1 .0);
                deck_data.save_to_json().unwrap();                
            } else {
                println!("You're unathorized");
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("reset") {
        //Reset the vault
        status = Some(true);
    }

    //Creating Admin
    // let admin_password = generate_password(12);
    // let mut admin = Admin::new("Michael", &admin_password);

    // admin.hash_password();
    // println!("admin's password after hashing: {:?}", admin.password);

    // //Adding a deck
    // let domain = "www.github.com";
    // let password = generate_password(12);
    // let deck1 = Deck::new(&domain, &password);

    // let enc_data = deck1.encrypt();

    // let deck_data = DeckData::new(
    //     admin,
    //     domain.to_string(),
    //     enc_data.0,
    //     enc_data.1 .1,
    //     enc_data.1 .0,
    // );

    // deck_data.save_to_json().unwrap();
    // println!("Deck data saved to json");

    // let deck_data_buf = deck_data.read_data_from_json().unwrap();
    // println!("Deck data read from json: {:?}", deck_data_buf);

    // println!("password: {:?}", password);
    // println!(
    //     "Decrypted data: {:?}",
    //     String::from_utf8_lossy(&deck_data_buf.decrypt())
    //);
}

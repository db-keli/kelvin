#![allow(unused)]
use kelvin_rs::{
    admin::Admin,
    data::{load_vault, remove_vault, save_vault, VaultFile},
    deck::Deck,
    deckdata::DeckData,
    password::generate_password,
    prompt::{
        clip, prompt_deck, prompt_deck_open_sesame, prompt_env_var, prompt_logins,
        prompt_master_password, prompt_new_master_password, prompt_notes, vault_encrypted_path,
    },
};

use clap::{Arg, ArgAction, Command};
use rand::thread_rng;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};

fn main() {
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

    // generate needs no vault or auth
    if let Some(matches) = matches.subcommand_matches("generate") {
        let length = matches
            .get_one::<String>("length")
            .and_then(|l| l.parse::<usize>().ok())
            .unwrap_or(12);
        let password = generate_password(length);
        clip(&password);
        return;
    }

    // all other commands need the master password
    let master_password = if vault_encrypted_path().exists() {
        prompt_master_password().expect("Failed to read master password")
    } else {
        let pw = prompt_new_master_password().expect("Failed to read master password");
        save_vault(&VaultFile::default(), &pw).expect("Failed to create vault");
        pw
    };

    // create-admin only needs master password, not admin login
    if let Some(_) = matches.subcommand_matches("create-admin") {
        let mut vault = load_vault(&master_password).unwrap_or_default();
        let (username, password) = prompt_logins().unwrap();
        let mut admin = Admin::new(&username, &password);
        admin.hash_password();
        vault.admin = Some(admin);
        save_vault(&vault, &master_password).expect("Failed to save vault");
        println!("Admin created.");
        return;
    }

    // load vault once for all remaining commands
    let mut vault = match load_vault(&master_password) {
        Ok(v) => v,
        Err(e) => {
            println!("Failed to open vault: {}", e);
            return;
        }
    };

    // admin auth required for all remaining commands
    let (admin_username, admin_password) = prompt_logins().expect("Failed to read credentials");
    let stored_admin = match vault.admin.clone() {
        Some(a) => a,
        None => {
            println!("No admin found. Run create-admin first.");
            return;
        }
    };
    if stored_admin.username != admin_username || !stored_admin.verify_password(&admin_password) {
        println!("You're unauthorized");
        return;
    }

    if let Some(_) = matches.subcommand_matches("list") {
        if vault.decks.is_empty() {
            println!("No entries in vault.");
        } else {
            for deck in &vault.decks {
                match &deck.notes {
                    Some(n) => println!("{}\t{}", deck.domain, n),
                    None => println!("{}", deck.domain),
                }
            }
        }
    } else if let Some(_) = matches.subcommand_matches("deck") {
        let (domain, password) = prompt_deck().unwrap();
        let notes = prompt_notes().unwrap();
        let deck = Deck::new(&domain, &password);
        let encrypted = deck.encrypt();
        let deck_data = DeckData::new(
            stored_admin,
            deck.domain.clone(),
            encrypted.0,
            encrypted.1 .1,
            encrypted.1 .0,
            notes,
        );
        if let Some(pos) = vault.decks.iter().position(|d| d.domain == deck.domain) {
            vault.decks[pos] = deck_data;
        } else {
            vault.decks.push(deck_data);
        }
        save_vault(&vault, &master_password).unwrap();
        println!("Deck saved.");
    } else if let Some(m) = matches.subcommand_matches("open-sesame") {
        let domain = prompt_deck_open_sesame().unwrap();
        match vault.decks.iter().find(|d| d.domain == domain) {
            None => println!("Domain '{}' not found.", domain),
            Some(data) => {
                if let Some(notes) = &data.notes {
                    println!("Notes: {}", notes);
                }
                let password = String::from_utf8(data.decrypt()).unwrap();
                if m.get_flag("stdout") {
                    println!("{}", password);
                } else {
                    clip(&password);
                }
            }
        }
    } else if let Some(_) = matches.subcommand_matches("delete") {
        let domain = prompt_deck_open_sesame().unwrap();
        let before = vault.decks.len();
        vault.decks.retain(|d| d.domain != domain);
        if vault.decks.len() < before {
            save_vault(&vault, &master_password).unwrap();
            println!("Deleted {}", domain);
        } else {
            println!("Domain '{}' not found.", domain);
        }
    } else if let Some(_) = matches.subcommand_matches("update") {
        let domain = prompt_deck_open_sesame().unwrap();
        if let Some(pos) = vault.decks.iter().position(|d| d.domain == domain) {
            let notes = vault.decks[pos].notes.clone();
            let (_, new_password) = prompt_deck().unwrap();
            let mut rng = thread_rng();
            let private_key = RsaPrivateKey::new(&mut rng, 2048).unwrap();
            let public_key = RsaPublicKey::from(&private_key);
            let ciphertext = public_key
                .encrypt(&mut rng, Pkcs1v15Encrypt, new_password.as_bytes())
                .unwrap();
            vault.decks[pos] = DeckData::new(
                stored_admin,
                domain.clone(),
                ciphertext,
                public_key,
                private_key,
                notes,
            );
            save_vault(&vault, &master_password).unwrap();
            println!("Updated {}", domain);
        } else {
            println!("Domain '{}' not found.", domain);
        }
    } else if let Some(_) = matches.subcommand_matches("env") {
        let domain = prompt_deck_open_sesame().unwrap();
        let var_name = prompt_env_var().unwrap();
        match vault.decks.iter().find(|d| d.domain == domain) {
            None => println!("Domain '{}' not found.", domain),
            Some(data) => {
                let password = String::from_utf8(data.decrypt()).unwrap();
                println!("export {}={}", var_name, password);
            }
        }
    } else if let Some(_) = matches.subcommand_matches("reset") {
        remove_vault().unwrap();
    }
}

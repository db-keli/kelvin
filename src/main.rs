mod admin;
mod deck;
mod deckdata;


use admin::admin::Admin;
use clap::{Arg, Command};
use deck::deck::Deck;
use deckdata::deckdata::DeckData;
use kelvin::{generate_password, prompt_password};
use bcrypt::{hash, DEFAULT_COST};

fn main() {
    let matches = Command::new("kelvin")
        .version("0.0.1")
        .author("Dompeh Kofi Bright, kekelidompeh@gmail.com")
        .about("A password managements system, more of a vault admin")
        .subcommand(
            Command::new("admin").about("Creates an admin account").arg(
                Arg::new("username")
                    .short('u')
                    .long("username")
                    .help("Sets the username of the admin")
                    .required(true),
            ),
        )
        // .subcommand(
        //     Command::new("login")
        //     .about("Let's you login")
        // )
        .subcommand(
            Command::new("deck")
                .about("Adds a deck to the vault")
                .arg(
                    Arg::new("domain")
                        .short('d')
                        .long("domain")
                        .help("Sets the domain of the deck")
                        .required(true),
                )
                .arg(
                    Arg::new("password")
                        .short('p')
                        .long("password")
                        .help("Sets the password of the deck")
                        .required(true),
                ),
        )
        .subcommand(Command::new("reset").about("Resets the vault"))
        .get_matches();

    let mut status: Option<bool> = None;
    let mut admin :Option<Admin> = None;
    
    
    if let Some(matches) = matches.subcommand_matches("admin") {
        let username = matches.get_one::<String>("username").unwrap();
        let password = prompt_password("Enter admin password: ").unwrap();
        admin = Some(Admin::new(username, &password));
        if let Some(ref mut admin) = admin {
            admin.hash_password();
            let _ = admin.save_to_json();
            println!("admin's password after hashing: {:?}", admin.password);
        }
    // } else if let Some(matches) = matches.subcommand_matches("login") {
    //     let logins = prompt_login().unwrap();

    //     if let Some(ref mut status) = status {
    //         *status = logins.2;
    //         println!("{:?}", status);
    //     } else {
    //         println!("Never reached");
    //     }
        
        
    // }
    else if let Some(matches) = matches.subcommand_matches("deck") {
        //Login
            //prompt_auth
            //check if file with username exists
            //If it doesn't tell them they're not authorized  
        if let Some(admin) = admin {
            let domain = matches.get_one::<String>("domain").unwrap();
            let password = matches.get_one::<String>("password").unwrap();
            let deck1 = Deck::new(&domain, &password);
            let enc_data = deck1.encrypt();
            let deck_data = DeckData::new(
                admin,
                domain.to_string(),
                enc_data.0,
                enc_data.1 .1,
                enc_data.1 .0,
            );
            deck_data.save_to_json().unwrap();
            println!("Deck data saved to json");
            let deck_data_buf = deck_data.read_data_from_json().unwrap();
            println!("Deck data read from json: {:?}", deck_data_buf);
            println!("password: {:?}", password);
            println!(
                "Decrypted data: {:?}",
                String::from_utf8_lossy(&deck_data_buf.decrypt())
            );
         } //else {
        //     if let Some(ref mut admin) = admin {
        //         let temp_admin = admin.read_data_from_json().unwrap();
        //         let logins = prompt_login().unwrap();
        //         let username = logins.0;
        //         let password = hash(&logins.1, DEFAULT_COST).expect("Failed to hash password");
        //     }

        // }
    } else if let Some(matches) = matches.subcommand_matches("reset") {
        println!("Vault reset");
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
}

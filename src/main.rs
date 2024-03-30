mod admin;
mod deck;

use kelvin::{admin::admin::Admin, generate_password, DeckData};
use serde::{Deserialize, Serialize};

use deck::deck::*;

fn main() {
    //initialize
    let length = 20;
    let username = String::from("Michael");
    let pass = generate_password(length);
    println!("Admin's password is {}", pass);

    let mut admin1 = Admin::new(&username, &pass);

    // Hash password
    admin1.hash_password();

    //Add a dack and encrypt
    //let deck1 = Deck::new(&admin1.username, &admin1.password);
    let deck2 = Deck::new("google.com", "kekelidompehfuckseveryone");
    //let dec_pass = deck1.encrypt();
    let dec_pass2 = deck2.encrypt();

    // Decrypting the data and printing it out
    //let dec_data = deck1.decrypt();
    //let pass_test = String::from_utf8(dec_data);

    //match pass_test {
    //    Ok(string) => {
    //        println!("{}", string);
    //      }
    //    Err(err) => {
    //       println!("{}", err);
    //    }
    // }

    // Data to json to save to file
    //let data = DeckData::new(admin1, deck1.domain, dec_pass.0);
    //
    let data2 = DeckData::new(admin1, deck2.domain.clone(), dec_pass2.0);
    data2.serialize_struct();
    let _ = data2.save_to_json();

    //Load Data from json

    let filepath = "data.json";
    match DeckData::load_from_json(filepath, &deck2.domain) {
        Ok(deck_data) => println!("{:?}", deck_data),
        Err(err) => eprintln!("Error: {}", err),
    }
}

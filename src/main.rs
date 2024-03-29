mod admin;
mod deck;

use kelvin::{admin::admin::Admin, generate_password, DeckData};
use serde::{Deserialize, Serialize};

use deck::deck::*;

fn main() {
    let length = 20;
    let username = String::from("Michael");
    let pass = generate_password(length);
    println!("Admin's password is {}", pass);

    let admin1 = Admin::new(&username, &pass);
    let deck1 = Deck::new(&admin1.username, &admin1.password);

    let dec_pass = deck1.encrypt();

    println!("After encyption, password is{}", deck1.plaintext);
    let dec_data = deck1.decrypt();

    let pass_test = String::from_utf8(dec_data);

    match pass_test {
        Ok(string) => {
            println!("{}", string);
        }
        Err(err) => {
            println!("{}", err);
        }
    }

    let data = DeckData::new(admin1, deck1.domain, dec_pass.0);
    data.serialize_struct();
}

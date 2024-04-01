mod deck;
mod deckdata;
mod admin;

use admin::admin::Admin;
use deck::deck::Deck;
use deckdata::deckdata::DeckData;
use kelvin::generate_password;

fn main() {
    //Creating Admin
    let admin_password = generate_password(12);
    let mut admin = Admin::new("Michael", &admin_password);

    admin.hash_password();
    println!("admin's password after hashing: {:?}", admin.password);

    //Adding a deck
    let domain = "google.com";
    let password = generate_password(12);
    let deck1 = Deck::new(&domain, &password);

    let enc_data = deck1.encrypt();

    let deck_data = DeckData::new(admin, domain.to_string(), enc_data.0, enc_data.1.1, enc_data.1.0);

    deck_data.save_to_json().unwrap();
    println!("Deck data saved to json");

    let deck_data_buf = deck_data.read_data_from_json().unwrap();
    println!("Deck data read from json: {:?}", deck_data_buf);

    println!("password: {:?}", password);
    println!("Decrypted data: {:?}", String::from_utf8_lossy(&deck_data_buf.decrypt()));
}

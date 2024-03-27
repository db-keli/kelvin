mod admin;
mod deck;
use kelvin::generate_password;

use deck::deck::*;
use admin::admin::*;
fn main() {
    let length = 20;
    let username = String::from("Michael");
    let pass = generate_password(length);

    let mut admin1 = Admin::new(&username, &pass);
    let deck1 = Deck::new(&admin1.username, &admin1.password);

    println!(
        "His name is {} and his admin password is {}",
        admin1.username, admin1.password
    );
    println!(
        "He added a new acount name {} with a password of {}",
        deck1.domain, deck1.plaintext
    );

    admin1.hash_password();
    println!("Password updated to {}", admin1.password);
    let password_to_verify = &pass;

    if admin1.verify_password(password_to_verify) {
        println!("FInally");
    } else {
        println!("Fuck")
    }
}

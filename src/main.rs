use kelvin::{generate_password, Admin, Deck};

fn main() {
    let length = 20;
    let username = String::from("Michael");
    let pass = generate_password(length);

    println!("The password is {}", pass);

    let admin1 = Admin::new(&username, &pass);
    let deck1 = Deck::new(&admin1.username, &admin1.password);

    println!(
        "His name is {} and his admin password is {}",
        admin1.username, admin1.password
    );
    println!(
        "He added a new acount name {} with a password of {}",
        deck1.domain, deck1.plaintext
    );
}

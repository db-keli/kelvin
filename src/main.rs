use kelvin::{generate_password, Admin, Deck};

fn main() {
    let length = 20;
    let username = String::from("Michael");
    let pass = generate_password(length);

    println!("The password is {}", pass);

    let admin1 = Admin::new(&username, &pass);
    let deck1 = Deck::new(&admin1.username, &admin1.password);
}

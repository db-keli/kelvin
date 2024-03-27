mod admin;
mod deck;
use kelvin::generate_password;

use deck::deck::*;
use admin::admin::*;
fn main() {
    let length = 20;
    let username = String::from("Michael");
    let pass = generate_password(length);
    println!("Admin's password is {}", pass);

    let admin1 = Admin::new(&username, &pass);
    let deck1 = Deck::new(&admin1.username, &admin1.password);
    
    deck1.encrypt();

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
    
}

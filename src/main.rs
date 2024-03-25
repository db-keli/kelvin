use kelvin::{generate_password, Admin};

fn main() {
    let length = 20;
    let username = String::from("Michael");
    let pass = generate_password(length);

    println!("The password is {}", pass);

    let _admin = Admin {
        username,
        password: pass,
    };
}

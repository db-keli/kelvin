use kelvin::generate_password;

fn main() {
    let length = 20;

    let pass = generate_password(length);

    println!("The password is {}", pass);
}
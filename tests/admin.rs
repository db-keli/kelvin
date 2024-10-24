use bcrypt::{hash, DEFAULT_COST};
use kelvin_rs::admin::*;
use kelvin_rs::password::generate_password;
use kelvin_rs::prompt::initialize_vault;

#[test]
fn constructor_valid() {
    let password_test: String = generate_password(12);
    let name: String = String::from("Michael");
    let admin: Admin = Admin::new(&name, &password_test);
    assert_eq!(admin.password, password_test);
}

#[test]
fn test_hash_function() {
    let password_test: String = generate_password(12);
    let name: String = String::from("Michael");
    let mut admin: Admin = Admin::new(&name, &password_test);

    admin.hash_password();

    assert_ne!(admin.password, password_test);
}

#[test]
fn test_hashing() {
    let password: String = generate_password(12);
    let name: String = String::from("Michael");
    let mut admin: Admin = Admin::new(&name, &password);

    admin.hash_password();
    let hashed_password = hash(&password, DEFAULT_COST).expect("Failed to hash password");

    assert_ne!(admin.password, hashed_password);
}

#[test]
fn test_verify_function() {
    let password: String = generate_password(12);
    let name: String = String::from("Michael");
    let mut admin: Admin = Admin::new(&name, &password);

    admin.hash_password();

    let input_password = generate_password(12);
    assert!(!admin.verify_password(&input_password));
}

#[test]
fn test_save_read_to_json() {
    let _ = initialize_vault();

    let password: String = generate_password(12);
    let name = String::from("Michael");
    let admin = Admin::new(&name, &password);

    let _ = admin.test_save_to_json();
    let data = admin.test_read_data_from_json().unwrap();

    assert_eq!(data, admin);
}

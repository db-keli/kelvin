pub mod admin;
use admin::*;
use bcrypt::{hash, DEFAULT_COST};

use aes_gcm::{Aes256Gcm, Key, Nonce, Tag};
use rand::thread_rng;
use rand::Rng;

//Generate Password
pub fn generate_password(length: usize) -> String {
    let ascii_chars: Vec<char> = (33..=126).map(|c| c as u8 as char).collect();
    let mut rng = thread_rng();
    let mut password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..ascii_chars.len());
            ascii_chars[idx]
        })
        .collect();

    let mut password_vector: Vec<char> = password.chars().collect();
    let mut k = length;
    while k > 1 {
        let i = rng.gen_range(0..length);
        k -= 1;
        password_vector.swap(k, i);
    }

    password = password_vector.iter().collect();
    password
}
//Generic function to save into a file
//Save admin name and hashed_password
//Save a deck thus domain, ciphertext, nonce and key
pub fn save<T>(_element: &T) {
    println!("Saving elements come here");
}

//Add a deck
pub struct Deck {
    pub domain: String,
    pub plaintext: String,
}

impl Deck {
    pub fn new(domain: &str, plaintext: &str) -> Deck {
        let domain = domain.to_string();
        let plaintext = plaintext.to_string();

        Deck { domain, plaintext }
    }

    pub fn encrypt(&self) {}
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn valid_password_generation() {
        let ascii_chars: Vec<char> = (33..=126).map(|c| c as u8 as char).collect();
        let length = 7;
        let password_vector: Vec<char> = generate_password(length).chars().collect();

        assert!(password_vector.iter().all(|&x| ascii_chars.contains(&x)));
    }

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
}

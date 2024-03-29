use rand::thread_rng;
use rand::Rng;

pub mod admin;
pub mod deck;

use admin::admin::Admin;
use serde::{Deserialize, Serialize};
use serde_json::to_string;

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
#[derive(Serialize, Deserialize)]
pub struct DeckData {
    admin_data: Admin,
    domain: String,
    ciphertext: Vec<u8>,
}

impl DeckData {
    pub fn new(admin_data: Admin, domain: String, ciphertext: Vec<u8>) -> DeckData {
        DeckData {
            admin_data,
            domain,
            ciphertext,
        }
    }

    pub fn serialize_struct(&self) {
        let dat_ser = to_string(self);
        if let Err(err) = &dat_ser {
            eprintln!("Serialization error: {}", err);
        }

        println!("{}", dat_ser.ok().unwrap());
    }
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
}

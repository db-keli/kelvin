use rand::{rngs::ThreadRng, thread_rng};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Result, Write};

use crate::data::{decrypt_directory, encrypt_directory};
use crate::deckdata::DeckData;
static VAULT_PATH: &str = "./.vault";

#[derive(Serialize, Deserialize, Debug)]
pub struct Deck {
    pub domain: String,
    pub plaintext: String,
}

pub fn get_keys() -> (RsaPrivateKey, RsaPublicKey, ThreadRng) {
    let mut rng = thread_rng();
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);

    (private_key, public_key, rng)
}

impl Deck {
    pub fn new(domain: &str, plaintext: &str) -> Deck {
        let domain = domain.to_string();
        let plaintext = plaintext.to_string();

        Deck { domain, plaintext }
    }

    pub fn encrypt(&self) -> (Vec<u8>, (RsaPrivateKey, RsaPublicKey, ThreadRng)) {
        let mut keys = get_keys();

        let plaintext = &self.plaintext.as_bytes();

        let encrypted_data = keys
            .1
            .encrypt(&mut keys.2, Pkcs1v15Encrypt, &plaintext[..])
            .expect("Failed to encrypt");

        (encrypted_data, keys)
    }

    #[allow(dead_code)]
    pub fn read_data_from_json(&self) -> Result<DeckData> {
        let filepath = format!("{}/{}.json", VAULT_PATH, self.domain);
        let _ = decrypt_directory();
        let mut file = File::open(filepath)?;
        let mut json_data = String::new();
        file.read_to_string(&mut json_data)?;
        file.flush()?;
        let _ = encrypt_directory();

        println!("{}", json_data);
        let deck_data_vec: Vec<DeckData> = serde_json::from_str(&json_data)?;

        let deck_data = deck_data_vec
            .into_iter()
            .next()
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "No data found in JSON"))?;

        Ok(deck_data)
    }
}

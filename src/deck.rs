use rand::{rngs::ThreadRng, thread_rng};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};

use crate::data::read_vault;

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
        let mut domain = domain.to_string();
        let mut counter = 1;
        while Self::domain_exists(&domain) {
            domain = format!("{}_{}", domain, counter);
            counter += 1;
        }
        Deck {
            domain,
            plaintext: plaintext.to_string(),
        }
    }

    pub fn from_domain(domain: &str) -> Deck {
        Deck {
            domain: domain.to_string(),
            plaintext: String::new(),
        }
    }

    fn domain_exists(domain: &str) -> bool {
        read_vault().decks.iter().any(|d| d.domain == domain)
    }

    pub fn encrypt(&self) -> (Vec<u8>, (RsaPrivateKey, RsaPublicKey, ThreadRng)) {
        let mut keys = get_keys();
        let encrypted_data = keys
            .1
            .encrypt(&mut keys.2, Pkcs1v15Encrypt, self.plaintext.as_bytes())
            .expect("Failed to encrypt");
        (encrypted_data, keys)
    }
}

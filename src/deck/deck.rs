use rand::{rngs::ThreadRng, thread_rng};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};

//Add a deck
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

    pub fn decrypt(&self) -> Vec<u8> {
        let encrypted_data = self.encrypt();
        let dec_data = encrypted_data
            .1
             .0
            .decrypt(Pkcs1v15Encrypt, &encrypted_data.0)
            .expect("failed to decrypt");

        dec_data
    }
}

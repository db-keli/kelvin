//use aes_gcm::{Aes256Gcm, Key, Nonce, Tag};

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

    pub fn _encrypt(&self) {}
}
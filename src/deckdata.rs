use crate::admin::Admin;
use crate::data::{read_vault, write_vault};
use rsa::{
    pkcs1::{
        self, DecodeRsaPrivateKey, DecodeRsaPublicKey, EncodeRsaPrivateKey, EncodeRsaPublicKey,
    },
    Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
};

use serde::{Deserialize, Serialize};
use std::io::{self, Result};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct DeckData {
    pub domain: String,
    pub ciphertext: Vec<u8>,
    pub admin_data: Admin,
    pub rsa_public_key: String,
    pub rsa_private_key: String,
    pub notes: Option<String>,
}

impl DeckData {
    pub fn new(
        admin_data: Admin,
        domain: String,
        ciphertext: Vec<u8>,
        rsa_public_key: RsaPublicKey,
        rsa_private_key: RsaPrivateKey,
        notes: Option<String>,
    ) -> DeckData {
        let pem_pub = rsa_public_key.to_pkcs1_pem(pkcs1::LineEnding::LF).unwrap();
        let pem_prv = rsa_private_key
            .to_pkcs1_pem(pkcs1::LineEnding::LF)
            .unwrap()
            .to_string();
        DeckData {
            domain,
            ciphertext,
            admin_data,
            rsa_public_key: pem_pub,
            rsa_private_key: pem_prv,
            notes,
        }
    }

    pub fn decrypt(&self) -> Vec<u8> {
        let private_key = RsaPrivateKey::from_pkcs1_pem(&self.rsa_private_key).unwrap();
        private_key
            .decrypt(Pkcs1v15Encrypt, &self.ciphertext)
            .expect("Failed to decrypt")
    }

    pub fn test_save_to_json(&self) -> Result<()> {
        let mut vault = read_vault();
        if let Some(pos) = vault.decks.iter().position(|d| d.domain == self.domain) {
            vault.decks[pos] = self.clone();
        } else {
            vault.decks.push(self.clone());
        }
        write_vault(&vault)?;
        Ok(())
    }

    pub fn test_read_data_from_json(&self) -> Result<DeckData> {
        let vault = read_vault();
        vault
            .decks
            .into_iter()
            .find(|d| d.domain == self.domain)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "No data found"))
    }
}

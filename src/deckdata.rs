use crate::admin::Admin;
use crate::data::{decrypt_vault, encrypt_vault, read_vault, write_vault};
use rand::thread_rng;
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

    pub fn save_to_json(&self) -> Result<()> {
        let _ = decrypt_vault();
        let mut vault = read_vault();
        if let Some(pos) = vault.decks.iter().position(|d| d.domain == self.domain) {
            vault.decks[pos] = self.clone();
        } else {
            vault.decks.push(self.clone());
        }
        write_vault(&vault)?;
        let _ = encrypt_vault();
        Ok(())
    }

    #[allow(dead_code)]
    pub fn read_data_from_json(&self) -> Result<DeckData> {
        let _ = decrypt_vault();
        let vault = read_vault();
        let _ = encrypt_vault();
        vault
            .decks
            .into_iter()
            .find(|d| d.domain == self.domain)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "No data found in JSON"))
    }

    #[allow(dead_code)]
    pub fn decrypt(&self) -> Vec<u8> {
        let private_key = RsaPrivateKey::from_pkcs1_pem(&self.rsa_private_key).unwrap();
        let public_key = RsaPublicKey::from_pkcs1_pem(&self.rsa_public_key).unwrap();

        let keys = (private_key, public_key);
        keys.0
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

    #[allow(dead_code)]
    pub fn test_read_data_from_json(&self) -> Result<DeckData> {
        let vault = read_vault();
        vault
            .decks
            .into_iter()
            .find(|d| d.domain == self.domain)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "No data found in JSON"))
    }
}

pub fn delete_deck(domain: &str) -> Result<()> {
    let _ = decrypt_vault();
    let mut vault = read_vault();
    vault.decks.retain(|d| d.domain != domain);
    write_vault(&vault)?;
    let _ = encrypt_vault();
    Ok(())
}

pub fn update_deck(domain: &str, new_password: &str, admin: Admin) -> Result<()> {
    let _ = decrypt_vault();
    let mut vault = read_vault();
    if let Some(pos) = vault.decks.iter().position(|d| d.domain == domain) {
        let notes = vault.decks[pos].notes.clone();
        let mut rng = thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("Failed to generate key");
        let public_key = RsaPublicKey::from(&private_key);
        let ciphertext = public_key
            .encrypt(&mut rng, Pkcs1v15Encrypt, new_password.as_bytes())
            .expect("Failed to encrypt");
        vault.decks[pos] = DeckData::new(
            admin,
            domain.to_string(),
            ciphertext,
            public_key,
            private_key,
            notes,
        );
    } else {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Domain not found"));
    }
    write_vault(&vault)?;
    let _ = encrypt_vault();
    Ok(())
}

pub fn test_delete_deck(domain: &str) -> Result<()> {
    let mut vault = read_vault();
    vault.decks.retain(|d| d.domain != domain);
    write_vault(&vault)?;
    Ok(())
}

pub fn test_update_deck(domain: &str, new_password: &str, admin: Admin) -> Result<()> {
    let mut vault = read_vault();
    if let Some(pos) = vault.decks.iter().position(|d| d.domain == domain) {
        let notes = vault.decks[pos].notes.clone();
        let mut rng = thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("Failed to generate key");
        let public_key = RsaPublicKey::from(&private_key);
        let ciphertext = public_key
            .encrypt(&mut rng, Pkcs1v15Encrypt, new_password.as_bytes())
            .expect("Failed to encrypt");
        vault.decks[pos] = DeckData::new(
            admin,
            domain.to_string(),
            ciphertext,
            public_key,
            private_key,
            notes,
        );
    } else {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Domain not found"));
    }
    write_vault(&vault)?;
    Ok(())
}

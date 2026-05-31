use std::fs;

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use argon2::Argon2;
use rand::RngCore;
use serde::{Deserialize, Serialize};

use crate::admin::Admin;
use crate::deckdata::DeckData;
use crate::prompt::{vault_encrypted_path, vault_path};

const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;

#[derive(Serialize, Deserialize, Default)]
pub struct VaultFile {
    pub admin: Option<Admin>,
    pub decks: Vec<DeckData>,
}

// used by tests only — reads the plain unencrypted vault file
pub fn read_vault() -> VaultFile {
    if let Ok(content) = fs::read_to_string(vault_path()) {
        if let Ok(vault) = serde_json::from_str(&content) {
            return vault;
        }
    }
    VaultFile::default()
}

// used by tests only
pub fn write_vault(vault: &VaultFile) -> std::io::Result<()> {
    let content = serde_json::to_string(vault).unwrap();
    fs::write(vault_path(), content)?;
    Ok(())
}

pub fn load_vault(password: &str) -> std::io::Result<VaultFile> {
    let data = fs::read(vault_encrypted_path())?;

    if data.len() < SALT_LEN + NONCE_LEN {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Vault file is corrupted",
        ));
    }

    let salt = &data[..SALT_LEN];
    let nonce_bytes = &data[SALT_LEN..SALT_LEN + NONCE_LEN];
    let ciphertext = &data[SALT_LEN + NONCE_LEN..];

    let mut key_bytes = [0u8; 32];
    Argon2::default()
        .hash_password_into(password.as_bytes(), salt, &mut key_bytes)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key_bytes));
    let plaintext = cipher
        .decrypt(Nonce::from_slice(nonce_bytes), ciphertext)
        .map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "Wrong password or corrupted vault",
            )
        })?;

    serde_json::from_slice(&plaintext)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))
}

pub fn save_vault(vault: &VaultFile, password: &str) -> std::io::Result<()> {
    let mut salt = [0u8; SALT_LEN];
    let mut nonce_bytes = [0u8; NONCE_LEN];
    rand::thread_rng().fill_bytes(&mut salt);
    rand::thread_rng().fill_bytes(&mut nonce_bytes);

    let mut key_bytes = [0u8; 32];
    Argon2::default()
        .hash_password_into(password.as_bytes(), &salt, &mut key_bytes)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key_bytes));
    let plaintext = serde_json::to_vec(vault).unwrap();
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce_bytes), plaintext.as_slice())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    let mut data = Vec::with_capacity(SALT_LEN + NONCE_LEN + ciphertext.len());
    data.extend_from_slice(&salt);
    data.extend_from_slice(&nonce_bytes);
    data.extend_from_slice(&ciphertext);

    fs::write(vault_encrypted_path(), data)?;
    Ok(())
}

pub fn remove_vault() -> std::io::Result<()> {
    let path = vault_encrypted_path();
    if path.exists() {
        fs::remove_file(&path)?;
    }
    println!("Vault reset successfully");
    Ok(())
}

use std::fs;
use std::process::Command;

use serde::{Deserialize, Serialize};

use crate::admin::Admin;
use crate::deckdata::DeckData;
use crate::prompt::{vault_encrypted_path, vault_path};

#[derive(Serialize, Deserialize, Default)]
pub struct VaultFile {
    pub admin: Option<Admin>,
    pub decks: Vec<DeckData>,
}

pub fn read_vault() -> VaultFile {
    if let Ok(content) = fs::read_to_string(vault_path()) {
        if let Ok(vault) = serde_json::from_str(&content) {
            return vault;
        }
    }
    VaultFile::default()
}

pub fn write_vault(vault: &VaultFile) -> std::io::Result<()> {
    let content = serde_json::to_string(vault).unwrap();
    fs::write(vault_path(), content)?;
    Ok(())
}

pub fn encrypt_vault() -> std::io::Result<()> {
    println!("Locking data.....");
    let path = vault_path();
    if !path.exists() {
        return Ok(());
    }
    let output = Command::new("gpg")
        .args(["-c", "--no-use-agent", path.to_str().unwrap()])
        .output()?;
    if output.status.success() {
        let _ = fs::remove_file(&path);
    } else {
        println!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
    Ok(())
}

pub fn decrypt_vault() -> std::io::Result<()> {
    let encrypted_path = vault_encrypted_path();
    if !encrypted_path.exists() {
        return Ok(());
    }
    let output = Command::new("gpg")
        .arg(encrypted_path.to_str().unwrap())
        .output()?;
    if output.status.success() {
        let _ = fs::remove_file(&encrypted_path);
    } else {
        println!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
    Ok(())
}

pub fn remove_vault() -> std::io::Result<()> {
    let output = Command::new("sudo")
        .args([
            "rm",
            "-rf",
            vault_path().to_str().unwrap(),
            vault_encrypted_path().to_str().unwrap(),
        ])
        .output()?;
    if output.status.success() {
        println!("Vault reset successfully");
    } else {
        println!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
    Ok(())
}

pub fn list_decks() -> Vec<(String, Option<String>)> {
    read_vault()
        .decks
        .into_iter()
        .map(|d| (d.domain, d.notes))
        .collect()
}

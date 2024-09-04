use std::fs;
use std::fs::{read_dir, read_to_string};
use std::process::Command;

use crate::{admin::Admin, deckdata, prompt::vault_path};

pub fn check_file_exists(username: &str, directory_path: &str) -> bool {
    if let Ok(entries) = read_dir(directory_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(filename) = entry.file_name().to_str() {
                    return filename == username;
                }
            }
        }
    }
    false
}

pub fn read_user_data(username: &str, directory_path: &str) -> Option<Admin> {
    let file_path = format!("{}/{}", directory_path, username);

    if check_file_exists(username, directory_path) {
        if let Ok(file_content) = read_to_string(&file_path) {
            if let Ok(user_data) = serde_json::from_str(&file_content) {
                return Some(user_data);
            }
        }
    }

    None
}

pub fn read_deck_data(domain: &str) -> Option<deckdata::DeckData> {
    let vault_dir = vault_path();
    let file_path = format!("{}/{}.json", vault_dir.display(), domain);
    if let Ok(file_content) = read_to_string(file_path) {
        if let Ok(deck_data) = serde_json::from_str(&file_content) {
            return Some(deck_data);
        }
    }
    None
}

/// This function was specifically made to encrypt ./vault.tar.gz directory where encrypted data is stored
///
/// ## Usage
///
/// let _ = encrypt_directory();
///
pub fn encrypt_directory() -> std::io::Result<()> {
    println!("Locking data.....");

    let vault_dir = vault_path();
    let vault_tar = vault_dir.with_extension("tar.gz");

    let output = Command::new("tar")
        .arg("-czvf")
        .arg(&vault_tar)
        .arg(&vault_dir)
        .output()?;

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));

        // Encrypt the tar.gz file
        let output2 = Command::new("gpg")
            .args(["-c", "--no-use-agent", vault_tar.to_str().unwrap()])
            .output()?;
        if output2.status.success() {
            println!("{}", String::from_utf8_lossy(&output2.stdout));

            // Remove the original vault directory and the tar.gz file
            let _ = fs::remove_dir_all(&vault_dir);
            let _ = fs::remove_file(&vault_tar);
        } else {
            println!("Error: {}", String::from_utf8_lossy(&output2.stderr));
        }
    } else {
        println!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}

/// This function was specifically made to decrypt ./vault.tar.gz directory where encrypted data is stored
///
/// ## Usage
///
/// let _ = decrypt_directory()
///
pub fn decrypt_directory() -> std::io::Result<()> {
    let vault_dir = vault_path();
    let vault_tar_gpg = vault_dir.with_extension("tar.gz.gpg");

    let output = Command::new("gpg")
        .arg(vault_tar_gpg.to_str().unwrap())
        .output()?;
    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));

        // Extract the tar.gz file
        let vault_tar = vault_dir.with_extension("tar.gz");
        let output2 = Command::new("tar")
            .args(["-xf", vault_tar.to_str().unwrap()])
            .output()?;

        if output2.status.success() {
            println!("{}", String::from_utf8_lossy(&output2.stdout));

            // Remove the tar.gz file
            let _ = fs::remove_file(vault_tar);
        } else {
            println!("Error: {}", String::from_utf8_lossy(&output2.stderr));
        }
    } else {
        println!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}

pub fn remove_vault() -> std::io::Result<()> {
    let vault_dir = vault_path();
    let vault_tar_gpg = vault_dir.with_extension("tar.gz.gpg");
    let output = Command::new("sudo")
        .args([
            "rm",
            "-rf",
            vault_dir.to_str().unwrap(),
            vault_tar_gpg.to_str().unwrap(),
        ])
        .output()?;
    if output.status.success() {
        println!("Vault reset successfully");
    } else {
        println!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
    Ok(())
}

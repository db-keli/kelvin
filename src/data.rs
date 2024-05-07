use crate::{admin, deckdata};
use std::process::Command;

use admin::Admin;
use std::fs::{read_dir, read_to_string};

static VAULT_PATH: &str = "/etc/.vault";


pub fn check_file_exists(username: &str, directory_path: &str) -> bool {
    if let Ok(entries) = read_dir(directory_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(filename) = entry.file_name().to_str() {
                    if filename == username {
                        return true;
                    }
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
    let file_path = format!("./.vault/{}.json", domain);
    if let Ok(file_content) = read_to_string(file_path) {
        if let Ok(deck_data) = serde_json::from_str(&file_content) {
            return Some(deck_data);
        }
    }
    None
}

/// Encrypt the directory where encrypted password info lives
pub fn encrypt_directory() -> std::io::Result<()> {
    //! This function was specifically made to encrypt ./vault.tar.gz directory where encrypted data is stored
    //! 
    //! ## Usage
    //! ```
    //!     let _ = encrypt_directory()
    //! ```
    
    println!("Locking data.....");
    let output = Command::new("tar")
        .arg("-czvf")
        .arg("/etc/.vault.tar.gz")
        .arg(VAULT_PATH)
        .output()?;

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        println!("{}", s);
        let output2 = Command::new("gpg")
            .args(["-c", "--no-use-agent", "/etc/.vault.tar.gz", "-y"])
            .output()?;
        if !output.status.success() {
            let s = String::from_utf8_lossy(&output2.stderr);
            println!("Error: {}", s);
        } else {
            let _ = Command::new("rm")
                .args(["-rf", "/etc/.vault", "/etc/.vault.tar.gz"])
                .output()?;
        }
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        println!("Error: {}", s);
    }
    Ok(())
}

/// Decrypts the directory where all the encrypted data is stored 
pub fn decrypt_directory() -> std::io::Result<()> {
    //! This function was specifically made to decrypt ./vault.tar.gz directory where encrypted data is stored
    //! 
    //! ## Usage
    //! ```
    //!     let _ = decrypt_directory()
    //! ```

    let output = Command::new("gpg").arg("/etc/.vault.tar.gz.gpg").output()?;
    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        println!("{}", s);

        let output2 = Command::new("tar")
            .args(["-xf", "/etc/.vault.tar.gz"])
            .output()?;
        if output2.status.success() {
            let s = String::from_utf8_lossy(&output2.stdout);
            println!("{}", s);

            let _ = Command::new("rm").args(["-rf", "/etc/.vault.tar.gz"]).output()?;
        } else {
            let s = String::from_utf8_lossy(&output.stderr);
            println!("{}", s);
        }
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        println!("{}", s);
    }

    Ok(())
}

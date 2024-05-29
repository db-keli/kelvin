use crate::{admin, deckdata, prompt};
use std::process::Command;
use std::path::Path;

use prompt::vault_path;
use admin::Admin;
use std::fs::{read_dir, read_to_string};

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
    let file_path = format!("{}/{}.json",vault_path(),domain);
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
        .arg(format!("{}.tar.gz",vault_path()))
        .arg(vault_path())
        .output()?;

    let file_path = format!("{}.tar.gz.gpg",vault_path());
    // Check if the file exists and delete it to avoid prompt
    if Path::new(file_path.as_str()).exists() {
        std::fs::remove_file(file_path)?;
    }


    if output.status.success() {
        let output2 = Command::new("gpg")
            .args(["-c", "--no-use-agent", format!("{}.tar.gz", vault_path()).as_str()])
            .output()?;
        if !output.status.success() {
            let s = String::from_utf8_lossy(&output2.stderr);
            println!("Error: {}", s);
        } else {
            let _ = Command::new("rm")
                .args(["-rf", vault_path().as_str(), format!("{}.tar.gz",vault_path()).as_str()])
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

    let output = Command::new("gpg").arg(format!("{}.tar.gz.gpg",vault_path()).as_str()).output()?;
    if output.status.success() {
        let output2 = Command::new("tar")
            .args(["-xf", format!("{}.tar.gz",vault_path()).as_str()])
            .output()?;
        if output2.status.success() {
            let _ = Command::new("rm")
                .args(["-rf", format!("{}.tar.gz",vault_path()).as_str()])
                .output()?;
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

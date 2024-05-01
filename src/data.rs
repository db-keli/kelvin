use crate::{admin, deckdata};
use std::process::Command;

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
    let file_path = format!("./.vault/{}.json", domain);
    if let Ok(file_content) = read_to_string(file_path) {
        if let Ok(deck_data) = serde_json::from_str(&file_content) {
            return Some(deck_data);
        }
    }
    None
}

//function to encrypt the directory
pub fn encrypt_directory() -> std::io::Result<()> {
    println!("Locking data.....");
    let output = Command::new("tar")
        .arg("-czvf")
        .arg(".vault.tar.gz")
        .arg(".vault")
        .output()?;

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        println!("{}", s);
        let output2 = Command::new("gpg").args(["-c", ".vault.tar.gz"]).output()?;
        if !output.status.success() {
            let s = String::from_utf8_lossy(&output2.stderr);
            println!("Error: {}", s);
        } else {
            let _ = Command::new("rm")
                .args(["-rf", ".vault", ".vault.tar.gz"])
                .output()?;
        }
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        println!("Error: {}", s);
    }
    Ok(())
}

//function to the decrypt the directory

pub fn decrypt_directory() -> std::io::Result<()> {
    let output = Command::new("gpg").arg(".vault.tar.gz.gpg").output()?;
    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        println!("{}", s);

        let output2 = Command::new("tar")
            .args(["-xf", ".vault.tar.gz"])
            .output()?;
        if output2.status.success() {
            let s = String::from_utf8_lossy(&output2.stdout);
            println!("{}", s);

            let _ = Command::new("rm").args(["-rf", ".vault.tar.gz"]).output()?;
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

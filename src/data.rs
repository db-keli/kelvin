use crate::{admin, deckdata};


use admin::Admin;
use std::fs::{read_dir, read_to_string};

pub fn check_file_exists(username: &str, directory_path: &str) -> bool {
    println!("got here");
    if let Ok(entries) = read_dir(directory_path) {
        println!("GOtcha");
        for entry in entries {
            println!("haha");
            if let Ok(entry) = entry {
                println!("Got here");
                if let Some(filename) = entry.file_name().to_str() {
                    println!("{}", filename.to_string());
                    if filename == username {
                        println!("{}", username);
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
    let file_path = format!("./data/{}.json", domain);
    if let Ok(file_content) = read_to_string(file_path) {
        if let Ok(deck_data) = serde_json::from_str(&file_content) {
            return Some(deck_data);
        }
    }
    None
}
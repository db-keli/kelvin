use admin::admin::Admin;
use rand::thread_rng;
use rand::Rng;
use std::io::{Result, stdout, Write, stdin};
use std::fs::{read_dir, read_to_string};
pub mod admin;
pub mod deck;
pub mod deckdata;
use rpassword;

//Generate Password
pub fn generate_password(length: usize) -> String {
    let ascii_chars: Vec<char> = (33..=126).map(|c| c as u8 as char).collect();
    let mut rng = thread_rng();
    let mut password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..ascii_chars.len());
            ascii_chars[idx]
        })
        .collect();

    let mut password_vector: Vec<char> = password.chars().collect();
    let mut k = length;
    while k > 1 {
        let i = rng.gen_range(0..length);
        k -= 1;
        password_vector.swap(k, i);
    }

    password = password_vector.iter().collect();
    password
}


pub fn prompt_password(prompt: &str) -> Result<String> {
    let _ = stdout().flush();

    print!("{}", prompt);
    stdout().flush()?;

    let password = rpassword::read_password()?;
    println!();

    Ok(password)
}

pub fn prompt_deck() -> Result<(String, String)>{
    let _ = stdout().flush();

    print!("Enter domain:");
    stdout().flush()?;

    let mut username = String::new();
    stdin()
        .read_line(&mut username)
        .expect("Failed to read line");
    username = username.trim().to_string();
    print!("Enter password:");
    stdout().flush()?;
    let password = rpassword::read_password()?;
    print!("");
    println!(); 
    
    Ok((username, password))
}

pub fn prompt_logins() -> Result<(String, String)> {
    let _ = stdout().flush();

    print!("Enter username:");
    stdout().flush()?;

    let mut username = String::new();
    stdin()
        .read_line(&mut username)
        .expect("Failed to read line");
    username = username.trim().to_string();
    print!("Enter password:");
    stdout().flush()?;
    let password = rpassword::read_password()?;
    print!("");
    println!();

    Ok((username, password))
}

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
use std::path::{Path, PathBuf};

pub fn check_file_exists1(username: &str, directory_path: &str) -> bool {
    let mut path = PathBuf::from(directory_path);
    path.push(username);

    Path::new(&path).exists()
}

pub fn read_user_data(username: &str, directory_path: &str) -> Option<Admin> {
    // Construct the file path
    let file_path = format!("{}/{}", directory_path, username);

    // Check if the file exists
    if check_file_exists(username, directory_path) {
        // Read the file content
        if let Ok(file_content) = read_to_string(&file_path) {
            // Deserialize the file content into a UserData struct
            if let Ok(user_data) = serde_json::from_str(&file_content) {
                return Some(user_data); // Return the deserialized struct
            }
        }
    }

    None // Return None if file doesn't exist or deserialization fails
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn valid_password_generation() {
        let ascii_chars: Vec<char> = (33..=126).map(|c| c as u8 as char).collect();
        let length = 7;
        let password_vector: Vec<char> = generate_password(length).chars().collect();

        assert!(password_vector.iter().all(|&x| ascii_chars.contains(&x)));
    }
}

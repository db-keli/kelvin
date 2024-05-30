/// src/admin.rs
///
///
use bcrypt::verify;
use bcrypt::{hash, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Result, Write};
use  crate::prompt::vault_path;
use crate::data::{decrypt_directory, encrypt_directory};

#[derive(Serialize, Deserialize, Debug)]
#[warn(dead_code)]
pub struct Admin {
    pub username: String,
    pub password: String,
}

#[warn(dead_code)]
impl Admin {
    pub fn new(name: &str, pass: &str) -> Admin {
        let username = name.to_string();
        let password = pass.to_string();

        Admin { username, password }
    }

    pub fn hash_password(&mut self) {
        let hashed_password = hash(&self.password, DEFAULT_COST).expect("Failed to hash password");

        self.password = hashed_password;
    }

    pub fn verify_password(&self, input_password: &str) -> bool {
        matches!(verify(input_password, &self.password), Ok(true))
    }

    pub fn save_to_json(&self) -> Result<()> {
        let contents = serde_json::to_string(&self)?;
        
        let home_dir = std::env::var("HOME").expect("Unable to get home directory");
        let vault_path_temp = std::path::PathBuf::from(home_dir).join(".vault");

        if !vault_path_temp.exists() {
            std::fs::create_dir_all(&vault_path_temp)?;
        }

        let filepath = format!("{}/{}.json",vault_path(), self.username);

        let mut file = File::create(filepath)?;
        writeln!(file, "{}", contents)?;
        file.flush()?;
        encrypt_directory().unwrap();
        Ok(())
    }

    pub fn read_data_from_json(&self) -> Result<Admin> {
        let filepath = format!("{}/{}.json",vault_path(), self.username);
        let _ = decrypt_directory();
        let mut file = File::open(filepath)?;
        let mut json_data = String::new();
        file.read_to_string(&mut json_data)?;
        file.flush()?;

        let admin_data: Admin = serde_json::from_str(&json_data)?;

        Ok(admin_data)
    }

    pub fn prompt_auth(&self, username: String, password: String) -> Result<bool> {
        let temp_admin = self.read_data_from_json().unwrap();

        if temp_admin.username == username && self.verify_password(&password) {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

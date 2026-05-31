use bcrypt::verify;
use bcrypt::{hash, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind, Result};

use crate::data::{decrypt_vault, encrypt_vault, read_vault, write_vault};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[warn(dead_code)]
pub struct Admin {
    pub username: String,
    pub password: String,
}

#[warn(dead_code)]
impl Admin {
    pub fn new(name: &str, password: &str) -> Admin {
        let username = name.to_string();
        let password = password.to_string();

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
        let _ = decrypt_vault();
        let mut vault = read_vault();
        vault.admin = Some(self.clone());
        write_vault(&vault)?;
        encrypt_vault().unwrap();
        Ok(())
    }

    pub fn read_data_from_json(&self) -> Result<Admin> {
        let _ = decrypt_vault();
        let vault = read_vault();
        let _ = encrypt_vault();
        vault
            .admin
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "No such file or directory"))
    }

    pub fn prompt_auth(&self, username: String, password: String) -> Result<bool> {
        let temp_admin = self.read_data_from_json().unwrap();

        if temp_admin.username == username && self.verify_password(&password) {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn test_save_to_json(&self) -> Result<()> {
        let mut vault = read_vault();
        vault.admin = Some(self.clone());
        write_vault(&vault)?;
        Ok(())
    }

    pub fn test_read_data_from_json(&self) -> Result<Admin> {
        let vault = read_vault();
        vault
            .admin
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "No such file or directory"))
    }
}

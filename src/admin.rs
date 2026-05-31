use bcrypt::verify;
use bcrypt::{hash, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind, Result};

use crate::data::{read_vault, write_vault};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Admin {
    pub username: String,
    pub password: String,
}

impl Admin {
    pub fn new(name: &str, password: &str) -> Admin {
        Admin {
            username: name.to_string(),
            password: password.to_string(),
        }
    }

    pub fn hash_password(&mut self) {
        self.password = hash(&self.password, DEFAULT_COST).expect("Failed to hash password");
    }

    pub fn verify_password(&self, input_password: &str) -> bool {
        matches!(verify(input_password, &self.password), Ok(true))
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
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "No admin found"))
    }
}

use bcrypt::verify;
use bcrypt::{hash, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{stdin, stdout, Error, ErrorKind, Read, Result, Write};
//Admin Account Boilerplate

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

    //Could be generic
    pub fn hash_password(&mut self) {
        let hashed_password = hash(&self.password, DEFAULT_COST).expect("Failed to hash password");

        self.password = hashed_password;
    }

    //Could be generic
    pub fn verify_password(&self, input_password: &str) -> bool {
        matches!(verify(input_password, &self.password), Ok(true))
    }

    pub fn save_to_json(&self) -> Result<()> {
        let contents = serde_json::to_string(&self)?;
        let filepath = format!("./data/{}.json", self.username);

        let mut file = File::create(filepath)?;
        writeln!(file, "{}", contents)?;
        file.flush()?;

        Ok(())
    }

    pub fn read_data_from_json(&self) -> Result<Admin> {
        let filepath = format!("./data/{}.json", self.username);
        let mut file = File::open(filepath)?;
        let mut json_data = String::new();
        file.read_to_string(&mut json_data)?;
        file.flush()?;

        println!("{}", json_data);
        let deck_data_vec: Vec<Admin> = serde_json::from_str(&json_data)?;

        // Extract the first item from the vector (assuming it contains only one item)
        let deck_data = deck_data_vec
            .into_iter()
            .next()
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "No data found in JSON"))?;

        Ok(deck_data)
    }

    pub fn prompt_auth(&self) -> Result<(String, String, bool)> {
        let _ = stdout().flush();

        print!("Enter username:");
        stdout().flush()?;

        let mut username = String::new();
        stdin()
            .read_line(&mut username)
            .expect("Failed to read line");

        print!("Enter password:");
        stdout().flush()?;
        let password = rpassword::read_password()?;
        print!("");

        Ok((username, password, true))
    }
}

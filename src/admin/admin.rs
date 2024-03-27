use bcrypt::verify;
use bcrypt::{hash, DEFAULT_COST};

//Admin Account Boilerplate
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
        let hashed_password =
            hash(&self.password, DEFAULT_COST).expect("Failed to hash password");

        self.password = hashed_password;
    }

    //Could be generic
    pub fn verify_password(&self, input_password: &str) -> bool {
        matches!(verify(input_password, &self.password), Ok(true))
    }
}


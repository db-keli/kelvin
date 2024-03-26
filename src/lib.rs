use bcrypt::BcryptError;
use bcrypt::{hash, verify, DEFAULT_COST};
use rand::thread_rng;
use rand::Rng;

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
    pub fn hash_password(&self) -> Result<String, BcryptError> {
        hash(&self.password, DEFAULT_COST)
    }

    //Could be generic
    pub fn verify_password(&self, input_password: String) -> Result<bool, BcryptError> {
        let p = self.hash_password().expect("Failed to hash password");

        verify(p, &input_password)

        //.expect("Failed to hash password");
        //match verify(p, &input_password) {
        //    Ok(true) => "Access Accepted",
        //    Ok(false) => "Access Denied",
        //    Err(_) => "There's a failure somewhere",
        // }
    }
}

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

//Add a deck
pub struct Deck {
    pub domain: String,
    pub plaintext: String,
}

impl Deck {
    pub fn new(domain: &str, plaintext: &str) -> Deck {
        let domain = domain.to_string();
        let plaintext = plaintext.to_string();

        Deck { domain, plaintext }
    }
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

    #[test]
    fn constructor_valid() {
        let password_test: String = generate_password(12);
        let name: String = String::from("Michael");
        let admin: Admin = Admin::new(&name, &password_test);
        assert_eq!(admin.password, password_test);
    }

    //#[test]
    //fn test_verify_function() {
    //    let password_test: String = generate_password(12);
    //    let name: String = String::from("Michael");
    //    let admin: Admin = Admin::new(&name, &password_test);
    //    let _ = admin.hash_password();
    //    let _input_password: String = String::from("342323423884324");

    //    let end = admin.verify_password(password_test);

    //   assert!(end.contains("There's a failure somewhere"));
    //}
}

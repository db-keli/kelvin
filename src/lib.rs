use rand::thread_rng;
use rand::Rng;

#[warn(dead_code)]
pub struct Admin {
    pub username: String,
    pub password: String,
}

impl Admin {
    pub fn _new(name: &str, pass: &str) -> Admin {
        let username = name.to_string();
        let password = pass.to_string();

        Admin { username, password }
    }
}

#[warn(dead_code)]
impl Admin {
    pub fn hash_password(&self, pass: String) -> String {
        pass
    }
}

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid_password() {
        let ascii_chars: Vec<char> = (33..=126).map(|c| c as u8 as char).collect();
        let length = 7;
        let password_vector: Vec<char> = generate_password(length).chars().collect();

        assert!(password_vector.iter().all(|&x| ascii_chars.contains(&x)));
    }

    #[test]
    fn constructor_valid() {
        let password_test = generate_password(12);
        let name_test = String::from("Michael");
        let admin = Admin::_new(&name_test, &password_test);
        assert_eq!(admin.password, password_test);
    }
}

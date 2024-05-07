use rpassword;
use std::io::{stdin, stdout, Result, Write};
use std::fs;
use std::path::Path;

static VAULT_PATH: &str = "/etc/.vault";

pub fn prompt_deck() -> Result<(String, String)> {
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

pub fn initialize_vault() -> Result<()> {
    let path = Path::new(VAULT_PATH);
    if !path.exists(){
        fs::create_dir(VAULT_PATH)?;
    }
    Ok(())
}
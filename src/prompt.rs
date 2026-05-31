use clipboard::{ClipboardContext, ClipboardProvider};
use rpassword;
use std::fs;
use std::io::{stdin, stdout, Result, Write};
use std::thread;
use std::time::Duration;

use dirs;
use std::path::PathBuf;

pub fn vault_path() -> PathBuf {
    let home_dir = dirs::home_dir().expect("Unable to find home directory");
    home_dir.join(".vault.json")
}

pub fn vault_encrypted_path() -> PathBuf {
    let home_dir = dirs::home_dir().expect("Unable to find home directory");
    home_dir.join(".vault.json.gpg")
}

pub fn prompt_deck() -> Result<(String, String)> {
    let _ = stdout().flush();

    print!("Enter domain:");
    stdout().flush()?;

    let mut domain = String::new();
    stdin().read_line(&mut domain).expect("Failed to read line");
    domain = domain.trim().to_string();

    print!("Enter the domain's password:");
    stdout().flush()?;
    let password = rpassword::read_password()?;
    println!();

    Ok((domain, password))
}

pub fn prompt_deck_open_sesame() -> Result<String> {
    let _ = stdout().flush();

    print!("Enter domain:");
    stdout().flush()?;

    let mut domain = String::new();
    stdin().read_line(&mut domain).expect("Failed to read line");
    domain = domain.trim().to_string();
    println!();

    Ok(domain)
}

pub fn prompt_logins() -> Result<(String, String)> {
    let _ = stdout().flush();

    print!("Enter admin username:");
    stdout().flush()?;

    let mut username = String::new();
    stdin().read_line(&mut username).expect("Failed to read line");
    username = username.trim().to_string();

    print!("Enter admin password:");
    stdout().flush()?;
    let password = rpassword::read_password()?;
    print!("");
    println!();

    Ok((username, password))
}

pub fn prompt_notes() -> Result<Option<String>> {
    print!("Enter notes (hostname, port, user, etc.) [optional]:");
    stdout().flush()?;

    let mut notes = String::new();
    stdin().read_line(&mut notes).expect("Failed to read line");
    let notes = notes.trim().to_string();

    if notes.is_empty() {
        Ok(None)
    } else {
        Ok(Some(notes))
    }
}

pub fn prompt_env_var() -> Result<String> {
    print!("Enter variable name:");
    stdout().flush()?;

    let mut var = String::new();
    stdin().read_line(&mut var).expect("Failed to read line");

    Ok(var.trim().to_string())
}

pub fn initialize_vault() -> Result<()> {
    let path = vault_path();
    let encrypted_path = vault_encrypted_path();
    if !path.exists() && !encrypted_path.exists() {
        fs::write(&path, r#"{"admin":null,"decks":[]}"#)?;
    }
    Ok(())
}

pub fn clip(text: &str) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(text.to_owned()).unwrap();
    println!("Clearing clipboard in 30 seconds...");
    thread::sleep(Duration::from_secs(30));
    ctx.set_contents(String::new()).unwrap();
}

use clipboard::{ClipboardContext, ClipboardProvider};
use rpassword;
use std::fs;
use std::io::{stdin, stdout, Result, Write};
use std::thread;
use std::time::Duration;

use dirs;
use std::env;
use std::path::PathBuf;

pub fn vault_path() -> PathBuf {
    let home_dir = dirs::home_dir().expect("Unable to find home directory");
    home_dir.join(".vault")
}

pub fn prompt_deck() -> Result<(String, String)> {
    let _ = stdout().flush();

    print!("Enter domain:");
    stdout().flush()?;

    let mut username = String::new();
    stdin()
        .read_line(&mut username)
        .expect("Failed to read line");
    username = username.trim().to_string();
    print!("Enter the domain's password:");
    stdout().flush()?;
    let password = rpassword::read_password()?;
    println!();

    Ok((username, password))
}

pub fn prompt_deck_open_sesame() -> Result<String> {
    let _ = stdout().flush();

    print!("Enter domain:");
    stdout().flush()?;

    let mut username = String::new();
    stdin()
        .read_line(&mut username)
        .expect("Failed to read line");
    username = username.trim().to_string();
    println!();

    Ok(username)
}

pub fn prompt_logins() -> Result<(String, String)> {
    let _ = stdout().flush();

    print!("Enter admin username:");
    stdout().flush()?;

    let mut username = String::new();
    stdin()
        .read_line(&mut username)
        .expect("Failed to read line");
    username = username.trim().to_string();

    print!("Enter admin password:");
    stdout().flush()?;
    let password = rpassword::read_password()?;
    print!("");
    println!();

    Ok((username, password))
}

pub fn initialize_vault() -> Result<()> {
    let path = vault_path();
    if !path.exists() {
        fs::create_dir(path)?;
    }
    Ok(())
}

pub fn clip(text: &str) -> () {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    ctx.set_contents(text.to_owned()).unwrap();
    thread::sleep(Duration::from_secs(2));
    return;
}

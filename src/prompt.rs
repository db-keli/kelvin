use crate::admin::VAULT_PATH;
use clipboard::{ClipboardContext, ClipboardProvider};
use rpassword;
use std::env;
use std::fs;
use std::io::{stdin, stdout, Result, Write};
use std::path::Path;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

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
    let vault = vault_path();
    let path = Path::new(vault.as_str());
    if !path.exists() {
        fs::create_dir(vault)?;
    }
    Ok(())
}

pub fn clip(text: &str) -> () {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    ctx.set_contents(text.to_owned()).unwrap();
    thread::sleep(Duration::from_secs(2));
    return;
}

#[allow(deprecated)]
pub fn vault_path() -> String {
    // let home = env::home_dir()
    //     .unwrap()
    //     .to_str()
    //     .unwrap()
    //     .to_string()
    //     .trim()
    //     .to_string();

    let home_dir = env::var("HOME").expect("Unable to get home directory");
    let vault = PathBuf::from(home_dir).join(VAULT_PATH);

    return vault.to_str().unwrap().to_string();
}

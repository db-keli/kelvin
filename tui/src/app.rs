use crossterm::event::{KeyCode, KeyEvent};
use kelvin_rs::{
    admin::Admin,
    data::{load_vault, save_vault, VaultFile},
    deck::Deck,
    deckdata::DeckData,
    password::generate_password,
    prompt::vault_encrypted_path,
};
use clipboard::ClipboardProvider;

use crate::input::InputField;

pub enum Screen {
    MasterPassword,
    CreateMasterPassword,
    AdminLogin,
    Main,
    AddDeck,
    ViewDeck,
    ConfirmDelete,
    GeneratePassword,
}

pub struct App {
    pub screen: Screen,
    pub should_quit: bool,

    pub vault: VaultFile,
    pub master_password: String,
    pub admin: Option<Admin>,

    pub selected_index: usize,

    pub input: InputField,
    pub domain_input: InputField,
    pub password_input: InputField,
    pub notes_input: InputField,
    pub username_input: InputField,
    pub admin_pass_input: InputField,

    pub active_form_field: usize,

    pub generated_password: String,
    pub decrypted_password: String,
    pub selected_deck_domain: String,

    pub message: Option<String>,
    pub error: Option<String>,
}

impl App {
    pub fn new() -> Self {
        let screen = if vault_encrypted_path().exists() {
            Screen::MasterPassword
        } else {
            Screen::CreateMasterPassword
        };

        App {
            screen,
            should_quit: false,
            vault: VaultFile::default(),
            master_password: String::new(),
            admin: None,
            selected_index: 0,
            input: InputField::new(true),
            domain_input: InputField::new(false),
            password_input: InputField::new(true),
            notes_input: InputField::new(false),
            username_input: InputField::new(false),
            admin_pass_input: InputField::new(true),
            active_form_field: 0,
            generated_password: String::new(),
            decrypted_password: String::new(),
            selected_deck_domain: String::new(),
            message: None,
            error: None,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        self.message = None;
        self.error = None;

        match self.screen {
            Screen::MasterPassword => self.handle_master_password_key(key),
            Screen::CreateMasterPassword => self.handle_create_master_password_key(key),
            Screen::AdminLogin => self.handle_admin_login_key(key),
            Screen::Main => self.handle_main_key(key),
            Screen::AddDeck => self.handle_add_deck_key(key),
            Screen::ViewDeck => self.handle_view_deck_key(key),
            Screen::ConfirmDelete => self.handle_confirm_delete_key(key),
            Screen::GeneratePassword => self.handle_generate_password_key(key),
        }
    }

    fn handle_master_password_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                let pw = self.input.value.clone();
                if pw.is_empty() {
                    self.error = Some("Password cannot be empty".to_string());
                    return;
                }
                match load_vault(&pw) {
                    Ok(vault) => {
                        self.vault = vault;
                        self.master_password = pw;
                        if self.vault.admin.is_some() {
                            self.screen = Screen::AdminLogin;
                        } else {
                            self.screen = Screen::Main;
                            self.error = Some("No admin account set up. Use create-admin CLI first.".to_string());
                        }
                    }
                    Err(e) => {
                        self.error = Some(format!("Failed to open vault: {}", e));
                    }
                }
            }
            KeyCode::Esc => self.should_quit = true,
            _ => self.input.handle_key(key),
        }
    }

    fn handle_create_master_password_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                let pw = self.input.value.clone();
                if pw.is_empty() {
                    self.error = Some("Password cannot be empty".to_string());
                    return;
                }
                let vault = VaultFile::default();
                match save_vault(&vault, &pw) {
                    Ok(()) => {
                        self.vault = vault;
                        self.master_password = pw;
                        self.screen = Screen::AdminLogin;
                        self.message = Some("New vault created! Enter admin credentials.".to_string());
                    }
                    Err(e) => {
                        self.error = Some(format!("Failed to create vault: {}", e));
                    }
                }
            }
            KeyCode::Esc => self.should_quit = true,
            _ => self.input.handle_key(key),
        }
    }

    fn handle_admin_login_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Tab | KeyCode::Down => {
                self.active_form_field = 1;
            }
            KeyCode::Up => {
                self.active_form_field = 0;
            }
            KeyCode::Enter => {
                let username = self.username_input.value.clone();
                let password = self.admin_pass_input.value.clone();
                if username.is_empty() || password.is_empty() {
                    self.error = Some("Both fields are required".to_string());
                    return;
                }

                let stored = match &self.vault.admin {
                    Some(a) => a.clone(),
                    None => {
                        let mut admin = Admin::new(&username, &password);
                        admin.hash_password();
                        self.vault.admin = Some(admin);
                        if let Err(e) = save_vault(&self.vault, &self.master_password) {
                            self.error = Some(format!("Failed to save admin: {}", e));
                            return;
                        }
                        self.message = Some("Admin created successfully!".to_string());
                        self.screen = Screen::Main;
                        return;
                    }
                };

                if stored.username == username && stored.verify_password(&password) {
                    self.admin = Some(stored);
                    self.screen = Screen::Main;
                } else {
                    self.error = Some("Invalid admin credentials".to_string());
                    self.admin_pass_input.clear();
                }
            }
            KeyCode::Esc => {
                self.screen = if vault_encrypted_path().exists() {
                    Screen::MasterPassword
                } else {
                    Screen::CreateMasterPassword
                };
            }
            _ => {
                if self.active_form_field == 0 {
                    self.username_input.handle_key(key);
                } else {
                    self.admin_pass_input.handle_key(key);
                }
            }
        }
    }

    fn handle_main_key(&mut self, key: KeyEvent) {
        let len = self.vault.decks.len();

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Up | KeyCode::Char('k') => {
                if len > 0 {
                    self.selected_index = self.selected_index.saturating_sub(1);
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if len > 0 && self.selected_index + 1 < len {
                    self.selected_index += 1;
                }
            }
            KeyCode::Home => self.selected_index = 0,
            KeyCode::End => {
                if len > 0 {
                    self.selected_index = len - 1;
                }
            }
            KeyCode::Enter | KeyCode::Char('v') => {
                if !self.vault.decks.is_empty() {
                    let deck = &self.vault.decks[self.selected_index];
                    self.selected_deck_domain = deck.domain.clone();
                    self.decrypted_password =
                        String::from_utf8(deck.decrypt()).unwrap_or_default();
                    self.screen = Screen::ViewDeck;
                }
            }
            KeyCode::Char('a') => {
                self.domain_input.clear();
                self.password_input.clear();
                self.notes_input.clear();
                self.active_form_field = 0;
                self.screen = Screen::AddDeck;
            }
            KeyCode::Char('d') => {
                if !self.vault.decks.is_empty() {
                    self.screen = Screen::ConfirmDelete;
                }
            }
            KeyCode::Char('g') => {
                self.generated_password = generate_password(16);
                self.screen = Screen::GeneratePassword;
            }
            _ => {}
        }
    }

    fn handle_add_deck_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Tab => {
                self.active_form_field = (self.active_form_field + 1) % 3;
            }
            KeyCode::Up => {
                self.active_form_field = if self.active_form_field == 0 {
                    2
                } else {
                    self.active_form_field - 1
                };
            }
            KeyCode::Down => {
                self.active_form_field = (self.active_form_field + 1) % 3;
            }
            KeyCode::Enter => {
                let domain = self.domain_input.value.clone();
                let password = self.password_input.value.clone();
                if domain.is_empty() || password.is_empty() {
                    self.error = Some("Domain and password are required".to_string());
                    return;
                }
                let notes = if self.notes_input.value.is_empty() {
                    None
                } else {
                    Some(self.notes_input.value.clone())
                };

                let admin = match &self.admin {
                    Some(a) => a.clone(),
                    None => {
                        self.error = Some("No admin credentials".to_string());
                        return;
                    }
                };

                let deck = Deck::new(&domain, &password);
                let encrypted = deck.encrypt();
                let deck_data = DeckData::new(
                    admin,
                    deck.domain.clone(),
                    encrypted.0,
                    encrypted.1 .1,
                    encrypted.1 .0,
                    notes,
                );

                if let Some(pos) = self
                    .vault
                    .decks
                    .iter()
                    .position(|d| d.domain == deck.domain)
                {
                    self.vault.decks[pos] = deck_data;
                } else {
                    self.vault.decks.push(deck_data);
                }

                if let Err(e) = save_vault(&self.vault, &self.master_password) {
                    self.error = Some(format!("Failed to save: {}", e));
                    return;
                }

                self.message = Some("Deck saved successfully!".to_string());
                self.screen = Screen::Main;
            }
            KeyCode::Esc => {
                self.screen = Screen::Main;
            }
            _ => match self.active_form_field {
                0 => self.domain_input.handle_key(key),
                1 => self.password_input.handle_key(key),
                2 => self.notes_input.handle_key(key),
                _ => {}
            },
        }
    }

    fn handle_view_deck_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc | KeyCode::Enter => {
                self.screen = Screen::Main;
            }
            KeyCode::Char('c') => {
                self.copy_to_clipboard(&self.decrypted_password);
                self.message = Some("Password copied to clipboard!".to_string());
            }
            _ => {}
        }
    }

    fn handle_confirm_delete_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('y') | KeyCode::Enter => {
                let domain = self.selected_deck_domain.clone();
                self.vault.decks.retain(|d| d.domain != domain);
                if let Err(e) = save_vault(&self.vault, &self.master_password) {
                    self.error = Some(format!("Failed to save: {}", e));
                } else {
                    self.message = Some(format!("Deleted {}", domain));
                }
                if self.selected_index >= self.vault.decks.len() && !self.vault.decks.is_empty() {
                    self.selected_index = self.vault.decks.len() - 1;
                }
                self.screen = Screen::Main;
            }
            KeyCode::Char('n') | KeyCode::Esc => {
                self.screen = Screen::Main;
            }
            _ => {}
        }
    }

    fn handle_generate_password_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('g') => {
                self.generated_password = generate_password(16);
            }
            KeyCode::Char('c') => {
                self.copy_to_clipboard(&self.generated_password);
                self.message = Some("Password copied to clipboard!".to_string());
            }
            KeyCode::Esc => {
                self.screen = Screen::Main;
            }
            _ => {}
        }
    }

    fn copy_to_clipboard(&self, text: &str) {
        if let Ok(mut ctx) = clipboard::ClipboardContext::new() {
            let _: Result<(), Box<dyn std::error::Error>> = ctx.set_contents(text.to_owned());
        }
    }
}

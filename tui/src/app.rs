use kelvin::{
    admin::Admin,
    password::generate_password,
};

enum Mainmenu {
    Mainmenu,
    GenPasswd,
    CreateAdmin,
    VerifyAdmin,
    AddDeck,
    OpenSesame,
    Reset,
}

struct GenPasswd {
    length: Option<usize>,
}

pub struct App {
    pub popup: bool,
    pub mainmenu: Option<Mainmenu>,
}

impl App {
    pub fn new() -> Self {
        Self {
            popup: true,
            mainmenu: None,
        }
    }
    pub fn main_menu(&mut self) {
        self.popup = false;
        self.mainmenu = Some(Mainmenu::Mainmenu);
    }
    pub fn gen_passwd(&mut self) {
        self.popup = false;
        self.mainmenu = Some(Mainmenu::GenPasswd);
    }
    pub fn create_admin(&mut self) {
        self.popup = false;
        self.mainmenu = Some(Mainmenu::CreateAdmin);
    }
    pub fn verify_admin(&mut self) {
        self.popup = false;
        self.mainmenu = Some(Mainmenu::VerifyAdmin);
    }
    pub fn create_deck(&mut self) {
        self.popup = false;
        self.mainmenu = Some(Mainmenu::AddDeck);
    } 
    pub fn check_deck_contents(&mut self) {
        self.popup = false;
        self.mainmenu = Some(Mainmenu::OpenSesame);
    }
    pub fn reset_vault(&mut self) {
        self.popup = false;
        self.mainmenu = Some(Mainmenu::Reset);
    }
}

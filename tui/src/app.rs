use kelvin::admin::Admin;
use crate::ui::get_admin_details;

enum MainmenuState {
    GenPasswd,
    CreateAdmin,
    AddDeck,
    OpenSesame,
    Reset,
    Exit,
}

struct GenPasswd {
    length: Option<usize>,
}

enum StartupState {
    Initialize,
    CheckAdmin,
}

pub struct App {
    pub startup: StartupState,
    pub mainmenu: MainmenuState,
    pub admin: Option<Admin>,
    pub gen_passwd: Option<GenPasswd>,
}

impl App {
    pub fn new() -> App {
        App {
            startup: StartupState::Initialize,
            mainmenu: MainmenuState::GenPasswd,
            admin: None,
            gen_passwd: None,
        }
    }

    pub fn create_admin(&mut self) {
        let admin = get_admin_details();
        self.admin = Some(admin);
    }

    pub fn gen_passwd(&mut self) {

    }
}
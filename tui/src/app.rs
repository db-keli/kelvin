enum MainmenuState {
    GenPasswd,
    CreateAdmin,
    AddDeck,
    OpenSesame,
    Reset,
}

struct GenPasswd {
    // Define fields needed for generating passwords
    length: usize,
}

enum StartupState {
    Initialize,
    CheckAdmin,
    UnlockVault,
}

pub struct Admin {
    pub username: String,
    pub password: String,
}

pub struct App {
    pub startup: StartupState,
    pub mainmenu: MainmenuState,
    pub admin: Option<Admin>, // Admin details, if any
    pub gen_passwd: Option<GenPasswd>, // State for generating password
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
}

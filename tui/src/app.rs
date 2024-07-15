use kelvin::admin::Admin;

enum Mainmenu {
    GenPasswd,
    CreateAdmin,
    VerifyAdmin,
    AddDeck,
    OpenSesame,
    Reset,
    ExitApp,
}

struct GenPasswd {
    length: Option<usize>,
}

enum Startup {
    WelcomePopUp,
}

pub struct App {
    pub startup: Option<Startup>,
    pub mainmenu: Option<Mainmenu>,
}

impl App {
    pub fn new() -> Self {
        Self {
            startup: Some(Startup::WelcomePopUp),
            mainmenu: None,
        }
    }
    pub fn main_menu(&mut self) {
        self.startup = None;
        self.mainmenu = Some(Mainmenu);
    }

    pub fn gen_passwd(&mut self) {
        self.startup = None;
        self.mainmenu = Some(Mainmenu::GenPasswd);
    }

    pub fn create_admin(&mut self) {
        self.startup = None;
        self.mainmenu = Some(Mainmenu::CreateAdmin);
    }

    pub fn verify_admin(&mut self) {
        self.startup = None;
        self.mainmenu = Some(Mainmenu::VerifyAdmin);
    }

    pub fn create_deck(&mut self) {
        self.startup = None;
        self.mainmenu = Some(Mainmenu::AddDeck);
    } 

    pub fn check_deck_contents(&mut self) {
        self.startup = None;
        self.mainmenu = Some(Mainmenu::OpenSesame);
    }

    pub fn reset_vault(&mut self) {
        self.startup = None;
        self.mainmenu = Some(Mainmenu::Reset);
    }

    pub fn quit_kelvin(&mut self) {
        self.startup = None;
        self.mainmenu = Some(Mainmenu::ExitApp);
    }
}

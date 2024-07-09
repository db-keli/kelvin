enum MainmenuState{
    GenPasswd,
    CreateAdmin,
    AddDeck,
    OpenSesame,
    Reset
}

enum StartupState {
    
}

pub struct Admin {
    pub username: String,
    pub password: String,
}

pub struct  App {
    pub Startup: StartupState,
    pub Mainmenu: MainmenuState,


}
impl App {
    pub fn new() -> App {
        App {
            Startup,
            Mainmenu,
        }
    }
}
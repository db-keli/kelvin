use std::{error::Error, io};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    prelude::*,
    Terminal,
};

mod app;
mod ui;

use crate::{
    app::App,
    ui::ui,
};

fn main() -> Result<(), Box<dyn Error>> {
    // initialize terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    // Switch to the alternate screen and enable mouse capture 
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    // create app and run it
    let mut app = app::App::new();
    let res = run_tui(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {}", err);
    }

    Ok(())
}

///runs the TUI loop.
fn run_tui<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;
        //key.kind accesses the kind of key event,which can indicate whether the key was pressed, released, or repeated
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                //skips events that arent KeyEventKind::Press
                continue;
            } else if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('g') => app.gen_passwd(),
                    KeyCode::Char('a') => app.create_admin(),
                    KeyCode::Char('v') => app.verify_admin(),
                    KeyCode::Char('d') => app.create_deck(),
                    KeyCode::Char('o') => app.check_deck_contents(),
                    KeyCode::Char('r') => app.reset_vault(),
                    KeyCode::Char('q') => app.quit_kelvin(),
            }
        }
    }
}
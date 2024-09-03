use std::{error::Error, io};
use std::{thread, time::Duration};

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
mod ui_management;

use app::App;
use crate::ui_management::components::dbox;
use crate::ui_management::pages::loader::LoaderPage;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let mut app = app::App::new();
    let res = run_tui(&mut terminal, &mut app);

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

fn run_tui<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<(), std::io::Error> {
    let area = terminal.size()?;
    let mut loader_page = LoaderPage::new(area);
    let loading_thread = thread::spawn(move || {
        while loader_page.progress_count < 100 {
            loader_page.update_progress();
            thread::sleep(Duration::from_millis(100));
        }
    });

    loop {
        let mut dbox = Box::new("KELVIN", Rect::new(0, 0, 30, 3));
        terminal.draw(|f| loader_page.render(f))?;
        //if let Event::Key(key) = event::read()? {
            //if key.kind == event::KeyEventKind::Release {
               // continue;
           // } else if key.kind == event::KeyEventKind::Press {
           //     match key.code {
             //       KeyCode::Char('q') => return Ok(()),
               //     _ => {}
                //}
            //}
        //}
        if loader_page.progress_count >= 100 {
            break;
        }
    }
    loading_thread.join().unwrap();
}
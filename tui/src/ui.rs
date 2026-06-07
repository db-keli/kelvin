use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::{App, Screen};

pub fn draw(f: &mut Frame, app: &App) {
    let area = f.size();

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(if app.message.is_some() || app.error.is_some() {
                1
            } else {
                0
            }),
        ])
        .split(area);

    render_title_bar(f, main_layout[0], app);
    render_screen(f, main_layout[1], app);
    render_status_line(f, main_layout[2], app);
}

fn render_title_bar(f: &mut Frame, area: Rect, _app: &App) {
    let title = Line::from(" Kelvin Password Manager ")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);
    f.render_widget(title, area);
}

fn render_status_line(f: &mut Frame, area: Rect, app: &App) {
    if let Some(msg) = &app.message {
        let line = Line::from(Span::styled(
            format!(" {}", msg),
            Style::default().fg(Color::Green),
        ));
        f.render_widget(line, area);
    } else if let Some(err) = &app.error {
        let line = Line::from(Span::styled(
            format!(" {}", err),
            Style::default().fg(Color::Red),
        ));
        f.render_widget(line, area);
    }
}

fn render_screen(f: &mut Frame, area: Rect, app: &App) {
    match app.screen {
        Screen::MasterPassword => render_master_password(f, area, app),
        Screen::CreateMasterPassword => render_create_master_password(f, area, app),
        Screen::AdminLogin => render_admin_login(f, area, app),
        Screen::Main => render_main(f, area, app),
        Screen::AddDeck => render_add_deck(f, area, app),
        Screen::ViewDeck => render_view_deck(f, area, app),
        Screen::ConfirmDelete => render_confirm_delete(f, area, app),
        Screen::GeneratePassword => render_generate_password(f, area, app),
    }
}

fn render_master_password(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Unlock Vault ")
        .border_type(BorderType::Rounded);

    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Length(1)])
        .horizontal_margin(4)
        .vertical_margin(inner.height / 3)
        .split(inner);

    let display = app.input.display();
    let input_style = Style::default().fg(Color::White);
    let input = Paragraph::new(display)
        .style(input_style)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Master Password "),
        );
    f.render_widget(input, chunks[0]);

    let hint = Line::from(" Enter to unlock  |  Esc to quit ")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(hint, chunks[1]);
}

fn render_create_master_password(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" New Vault ")
        .border_type(BorderType::Rounded);

    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Length(1)])
        .horizontal_margin(4)
        .vertical_margin(inner.height / 3)
        .split(inner);

    let display = app.input.display();
    let input = Paragraph::new(display)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Choose a Master Password "),
        );
    f.render_widget(input, chunks[0]);

    let hint = Line::from(" Enter to create vault  |  Esc to quit ")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(hint, chunks[1]);
}

fn render_admin_login(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Admin Login ")
        .border_type(BorderType::Rounded);

    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Length(3), Constraint::Length(1)])
        .horizontal_margin(4)
        .vertical_margin(inner.height / 4)
        .split(inner);

    let username_style = if app.active_form_field == 0 {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };
    let username = Paragraph::new(app.username_input.display())
        .style(username_style)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Username "),
        );
    f.render_widget(username, chunks[0]);

    let pass_style = if app.active_form_field == 1 {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };
    let password = Paragraph::new(app.admin_pass_input.display())
        .style(pass_style)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Password "),
        );
    f.render_widget(password, chunks[1]);

    let hint = Line::from(" Tab/↑↓: Switch field  |  Enter: Login  |  Esc: Back ")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(hint, chunks[2]);
}

fn render_main(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(2)])
        .split(area);

    let empty_text = if app.vault.decks.is_empty() {
        " No entries in vault. Press 'a' to add one."
    } else {
        ""
    };

    let items: Vec<ListItem> = if app.vault.decks.is_empty() {
        vec![ListItem::new(Line::from(Span::styled(
            empty_text,
            Style::default().fg(Color::DarkGray),
        )))]
    } else {
        app.vault
            .decks
            .iter()
            .map(|d| {
                let label = match &d.notes {
                    Some(n) => format!(" {}    {}", d.domain, n),
                    None => format!(" {}", d.domain),
                };
                ListItem::new(Line::from(Span::raw(label)))
            })
            .collect()
    };

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Decks ")
                .border_type(BorderType::Rounded),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    f.render_stateful_widget(
        list,
        chunks[0],
        &mut ratatui::widgets::ListState::default().with_selected(if app.vault.decks.is_empty() {
            None
        } else {
            Some(app.selected_index)
        }),
    );

    let help = Line::from(Span::styled(
        " [a] Add  [d] Delete  [v] View  [g] Generate  [q] Quit ",
        Style::default().fg(Color::DarkGray),
    ))
    .alignment(Alignment::Center);
    f.render_widget(help, chunks[1]);
}

fn render_add_deck(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Add New Deck ")
        .border_type(BorderType::Rounded);

    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .horizontal_margin(4)
        .vertical_margin(inner.height / 5)
        .split(inner);

    let domain_style = if app.active_form_field == 0 {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };
    let domain = Paragraph::new(app.domain_input.display())
        .style(domain_style)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Domain "),
        );
    f.render_widget(domain, chunks[0]);

    let pass_style = if app.active_form_field == 1 {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };
    let password = Paragraph::new(app.password_input.display())
        .style(pass_style)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Password "),
        );
    f.render_widget(password, chunks[1]);

    let notes_style = if app.active_form_field == 2 {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };
    let notes = Paragraph::new(app.notes_input.display())
        .style(notes_style)
        .block(
            Block::default().borders(Borders::ALL).title(" Notes (optional) "),
        );
    f.render_widget(notes, chunks[2]);

    let hint = Line::from(Span::styled(
        " Tab/↑↓: Switch field  |  Enter: Save  |  Esc: Cancel ",
        Style::default().fg(Color::DarkGray),
    ))
    .alignment(Alignment::Center);
    f.render_widget(hint, chunks[3]);
}

fn render_view_deck(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" Deck: {} ", app.selected_deck_domain))
        .border_type(BorderType::Rounded);

    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(3),
            Constraint::Length(1),
        ])
        .horizontal_margin(4)
        .vertical_margin(inner.height / 4)
        .split(inner);

    let password_text = Line::from(Span::styled(
        format!(" {}", app.decrypted_password),
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    ));
    let password = Paragraph::new(password_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Password "),
        );
    f.render_widget(password, chunks[0]);

    let notes = app
        .vault
        .decks
        .iter()
        .find(|d| d.domain == app.selected_deck_domain)
        .and_then(|d| d.notes.as_deref())
        .unwrap_or("");

    if !notes.is_empty() {
        let notes_text = Line::from(Span::raw(format!(" {}", notes)));
        let notes_widget = Paragraph::new(notes_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Notes "),
            );
        f.render_widget(notes_widget, chunks[1]);
    }

    let hint = Line::from(Span::styled(
        " [c] Copy to clipboard  |  [Esc] Back ",
        Style::default().fg(Color::DarkGray),
    ))
    .alignment(Alignment::Center);
    f.render_widget(hint, chunks[2]);
}

fn render_confirm_delete(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Confirm Delete ")
        .border_type(BorderType::Rounded);

    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Length(1)])
        .horizontal_margin(4)
        .vertical_margin(inner.height / 3)
        .split(inner);

    let question = Line::from(Span::styled(
        format!(" Delete \"{}\"? ", app.selected_deck_domain),
        Style::default()
            .fg(Color::Red)
            .add_modifier(Modifier::BOLD),
    ))
    .alignment(Alignment::Center);
    f.render_widget(question, chunks[0]);

    let hint = Line::from(Span::styled(
        " [y] Yes  |  [n] No  |  [Esc] Cancel ",
        Style::default().fg(Color::DarkGray),
    ))
    .alignment(Alignment::Center);
    f.render_widget(hint, chunks[1]);
}

fn render_generate_password(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Generate Password ")
        .border_type(BorderType::Rounded);

    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Length(1)])
        .horizontal_margin(4)
        .vertical_margin(inner.height / 3)
        .split(inner);

    let pw = Line::from(Span::styled(
        format!(" {} ", app.generated_password),
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    ))
    .alignment(Alignment::Center);
    f.render_widget(pw, chunks[0]);

    let hint = Line::from(Span::styled(
        " [c] Copy  |  [g] Regenerate  |  [Esc] Back ",
        Style::default().fg(Color::DarkGray),
    ))
    .alignment(Alignment::Center);
    f.render_widget(hint, chunks[1]);
}

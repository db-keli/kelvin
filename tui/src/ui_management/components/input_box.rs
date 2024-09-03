use ratatui::{
    prelude::*,
    crossterm::event::*,
    widgets::*,
    Frame,
    style::{Color, Style},
};

pub struct InputBox {
    pub title: String,
    pub text: String,
    pub cursor_position: usize,
    pub area: Rect,
    pub show_cursor: bool,
    pub border_color: Color,
}

impl InputBox {
    pub fn new(title: &str, area: Rect) -> Self {
        Self {
            title: String::from(title),
            text: String::new(),
            cursor_position: 0,
            area,
            show_cursor: true,
            border_color: Color::Yellow,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, new_text: &str) {
        self.text = String::from(new_text);
        self.cursor_position = self.text.len();
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>) {
        let input = Paragraph::new(self.text.as_str())
            .style(Style::default().fg(Color::Yellow))
            .block(
                Block::default()
                    .title(self.title.clone())
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(self.border_color)),
            );

        f.render_widget(input, self.area);

        if self.show_cursor {
            f.set_cursor(
                // puts cursor past the end of the input text
                self.area.x + self.cursor_position as u16 + 1,
                self.area.y + 1,
            );
        }
    }

    pub fn handle_key_event(&mut self, input: KeyEvent) {
        match input.code {
            //c here is a placeholder for any char that's being entered
            KeyCode::Char(c) => {
                self.text.insert(self.cursor_position, c);
                self.cursor_position += 1;
            }
            KeyCode::Backspace => {
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                    self.text.remove(self.cursor_position);
                }
            }
            KeyCode::Delete => {
                if self.cursor_position < self.text.len() {
                    self.text.remove(self.cursor_position);
                }
            }
            KeyCode::Left => {
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                }
            }
            KeyCode::Right => {
                if self.cursor_position < self.text.len() {
                    self.cursor_position += 1;
                }
            }
            KeyCode::Home => {
                self.cursor_position = 0;
            }
            KeyCode::End => {
                self.cursor_position = self.text.len();
            }
            _ => {}
        }
    }
}

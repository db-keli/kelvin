use ratatui::{
    prelude::*,
    widgets::*,
    Frame,
    style::{Color, Style},
};

pub struct Box {
    pub title: String,
    pub content: String,
    pub area: Rect,
    pub border_color: Color,
}

impl Box {
    pub fn new(title: &str, area: Rect) -> Self {
        Self {
            title: String::from(title),
            content: String::new(),
            area,
            border_color: Color::Yellow,
        }
    }

    pub fn set_content(&mut self, content: &str) {
        self.content = String::from(content);
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>) {
        let block = Block::default()
            .title(self.title.as_str())
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.border_color));

        f.render_widget(block, self.area);

        let text = Paragraph::new(self.content.as_str())
            .alignment(Alignment::Center)
            .block(Block::default());
        let content_area = Rect {
            x: self.area.x + 1,
            y: self.area.y + 1,
            width: self.area.width - 2,
            height: self.area.height - 2, // adjusted to fit within the box
        };

        f.render_widget(text, content_area);
    }
}
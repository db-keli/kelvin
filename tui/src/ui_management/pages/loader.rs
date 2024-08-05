use ratatui::prelude::*;
use ratatui::widgets::*;
use ratatui::Frame;

use crate::ui_management::components;
use components::dbox;
use dbox::Box;
pub struct LoaderPage {
    pub dbox: Box,
    pub progress_count: u16,
    pub area: Rect,
}

impl LoaderPage {
    pub fn new(area: Rect) -> Self {
        Self {
            dbox: Box::new("KELVIN", Rect::new(0, 0, area.width / 2, area.height / 2)),
            progress_count: 0,
            area,
        }
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>) {
        let area = centered_rect(60, 20, self.area);
        self.dbox.area = area;
        self.dbox.set_content("WELCOME TO KELVIN!\nA TERMINAL PASSWORD MANAGER YOU CAN TRUST");
        
        self.dbox.render(f);

        // loading bar
        let loading_area = Rect::new(area.x + 1, area.y + area.height - 4, area.width - 2, 3);
        let loading_block = Block::default()
            .borders(Borders::ALL);

        let gauge = Gauge::default()
            .block(loading_block)
            .gauge_style(Style::default().fg(Color::Green))
            .ratio(self.progress_count as f64 / 100.0)
            .label(format!("{}/100", self.progress_count));
        
        f.render_widget(gauge, loading_area);
    }

    pub fn update_progress(&mut self) {
        if self.progress_count < 100 {
            self.progress_count += 1;
        }
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
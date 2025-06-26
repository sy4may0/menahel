use crate::client::event::AppEvent;
use crate::client::component::Component;
use anyhow::Result;
use ratatui::{
    text::Line,
    style::{Style, Color},
    widgets::Paragraph,
    layout::Rect,
    Frame,
};

const MAX_LOG_LINES: usize = 8;

pub struct Log {
    log: Vec<Line<'static>>,
    focus: bool,
}

impl Log {
    pub fn new() -> Self {
        Self { log: Vec::new(), focus: false }
    }

    fn add_error_log(&mut self, error: &String) {
        self.log.push(
            Line::from(format!("[ERROR] {}", error)).style(Style::new().fg(Color::Red))
        );
    }

    fn add_command_log(&mut self, command: &String) {
        self.log.push(
            Line::from(format!(": {}", command)).style(Style::new().fg(Color::White))
        );
    }

    fn get_visible_lines(&self) -> Vec<Line<'static>> {
        self.log.iter().rev().take(MAX_LOG_LINES).cloned().collect()
    }
}

impl Component for Log {
    fn handle_app_event(&mut self, event: &AppEvent) -> Result<()> {
        match event {
            AppEvent::ErrorLog(error) => self.add_error_log(error),
            AppEvent::CommandLog(command) => self.add_command_log(command),
            _ => {}
        }
        Ok(())
    }

    fn set_focus(&mut self, focus: bool) {
        self.focus = focus;
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let visible_lines = self.get_visible_lines();
        let block = Paragraph::new(visible_lines);
        frame.render_widget(block, area);
        Ok(())
    }
}
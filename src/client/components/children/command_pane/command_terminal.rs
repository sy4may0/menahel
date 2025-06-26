use crate::client::event::{AppEvent, Tx};
use ratatui::crossterm::event::{KeyEvent, KeyCode};
use anyhow::Result;

use crate::client::component::Component;
use ratatui::{
    text::Line,
    style::{Style, Color, Stylize},
    widgets::Paragraph,
    layout::Rect,
    Frame,
};

pub struct CommandTerminal {
    sender: Tx,
    command: String,
    focus: bool,
    edit_ready: bool,
    history: Vec<String>,
    history_index: usize,
}

impl CommandTerminal {
    pub fn new(sender: Tx) -> Self {
        Self {
            sender,
            command: String::new(),
            focus: false,
            edit_ready: false,
            history: Vec::new(),
            history_index: 0,
        }
    }
}

impl Component for CommandTerminal {
    fn handle_key_event(&mut self, key_event: &KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char(c) => {
                if self.edit_ready {
                    self.command.push(c);
                }
            },
            KeyCode::Backspace => {
                if self.edit_ready {
                    let _ = self.command.pop();
                }
            },
            KeyCode::Enter => {
                self.sender.send_app_event(AppEvent::ExecCommand(self.command.clone()))?;
                self.sender.send_app_event(AppEvent::CommandLog(self.command.clone()))?;
                self.history.push(self.command.clone());
                self.command = String::new();
                self.sender.send_app_event(AppEvent::FocusBack)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn set_focus(&mut self, focus: bool) {
        self.focus = focus;
        if focus {
            self.command = String::new();
            self.edit_ready = true;
        } else {
            self.edit_ready = false;
        }
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        match self.focus {
            true => {
                let command_line = Line::from(format!(":{}", self.command.clone()));
                let paragraph = Paragraph::new(command_line)
                    .style(Style::new().fg(Color::Green).bold());
                frame.render_widget(paragraph, area);
            }
            false => {
                let command_line = Line::from(":");
                let paragraph = Paragraph::new(command_line)
                    .style(Style::new().fg(Color::Gray));
                frame.render_widget(paragraph, area);
            }
        }
        Ok(())
    }
}
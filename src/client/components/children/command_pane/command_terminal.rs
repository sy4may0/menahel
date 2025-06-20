use tokio::sync::mpsc;

use crate::client::event::{Event, CommandEvent};
use ratatui::crossterm::event::{KeyEvent, KeyCode};
use color_eyre::Result;

use crate::client::event::AppEvent;
use crate::client::components::component::Component;
use ratatui::{
    text::Line,
    style::{Style, Color, Stylize},
    widgets::Paragraph,
    layout::Rect,
    Frame,
};

pub struct CommandTerminal {
    sender: mpsc::UnboundedSender<Event>,
    command: String,
    focus: bool,
    edit_ready: bool,
}

impl CommandTerminal {
    pub fn new(sender: mpsc::UnboundedSender<Event>) -> Self {
        Self {
            sender,
            command: String::new(),
            focus: false,
            edit_ready: false,
        }
    }
}

impl Component for CommandTerminal {
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
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
                let _ = self.sender.send(Event::Command(CommandEvent::Command(self.command.clone())));
                let _ = self.sender.send(Event::App(AppEvent::FocusPreviousPane));
                self.command = String::new();
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
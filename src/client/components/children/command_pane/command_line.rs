use crate::client::component::Component;
use crate::client::components::children::command_pane::command_terminal::CommandTerminal;
use crate::client::components::children::command_pane::log::Log;
use crate::client::event::{AppEvent, Tx};
use ratatui::{
    text::Line,
    style::{Style, Color},
    widgets::Paragraph,
    layout::{Constraint, Layout},
    layout::Rect,
    Frame,
};
use anyhow::Result;
use crossterm::event::KeyEvent;

pub struct CommandLine {
    pub sender: Tx,
    pub command_terminal: CommandTerminal,
    pub log: Log,
    pub focus: bool,
}

impl CommandLine {
    pub fn new(sender: Tx) -> Self {
        Self { 
            sender: sender.clone(),
            command_terminal: CommandTerminal::new(sender.clone()),
            log: Log::new(),
            focus: false,
        }
    }

    fn draw_with_terminal(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let row = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .split(area);

        self.command_terminal.draw(frame, row[0])?;
        
        // 水平線を表示
        let horizontal_line = Line::from("─".repeat(area.width as usize))
            .style(Style::new().fg(Color::Green));
        let line_paragraph = Paragraph::new(horizontal_line);
        frame.render_widget(line_paragraph, row[1]);
        
        self.log.draw(frame, row[2])?;
        Ok(())
    }

    fn draw_only_log(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        self.log.draw(frame, area)?;
        Ok(())
    }
}

impl Component for CommandLine {
    fn handle_app_event(&mut self, event: &AppEvent) -> Result<()> {
        self.command_terminal.handle_app_event(event)?;
        self.log.handle_app_event(event)?;
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: &KeyEvent) -> Result<()> {
        self.command_terminal.handle_key_event(key_event)?;
        Ok(())
    }

    fn set_focus(&mut self, focus: bool) {
        self.focus = focus;
        self.command_terminal.set_focus(focus);
        self.log.set_focus(focus);
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        if self.focus {
            self.draw_with_terminal(frame, area)?;
        } else {
            self.draw_only_log(frame, area)?;
        }
        Ok(())
    }
}
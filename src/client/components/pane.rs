use ratatui::{
    layout::{Margin, Rect},
    widgets::{Block, BorderType},
    Frame,
    text::{Line},
    style::{Style, Color, Stylize},
};
use tokio::sync::mpsc;
use color_eyre::Result;

use super::component::Component;
use crate::client::event::{AppEvent, Event};
use ratatui::crossterm::event::KeyEvent;


use super::children::command_pane::command_line::CommandLine;
use super::children::header_panes::project_name::ProjectName;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PaneId {
    ProjectPane,
    ProjectStatisticsPane,
    StatusPane,
    TaskGroupPane,
    TaskPane,
    CommandPane,
}

pub struct Pane {
    pub title: String,
    pub sender: mpsc::UnboundedSender<Event>,
    pub child: Option<Box<dyn Component>>,
    pub focus: bool,
    pub pane_id: PaneId,
}


impl Pane {
    pub fn new(title: String, sender: mpsc::UnboundedSender<Event>, initial_focus: bool, pane_id: PaneId) -> Self {
        let child = build_child(pane_id, sender.clone());
        Self {
            title,
            sender,
            child,
            focus: initial_focus,
            pane_id,
        }
    }
}

fn build_child(pane_id: PaneId, sender: mpsc::UnboundedSender<Event>) -> Option<Box<dyn Component>> {
    match pane_id {
        PaneId::CommandPane => Some(Box::new(CommandLine::new(sender))),
        PaneId::ProjectPane => Some(Box::new(ProjectName::new("Not Set".to_string(), sender))),
        _ => None
    }
}

impl Component for Pane {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        if let Some(child) = &mut self.child {
            let inner_area = area.inner(
                Margin {
                    vertical: 1,
                    horizontal: 1,
                }
            );
            child.draw(frame, inner_area)?;
        }

        let block = match self.focus {
            true => {
                let t = Line::from(format!("*{}*", self.title.clone()));
                Block::bordered()
                .title(t)
                .border_type(BorderType::Plain)
                .border_style(Style::new().fg(Color::Green).bold())
                .title_style(Style::new().fg(Color::Green).bold())
            }
            false => {
                let t = Line::from(self.title.clone());
                Block::bordered()
                .title(t)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(Color::Gray))
                .title_style(Style::default().fg(Color::Gray))
            }
        };

        frame.render_widget(
            block,
            area,
        );
        Ok(())
    }

    fn handle_event(&mut self, event: AppEvent) -> Result<()> {
        if let Some(child) = &mut self.child {
            child.handle_event(event)?;
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        if let Some(child) = &mut self.child {
            child.handle_key_event(key_event)?;
        }
        Ok(())
    }

    fn set_focus(&mut self, focus: bool) {
        self.focus = focus;
        if let Some(child) = &mut self.child {
            child.set_focus(focus);
        }
    }

}
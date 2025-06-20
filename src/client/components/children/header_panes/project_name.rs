use crate::client::event::{AppEvent, Event};
use crate::client::components::component::Component;
use tokio::sync::mpsc;

use ratatui::{
    widgets::Paragraph,
    style::{Style, Modifier, Color},
    text::Line,
    layout::Rect,
    Frame,
};
use color_eyre::Result;

pub struct ProjectName {
    pub project_name: String,
    pub sender: mpsc::UnboundedSender<Event>,
}

impl ProjectName {
    pub fn new(project_name: String, sender: mpsc::UnboundedSender<Event>) -> Self {
        Self {
            project_name,
            sender,
        }
    }

    fn set_project_name(&mut self, project_name: String) {
        self.project_name = project_name;
    }
        
}

impl Component for ProjectName {
    fn handle_event(&mut self, event: AppEvent) -> Result<()> {
        match event {
            AppEvent::ChangeProject(project) => {
                self.set_project_name(project.name);
                Ok(())
            },
            _ => Ok(())
        }
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let paragraph = Paragraph::new(
            Line::from(format!(" {}", self.project_name))
        ).style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
        frame.render_widget(paragraph, area);
        Ok(())
    }
}
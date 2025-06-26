use crate::client::event::{AppEvent, Event, Tx};
use crate::client::component::Component;

use ratatui::{
    widgets::Paragraph,
    style::{Style, Modifier, Color},
    text::Line,
    layout::Rect,
    Frame,
};
use anyhow::Result;

pub struct ProjectName {
    project_name: String,
}

impl ProjectName {
    pub fn new(project_name: String) -> Self {
        Self {
            project_name,
        }
    }

    fn set_project_name(&mut self, project_name: String) {
        self.project_name = project_name;
    }
        
}

impl Component for ProjectName {
    fn handle_app_event(&mut self, event: &AppEvent) -> Result<()> {
        match event {
            AppEvent::SetProject(project) => {
                self.set_project_name(project.name.clone());
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
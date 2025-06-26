use crate::client::components::pane::{Pane};
use crate::client::event::{
    AppEvent,
    Tx,
};
use crate::client::component::Component;
use crate::client::components::children::command_pane::command_line::CommandLine;
use crate::client::components::children::header_panes::project_name::ProjectName;
use ratatui::{
    Frame,
    crossterm::event::KeyEvent,
    layout::Rect,
};
use anyhow::Result;
use crate::client::key_map::UiKeyMap;
use ratatui::layout::{
    Layout,
    Constraint,
};


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PaneId {
    ProjectPane,
    ProjectStatisticsPane,
    StatusPane,
    TaskGroupPane,
    TaskPane,
    CommandPane,
}


pub struct UI {
    project_pane: Pane,
    project_statistics_pane: Pane,
    status_pane: Pane,
    task_group_pane: Pane,
    task_pane: Pane,
    command_pane: Pane,
    focus: PaneId,
    previous_focus: PaneId,
    sender: Tx,
}

impl UI {
    pub fn new(sender: Tx) -> Self {
        Self {
            project_pane: Pane::new(
                "Project".to_string(),
                sender.clone(),
                false,
                PaneId::ProjectPane,
            ),
            project_statistics_pane: Pane::new(
                "Project Statistics".to_string(),
                sender.clone(),
                false,
                PaneId::ProjectStatisticsPane,
            ),
            status_pane: Pane::new(
                "Status".to_string(),
                sender.clone(),
                false,
                PaneId::StatusPane,
            ),
            task_group_pane: Pane::new(
                "Task Group".to_string(),
                sender.clone(),
                true,
                PaneId::TaskGroupPane,
            ),
            task_pane: Pane::new(
                "Task".to_string(),
                sender.clone(),
                false,
                PaneId::TaskPane,
            ),
            command_pane: Pane::new(
                "Command".to_string(),
                sender.clone(),
                false,
                PaneId::CommandPane,
            ),
            focus: PaneId::TaskGroupPane,
            previous_focus: PaneId::TaskGroupPane,
            sender: sender,
        }
    }

    pub fn init_pane(&mut self) {
        self.command_pane.set_child(
            Box::new(CommandLine::new(self.sender.clone()))
        );
        self.project_pane.set_child(
            Box::new(ProjectName::new("Not Set".to_string()))
        );
    }

    fn handle_ui_app_event(&mut self, event: &AppEvent) -> Result<()> {
        match event {
            AppEvent::FocusPane(pane_id) => {
                self.previous_focus = self.focus.clone();
                self.focus = *pane_id;
            }
            AppEvent::FocusBack => {
                self.focus = self.previous_focus;
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_ui_key_event(&mut self, key_event: &KeyEvent) -> Result<()> {
        match (self.focus, UiKeyMap::key(key_event.code)) {
            (PaneId::TaskGroupPane, UiKeyMap::FocusToCommandPane) => {
                self.sender.send_app_event(AppEvent::FocusPane(PaneId::CommandPane))?;
            }
            (PaneId::TaskGroupPane, UiKeyMap::ChangeFocusToRight) => {
                self.sender.send_app_event(AppEvent::FocusPane(PaneId::TaskPane))?;
            }
            (PaneId::TaskPane, UiKeyMap::FocusToCommandPane) => {
                self.sender.send_app_event(AppEvent::FocusPane(PaneId::CommandPane))?;
            }
            (PaneId::TaskPane, UiKeyMap::ChangeFocusToLeft) => {
                self.sender.send_app_event(AppEvent::FocusPane(PaneId::TaskGroupPane))?;
            }
            (PaneId::CommandPane, UiKeyMap::FocusBackFromCommandPane) => {
                self.sender.send_app_event(AppEvent::FocusBack)?;
            }
            _ => {}
        }
        Ok(())
    }
}

impl Component for UI {
    #[allow(unused_variables)]
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let row = Layout::vertical([
                Constraint::Length(3),
                Constraint::Fill(1),
                Constraint::Length(10),
            ])
            .split(frame.area());

        let col1 = Layout::horizontal([
            Constraint::Percentage(30),
            Constraint::Percentage(50),
            Constraint::Percentage(20),
        ])
        .split(row[0]);

        let col2 = Layout::horizontal([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(row[1]);

        let col3 = Layout::horizontal([
            Constraint::Percentage(100),
        ])
        .split(row[2]);

        self.project_pane.draw(frame, col1[0])?;
        self.project_statistics_pane.draw(frame, col1[1])?;
        self.status_pane.draw(frame, col1[2])?;
        self.task_group_pane.draw(frame, col2[0])?;
        self.task_pane.draw(frame, col2[1])?;
        self.command_pane.draw(frame, col3[0])?;

        Ok(())
    }

    fn handle_app_event(&mut self, event: &AppEvent) -> Result<()> {
        self.handle_ui_app_event(event)?;
        self.project_pane.handle_app_event(event)?;
        self.project_statistics_pane.handle_app_event(event)?;
        self.status_pane.handle_app_event(event)?;
        self.task_group_pane.handle_app_event(event)?;
        self.task_pane.handle_app_event(event)?;
        self.command_pane.handle_app_event(event)?;
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: &KeyEvent) -> Result<()> {
        self.handle_ui_key_event(key_event)?;
        match self.focus {
            PaneId::ProjectPane => self.project_pane.handle_key_event(key_event)?,
            PaneId::ProjectStatisticsPane => self.project_statistics_pane.handle_key_event(key_event)?,
            PaneId::StatusPane => self.status_pane.handle_key_event(key_event)?,
            PaneId::TaskGroupPane => self.task_group_pane.handle_key_event(key_event)?,
            PaneId::TaskPane => self.task_pane.handle_key_event(key_event)?,
            PaneId::CommandPane => self.command_pane.handle_key_event(key_event)?,
        }
        Ok(())
    }
}
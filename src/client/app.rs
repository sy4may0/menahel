use crate::client::event::{AppEvent, Event, EventHandler, CommandEvent, RepositoryEvent};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Layout},
    Frame,
};
use color_eyre::Result;

use super::components::pane::{Pane, PaneId};
use super::components::component::Component;
use super::command::CommandKind;
use super::repository::Repository;

pub struct App {
    pub running: bool,
    pub counter: u8,
    pub events: EventHandler,
    pub project_pane: Pane,
    pub project_statistics_pane: Pane,
    pub status_pane: Pane,
    pub task_group_pane: Pane,
    pub task_pane: Pane,
    pub command_pane: Pane,
    pub focus: PaneId,
    pub previous_focus: PaneId,
    pub repository: Repository,
}

impl Default for App {
    fn default() -> Self {
        let handler = EventHandler::new();
        Self {
            running: true,
            counter: 0,
            project_pane: Pane::new(
                "Project".to_string(), handler.sender.clone(),
                false,
                PaneId::ProjectPane,
            ),
            project_statistics_pane: Pane::new(
                "Project Statistics".to_string(), handler.sender.clone(),
                false,
                PaneId::ProjectStatisticsPane,
            ),
            status_pane: Pane::new(
                "Status".to_string(), handler.sender.clone(),
                false,
                PaneId::StatusPane,
            ),
            task_group_pane: Pane::new(
                "Task Group".to_string(), handler.sender.clone(), 
                true,
                PaneId::TaskGroupPane,
            ),
            task_pane: Pane::new("Task".to_string(), handler.sender.clone(), 
                false,
                PaneId::TaskPane,
            ),
            command_pane: Pane::new(
                "Command".to_string(), handler.sender.clone(),
                false,
                PaneId::CommandPane,
            ),
            repository: Repository::new(handler.sender.clone()),
            events: handler,
            focus: PaneId::TaskGroupPane,
            previous_focus: PaneId::TaskGroupPane,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| {
                let _ = self.draw(frame);
            })?;
            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key_event) => self.handle_key_events(key_event),
                    _ => {}
                },
                Event::App(app_event) => match self.handle_app_events(app_event) {
                    Ok(_) => {}
                    Err(e) => self.handle_error(e.to_string()),
                },
                Event::Command(command_event) => match command_event {
                    CommandEvent::Command(command) => self.handle_command(command),
                },
                Event::Repository(repository_event) => {
                    match self.repository.handle_repository_events(repository_event) {
                        Ok(_) => {}
                        Err(e) => self.handle_error(e.to_string()),
                    }
                },
            }
        }
        Ok(())
    }

    pub fn handle_app_events(&mut self, app_event: AppEvent) -> color_eyre::Result<()> {
        match app_event.clone() {
            AppEvent::Quit => self.quit(),
            AppEvent::FocusPane(pane_id) => self.change_focus(pane_id),
            AppEvent::FocusPreviousPane => self.change_focus(self.previous_focus),
            AppEvent::Error(error) => self.handle_error(error),
            _ => {}
        }
        self.handle_component_events(app_event)?;
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> () {
        match self.focus {
            PaneId::TaskGroupPane => {
                match key_event.code {
                    KeyCode::Char(':') => {
                        self.events.send(AppEvent::FocusPane(PaneId::CommandPane));
                    }
                    KeyCode::Char('l') => {
                        self.events.send(AppEvent::FocusPane(PaneId::TaskPane));
                    }
                    _ => {}
                }
            }
            PaneId::TaskPane => {
                match key_event.code {
                    KeyCode::Char(':') => {
                        self.events.send(AppEvent::FocusPane(PaneId::CommandPane));
                    }
                    KeyCode::Char('h') => {
                        self.events.send(AppEvent::FocusPane(PaneId::TaskGroupPane));
                    }
                    _ => {}
                }
            }
            PaneId::CommandPane => {
                match key_event.code {
                    KeyCode::Esc => {
                        self.events.send(AppEvent::FocusPreviousPane);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        match self.handle_component_key_events(key_event) {
            Ok(_) => {}
            Err(e) => self.handle_error(e.to_string()),
        }
    }

    pub fn handle_command(&mut self, command: String) -> () {
        match CommandKind::parse_command(&command) {
            Ok(CommandKind::ChangeProject(project_name)) => {
                self.events.send_repository_event(RepositoryEvent::GetProject(project_name));
            },
            Ok(CommandKind::CloseApp) => self.events.send(AppEvent::Quit),
            Ok(CommandKind::EmptyCommand) => {}
            Err(e) => self.events.send(AppEvent::Error(e.to_string())),
        }
        self.events.send(AppEvent::CommandLog(command));
    }

    pub fn handle_component_events(&mut self, event: AppEvent) -> color_eyre::Result<()> {
        self.project_pane.handle_event(event.clone())?;
        self.project_statistics_pane.handle_event(event.clone())?;
        self.status_pane.handle_event(event.clone())?;
        self.task_group_pane.handle_event(event.clone())?;
        self.task_pane.handle_event(event.clone())?;
        self.command_pane.handle_event(event.clone())?;
        Ok(())
    }

    pub fn handle_component_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        self.project_pane.handle_key_event(key_event.clone())?;
        self.project_statistics_pane.handle_key_event(key_event.clone())?;
        self.status_pane.handle_key_event(key_event.clone())?;
        self.task_group_pane.handle_key_event(key_event.clone())?;
        self.task_pane.handle_key_event(key_event.clone())?;
        self.command_pane.handle_key_event(key_event.clone())?;
        Ok(())
    }

    pub fn change_focus(&mut self, pane_id: PaneId) {
        self.previous_focus = self.focus;
        self.focus = pane_id;
        self.project_pane.set_focus(false);
        self.project_statistics_pane.set_focus(false);
        self.status_pane.set_focus(false);
        self.task_group_pane.set_focus(false);
        self.task_pane.set_focus(false);
        self.command_pane.set_focus(false);
        match pane_id {
            PaneId::ProjectPane => {
                self.project_pane.set_focus(true);
            }
            PaneId::ProjectStatisticsPane => {
                self.project_statistics_pane.set_focus(true);
            }
            PaneId::StatusPane => {
                self.status_pane.set_focus(true);
            }
            PaneId::TaskGroupPane => {
                self.task_group_pane.set_focus(true);
            }
            PaneId::TaskPane => {
                self.task_pane.set_focus(true);
            }
            PaneId::CommandPane => {
                self.command_pane.set_focus(true);
            }
        }
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn handle_error(&mut self, error: String) {
        self.events.send(AppEvent::ErrorLog(error));
    }

    pub fn increment_counter(&mut self) {
        self.counter = self.counter.saturating_add(1);
    }

    pub fn decrement_counter(&mut self) {
        self.counter = self.counter.saturating_sub(1);
    }

    pub fn draw(&mut self, frame: &mut Frame) -> Result<()> {
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
}

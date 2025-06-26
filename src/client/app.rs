use ratatui::{
    DefaultTerminal,
    crossterm::event::KeyEvent,
    layout::Rect,
};

use crate::client::ui::UI;
use crate::client::component::Component;
use crate::client::event::{
    EventHandler,
    Event,
    AppEvent,
    RepositoryEvent,
};
use crate::client::command_map::parse_command;
use crate::client::repository::Repository;
use crate::client::command_map::Command;

pub enum ErrorType {
    InvalidCommand(String),
    HandlerError(String),
    RepositoryError(String),
}


pub struct App {
    pub running: bool,
    pub ui: UI,
    pub events: EventHandler,
    pub repository: Repository,
}

impl Default for App {
    fn default() -> Self {
        let handler = EventHandler::new();
        let mut ui = UI::new(handler.sender.clone());
        let repository = Repository::new(handler.sender.clone());
        ui.init_pane();
        Self {
            running: true,
            ui,
            events: handler,
            repository,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn run(
        mut self,
        mut terminal: DefaultTerminal,
    ) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| {
                let _ = self.ui.draw(frame, Rect::default());
            })?;
            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key_event) => self.handle_key_events(&key_event),
                    _ => {}
                },
                Event::App(app_event) => {
                    self.handle_app_events(app_event)
                }
                Event::Repository(repository_event) => {
                    self.handle_repository_events(repository_event)
                }
            }
        }

        Ok(())
    }

    pub fn handle_app_events(
        &mut self, app_event: AppEvent
    ) -> () {
        match &app_event {
            AppEvent::Quit => self.running = false,
            AppEvent::ExecCommand(command) => self.handle_command(command),
            _ => {}
        }
        match self.ui.handle_app_event(&app_event) {
            Ok(_) => {}
            Err(e) => self.handle_error(ErrorType::HandlerError(e.to_string())),
        }
    }

    pub fn handle_key_events(&mut self, key_event: &KeyEvent) -> () {
        match self.ui.handle_key_event(key_event) {
            Ok(_) => {}
            Err(e) => self.handle_error(ErrorType::HandlerError(e.to_string())),
        }
    }

    pub fn handle_command(&mut self, command: &str) -> () {
        let command_event = parse_command(command);
        self.handle_command_events(command_event);
    }

    pub fn handle_command_events(&mut self, command: Command) -> () {
        match command {
            Command::EmptyCommand => {}
            Command::Quit => self.quit(),
            Command::InvalidCommand(command) => {
                self.handle_error(ErrorType::InvalidCommand(command))
            },
            Command::SetProject(project_name) => {
                match self.events.sender.send_repository_event(RepositoryEvent::RequestProject(project_name)) {
                    Ok(_) => {}
                    Err(e) => self.handle_error(ErrorType::HandlerError(e.to_string())),
                }
            },
        }
    }

    pub fn handle_repository_events(&mut self, repository_event: RepositoryEvent) -> () {
        match &repository_event {
            RepositoryEvent::Error(error) => {
                self.handle_error(ErrorType::RepositoryError(error.clone()));
            }
            _ => {}
        }
        match self.repository.handle_repository_events(repository_event) {
            Ok(_) => {}
            Err(e) => self.handle_error(ErrorType::RepositoryError(e.to_string())),
        }
    }

    pub fn handle_error(&mut self, error: ErrorType) -> () {
        let error_message = match error {
            ErrorType::InvalidCommand(command) => format!("Invalid command: {}", command),
            ErrorType::HandlerError(error) => format!("Handler error: {}", error),
            ErrorType::RepositoryError(error) => format!("Repository error: {}", error),
        };
        match self.events.sender.send_app_event(AppEvent::ErrorLog(error_message)) {
            Ok(_) => {}
            Err(e) => tracing::error!("Error sending app event: {}", e),
        }
    }

    pub fn tick(&self) -> () {
        {}
    }

    pub fn quit(&mut self) -> () {
        self.running = false;
    }
}
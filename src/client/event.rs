use tokio::sync::mpsc;
use tokio::sync::mpsc::error::SendError;
use std::time::Duration;
use ratatui::crossterm::event::Event as CrosstermEvent;
use futures::{FutureExt, StreamExt};
use color_eyre::eyre::OptionExt;


use crate::models::Project;
use crate::client::ui::PaneId;



const TICK_FPS: f64 = 30.0;

#[derive(Clone, Debug)]
pub enum Event {
    Tick,
    Crossterm(CrosstermEvent),
    App(AppEvent),
    Repository(RepositoryEvent),
}

#[derive(Clone, Debug)]
pub enum AppEvent {
    Quit,
    Error(String),
    CommandLog(String),
    ErrorLog(String),
    FocusPane(PaneId),
    FocusBack,
    ExecCommand(String),
    SetProject(Project),
}

#[derive(Clone, Debug)] 
pub enum RepositoryEvent {
    RequestProject(String),
    ResponseProject(Project),
    Error(String),
}

#[derive(Clone, Debug)]
pub struct Tx(mpsc::UnboundedSender<Event>);

impl Tx {
    pub fn new(sender: mpsc::UnboundedSender<Event>) -> Self {
        Self(sender)
    }

    pub fn send(&self, ev: Event) -> Result<(), SendError<Event>> {
        self.0.send(ev)
    }

    pub fn send_app_event(&self, event: AppEvent) -> Result<(), SendError<Event>> {
        self.send(Event::App(event))
    }

    pub fn send_repository_event(&self, event: RepositoryEvent) -> Result<(), SendError<Event>> {
        self.send(Event::Repository(event))
    }
}

#[derive(Debug)]
pub struct EventHandler {
    pub sender: Tx,
    receiver: mpsc::UnboundedReceiver<Event>,
}

impl EventHandler {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let actor = EventTask::new(sender.clone());
        tokio::spawn(async { actor.run().await });
        Self {
            sender: Tx::new(sender),
            receiver,
        }
    }

    pub fn sender(&self) -> Tx {
        self.sender.clone()
    }

    pub async fn next(&mut self) -> color_eyre::Result<Event> {
        self.receiver
            .recv()
            .await
            .ok_or_eyre("Failed to receive event")
    }
}

struct EventTask {
    sender: mpsc::UnboundedSender<Event>,
}

impl EventTask {
    fn new(sender: mpsc::UnboundedSender<Event>) -> Self {
        Self { sender }
    }

    async fn run(self) -> color_eyre::Result<()> {
        let tick_rate = Duration::from_secs_f64(1.0 / TICK_FPS);
        let mut reader = crossterm::event::EventStream::new();
        let mut tick = tokio::time::interval(tick_rate);
        loop {
            let tick_delay = tick.tick();
            let crossterm_event = reader.next().fuse();
            tokio::select! {
              _ = self.sender.closed() => {
                break;
              }
              _ = tick_delay => {
                self.send(Event::Tick);
              }
              Some(Ok(evt)) = crossterm_event => {
                self.send(Event::Crossterm(evt));
              }
            };
        }
        Ok(())
    }

    fn send(&self, event: Event) {
        let _ = self.sender.send(event);
    }
}

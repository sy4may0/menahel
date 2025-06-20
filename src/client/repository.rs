use crate::models::Project;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::models::TaskWithUser;
use crate::client::event::Event;
use tokio::sync::mpsc;
use crate::client::api::get_project;
use crate::client::event::{AppEvent, RepositoryEvent};

pub struct Repository {
    pub sender: mpsc::UnboundedSender<Event>,
    pub project: Project,
    pub task_tree: HashMap<i64, TaskTree>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskTree {
    pub own: TaskWithUser,
    pub children: Option<HashMap<i64, TaskTree>>,
}

impl Repository {
    pub fn new(sender: mpsc::UnboundedSender<Event>) -> Self {
        Self {
            sender: sender,
            project: Project::new("Not Set".to_string()),
            task_tree: HashMap::new(),
        }
    }

    pub fn set_project(&mut self, project: Project) {
        self.project = project;
    }

    pub fn set_task_tree(&mut self, task_tree: HashMap<i64, TaskTree>) {
        self.task_tree = task_tree;
    }

    pub fn handle_change_project(&mut self, project_name: String) -> () {
        let sender_clone = self.sender.clone();
        tokio::spawn(async move {
            let project = get_project(&project_name).await;
            match project {
                Ok(project) => {
                    let _ = sender_clone.send(Event::App(AppEvent::ChangeProject(project.clone())));
                    let _ = sender_clone.send(Event::Repository(RepositoryEvent::UpdateRepoProject(project)));
                }
                Err(e) => {
                    let _ = sender_clone.send(Event::App(AppEvent::Error(e.to_string())));
                }
            }
        });
    }

    pub fn handle_repository_events(&mut self, event: RepositoryEvent) -> color_eyre::Result<()> {
        match event {
            RepositoryEvent::GetProject(project_name) => {
                self.handle_change_project(project_name);
                Ok(())
            }
            RepositoryEvent::UpdateRepoProject(project) => {
                self.set_project(project);
                Ok(())
            }
        }
    }
}
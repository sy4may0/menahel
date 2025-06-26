use crate::client::event::{
    Tx,
    AppEvent,
    RepositoryEvent,
};
use crate::models::Project;
use crate::models::TaskFilter;
use crate::models::TaskWithUser;
use crate::client::api::get_project;
use std::collections::HashMap;
use anyhow::Result;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskTree {
    pub own: TaskWithUser,
    pub children: Option<HashMap<i64, TaskTree>>,
}
pub struct Repository {
  sender: Tx,
  project: Project,
  task_filter: TaskFilter,
  task_tree: HashMap<i64, TaskTree>,
}

impl Repository {
    pub fn new(sender: Tx) -> Self {
        Self {
            sender,
            project: Project::new("Not Set".to_string()),
            task_filter: TaskFilter::new(),
            task_tree: HashMap::new(),
        }
    }

    pub fn set_project(&mut self, project: Project) {
        self.project = project;
    }

    pub fn project(&self) -> &Project {
        &self.project
    }

    pub fn set_task_filter(&mut self, task_filter: TaskFilter) {
        self.task_filter = task_filter;
    }

    pub fn task_filter(&self) -> &TaskFilter {
        &self.task_filter
    }

    pub fn set_task_tree(&mut self, task_tree: HashMap<i64, TaskTree>) {
        self.task_tree = task_tree;
    }

    pub fn task_tree(&self) -> &HashMap<i64, TaskTree> {
        &self.task_tree
    }

    pub fn handle_repository_events(&mut self, event: RepositoryEvent) -> Result<()> {
        match event {
            RepositoryEvent::RequestProject(project_name) => self.handle_request_project(project_name),
            RepositoryEvent::ResponseProject(project) => self.handle_response_project(project),
            _ => Ok(())
        }
    }

    fn handle_request_project(&mut self, project_name: String) -> Result<()> {
        let sender_clone = self.sender.clone();
        tokio::spawn(async move {
            let project = get_project(&project_name).await;
            let result = match project {
                Ok(project) => {
                    RepositoryEvent::ResponseProject(project)
                }
                Err(e) => {
                    RepositoryEvent::Error(e.to_string())
                }
            };
            match sender_clone.send_repository_event(result) {
                Ok(_) => {}
                Err(e) => {
                    let _ = sender_clone.send_repository_event(
                        RepositoryEvent::Error(e.to_string())
                    );
                }
            }
        });
        Ok(())
    }

    fn handle_response_project(&mut self, project: Project) -> Result<()> {
        self.project = project;
        self.sender.send_app_event(AppEvent::SetProject(self.project.clone()))?;
        Ok(())
    }
}
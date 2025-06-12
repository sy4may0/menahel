use serde::{Deserialize, Serialize};
use crate::models::repository_model::task::Task;
use crate::models::repository_model::user::UserNoPassword;
use crate::models::repository_model::task::TaskFilter;
use crate::models::repository_model::user::UserFilter;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FixedTaskWithUser {
    pub task: Task,
    pub users: Vec<UserNoPassword>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FixedUserWithTask {
    pub user: UserNoPassword,
    pub tasks: Vec<Task>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TaskWithUserFilter {
    pub project_id: Option<i64>,
    pub parent_id: Option<i64>,
    pub level: Option<i64>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<i64>,
    pub deadline_from: Option<i64>,
    pub deadline_to: Option<i64>,
    pub created_at_from: Option<i64>,
    pub created_at_to: Option<i64>,
    pub updated_at_from: Option<i64>,
    pub updated_at_to: Option<i64>,
    pub username: Option<String>,
    pub email: Option<String>,
}

impl TaskWithUserFilter {
    pub fn new() -> Self {
        Self {
            project_id: None,
            parent_id: None,
            level: None,
            name: None,
            description: None,
            status: None,
            deadline_from: None,
            deadline_to: None,
            created_at_from: None,
            created_at_to: None,
            updated_at_from: None,
            updated_at_to: None,
            username: None,
            email: None,
        }
    }

    pub fn set_project_id(&mut self, project_id: i64) {
        self.project_id = Some(project_id);
    }

    pub fn set_parent_id(&mut self, parent_id: i64) {
        self.parent_id = Some(parent_id);
    }

    pub fn set_level(&mut self, level: i64) {
        self.level = Some(level);
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn set_status(&mut self, status: i64) {
        self.status = Some(status);
    }

    pub fn set_deadline_from(&mut self, deadline_from: i64) {
        self.deadline_from = Some(deadline_from);
    }

    pub fn set_deadline_to(&mut self, deadline_to: i64) {
        self.deadline_to = Some(deadline_to);
    }

    pub fn set_created_at_from(&mut self, created_at_from: i64) {
        self.created_at_from = Some(created_at_from);
    }

    pub fn set_created_at_to(&mut self, created_at_to: i64) {
        self.created_at_to = Some(created_at_to);
    }

    pub fn set_updated_at_from(&mut self, updated_at_from: i64) {
        self.updated_at_from = Some(updated_at_from);
    }

    pub fn set_updated_at_to(&mut self, updated_at_to: i64) {
        self.updated_at_to = Some(updated_at_to);
    }

    pub fn set_username(&mut self, username: String) {
        self.username = Some(username);
    }

    pub fn set_email(&mut self, email: String) {
        self.email = Some(email);
    }

    pub fn is_empty(&self) -> bool {
        self.project_id.is_none() &&
        self.parent_id.is_none() &&
        self.level.is_none() &&
        self.name.is_none() &&
        self.description.is_none() &&
        self.status.is_none() &&
        self.deadline_from.is_none() &&
        self.deadline_to.is_none() &&
        self.created_at_from.is_none() &&
        self.created_at_to.is_none() &&
        self.updated_at_from.is_none() &&
        self.updated_at_to.is_none() &&
        self.username.is_none() &&
        self.email.is_none()
    }

    pub fn build_task_filter(&self) -> Option<TaskFilter> {
        let task_filter = TaskFilter {
            project_id: self.project_id,
            parent_id: self.parent_id,
            level: self.level,
            name: self.name.clone(),
            description: self.description.clone(),
            status: self.status,
            deadline_from: self.deadline_from,
            deadline_to: self.deadline_to,
            created_at_from: self.created_at_from,
            created_at_to: self.created_at_to,
            updated_at_from: self.updated_at_from,
            updated_at_to: self.updated_at_to,
            assignee_id: None
        };
        match self.is_empty() {
            true => None,
            false => Some(task_filter),
        }
    }

    pub fn build_user_filter(&self) -> Option<UserFilter> {
        let user_filter = UserFilter {
            username: self.username.clone(),
            email: self.email.clone(),
        };
        match self.is_empty() {
            true => None,
            false => Some(user_filter),
        }
    }
}
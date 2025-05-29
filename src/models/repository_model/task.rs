use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Task {
    pub id: Option<i64>,
    pub project_id: i64,
    pub parent_id: Option<i64>,
    pub level: i64,
    pub name: String,
    pub description: Option<String>,
    pub status: i64,
    pub deadline: Option<i64>,
    pub created_at: i64,
    pub updated_at: Option<i64>,
}

impl Task {
    pub fn new(
        project_id: i64,
        parent_id: Option<i64>,
        level: i64,
        name: String,
        description: Option<String>,
        status: i64,
        deadline: Option<i64>,
    ) -> Self {
        Self {
            id: None,
            project_id,
            parent_id,
            level,
            name,
            description,
            status,
            deadline,
            created_at: 0,
            updated_at: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TaskFilter {
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
}

impl TaskFilter {
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
}
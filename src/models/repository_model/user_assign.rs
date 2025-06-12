use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserAssign {
    pub user_assign_id: Option<i64>,
    pub user_id: i64,
    pub task_id: i64,
}

impl UserAssign {
    pub fn new(user_id: i64, task_id: i64) -> Self {
        Self {
            user_assign_id: None,
            user_id,
            task_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserAssignFilter {
    pub user_id: Option<i64>,
    pub task_id: Option<i64>,
}

impl UserAssignFilter {
    pub fn new() -> Self {
        Self {
            user_id: None,
            task_id: None,
        }
    }

    pub fn set_user_id(&mut self, user_id: i64) {
        self.user_id = Some(user_id);
    }

    pub fn set_task_id(&mut self, task_id: i64) {
        self.task_id = Some(task_id);
    }

    pub fn is_empty(&self) -> bool {
        self.user_id.is_none() && self.task_id.is_none()
    }
}
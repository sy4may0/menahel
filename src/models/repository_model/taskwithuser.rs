use serde::{Deserialize, Serialize};
use crate::models::repository_model::task::Task;
use crate::models::repository_model::user::UserNoPassword;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct TaskWithUser {
    pub task_id: i64,
    pub project_id: i64,
    pub parent_id: Option<i64>,
    pub level: i64,
    pub name: String,
    pub description: Option<String>,
    pub status: i64,
    pub deadline: Option<i64>,
    pub created_at: i64,
    pub updated_at: Option<i64>,
    pub users: sqlx::types::Json<Vec<UserNoPassword>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct FixedTaskWithUser {
    pub task: Task,
    pub users: Vec<UserNoPassword>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct FixedUserWithTask {
    pub user: UserNoPassword,
    pub tasks: Vec<Task>,
}

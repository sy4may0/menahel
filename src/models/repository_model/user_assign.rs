use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserAssign {
    pub id: Option<i64>,
    pub user_id: i64,
    pub task_id: i64,
}

impl UserAssign {
    pub fn new(user_id: i64, task_id: i64) -> Self {
        Self {
            id: None,
            user_id,
            task_id,
        }
    }
}
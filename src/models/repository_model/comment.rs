use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Comment {
    pub comment_id: Option<i64>,
    pub user_id: i64,
    pub task_id: i64,
    pub content: String,
    pub created_at: i64,
    pub updated_at: Option<i64>,
}

impl Comment {
    pub fn new(user_id: i64, task_id: i64, content: String) -> Self {
        Self {
            comment_id: None,
            user_id,
            task_id,
            content,
            created_at: 0,
            updated_at: None,
        }
    }
}
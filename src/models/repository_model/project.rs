use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Project {
    pub project_id: Option<i64>,
    pub name: String,
}

impl Project {
    pub fn new(name: String) -> Self {
        Self {
            project_id: None,
            name,
        }
    }
}

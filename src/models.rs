use serde::{Deserialize, Serialize};
use crate::utils::validate_user_password;
use sha2::{Sha256, Digest};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct User {
    pub id: Option<i64>,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

impl User {
    pub fn new(username: String, email: String, password: String) -> Self {
        let password_hash = match validate_user_password(&password) {
            Ok(()) => password,
            _ => {
                format!("{:x}", Sha256::digest(password.as_bytes()))
            }
        };

        Self {
            id: None,
            username,
            email,
            password_hash,
        }
    }
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Project {
    pub id: Option<i64>,
    pub name: String,
}

impl Project {
    pub fn new(name: String) -> Self {
        Self {
            id: None,
            name,
        }
    }
}

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

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use crate::repository::validations::validate_user_password;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct User {
    pub user_id: Option<i64>,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct UserNoPassword {
    pub user_id: Option<i64>,
    pub username: String,
    pub email: String,
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
            user_id: None,
            username,
            email,
            password_hash,
        }
    }

    pub fn to_user_no_password(&self) -> UserNoPassword {
        UserNoPassword {
            user_id: self.user_id,
            username: self.username.clone(),
            email: self.email.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct UserFilter {
    pub username: Option<String>,
    pub email: Option<String>,
}

impl UserFilter {
    pub fn new() -> Self {
        Self {
            username: None,
            email: None,
        }
    }

    pub fn set_username(&mut self, username: String) {
        self.username = Some(username);
    }

    pub fn set_email(&mut self, email: String) {
        self.email = Some(email);
    }

    pub fn is_empty(&self) -> bool {
        self.username.is_none() && self.email.is_none()
    }
}
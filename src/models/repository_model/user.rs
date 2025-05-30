use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use crate::repository::validations::validate_user_password;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
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
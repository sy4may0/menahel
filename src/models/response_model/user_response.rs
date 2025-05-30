use serde::{Deserialize, Serialize};
use crate::models::User;
use super::common_models::{Pagination, ResponseMetadata};

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    pub results: Vec<User>,
    pub count: i64,
    pub rc: i32,
    pub message: String,
    pub pagination: Option<Pagination>,
    pub metadata: Option<ResponseMetadata>,
}

impl UserResponse {
    pub fn new(results: Vec<User>, count: i64, pagination: Option<Pagination>, metadata: Option<ResponseMetadata>) -> Self {
        Self { results, count, rc: 0, message: "OK".to_string(), pagination, metadata }
    }

    pub fn new_error(message: String, rc: i32) -> Self {
        Self { results: vec![], count: 0, rc, message, pagination: None, metadata: None }
    }

}
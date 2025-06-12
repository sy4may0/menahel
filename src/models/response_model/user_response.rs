use serde::{Deserialize, Serialize};
use crate::models::UserNoPassword;
use super::common_models::{Pagination, ResponseMetadata};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    pub results: Vec<UserNoPassword>,
    pub count: i64,
    pub rc: i32,
    pub message: String,
    pub pagination: Option<Pagination>,
    pub metadata: Option<ResponseMetadata>,
}

impl UserResponse {
    pub fn new(results: Vec<UserNoPassword>, count: i64, pagination: Option<Pagination>, metadata: Option<ResponseMetadata>) -> Self {
        Self { results, count, rc: 0, message: "OK".to_string(), pagination, metadata }
    }
}
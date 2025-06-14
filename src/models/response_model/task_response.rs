use serde::{Deserialize, Serialize};
use crate::models::Task;
use super::common_models::{Pagination, ResponseMetadata};

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskResponse {
    pub results: Vec<Task>,
    pub count: i64,
    pub rc: i32,
    pub message: String,
    pub pagination: Option<Pagination>,
    pub metadata: Option<ResponseMetadata>,
}

impl TaskResponse {
    pub fn new(results: Vec<Task>, count: i64, pagination: Option<Pagination>, metadata: Option<ResponseMetadata>) -> Self {
        Self { results, count, rc: 0, message: "OK".to_string(), pagination, metadata }
    }
}
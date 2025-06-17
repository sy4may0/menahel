use serde::{Deserialize, Serialize};
use crate::models::{Comment, CommentWithUser};
use super::common_models::{Pagination, ResponseMetadata};

#[derive(Serialize, Deserialize, Debug)]
pub struct CommentResponse {
    pub results: Vec<Comment>,
    pub count: i64,
    pub rc: i32,
    pub message: String,
    pub pagination: Option<Pagination>,
    pub metadata: Option<ResponseMetadata>,
}

impl CommentResponse {
    pub fn new(results: Vec<Comment>, count: i64, pagination: Option<Pagination>, metadata: Option<ResponseMetadata>) -> Self {
        Self { results, count, rc: 0, message: "OK".to_string(), pagination, metadata }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommentUserResponse {
    pub results: Vec<CommentWithUser>,
    pub count: i64,
    pub rc: i32,
    pub message: String,
    pub pagination: Option<Pagination>,
    pub metadata: Option<ResponseMetadata>,
}

impl CommentUserResponse {
    pub fn new(results: Vec<CommentWithUser>, count: i64, pagination: Option<Pagination>, metadata: Option<ResponseMetadata>) -> Self {
        Self { results, count, rc: 0, message: "OK".to_string(), pagination, metadata }
    }
}
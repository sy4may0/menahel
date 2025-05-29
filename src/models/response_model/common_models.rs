use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RootResponse {
    pub message: String,
    pub rc: i32,
    pub metadata: Option<ResponseMetadata>,
}

impl RootResponse {
    pub fn new(message: String, rc: i32, metadata: Option<ResponseMetadata>) -> Self {
        Self { message, rc, metadata }
    }
}

pub struct ErrorResponse {
    pub message: String,
    pub rc: i32,
    pub metadata: Option<ResponseMetadata>,
}

impl ErrorResponse {
    pub fn new(message: String, rc: i32, metadata: Option<ResponseMetadata>) -> Self {
        Self { message, rc, metadata }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Pagination {
    pub current_page: i32,
    pub page_size: i32,
    pub total_pages: i32,
    pub total_items: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseMetadata {
    pub request_id: String,
    pub processing_time_ms: u64,
    pub api_version: String,
}
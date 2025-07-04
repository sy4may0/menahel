use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RootResponse {
    pub message: String,
    pub rc: i32,
    pub metadata: Option<ResponseMetadata>,
}

impl RootResponse {
    pub fn new(message: String, rc: i32, metadata: Option<ResponseMetadata>) -> Self {
        Self {
            message,
            rc,
            metadata,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub message: String,
    pub rc: i32,
    pub metadata: Option<ResponseMetadata>,
}

impl ErrorResponse {
    pub fn new(message: String, rc: i32, metadata: Option<ResponseMetadata>) -> Self {
        Self {
            message,
            rc,
            metadata,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pagination {
    pub current_page: i32,
    pub page_size: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseMetadata {
    pub request_id: String,
    pub api_version: String,
}

impl ResponseMetadata {
    pub fn new(request_id: String) -> Self {
        Self {
            request_id,
            api_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

#[derive(Debug)]
pub enum PaginationStatus {
    Active,
    Inactive,
    Error,
}

#[derive(Debug)]
pub struct PaginationParams {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
    pub status: PaginationStatus,
}

impl PaginationParams {
    pub fn new(page: Option<i32>, page_size: Option<i32>) -> Self {
        Self {
            page,
            page_size,
            status: PaginationStatus::Error,
        }
    }

    pub fn validate(&mut self) {
        if self.page.is_none() && self.page_size.is_none() {
            self.status = PaginationStatus::Inactive;
        } else if self.page.is_some() && self.page_size.is_some() {
            if self.page.unwrap() <= 0 {
                self.status = PaginationStatus::Error;
            } else if self.page_size.unwrap() <= 0 || self.page_size.unwrap() > 101 {
                self.status = PaginationStatus::Error;
            } else {
                self.status = PaginationStatus::Active;
            }
        } else {
            self.status = PaginationStatus::Error;
        }
    }

    pub fn status(&self) -> &PaginationStatus {
        &self.status
    }

    pub fn page(&self) -> Option<&i32> {
        self.page.as_ref()
    }

    pub fn page_size(&self) -> Option<&i32> {
        self.page_size.as_ref()
    }
}

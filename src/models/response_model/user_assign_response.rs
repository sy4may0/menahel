use super::common_models::{Pagination, ResponseMetadata};
use crate::models::UserAssign;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserAssignResponse {
    pub results: Vec<UserAssign>,
    pub count: i64,
    pub rc: i32,
    pub message: String,
    pub pagination: Option<Pagination>,
    pub metadata: Option<ResponseMetadata>,
}

impl UserAssignResponse {
    pub fn new(
        results: Vec<UserAssign>,
        count: i64,
        pagination: Option<Pagination>,
        metadata: Option<ResponseMetadata>,
    ) -> Self {
        Self {
            results,
            count,
            rc: 0,
            message: "OK".to_string(),
            pagination,
            metadata,
        }
    }
}

use serde::{Deserialize, Serialize};
use crate::models::project::Project;
use crate::models::response_model::Pagination;
use crate::models::response_model::ResponseMetadata;


#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectResponse {
    pub results: Vec<Project>,
    pub count: i64,
    pub rc: i32,
    pub message: String,
    pub pagination: Option<Pagination>,
    pub metadata: Option<ResponseMetadata>,
}

impl ProjectResponse {
    pub fn new(results: Vec<Project>, count: i64, pagenation: Option<Pagination>, metadata: Option<ResponseMetadata>) -> Self {
        Self {
            results,
            count,
            rc: 0,
            message: "OK".to_string(),
            pagination: pagenation,
            metadata,
        }
    }
}
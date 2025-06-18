use crate::errors::db_error::DBAccessError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HandlerError {
    #[error("NotFound: {0}")]
    NotFound(String),

    #[error("InternalServerError: {0}")]
    InternalServerError(String),

    #[error("BadRequest: {0}")]
    BadRequest(String),
}

impl From<DBAccessError> for HandlerError {
    fn from(error: DBAccessError) -> Self {
        match error {
            DBAccessError::NotFoundError(msg) => HandlerError::NotFound(msg),
            DBAccessError::ValidationError(msg) => HandlerError::BadRequest(msg),
            DBAccessError::ConnectionError(e) => HandlerError::InternalServerError(e.to_string()),
            DBAccessError::QueryError(e) => HandlerError::InternalServerError(e.to_string()),
        }
    }
}

impl HandlerError {
    pub fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            HandlerError::NotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
            HandlerError::InternalServerError(_) => {
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            }
            HandlerError::BadRequest(_) => actix_web::http::StatusCode::BAD_REQUEST,
        }
    }
}

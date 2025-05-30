use thiserror::Error;
use crate::errors::db_error::DBAccessError;

#[derive(Error, Debug)]
pub enum HandlerError {
    #[error("ValidationError: {0}")]
    ValidationError(String),

    #[error("DBAccessError: {0}")]
    DBAccessError(DBAccessError),

    #[error("InvalidRequest: {0}")]
    InvalidRequest(String),
}
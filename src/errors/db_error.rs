use sqlx::Error as SqlxError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DBAccessError {
    #[error("ConnectionError: {0}")]
    ConnectionError(#[from] SqlxError),

    #[error("QueryError: {0}")]
    QueryError(#[from] anyhow::Error),

    #[error("TimeoutError: {0}")]
    TimeoutError(String),

    #[error("ResourceExhaustedError: {0}")]
    ResourceExhaustedError(String),

    #[error("ValidationError: {0}")]
    ValidationError(String),
}

impl DBAccessError {
    pub fn can_retry(&self) -> bool {
        match self {
            DBAccessError::ConnectionError(e) => {
                // 接続エラーの場合、一時的な問題であればリトライ可能
                matches!(
                    e,
                    SqlxError::PoolClosed | SqlxError::PoolTimedOut | SqlxError::Io(_)
                )
            }
            DBAccessError::TimeoutError(_) => true,
            DBAccessError::ResourceExhaustedError(_) => true,
            _ => false,
        }
    }
}

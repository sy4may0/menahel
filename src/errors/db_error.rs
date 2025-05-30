use sqlx::Error as SqlxError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DBAccessError {
    #[error("ConnectionError: {0}")]
    ConnectionError(#[from] SqlxError),

    #[error("QueryError: {0}")]
    QueryError(#[from] anyhow::Error),

    #[error("ValidationError: {0}")]
    ValidationError(String),

    #[error("NotFoundError: {0}")]
    NotFoundError(String),
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
            _ => false,
        }
    }
}

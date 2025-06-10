use crate::enums::TaskLevel;
use crate::errors::db_error::DBAccessError;
use crate::errors::messages::{ErrorKey, get_error_message};
use crate::models::Comment;
use crate::repository::task_repo::get_task_by_id_with_transaction;
use crate::repository::user_repo::get_user_by_id_with_transaction;
use crate::repository::validations::{
    validate_comment_content, validate_comment_task_id, validate_comment_user_id,
};
use anyhow::Result;
use chrono::Utc;
use sqlx::{Pool, Sqlite, Transaction};

pub struct CommentRepository {
    pool: Pool<Sqlite>,
}

impl CommentRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    async fn validate_target_user_and_task(
        &self,
        comment: &Comment,
        tx: &mut Transaction<'_, Sqlite>,
    ) -> Result<(), DBAccessError> {
        get_user_by_id_with_transaction(&comment.user_id, tx).await?;
        let task = get_task_by_id_with_transaction(comment.task_id, tx).await?;

        if task.level != TaskLevel::max_level() as i64 {
            return Err(DBAccessError::ValidationError(get_error_message(
                ErrorKey::CommentToNotMaxLevelTask,
                format!("ID = {}", comment.task_id),
            )));
        }
        Ok(())
    }

    pub async fn create_comment(&self, comment: Comment) -> Result<Comment, DBAccessError> {
        validate_comment_user_id(comment.user_id)?;
        validate_comment_task_id(comment.task_id)?;
        validate_comment_content(&comment.content)?;

        let mut tx = self.pool.begin().await?;

        self.validate_target_user_and_task(&comment, &mut tx)
            .await?;

        let now = Utc::now().timestamp();
        let result = sqlx::query_as!(
            Comment,
            r#"
                INSERT INTO comments (user_id, task_id, content, created_at)
                VALUES ($1, $2, $3, $4)
                RETURNING id, user_id, task_id, content, created_at, updated_at
            "#,
            comment.user_id,
            comment.task_id,
            comment.content,
            now,
        )
        .fetch_one(&mut *tx)
        .await;

        match result {
            Ok(comment) => {
                tx.commit().await.map_err(|e| {
                    DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                        ErrorKey::CommentCreateFailed,
                        e.to_string()
                    )))
                })?;
                Ok(comment)
            }
            Err(e) => {
                let _ = tx.rollback().await;
                Err(DBAccessError::QueryError(anyhow::anyhow!(
                    get_error_message(ErrorKey::CommentCreateFailed, e.to_string())
                )))
            }
        }
    }

    pub async fn get_comment_by_id(&self, id: i64) -> Result<Option<Comment>, DBAccessError> {
        sqlx::query_as!(
            Comment,
            r#"
                SELECT id, user_id, task_id, content, created_at, updated_at
                FROM comments
                WHERE id = $1
            "#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::CommentGetByIdFailed,
                e.to_string()
            )))
        })
    }

    pub async fn get_comment_by_task_id(
        &self,
        task_id: i64,
    ) -> Result<Vec<Comment>, DBAccessError> {
        sqlx::query_as!(
            Comment,
            r#"
                SELECT id, user_id, task_id, content, created_at, updated_at
                FROM comments
                WHERE task_id = $1
            "#,
            task_id,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::CommentGetByTaskIdFailed,
                e.to_string()
            )))
        })
    }

    pub async fn get_comment_by_user_id(
        &self,
        user_id: i64,
    ) -> Result<Vec<Comment>, DBAccessError> {
        sqlx::query_as!(
            Comment,
            r#"
                SELECT id, user_id, task_id, content, created_at, updated_at
                FROM comments
                WHERE user_id = $1
            "#,
            user_id,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::CommentGetByUserIdFailed,
                e.to_string()
            )))
        })
    }

    pub async fn update_comment(&self, comment: Comment) -> Result<Comment, DBAccessError> {
        validate_comment_user_id(comment.user_id)?;
        validate_comment_task_id(comment.task_id)?;
        validate_comment_content(&comment.content)?;

        let mut tx = self.pool.begin().await?;

        self.validate_target_user_and_task(&comment, &mut tx)
            .await?;

        let now = Utc::now().timestamp();
        let result = sqlx::query_as!(
            Comment,
            r#"
                UPDATE comments
                SET content = $1, user_id = $2, task_id = $3, updated_at = $4
                WHERE id = $5
                RETURNING id, user_id, task_id, content, created_at, updated_at
            "#,
            comment.content,
            comment.user_id,
            comment.task_id,
            now,
            comment.id,
        )
        .fetch_one(&mut *tx)
        .await;

        match result {
            Ok(comment) => {
                tx.commit().await.map_err(|e| {
                    DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                        ErrorKey::CommentUpdateFailed,
                        e.to_string()
                    )))
                })?;
                Ok(comment)
            }
            Err(e) => {
                let _ = tx.rollback().await;
                Err(DBAccessError::QueryError(anyhow::anyhow!(
                    get_error_message(ErrorKey::CommentUpdateFailed, e.to_string())
                )))
            }
        }
    }

    pub async fn delete_comment(&self, id: i64) -> Result<(), DBAccessError> {
        let result = sqlx::query!(
            r#"
                DELETE FROM comments
                WHERE id = $1
            "#,
            id,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::CommentDeleteFailed,
                e.to_string()
            )))
        })?;

        if result.rows_affected() == 0 {
            return Err(DBAccessError::ValidationError(get_error_message(
                ErrorKey::CommentDeleteFailedByIdNotFound,
                format!("ID = {}", id),
            )));
        }

        Ok(())
    }
}

pub async fn get_comment_by_id_with_transaction(
    id: i64,
    transaction: &mut Transaction<'_, Sqlite>,
) -> Result<Option<Comment>, DBAccessError> {
    sqlx::query_as!(
        Comment,
        r#"
            SELECT id, user_id, task_id, content, created_at, updated_at
            FROM comments
            WHERE id = $1
        "#,
        id,
    )
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|e| {
        DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
            ErrorKey::CommentGetByIdFailed,
            e.to_string()
        )))
    })
}

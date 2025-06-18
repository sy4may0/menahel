use crate::enums::TaskLevel;
use crate::errors::db_error::DBAccessError;
use crate::errors::messages::{ErrorKey, get_error_message};
use crate::models::Comment;
use crate::models::repository_model::comment::CommentWithUser;
use crate::repository::task_repo::get_task_by_id_with_transaction;
use crate::repository::user_repo::get_user_by_id_with_transaction;
use crate::repository::validations::{
    validate_comment_content, validate_comment_id, validate_comment_id_is_none,
    validate_comment_task_id, validate_comment_user_id, validate_pagination,
};
use anyhow::Result;
use chrono::Utc;
use sqlx::{Pool, Sqlite, Transaction};

pub struct CommentRepository {
    pool: Pool<Sqlite>,
}

pub enum CommentFilterValue {
    I64(i64),
    String(String),
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

    async fn validate_comment_id_is_exist(
        &self,
        id: i64,
        tx: &mut Transaction<'_, Sqlite>,
    ) -> Result<(), DBAccessError> {
        let result = get_comment_by_id_with_transaction(id, tx).await?;

        match result {
            Some(_) => Ok(()),
            None => Err(DBAccessError::NotFoundError(get_error_message(
                ErrorKey::CommentIdNotFound,
                format!("ID = {}", id),
            ))),
        }
    }

    pub async fn create_comment(&self, comment: Comment) -> Result<Comment, DBAccessError> {
        validate_comment_id_is_none(comment.comment_id)?;
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
                RETURNING comment_id, user_id, task_id, content, created_at, updated_at
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

    pub async fn get_all_comments(&self) -> Result<Vec<CommentWithUser>, DBAccessError> {
        let result = sqlx::query_as::<_, CommentWithUser>(
            r#"
                SELECT comments.comment_id, comments.user_id, comments.task_id, comments.content, comments.created_at, comments.updated_at,
                COALESCE(
                    json_object(
                        'user_id', users.user_id,
                        'username', users.username,
                        'email', users.email
                    ), '{}'
                ) AS user
            FROM comments
            INNER JOIN users ON comments.user_id = users.user_id
            ORDER BY comments.comment_id
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::CommentGetAllFailed,
                e.to_string()
            )))
        })?;
        log::debug!("Got all comments: {:?}", result);

        Ok(result)
    }

    pub async fn get_comments_with_pagination(
        &self,
        page: &i32,
        page_size: &i32,
    ) -> Result<Vec<CommentWithUser>, DBAccessError> {
        let mut tx = self.pool.begin().await.map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::CommentGetAllFailed,
                e.to_string()
            )))
        })?;

        let count = get_comment_count_with_transaction(&mut tx).await?;
        validate_pagination(Some(page), Some(page_size), &count)?;

        let offset = (*page - 1) * *page_size;
        let limit = *page_size;
        log::debug!(
            "Get comments with pagination: offset: {}, limit: {}",
            offset,
            limit
        );

        let result = sqlx::query_as::<_, CommentWithUser>(
            r#"
                SELECT comments.comment_id, comments.user_id, comments.task_id, comments.content, comments.created_at, comments.updated_at,
                COALESCE(
                    json_object(
                        'user_id', users.user_id,
                        'username', users.username,
                        'email', users.email
                    ), '{}'
                ) AS user
                FROM comments
                INNER JOIN users ON comments.user_id = users.user_id
                ORDER BY comments.comment_id
                LIMIT $1 OFFSET $2
            "#
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::CommentGetAllFailed,
                e.to_string()
            )))
        })?;

        tx.commit().await.map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::CommentGetAllFailed,
                e.to_string()
            )))
        })?;

        Ok(result)
    }

    pub async fn get_comment_by_id(&self, id: i64) -> Result<CommentWithUser, DBAccessError> {
        validate_comment_id(Some(id))?;

        let result = sqlx::query_as::<_, CommentWithUser>(
            r#"
                SELECT comments.comment_id, comments.user_id, comments.task_id, comments.content, comments.created_at, comments.updated_at,
                COALESCE(
                    json_object(
                        'user_id', users.user_id,
                        'username', users.username,
                        'email', users.email
                    ), '{}'
                ) AS user
                FROM comments
                INNER JOIN users ON comments.user_id = users.user_id
                WHERE comment_id = $1
            "#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::CommentGetByIdFailed,
                e.to_string()
            )))
        })?;

        match result {
            Some(comment) => Ok(comment),
            None => Err(DBAccessError::NotFoundError(get_error_message(
                ErrorKey::CommentGetByIdNotFound,
                format!("ID = {}", id),
            ))),
        }
    }

    pub async fn get_comments_with_pagination_by_task_id(
        &self,
        task_id: i64,
        page: &i32,
        page_size: &i32,
    ) -> Result<Vec<CommentWithUser>, DBAccessError> {
        validate_comment_task_id(task_id)?;

        let mut tx = self.pool.begin().await.map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::CommentGetByIdFailed,
                e.to_string()
            )))
        })?;

        let count = get_comment_count_by_task_id_with_transaction(task_id, &mut tx).await?;
        validate_pagination(Some(page), Some(page_size), &count)?;

        let offset = (*page - 1) * *page_size;
        let limit = *page_size;
        log::debug!(
            "Get comments with pagination by task id: offset: {}, limit: {}",
            offset,
            limit
        );

        let result = sqlx::query_as::<_, CommentWithUser>(
            r#"
                SELECT comments.comment_id, comments.user_id, comments.task_id, comments.content, comments.created_at, comments.updated_at,
                COALESCE(
                    json_object(
                        'user_id', users.user_id,
                        'username', users.username,
                        'email', users.email
                    ), '{}'
                ) AS user
                FROM comments
                INNER JOIN users ON comments.user_id = users.user_id
                WHERE comments.task_id = $1
                ORDER BY comments.comment_id
                LIMIT $2 OFFSET $3
            "#
        )
        .bind(task_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::CommentGetByTaskIdFailed,
                e.to_string()
            )))
        })?;

        tx.commit().await.map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::CommentGetByTaskIdFailed,
                e.to_string()
            )))
        })?;

        Ok(result)
    }

    pub async fn get_comment_by_task_id(
        &self,
        task_id: i64,
    ) -> Result<Vec<CommentWithUser>, DBAccessError> {
        validate_comment_task_id(task_id)?;

        sqlx::query_as::<_, CommentWithUser>(
            r#"
                SELECT comments.comment_id, comments.user_id, comments.task_id, comments.content, comments.created_at, comments.updated_at,
                COALESCE(
                    json_object(
                        'user_id', users.user_id,
                        'username', users.username,
                        'email', users.email
                    ), '{}'
                ) AS user
                FROM comments
                INNER JOIN users ON comments.user_id = users.user_id
                WHERE task_id = $1
            "#
        )
        .bind(task_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::CommentGetByTaskIdFailed,
                e.to_string()
            )))
        })
    }

    pub async fn get_comments_with_pagination_by_user_id(
        &self,
        user_id: i64,
        page: &i32,
        page_size: &i32,
    ) -> Result<Vec<CommentWithUser>, DBAccessError> {
        validate_comment_user_id(user_id)?;
        let mut tx = self.pool.begin().await.map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::CommentGetByUserIdFailed,
                e.to_string()
            )))
        })?;

        let count = get_comment_count_by_user_id_with_transaction(user_id, &mut tx).await?;
        validate_pagination(Some(page), Some(page_size), &count)?;

        let offset = (*page - 1) * *page_size;
        let limit = *page_size;
        log::debug!(
            "Get comments with pagination by user id: offset: {}, limit: {}",
            offset,
            limit
        );

        let result = sqlx::query_as::<_, CommentWithUser>(
            r#"
                SELECT comments.comment_id, comments.user_id, comments.task_id, comments.content, comments.created_at, comments.updated_at,
                COALESCE(
                    json_object(
                        'user_id', users.user_id,
                        'username', users.username,
                        'email', users.email
                    ), '{}'
                ) AS user
                FROM comments
                INNER JOIN users ON comments.user_id = users.user_id
                WHERE comments.user_id = $1
                ORDER BY comments.comment_id
                LIMIT $2 OFFSET $3
            "#
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::CommentGetByUserIdFailed,
                e.to_string()
            )))
        })?;

        tx.commit().await.map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::CommentGetByUserIdFailed,
                e.to_string()
            )))
        })?;

        Ok(result)
    }

    pub async fn get_comment_by_user_id(
        &self,
        user_id: i64,
    ) -> Result<Vec<CommentWithUser>, DBAccessError> {
        validate_comment_user_id(user_id)?;
        sqlx::query_as::<_, CommentWithUser>(
            r#"
                SELECT comments.comment_id, comments.user_id, comments.task_id, comments.content, comments.created_at, comments.updated_at,
                COALESCE(
                    json_object(
                        'user_id', users.user_id,
                        'username', users.username,
                        'email', users.email
                    ), '{}'
                ) AS user
                FROM comments
                INNER JOIN users ON comments.user_id = users.user_id
                WHERE comments.user_id = $1
            "#
        )
        .bind(user_id)
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
        validate_comment_id(comment.comment_id)?;
        validate_comment_user_id(comment.user_id)?;
        validate_comment_task_id(comment.task_id)?;
        validate_comment_content(&comment.content)?;

        let mut tx = self.pool.begin().await?;

        self.validate_target_user_and_task(&comment, &mut tx)
            .await?;

        let comment_id = match comment.comment_id {
            Some(id) => id,
            None => {
                return Err(DBAccessError::ValidationError(get_error_message(
                    ErrorKey::CommentIdInvalid,
                    format!("ID = {:?}", comment.comment_id),
                )));
            }
        };
        self.validate_comment_id_is_exist(comment_id, &mut tx)
            .await?;

        let now = Utc::now().timestamp();
        let result = sqlx::query_as!(
            Comment,
            r#"
                UPDATE comments
                SET content = $1, user_id = $2, task_id = $3, updated_at = $4
                WHERE comment_id = $5
                RETURNING comment_id, user_id, task_id, content, created_at, updated_at
            "#,
            comment.content,
            comment.user_id,
            comment.task_id,
            now,
            comment.comment_id,
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
        validate_comment_id(Some(id))?;

        let result = sqlx::query!(
            r#"
                DELETE FROM comments
                WHERE comment_id = $1
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
            return Err(DBAccessError::NotFoundError(get_error_message(
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
    validate_comment_id(Some(id))?;
    sqlx::query_as!(
        Comment,
        r#"
            SELECT comment_id, user_id, task_id, content, created_at, updated_at
            FROM comments
            WHERE comment_id = $1
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

pub async fn get_comment_count_with_transaction(
    transaction: &mut Transaction<'_, Sqlite>,
) -> Result<i64, DBAccessError> {
    let result = sqlx::query_scalar!(
        r#"
            SELECT COUNT(*) FROM comments
        "#,
    )
    .fetch_one(&mut **transaction)
    .await
    .map_err(|e| {
        DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
            ErrorKey::CommentGetCountFailed,
            e.to_string()
        )))
    })?;

    log::debug!("Got comment count: {:?}", result);
    Ok(result)
}

pub async fn get_comment_count_by_task_id_with_transaction(
    task_id: i64,
    transaction: &mut Transaction<'_, Sqlite>,
) -> Result<i64, DBAccessError> {
    validate_comment_task_id(task_id)?;
    let result = sqlx::query_scalar!(
        r#"
            SELECT COUNT(*) FROM comments WHERE task_id = $1
            "#,
        task_id,
    )
    .fetch_one(&mut **transaction)
    .await
    .map_err(|e| {
        DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
            ErrorKey::CommentGetCountFailed,
            e.to_string()
        )))
    })?;

    log::debug!("Got comment count by task id: {:?}", result);
    Ok(result)
}

pub async fn get_comment_count_by_user_id_with_transaction(
    user_id: i64,
    transaction: &mut Transaction<'_, Sqlite>,
) -> Result<i64, DBAccessError> {
    validate_comment_user_id(user_id)?;
    let result = sqlx::query_scalar!(
        r#"
            SELECT COUNT(*) FROM comments WHERE user_id = $1
            "#,
        user_id,
    )
    .fetch_one(&mut **transaction)
    .await
    .map_err(|e| {
        DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
            ErrorKey::CommentGetCountFailed,
            e.to_string()
        )))
    })?;

    log::debug!("Got comment count by user id: {:?}", result);
    Ok(result)
}

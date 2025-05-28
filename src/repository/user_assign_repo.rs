use sqlx::{Pool, Sqlite};
use crate::models::UserAssign;
use crate::enums::get_max_task_level;
use crate::errors::db_error::DBAccessError;
use crate::errors::messages::{get_error_message, ErrorKey};
use crate::repository::user_repo::UserRepository;
use crate::repository::task_repo::TaskRepository;
use crate::utils::{validate_user_assign_user_id, validate_user_assign_task_id};

pub struct UserAssignRepository {
    pool: Pool<Sqlite>,
    user_repo: UserRepository,
    task_repo: TaskRepository,
}

impl UserAssignRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        let user_repo = UserRepository::new(pool.clone());
        let task_repo = TaskRepository::new(pool.clone());
        Self { pool, user_repo, task_repo }
    }

    async fn validate_target_user_and_task(&self, user_assign: &UserAssign) -> Result<(), DBAccessError> {
        let user = self.user_repo.get_user_by_id(user_assign.user_id).await?;
        if user.is_none() {
            return Err(DBAccessError::QueryError(anyhow::anyhow!(get_error_message(ErrorKey::UserAssignUserIdNotFound, format!("ID = {}", user_assign.user_id)))));
        }
        let task = self.task_repo.get_task_by_id(user_assign.task_id).await?;
        if task.is_none() {
            return Err(DBAccessError::QueryError(anyhow::anyhow!(get_error_message(ErrorKey::UserAssignTaskIdNotFound, format!("ID = {}", user_assign.task_id)))));
        }
        Ok(())
    }

    async fn validate_user_assign_to_not_max_level_task(&self, user_assign: &UserAssign) -> Result<(), DBAccessError> {
        let task = self.task_repo.get_task_by_id(user_assign.task_id).await?;
        if task.is_none() {
            return Err(DBAccessError::QueryError(anyhow::anyhow!(get_error_message(ErrorKey::UserAssignTaskIdNotFound, format!("ID = {}", user_assign.task_id)))));
        }

        if task.unwrap().level != get_max_task_level() as i64 {
            return Err(DBAccessError::QueryError(anyhow::anyhow!(get_error_message(ErrorKey::UserAssignToNotMaxLevelTask, format!("ID = {}", user_assign.task_id)))));
        }

        Ok(())
    }

    async fn validate_user_assign_same_user_assign_exists(&self, user_assign: &UserAssign) -> Result<(), DBAccessError> {
        let user_assigns = self.get_user_assign_by_task_id(user_assign.task_id).await?;
        if user_assigns.iter().any(|assign| assign.user_id == user_assign.user_id) {
            return Err(DBAccessError::QueryError(anyhow::anyhow!(get_error_message(ErrorKey::UserAssignSameUserAssignExists, format!("ID = {}", user_assign.task_id)))));
        }
        Ok(())
    }

    pub async fn create_user_assign(&self, user_assign: UserAssign) -> Result<UserAssign, DBAccessError> {
        validate_user_assign_user_id(user_assign.user_id)?;
        validate_user_assign_task_id(user_assign.task_id)?;

        self.validate_target_user_and_task(&user_assign).await?;
        self.validate_user_assign_to_not_max_level_task(&user_assign).await?;
        self.validate_user_assign_same_user_assign_exists(&user_assign).await?;

        sqlx::query_as!(
            UserAssign,
            r#"
                INSERT INTO user_assign (user_id, task_id)
                VALUES ($1, $2)
                RETURNING id, user_id, task_id
            "#,
            user_assign.user_id,
            user_assign.task_id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DBAccessError::QueryError(anyhow::anyhow!(get_error_message(ErrorKey::UserAssignCreateFailed, e.to_string()))))
    }

    pub async fn get_user_assign_by_id(&self, id: i64) -> Result<Option<UserAssign>, DBAccessError> {
        sqlx::query_as!(
            UserAssign,
            r#"
                SELECT id, user_id, task_id
                FROM user_assign
                WHERE id = $1
            "#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DBAccessError::QueryError(anyhow::anyhow!(get_error_message(ErrorKey::UserAssignGetByIdFailed, e.to_string()))))
    }

    pub async fn get_user_assign_by_task_id(&self, task_id: i64) -> Result<Vec<UserAssign>, DBAccessError> {
        sqlx::query_as!(
            UserAssign,
            r#"
                SELECT id, user_id, task_id
                FROM user_assign
                WHERE task_id = $1
            "#,
            task_id,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DBAccessError::QueryError(anyhow::anyhow!(get_error_message(ErrorKey::UserAssignGetByTaskIdFailed, e.to_string()))))
    }

    pub async fn get_user_assign_by_user_id(&self, user_id: i64) -> Result<Vec<UserAssign>, DBAccessError> {
        sqlx::query_as!(
            UserAssign,
            r#"
                SELECT id, user_id, task_id
                FROM user_assign
                WHERE user_id = $1
            "#,
            user_id,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DBAccessError::QueryError(anyhow::anyhow!(get_error_message(ErrorKey::UserAssignGetByUserIdFailed, e.to_string()))))
    }

    pub async fn get_user_assign_by_user_id_and_task_id(&self, user_id: i64, task_id: i64) -> Result<Option<UserAssign>, DBAccessError> {
        sqlx::query_as!(
            UserAssign,
            r#"
                SELECT id, user_id, task_id
                FROM user_assign
                WHERE user_id = $1 AND task_id = $2
            "#,
            user_id, task_id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DBAccessError::QueryError(anyhow::anyhow!(get_error_message(ErrorKey::UserAssignGetByUserIdAndTaskIdFailed, e.to_string()))))
    }

    pub async fn get_all_user_assigns(&self) -> Result<Vec<UserAssign>, DBAccessError> {
        sqlx::query_as!(
            UserAssign,
            r#"
                SELECT id, user_id, task_id
                FROM user_assign
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DBAccessError::QueryError(anyhow::anyhow!(get_error_message(ErrorKey::UserAssignGetAllFailed, e.to_string()))))
    }

    pub async fn update_user_assign(&self, user_assign: UserAssign) -> Result<UserAssign, DBAccessError> {
        validate_user_assign_user_id(user_assign.user_id)?;
        validate_user_assign_task_id(user_assign.task_id)?;

        self.validate_target_user_and_task(&user_assign).await?;
        self.validate_user_assign_to_not_max_level_task(&user_assign).await?;
        self.validate_user_assign_same_user_assign_exists(&user_assign).await?;

        sqlx::query_as!(
            UserAssign,
            r#"
                UPDATE user_assign
                SET user_id = $1, task_id = $2
                WHERE id = $3
                RETURNING id, user_id, task_id
            "#,
            user_assign.user_id,
            user_assign.task_id,
            user_assign.id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DBAccessError::QueryError(anyhow::anyhow!(get_error_message(ErrorKey::UserAssignUpdateFailed, e.to_string()))))
    }

    pub async fn delete_user_assign(&self, id: i64) -> Result<(), DBAccessError> {
        let result = sqlx::query!(
            r#"
                DELETE FROM user_assign
                WHERE id = $1
            "#,
            id,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DBAccessError::QueryError(anyhow::anyhow!(get_error_message(ErrorKey::UserAssignDeleteFailed, e.to_string()))))?;

        if result.rows_affected() == 0 {
            return Err(DBAccessError::QueryError(anyhow::anyhow!(get_error_message(ErrorKey::UserAssignDeleteFailedByIdNotFound, format!("ID = {}", id)))));
        }

        Ok(())
    }
}
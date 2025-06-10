use crate::errors::db_error::DBAccessError;
use crate::errors::messages::{ErrorKey, get_error_message};
use crate::models::User;
use crate::repository::validations::{
    validate_pagination, validate_user_email, validate_user_id, validate_user_id_is_none, validate_user_name, validate_user_password
};
use sqlx::{Pool, Sqlite, Transaction};

pub struct UserRepository {
    pool: Pool<Sqlite>,
}

impl UserRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, user: User) -> Result<User, DBAccessError> {
        validate_user_id(user.id)?;
        validate_user_id_is_none(user.id)?;
        validate_user_name(&user.username)?;
        validate_user_email(&user.email)?;
        validate_user_password(&user.password_hash)?;

        let result = sqlx::query_as!(
            User,
            r#"
                INSERT INTO users (username, email, password_hash)
                VALUES ($1, $2, $3)
                RETURNING id, username, email, password_hash
            "#,
            user.username,
            user.email,
            user.password_hash,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::UserCreateFailed,
                e.to_string()
            )))
        })?;
        log::info!("Created user: {:?}", result);

        Ok(result)
    }

    pub async fn get_user_by_id(&self, id: i64) -> Result<User, DBAccessError> {
        let result = sqlx::query_as!(
            User,
            r#"
                SELECT id, username, email, password_hash
                FROM users
                WHERE id = $1
           "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::UserGetByIdFailed,
                e.to_string()
            )))
        })?;
        log::debug!("Get user by id: {:?}", result);

        match result {
            Some(user) => Ok(user),
            None => Err(DBAccessError::NotFoundError(get_error_message(
                ErrorKey::UserGetByIdNotFound,
                format!("ID = {}", id),
            )))
        }
    }

    pub async fn get_user_by_name(&self, name: &str) -> Result<User, DBAccessError> {
        let result = sqlx::query_as!(
            User,
            r#"
                SELECT id, username, email, password_hash
                FROM users
                WHERE username = $1
            "#,
            name
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::UserGetByNameFailed,
                e.to_string()
            )))
        })?;
        log::debug!("Get user by name: {:?}", result);

        match result {
            Some(user) => Ok(user),
            None => Err(DBAccessError::NotFoundError(get_error_message(
                ErrorKey::UserGetByNameNotFound,
                format!("Name = {}", name),
            )))
        }
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, DBAccessError> {
        let result = sqlx::query_as!(
            User,
            r#"
                SELECT id, username, email, password_hash
                FROM users
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::UserGetAllFailed,
                e.to_string()
            )))
        })?;
        log::debug!("Get all users: {:?}", result);

        Ok(result)
    }

    pub async fn get_users_count(&self) -> Result<i64, DBAccessError> {
        let result = sqlx::query_scalar!(
            r#"
                SELECT COUNT(*) FROM users
            "#,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::UserGetUsersCountFailed,
                e.to_string()
            )))
        })?;
        log::debug!("Get users count: {:?}", result);

        Ok(result)
    }

    pub async fn get_users_with_pagination(
        &self,
        page: &i32,
        page_size: &i32,
    ) -> Result<Vec<User>, DBAccessError> {
        validate_pagination(Some(page), Some(page_size))?;
        let offset = (page - 1) * page_size;
        let limit = page_size;

        let mut tx = self.pool.begin().await.map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::ProjectGetAllFailed,
                e.to_string()
            )))
        })?;
        let count = get_users_count_with_transaction(&mut tx).await?;

        if offset as i64 >= count {
            return Err(DBAccessError::NotFoundError(get_error_message(
                ErrorKey::UserGetUsersPaginationNotFound,
                format!("Offset: {}, Count: {}", offset, count),
            )));
        }
        log::debug!("Get users with pagination: offset: {}, limit: {}", offset, limit);

        let result = sqlx::query_as!(
            User,
            r#"
                SELECT id, username, email, password_hash
                FROM users
                ORDER BY id
                LIMIT $1 OFFSET $2
            "#,
            limit,
            offset,
        )
        .fetch_all(&mut *tx)
        .await;

        match result {
            Ok(users) => {
                log::debug!("Get users with pagination: {:?}", users);
                tx.commit().await.map_err(|e| {
                    DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                        ErrorKey::UserGetAllFailed,
                        e.to_string()
                    )))
                })?;
                Ok(users)
            }
            Err(e) => {
                let rollback = tx.rollback().await;
                match rollback {
                    Ok(_) => {
                        log::warn!("Transaction rolled back");
                    }
                    Err(e) => {
                        log::error!("Transaction rollback failed: {:?}", e);
                    }
                }
                Err(DBAccessError::QueryError(anyhow::anyhow!(
                    get_error_message(ErrorKey::UserGetAllFailed, e.to_string())
                )))
            }
        }
    }

    pub async fn update_user(&self, user: User) -> Result<User, DBAccessError> {
        validate_user_id(user.id)?;
        validate_user_name(&user.username)?;
        validate_user_email(&user.email)?;
        validate_user_password(&user.password_hash)?;

        if user.id.is_none() {
            return Err(DBAccessError::ValidationError(get_error_message(
                ErrorKey::UserIdInvalid,
                format!("ID = {:?}", user.id),
            )));
        }

        let mut tx = self.pool.begin().await?;
        let _ = get_user_by_id_with_transaction(&user.id.unwrap(), &mut tx).await?;

        let result = sqlx::query_as!(
            User,
            r#"
                UPDATE users
                SET username = $1, email = $2, password_hash = $3
                WHERE id = $4
                RETURNING id, username, email, password_hash
            "#,
            user.username,
            user.email,
            user.password_hash,
            user.id,
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::UserUpdateFailed,
                e.to_string()
            )))
        })?;

        tx.commit().await.map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::UserUpdateFailed,
                e.to_string()
            )))
        })?;

        log::info!("Updated user: {:?}", result);

        match result {
            Some(user) => Ok(user),
            None => Err(DBAccessError::NotFoundError(get_error_message(
                ErrorKey::UserGetByIdNotFound,
                format!("ID = {:?}", user.id),
            )))
        }
    }

    pub async fn delete_user(&self, id: i64) -> Result<(), DBAccessError> {
        validate_user_id(Some(id))?;
        let result = sqlx::query!(
            r#"
                DELETE FROM users
                WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::UserDeleteFailedByIdNotFound,
                e.to_string()
            )))
        })?;

        if result.rows_affected() == 0 {
            return Err(DBAccessError::NotFoundError(get_error_message(
                ErrorKey::UserDeleteFailedByIdNotFound,
                format!("ID = {}", id),
            )));
        }

        log::info!("Deleted user: {:?}", result);

        Ok(())
    }
}

pub async fn get_user_by_id_with_transaction(
    id: &i64,
    tx: &mut Transaction<'_, Sqlite>,
) -> Result<User, DBAccessError> {
    let result = sqlx::query_as!(
        User,
        r#"
            SELECT id, username, email, password_hash
            FROM users
            WHERE id = $1
        "#,
        id
    )
    .fetch_optional(&mut **tx)
    .await
    .map_err(|e| {
        DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
            ErrorKey::UserGetByIdFailed,
            e.to_string()
        )))
    })?;
    log::debug!("Get user by id with transaction: {:?}", result);

    match result {
        Some(user) => Ok(user),
        None => Err(DBAccessError::NotFoundError(get_error_message(
            ErrorKey::UserGetByIdNotFound,
            format!("ID = {}", id),
        )))
    }
}

pub async fn get_users_count_with_transaction(
    tx: &mut Transaction<'_, Sqlite>,
) -> Result<i64, DBAccessError> {
    let result = sqlx::query_scalar!(
        r#"
            SELECT COUNT(*) FROM users
        "#,
    )
    .fetch_one(&mut **tx)
    .await
    .map_err(|e| {
        DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
            ErrorKey::UserGetUsersCountFailed,
            e.to_string()
        )))
    })?;
    log::debug!("Get users count with transaction: {:?}", result);

    Ok(result)
}
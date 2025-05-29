use crate::errors::db_error::DBAccessError;
use crate::errors::messages::{ErrorKey, get_error_message};
use crate::models::User;
use crate::repository::validations::{
    validate_user_email, validate_user_id, validate_user_name, validate_user_password,
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
        validate_user_name(&user.username)?;
        validate_user_email(&user.email)?;
        validate_user_password(&user.password_hash)?;

        sqlx::query_as!(
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
        })
    }

    pub async fn get_user_by_id(&self, id: i64) -> Result<Option<User>, DBAccessError> {
        sqlx::query_as!(
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
        })
    }

    pub async fn get_user_by_name(&self, name: &str) -> Result<Option<User>, DBAccessError> {
        sqlx::query_as!(
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
        })
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, DBAccessError> {
        sqlx::query_as!(
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
        })
    }

    pub async fn update_user(&self, user: User) -> Result<User, DBAccessError> {
        validate_user_id(user.id)?;
        validate_user_name(&user.username)?;
        validate_user_email(&user.email)?;
        validate_user_password(&user.password_hash)?;

        sqlx::query_as!(
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
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::UserUpdateFailed,
                e.to_string()
            )))
        })
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
            return Err(DBAccessError::ValidationError(get_error_message(
                ErrorKey::UserDeleteFailedByIdNotFound,
                format!("ID = {}", id),
            )));
        }

        Ok(())
    }
}

pub async fn get_user_by_id_with_transaction(
    id: i64,
    tx: &mut Transaction<'_, Sqlite>,
) -> Result<Option<User>, DBAccessError> {
    sqlx::query_as!(
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
    })
}

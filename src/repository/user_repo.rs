use crate::models::User;
use sqlx::{Pool, Sqlite};
use anyhow::Result;
use crate::utils::{validate_user_name, validate_user_email, validate_user_password};

pub struct UserRepository {
    pool: Pool<Sqlite>,
}

impl UserRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, user: User) -> Result<User, anyhow::Error> {
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
        .map_err(anyhow::Error::from)
    }

    pub async fn get_user_by_id(&self, id: i64) -> Result<Option<User>, anyhow::Error> {
        sqlx::query_as!(
            User,
            r#"
                SELECT id, username, email, password_hash
                FROM users
                WHERE id = $1
           "#, id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(anyhow::Error::from)
    }

    pub async fn get_user_by_name(&self, name: &str) -> Result<Option<User>, anyhow::Error> {
        sqlx::query_as!(
            User,
            r#"
                SELECT id, username, email, password_hash
                FROM users
                WHERE username = $1
            "#, name
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(anyhow::Error::from)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, anyhow::Error> {
        sqlx::query_as!(
            User,
            r#"
                SELECT id, username, email, password_hash
                FROM users
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(anyhow::Error::from)
    }

    pub async fn update_user(&self, user: User) -> Result<User, anyhow::Error> {
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
        .map_err(anyhow::Error::from)
    }

    pub async fn delete_user(&self, id: i64) -> Result<(), anyhow::Error> {
        let result = sqlx::query!(
            r#"
                DELETE FROM users
                WHERE id = $1
            "#, id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(anyhow::anyhow!("User not found"));
        }

        Ok(())
    }
}
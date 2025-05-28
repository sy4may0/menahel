use crate::models::Project;
use sqlx::{Pool, Sqlite};
use anyhow::Result;
use crate::utils::{validate_project_id, validate_project_name};
use crate::errors::db_error::DBAccessError;
use crate::errors::messages::{get_error_message, ErrorKey};

pub struct ProjectRepository {
    pool: Pool<Sqlite>,
}

impl ProjectRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    pub async fn create_project(&self, project: Project) -> Result<Project, DBAccessError> {
        validate_project_id(project.id)?;
        validate_project_name(&project.name)?;
        sqlx::query_as!(
            Project,
            r#"
                INSERT INTO projects (name)
                VALUES ($1)
                RETURNING id, name
            "#,
            project.name,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DBAccessError::QueryError(anyhow::anyhow!(get_error_message(ErrorKey::ProjectCreateFailed, e.to_string()))))
    }

    pub async fn get_project_by_id(&self, id: i64) -> Result<Option<Project>, DBAccessError> {
        sqlx::query_as!(
            Project,
            r#"
                SELECT id, name
                FROM projects
                WHERE id = $1
            "#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DBAccessError::QueryError(anyhow::anyhow!(get_error_message(ErrorKey::ProjectGetByIdFailed, e.to_string()))))
    }

    pub async fn get_project_by_name(&self, name: &str) -> Result<Option<Project>, DBAccessError> {
        sqlx::query_as!(
            Project,
            r#"
                SELECT id, name
                FROM projects
                WHERE name = $1
            "#,
            name,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DBAccessError::QueryError(anyhow::anyhow!(get_error_message(ErrorKey::ProjectGetByNameFailed, e.to_string()))))
    }

    pub async fn get_all_projects(&self) -> Result<Vec<Project>, DBAccessError> {
        sqlx::query_as!(
            Project,
            r#"
                SELECT id, name
                FROM projects
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DBAccessError::QueryError(anyhow::anyhow!(get_error_message(ErrorKey::ProjectGetAllFailed, e.to_string()))))
    }

    pub async fn update_project(&self, project: Project) -> Result<Project, DBAccessError> {
        validate_project_id(project.id)?;
        validate_project_name(&project.name)?;

        sqlx::query_as!(
            Project,
            r#"
                UPDATE projects
                SET name = $1
                WHERE id = $2
                RETURNING id, name
            "#,
            project.name,
            project.id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DBAccessError::QueryError(anyhow::anyhow!(get_error_message(ErrorKey::ProjectUpdateFailed, e.to_string()))))
    }

    pub async fn delete_project(&self, id: i64) -> Result<(), DBAccessError> {
        validate_project_id(Some(id))?;

        let result = sqlx::query!(
            r#"
                DELETE FROM projects
                WHERE id = $1
            "#,id,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DBAccessError::QueryError(anyhow::anyhow!(get_error_message(ErrorKey::ProjectDeleteFailedByIdNotFound, e.to_string()))))?;

        if result.rows_affected() == 0 {
            return Err(DBAccessError::QueryError(anyhow::anyhow!(get_error_message(ErrorKey::ProjectDeleteFailedByIdNotFound, format!("ID = {}", id)))));
        }

        Ok(())
    }
}
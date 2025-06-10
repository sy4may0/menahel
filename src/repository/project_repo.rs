use crate::errors::db_error::DBAccessError;
use crate::errors::messages::{ErrorKey, get_error_message};
use crate::models::Project;
use crate::repository::validations::{validate_pagination, validate_project_id, validate_project_name};
use anyhow::Result;
use sqlx::{Pool, Sqlite, Transaction};

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
        let result = sqlx::query_as!(
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
        .map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::ProjectCreateFailed,
                e.to_string()
            )))
        })?;

        log::info!("Created project: {:?}", result);

        Ok(result)
    }

    pub async fn get_project_by_id(&self, id: i64) -> Result<Project, DBAccessError> {
        let result = sqlx::query_as!(
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
        .map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::ProjectGetByIdFailed,
                e.to_string()
            )))
        })?;

        log::debug!("Got project by id: {:?}", result);

        match result {
            Some(project) => Ok(project),
            None => Err(DBAccessError::NotFoundError(
                get_error_message(ErrorKey::ProjectGetByIdNotFound,
                format!("ID = {}", id)
            )))
        }
    }

    pub async fn get_project_by_name(&self, name: &str) -> Result<Project, DBAccessError> {
        let result = sqlx::query_as!(
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
        .map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::ProjectGetByNameFailed,
                e.to_string()
            )))
        })?;

        log::debug!("Got project by name: {:?}", result);

        match result {
            Some(project) => Ok(project),
            None => Err(DBAccessError::NotFoundError(
                get_error_message(ErrorKey::ProjectGetByNameNotFound,
                format!("Name = {}", name)
            )))
        }
    }

    pub async fn get_all_projects(&self) -> Result<Vec<Project>, DBAccessError> {
        let result = sqlx::query_as!(
            Project,
            r#"
                SELECT id, name
                FROM projects
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::ProjectGetAllFailed,
                e.to_string()
            )))
        })?;

        log::debug!("Got all projects: {:?}", result);

        Ok(result)
    }

    pub async fn get_projects_count(&self) -> Result<i64, DBAccessError> {
        let result = sqlx::query_scalar!(
            r#"
                SELECT COUNT(*) FROM projects
            "#,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::ProjectGetProjectsCountFailed,
                e.to_string()
            )))
        })?;
        log::debug!("Got projects count: {:?}", result);

        Ok(result)
    }

    pub async fn get_projects_with_pagination(
        &self,
        page: &i32,
        page_size: &i32,
    ) -> Result<Vec<Project>, DBAccessError> {
        validate_pagination(Some(page), Some(page_size))?;

        let offset = (*page - 1) * *page_size;
        let limit = *page_size;

        let mut tx = self.pool.begin().await.map_err(
            |e| {
                DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                    ErrorKey::ProjectGetAllFailed,
                    e.to_string()
                )))
            }
        )?;
        let count = get_projects_count_with_transaction(&mut tx).await?;

        if offset as i64 > count {
            return Err(DBAccessError::NotFoundError(get_error_message(
                ErrorKey::ProjectGetPaginationNotFound,
                format!("Offset = {}, Count = {}", offset, count)
            )));
        }
        log::debug!("Get users with pagination: offset: {}, limit: {}", offset, limit);

        let result = sqlx::query_as!(
            Project,
            r#"
                SELECT id, name
                FROM projects
                ORDER BY id
                LIMIT $1 OFFSET $2
            "#,
            limit,
            offset,
        )
        .fetch_all(&mut *tx)
        .await;

        match result {
            Ok(projects) => {
                log::debug!("Got projects with pagination: {:?}", projects);
                tx.commit().await.map_err(|e| {
                    DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                        ErrorKey::ProjectGetAllFailed,
                        e.to_string()
                    )))
                })?;
                Ok(projects)
            }
            Err(e) => {
                let rollback = tx.rollback().await;
                match rollback {
                    Ok(_) => {
                        log::warn!("Transaction rolled back");
                    }
                    Err(e) => {
                        log::error!("Failed to rollback transaction: {:?}", e);
                    }
                }
                Err(DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                    ErrorKey::ProjectGetAllFailed,
                    e.to_string()
                ))))
            }
        }
    }

    pub async fn update_project(&self, project: Project) -> Result<Project, DBAccessError> {
        validate_project_id(project.id)?;
        validate_project_name(&project.name)?;

        let result = sqlx::query_as!(
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
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::ProjectUpdateFailed,
                e.to_string()
            )))
        })?;

        log::info!("Updated project: {:?}", result);

        match result {
            Some(project) => Ok(project),
            None => Err(DBAccessError::NotFoundError(
                get_error_message(ErrorKey::ProjectUpdateFailed,
                format!("ID = {:?}", project.id)
            )))
        }
    }

    pub async fn delete_project(&self, id: i64) -> Result<(), DBAccessError> {
        validate_project_id(Some(id))?;

        let result = sqlx::query!(
            r#"
                DELETE FROM projects
                WHERE id = $1
            "#,
            id,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::ProjectDeleteFailedByIdNotFound,
                e.to_string()
            )))
        })?;

        if result.rows_affected() == 0 {
            return Err(DBAccessError::ValidationError(get_error_message(
                ErrorKey::ProjectDeleteFailedByIdNotFound,
                format!("ID = {}", id),
            )));
        }

        log::info!("Deleted project: ID = {}", id);

        Ok(())
    }
}

pub async fn get_project_by_id_with_transaction(
    id: i64,
    transaction: &mut Transaction<'_, Sqlite>,
) -> Result<Option<Project>, DBAccessError> {
    sqlx::query_as!(
        Project,
        r#"
            SELECT id, name
            FROM projects
            WHERE id = $1
        "#,
        id,
    )
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|e| {
        DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
            ErrorKey::ProjectGetByIdFailed,
            e.to_string()
        )))
    })
}

pub async fn get_projects_count_with_transaction(
    transaction: &mut Transaction<'_, Sqlite>,
) -> Result<i64, DBAccessError> {
    let result = sqlx::query_scalar!(
        r#"
            SELECT COUNT(*) FROM projects
        "#,
    )
    .fetch_one(&mut **transaction)
    .await
    .map_err(|e| {
        DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
            ErrorKey::ProjectGetProjectsCountFailed,
            e.to_string()
        )))
    })?;

    log::debug!("Got projects count: {:?}", result);

    Ok(result)
}

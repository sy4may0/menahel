use crate::models::Project;
use sqlx::{Pool, Sqlite};
use anyhow::Result;
use crate::utils::validate_project_name;

pub struct ProjectRepository {
    pool: Pool<Sqlite>,
}

impl ProjectRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    pub async fn create_project(&self, project: Project) -> Result<Project> {
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
        .map_err(anyhow::Error::from)
    }

    pub async fn get_project_by_id(&self, id: i64) -> Result<Option<Project>> {
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
        .map_err(anyhow::Error::from)
    }

    pub async fn get_project_by_name(&self, name: &str) -> Result<Option<Project>> {
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
        .map_err(anyhow::Error::from)
    }

    pub async fn get_all_projects(&self) -> Result<Vec<Project>> {
        sqlx::query_as!(
            Project,
            r#"
                SELECT id, name
                FROM projects
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(anyhow::Error::from)
    }

    pub async fn update_project(&self, project: Project) -> Result<Project> {
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
        .map_err(anyhow::Error::from)
    }

    pub async fn delete_project(&self, id: i64) -> Result<()> {
        let result = sqlx::query!(
            r#"
                DELETE FROM projects
                WHERE id = $1
            "#,id,
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(anyhow::anyhow!("Project not found"));
        }

        Ok(())
    }
}
use crate::models::{Task, TaskFilter};
use sqlx::{Pool, Sqlite};
use anyhow::Result;
use chrono::{Utc};

pub struct TaskRepository {
    pool: Pool<Sqlite>,
}

#[derive(Debug)]
enum FilterValue {
    I64(i64),
    String(String),
}

impl TaskRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    pub async fn create_task(&self, task: Task) -> Result<Task> {
        let now = Utc::now().timestamp();
        sqlx::query_as!(
            Task,
            r#"
                INSERT INTO tasks (project_id, parent_id, level, name, description, status, deadline, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                RETURNING id, project_id, parent_id, level, name, description, status, deadline, created_at, updated_at
            "#,
            task.project_id,
            task.parent_id,
            task.level,
            task.name,
            task.description,
            task.status,
            task.deadline,
            now,
            now,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(anyhow::Error::from)
    }

    pub async fn get_task_by_id(&self, id: i64) -> Result<Option<Task>> {
        sqlx::query_as!(
            Task,
            r#"
                SELECT id, project_id, parent_id, level, name, description, status, deadline, created_at, updated_at
                FROM tasks
                WHERE id = $1
            "#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(anyhow::Error::from)
    }

    pub async fn get_all_tasks(&self) -> Result<Vec<Task>> {
        sqlx::query_as!(
            Task,
            r#"
                SELECT id, project_id, parent_id, level, name, description, status, deadline, created_at, updated_at
                FROM tasks
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(anyhow::Error::from)
    }

    fn build_where_clause(&self, filter: TaskFilter) -> (String, Vec<FilterValue>) {
        let mut where_calses = Vec::new();
        let mut bind_values :Vec<FilterValue> = Vec::new();


        let mut index = 1;
        if filter.project_id.is_some() {
            where_calses.push(format!("project_id = ${}", index));
            bind_values.push(FilterValue::I64(filter.project_id.unwrap()));
            index += 1;
        }
        if filter.parent_id.is_some() {
            where_calses.push(format!("parent_id = ${}", index));
            bind_values.push(FilterValue::I64(filter.parent_id.unwrap()));
            index += 1;
        }
        if filter.level.is_some() {
            where_calses.push(format!("level = ${}", index));
            bind_values.push(FilterValue::I64(filter.level.unwrap()));
            index += 1;
        }
        if filter.name.is_some() {
            where_calses.push(format!("name LIKE '%' || ${} || '%'", index));
            bind_values.push(FilterValue::String(filter.name.unwrap()));
            index += 1;
        }
        if filter.description.is_some() {
            where_calses.push(format!("description LIKE '%' || ${} || '%'", index));
            bind_values.push(FilterValue::String(filter.description.unwrap()));
            index += 1;
        }
        if filter.status.is_some() {
            where_calses.push(format!("status = ${}", index));
            bind_values.push(FilterValue::I64(filter.status.unwrap()));
            index += 1;
        }

        if filter.deadline_from.is_some() && filter.deadline_to.is_some() {
            where_calses.push(format!("deadline >= ${} AND deadline <= ${}", index, index + 1));
            bind_values.push(FilterValue::I64(filter.deadline_from.unwrap()));
            bind_values.push(FilterValue::I64(filter.deadline_to.unwrap()));
            index += 2;
        } else if filter.deadline_from.is_some() && filter.deadline_to.is_none() {
            where_calses.push(format!("deadline >= ${}", index));
            bind_values.push(FilterValue::I64(filter.deadline_from.unwrap()));
            index += 1;
        } else if filter.deadline_to.is_some() && filter.deadline_from.is_none() {
            where_calses.push(format!("deadline <= ${}", index));
            bind_values.push(FilterValue::I64(filter.deadline_to.unwrap()));
            index += 1;
        }

        if filter.created_at_from.is_some() && filter.created_at_to.is_some() {
            where_calses.push(format!("created_at >= ${} AND created_at <= ${}", index, index + 1));
            bind_values.push(FilterValue::I64(filter.created_at_from.unwrap()));
            bind_values.push(FilterValue::I64(filter.created_at_to.unwrap()));
            index += 2;
        } else if filter.created_at_from.is_some() && filter.created_at_to.is_none() {
            where_calses.push(format!("created_at >= ${}", index));
            bind_values.push(FilterValue::I64(filter.created_at_from.unwrap()));
            index += 1;
        } else if filter.created_at_to.is_some() && filter.created_at_from.is_none() {
            where_calses.push(format!("created_at <= ${}", index));
            bind_values.push(FilterValue::I64(filter.created_at_to.unwrap()));
            index += 1;
        }

        if filter.updated_at_from.is_some() && filter.updated_at_to.is_some() {
            where_calses.push(format!("updated_at >= ${} AND updated_at <= ${}", index, index + 1));
            bind_values.push(FilterValue::I64(filter.updated_at_from.unwrap()));
            bind_values.push(FilterValue::I64(filter.updated_at_to.unwrap()));
        } else if filter.updated_at_from.is_some() && filter.updated_at_to.is_none() {
            where_calses.push(format!("updated_at >= ${}", index));
            bind_values.push(FilterValue::I64(filter.updated_at_from.unwrap()));
        } else if filter.updated_at_to.is_some() && filter.updated_at_from.is_none() {
            where_calses.push(format!("updated_at <= ${}", index));
            bind_values.push(FilterValue::I64(filter.updated_at_to.unwrap()));
        }

        if !where_calses.is_empty() {
            (format!(" WHERE {}", where_calses.join(" AND ")), bind_values)
        } else {
            ("".to_string(), bind_values)
        }
    }


    pub async fn get_tasks_by_filter(&self, filter: TaskFilter) -> Result<Vec<Task>> {
        let (where_clause, bind_values) = self.build_where_clause(filter);
        let query = format!(
            "SELECT id, project_id, parent_id, level, name, description, status, deadline, created_at, updated_at FROM tasks {}",
            where_clause
        );

        let mut query_builder = sqlx::query_as::<_, Task>(&query);

        for (_index, value) in bind_values.iter().enumerate() {
            match value {
                FilterValue::I64(v) => query_builder = query_builder.bind(v),
                FilterValue::String(v) => query_builder = query_builder.bind(v),
            }
        }

        query_builder
            .fetch_all(&self.pool)
            .await
            .map_err(anyhow::Error::from)
    }

    pub async fn update_task(&self, task: Task) -> Result<Task> {
        let now = Utc::now().timestamp();
        sqlx::query_as!(
            Task,
            r#"
                UPDATE tasks 
                SET parent_id = $1, level = $2, name = $3, description = $4, status = $5, deadline = $6, updated_at = $7
                WHERE id = $8
                RETURNING id, project_id, parent_id, level, name, description, status, deadline, created_at, updated_at
            "#,
            task.parent_id,
            task.level,
            task.name,
            task.description,
            task.status,
            task.deadline,
            now,
            task.id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(anyhow::Error::from)
    }

    pub async fn delete_task(&self, id: i64) -> Result<()> {
        let result = sqlx::query!(
            r#"
                DELETE FROM tasks WHERE id = $1
            "#,
            id,
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(anyhow::anyhow!("Task not found"));
        }

        Ok(())
    }
}
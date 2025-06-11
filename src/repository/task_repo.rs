use crate::errors::db_error::DBAccessError;
use crate::errors::messages::{ErrorKey, get_error_message};
use crate::models::{Task, task::TaskFilter};
use crate::repository::project_repo::get_project_by_id_with_transaction;
use crate::repository::validations::{
    validate_pagination, validate_task_description, validate_task_id, validate_task_level, validate_task_name, validate_task_parent_id, validate_task_project_id, validate_task_status, validate_task_unix_timestamp, validate_task_unix_timestamp_or_none
};
use anyhow::Result;
use chrono::Utc;
use sqlx::{Pool, Sqlite, Transaction};

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

    async fn validate_project_id_is_exist(
        &self,
        project_id: i64,
        tx: &mut Transaction<'_, Sqlite>,
    ) -> Result<(), DBAccessError> {
        let project = get_project_by_id_with_transaction(project_id, tx).await?;
        if project.is_none() {
            return Err(DBAccessError::ValidationError(get_error_message(
                ErrorKey::TaskProjectIdNotFound,
                format!("ID = {}", project_id),
            )));
        }
        Ok(())
    }

    async fn validate_parent_relation(
        &self,
        parent_id: Option<i64>,
        level: i64,
        self_id: Option<i64>,
        tx: &mut Transaction<'_, Sqlite>,
    ) -> Result<(), DBAccessError> {
        if parent_id.is_none() {
            if level != 0 {
                return Err(DBAccessError::ValidationError(get_error_message(
                    ErrorKey::TaskNoParentIdOnNonMajorTask,
                    format!("Level = {}", level),
                )));
            }
            return Ok(());
        } else {
            let parent_task = get_task_by_id_with_transaction(parent_id.unwrap(), tx).await?;
            if parent_task.level != level - 1 {
                return Err(DBAccessError::ValidationError(get_error_message(
                    ErrorKey::TaskParentLevelInvalid,
                    format!("Level = {}, Parent Level = {}", level, parent_task.level),
                )));
            }
        }

        if self_id.is_some() && parent_id.is_some() && parent_id.unwrap() == self_id.unwrap() {
            return Err(DBAccessError::ValidationError(get_error_message(
                ErrorKey::TaskParentIdCannotBeSameAsTaskId,
                format!("ID = {}", parent_id.unwrap()),
            )));
        }

        Ok(())
    }

    pub async fn create_task(&self, task: Task) -> Result<Task, DBAccessError> {
        validate_task_id(task.id)?;
        validate_task_project_id(task.project_id)?;
        validate_task_parent_id(task.parent_id)?;
        validate_task_level(task.level)?;
        validate_task_status(task.status)?;
        validate_task_name(&task.name)?;
        validate_task_description(task.description.as_ref())?;
        validate_task_unix_timestamp_or_none(task.deadline)?;

        let mut tx = self.pool.begin().await?;

        self.validate_project_id_is_exist(task.project_id, &mut tx)
            .await?;
        self.validate_parent_relation(task.parent_id, task.level, task.id, &mut tx)
            .await?;

        let now = Utc::now().timestamp();
        let result = sqlx::query_as!(
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
        .fetch_one(&mut *tx)
        .await;

        match result {
            Ok(task) => {
                tx.commit().await.map_err(|e| {
                    DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                        ErrorKey::TaskCreateFailed,
                        e.to_string()
                    )))
                })?;
                log::info!("Created task: {:?}", task);
                Ok(task)
            }
            Err(e) => {
                let _ = tx.rollback().await;
                Err(DBAccessError::QueryError(anyhow::anyhow!(
                    get_error_message(ErrorKey::TaskCreateFailed, e.to_string())
                )))
            }
        }
    }

    pub async fn get_task_by_id(&self, id: i64) -> Result<Task, DBAccessError> {
        validate_task_id(Some(id))?;

        let result = sqlx::query_as!(
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
        .map_err(|e| DBAccessError::QueryError(anyhow::anyhow!(get_error_message(ErrorKey::TaskGetByIdFailed, e.to_string()))))?;

        log::debug!("Get task by id: {:?}", result);

        match result {
            Some(task) => Ok(task),
            None => Err(DBAccessError::NotFoundError(get_error_message(
                ErrorKey::TaskGetByIdNotFound,
                format!("ID = {}", id),
            ))),
        }
    }

    pub async fn get_all_tasks(&self) -> Result<Vec<Task>, DBAccessError> {
        let result = sqlx::query_as!(
            Task,
            r#"
                SELECT id, project_id, parent_id, level, name, description, status, deadline, created_at, updated_at
                FROM tasks
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DBAccessError::QueryError(anyhow::anyhow!(get_error_message(ErrorKey::TaskGetAllFailed, e.to_string()))))?;

        log::debug!("Get all tasks: {:?}", result);

        Ok(result)
    }

    pub async fn get_tasks_count(&self) -> Result<i64, DBAccessError> {
        let result = sqlx::query_scalar::<_, i64>(r#"
            SELECT COUNT(*) FROM tasks
        "#)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DBAccessError::QueryError(anyhow::anyhow!(get_error_message(ErrorKey::TaskGetCountFailed, e.to_string()))))?;

        log::debug!("Get tasks count: {:?}", result);

        Ok(result)
    }

    pub async fn get_tasks_by_filter(
        &self,
        filter: Option<&TaskFilter>,
        page: Option<&i32>,
        page_size: Option<&i32>
    ) -> Result<Vec<Task>, DBAccessError> {
        validate_pagination(page, page_size)?;

        let mut query= String::from(r#"
            SELECT 
                tasks.id, tasks.project_id, tasks.parent_id, tasks.level, tasks.name, 
                tasks.description, tasks.status, tasks.deadline, tasks.created_at, tasks.updated_at
            FROM tasks
        "#);
        let mut page_bind_values: Vec<i32> = Vec::new();
        let mut filter_bind_values: Vec<FilterValue> = Vec::new();

        let mut tx = self.pool.begin().await.map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::TaskGetByFilterFailed,
                e.to_string()
            )))
        })?;

        // クエリのバインド値のインデックス
        let mut index = 1;

        // フィルターがある場合
        if filter.is_some() {
            if validate_filter(filter.as_ref().unwrap()).is_err() {
                return Ok(Vec::new());
            }
            let (where_clause, bind_values) = build_where_clause(filter.as_ref().unwrap());
            query.push_str(&format!(" {}", where_clause));

            // クエリのバインド値のインデックスを更新
            index = bind_values.len() + 1;
            filter_bind_values = bind_values;
        }

        // ページングがある場合
        if page.is_some() && page_size.is_some() {
            let page = page.unwrap();
            let page_size = page_size.unwrap();
            let offset = (*page - 1) * *page_size;
            let limit = *page_size;

            let count = get_tasks_count_with_transaction(&mut tx, filter).await?;

            if offset as i64 > count {
                return Err(DBAccessError::NotFoundError(get_error_message(
                    ErrorKey::TaskGetPaginationNotFound,
                    format!("Offset = {}, Count = {}", offset, count),
                )));
            }
            query.push_str(&format!(" LIMIT ${} OFFSET ${}", index, index + 1));
            page_bind_values.push(limit as i32);
            page_bind_values.push(offset as i32);
        }


        let mut query_builder = sqlx::query_as::<_, Task>(&query);

        // フィルターのバインド値がある場合
        if !filter_bind_values.is_empty() {
            for (_index, value) in filter_bind_values.iter().enumerate() {
                match value {
                    FilterValue::I64(v) => query_builder = query_builder.bind(v),
                    FilterValue::String(v) => query_builder = query_builder.bind(v),
                }
            }
        }

        // ページングのバインド値がある場合
        if !page_bind_values.is_empty() {
            for v in page_bind_values {
                query_builder = query_builder.bind(v);
            }
        }

        let result = query_builder.fetch_all(&mut *tx).await.map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::TaskGetByFilterFailed,
                e.to_string()
            )))
        })?;

        tx.commit().await.map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::TaskGetByFilterFailed,
                e.to_string()
            )))
        })?;
        log::debug!("Get tasks by filter: {:?}", result);

        Ok(result)
    }

    pub async fn update_task(&self, task: Task) -> Result<Task, DBAccessError> {
        if task.id.is_none() {
            return Err(DBAccessError::ValidationError(get_error_message(
                ErrorKey::TaskIdInvalid,
                format!("ID = {}", task.id.unwrap()),
            )));
        }

        validate_task_id(task.id)?;
        validate_task_project_id(task.project_id)?;
        validate_task_parent_id(task.parent_id)?;
        validate_task_level(task.level)?;
        validate_task_status(task.status)?;
        validate_task_name(&task.name)?;
        validate_task_description(task.description.as_ref())?;
        validate_task_unix_timestamp_or_none(task.deadline)?;

        let mut tx = self.pool.begin().await?;

        let _ = get_task_by_id_with_transaction(task.id.unwrap(), &mut tx).await?;

        self.validate_project_id_is_exist(task.project_id, &mut tx)
            .await?;
        self.validate_parent_relation(task.parent_id, task.level, task.id, &mut tx)
            .await?;

        let now = Utc::now().timestamp();
        let result = sqlx::query_as!(
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
        .fetch_one(&mut *tx)
        .await;

        match result {
            Ok(task) => {
                tx.commit().await.map_err(|e| {
                    DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                        ErrorKey::TaskUpdateFailed,
                        e.to_string()
                    )))
                })?;
                log::info!("Updated task: {:?}", task);
                Ok(task)
            }
            Err(e) => {
                let _ = tx.rollback().await;
                Err(DBAccessError::QueryError(anyhow::anyhow!(
                    get_error_message(ErrorKey::TaskUpdateFailed, e.to_string())
                )))
            }
        }
    }

    pub async fn delete_task(&self, id: i64) -> Result<(), DBAccessError> {
        validate_task_id(Some(id))?;

        let result = sqlx::query!(
            r#"
                DELETE FROM tasks WHERE id = $1
            "#,
            id,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::TaskDeleteFailedByIdNotFound,
                e.to_string()
            )))
        })?;

        if result.rows_affected() == 0 {
            return Err(DBAccessError::ValidationError(get_error_message(
                ErrorKey::TaskDeleteFailedByIdNotFound,
                format!("ID = {}", id),
            )));
        }
        log::info!("Deleted task: {:?}", id);

        Ok(())
    }
}


fn build_where_clause(filter: &TaskFilter) -> (String, Vec<FilterValue>) {
    let mut where_calses = Vec::new();
    let mut bind_values: Vec<FilterValue> = Vec::new();

    let mut index = 1;
    if filter.project_id.is_some() {
        where_calses.push(format!("tasks.project_id = ${}", index));
        bind_values.push(FilterValue::I64(filter.project_id.unwrap()));
        index += 1;
    }
    if filter.parent_id.is_some() {
        where_calses.push(format!("tasks.parent_id = ${}", index));
        bind_values.push(FilterValue::I64(filter.parent_id.unwrap()));
        index += 1;
    }
    if filter.level.is_some() {
        where_calses.push(format!("tasks.level = ${}", index));
        bind_values.push(FilterValue::I64(filter.level.unwrap()));
        index += 1;
    }
    if filter.name.is_some() {
        where_calses.push(format!("tasks.name LIKE '%' || ${} || '%'", index));
        bind_values.push(FilterValue::String(filter.name.as_ref().unwrap().clone()));
        index += 1;
    }
    if filter.description.is_some() {
        where_calses.push(format!("tasks.description LIKE '%' || ${} || '%'", index));
        bind_values.push(FilterValue::String(filter.description.as_ref().unwrap().clone()));
        index += 1;
    }
    if filter.status.is_some() {
        where_calses.push(format!("tasks.status = ${}", index));
        bind_values.push(FilterValue::I64(filter.status.unwrap()));
        index += 1;
    }

    if filter.deadline_from.is_some() && filter.deadline_to.is_some() {
        where_calses.push(format!(
            "tasks.deadline >= ${} AND tasks.deadline <= ${}",
            index,
            index + 1
        ));
        bind_values.push(FilterValue::I64(filter.deadline_from.unwrap()));
        bind_values.push(FilterValue::I64(filter.deadline_to.unwrap()));
        index += 2;
    } else if filter.deadline_from.is_some() && filter.deadline_to.is_none() {
        where_calses.push(format!("tasks.deadline >= ${}", index));
        bind_values.push(FilterValue::I64(filter.deadline_from.unwrap()));
        index += 1;
    } else if filter.deadline_to.is_some() && filter.deadline_from.is_none() {
        where_calses.push(format!("tasks.deadline <= ${}", index));
        bind_values.push(FilterValue::I64(filter.deadline_to.unwrap()));
        index += 1;
    }

    if filter.created_at_from.is_some() && filter.created_at_to.is_some() {
        where_calses.push(format!(
            "tasks.created_at >= ${} AND tasks.created_at <= ${}",
            index,
            index + 1
        ));
        bind_values.push(FilterValue::I64(filter.created_at_from.unwrap()));
        bind_values.push(FilterValue::I64(filter.created_at_to.unwrap()));
        index += 2;
    } else if filter.created_at_from.is_some() && filter.created_at_to.is_none() {
        where_calses.push(format!("tasks.created_at >= ${}", index));
        bind_values.push(FilterValue::I64(filter.created_at_from.unwrap()));
        index += 1;
    } else if filter.created_at_to.is_some() && filter.created_at_from.is_none() {
        where_calses.push(format!("tasks.created_at <= ${}", index));
        bind_values.push(FilterValue::I64(filter.created_at_to.unwrap()));
        index += 1;
    }

    if filter.updated_at_from.is_some() && filter.updated_at_to.is_some() {
        where_calses.push(format!(
            "tasks.updated_at >= ${} AND tasks.updated_at <= ${}",
            index,
            index + 1
        ));
        bind_values.push(FilterValue::I64(filter.updated_at_from.unwrap()));
        bind_values.push(FilterValue::I64(filter.updated_at_to.unwrap()));
    } else if filter.updated_at_from.is_some() && filter.updated_at_to.is_none() {
        where_calses.push(format!("tasks.updated_at >= ${}", index));
        bind_values.push(FilterValue::I64(filter.updated_at_from.unwrap()));
    } else if filter.updated_at_to.is_some() && filter.updated_at_from.is_none() {
        where_calses.push(format!("tasks.updated_at <= ${}", index));
        bind_values.push(FilterValue::I64(filter.updated_at_to.unwrap()));
    }

    if !where_calses.is_empty() {
        (
            format!(" WHERE {}", where_calses.join(" AND ")),
            bind_values,
        )
    } else {
        ("".to_string(), bind_values)
    }
}

fn validate_filter(filter: &TaskFilter) -> Result<()> {
    if filter.project_id.is_some() {
        validate_task_project_id(filter.project_id.unwrap())?;
    }
    if filter.parent_id.is_some() {
        validate_task_parent_id(filter.parent_id)?;
    }
    if let Some(level) = filter.level {
        validate_task_level(level)?;
    }
    if let Some(status) = filter.status {
        validate_task_status(status)?;
    }
    if let Some(name) = &filter.name {
        validate_task_name(name)?;
    }
    if let Some(description) = &filter.description {
        validate_task_description(Some(description))?;
    }
    if let Some(deadline) = filter.deadline_from {
        validate_task_unix_timestamp(deadline)?;
    }
    if let Some(deadline) = filter.deadline_to {
        validate_task_unix_timestamp(deadline)?;
    }
    if let Some(created_at) = filter.created_at_from {
        validate_task_unix_timestamp(created_at)?;
    }
    if let Some(created_at) = filter.created_at_to {
        validate_task_unix_timestamp(created_at)?;
    }
    if let Some(updated_at) = filter.updated_at_from {
        validate_task_unix_timestamp(updated_at)?;
    }
    if let Some(updated_at) = filter.updated_at_to {
        validate_task_unix_timestamp(updated_at)?;
    }
    Ok(())
}


pub async fn get_task_by_id_with_transaction(
    id: i64,
    transaction: &mut Transaction<'_, Sqlite>,
) -> Result<Task, DBAccessError> {
    let result = sqlx::query_as!(
        Task,
        r#"
            SELECT id, project_id, parent_id, level, name, description, status, deadline, created_at, updated_at
            FROM tasks
            WHERE id = $1
        "#,
        id
    )
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|e| DBAccessError::QueryError(anyhow::anyhow!(get_error_message(ErrorKey::TaskGetByIdFailed, e.to_string()))))?;

    log::debug!("Get task by id with transaction: {:?}", result);
    match result {
        Some(task) => Ok(task),
        None => Err(DBAccessError::NotFoundError(get_error_message(
            ErrorKey::TaskGetByIdNotFound,
            format!("ID = {}", id),
        ))),
    }
}

pub async fn get_tasks_count_with_transaction(
    tx: &mut Transaction<'_, Sqlite>,
    filter: Option<&TaskFilter>,
) -> Result<i64, DBAccessError> {
    let query = r#"
        SELECT COUNT(*) FROM tasks
    "#;

    let result  = match filter {
        Some(filter) => {
            if validate_filter(filter).is_err() {
                return Ok(0);
            }

            let (where_clause, bind_values) = build_where_clause(filter);
            let query = format!("{} {}", query, where_clause);
            let mut query_builder = sqlx::query_scalar::<_, i64>(&query);

            for (_index, value) in bind_values.iter().enumerate() {
                match value {
                    FilterValue::I64(v) => query_builder = query_builder.bind(v),
                    FilterValue::String(v) => query_builder = query_builder.bind(v),
                }
            }

            let count = query_builder.fetch_one(&mut **tx)
            .await
            .map_err(|e| DBAccessError::QueryError(anyhow::anyhow!(get_error_message(ErrorKey::TaskGetCountFailed, e.to_string()))))?;
            count
        }
        None => {
            let count = sqlx::query_scalar::<_, i64>(&query)
            .fetch_one(&mut **tx)
            .await
            .map_err(|e| DBAccessError::QueryError(anyhow::anyhow!(get_error_message(ErrorKey::TaskGetCountFailed, e.to_string()))))?;
            count
        }
    };

    log::debug!("Get tasks count with transaction: {:?}", result);
    Ok(result)
}

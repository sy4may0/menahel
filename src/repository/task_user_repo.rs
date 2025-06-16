use sqlx::{Pool, Sqlite};
use crate::errors::messages::{ErrorKey, get_error_message};
use crate::models::repository_model::task::TaskFilter;
use crate::models::repository_model::taskwithuser::TaskWithUser;
use crate::repository::task_repo::{validate_task_filter, build_task_where_clause, get_tasks_count_with_transaction};
use crate::errors::db_error::DBAccessError;
use crate::enums::TaskFilterValue;
use crate::repository::validations::validate_pagination;

pub struct TaskUserRepository {
    pool: Pool<Sqlite>,
}

impl TaskUserRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    pub async fn get_tasks_and_users_by_filter(
        &self,
        page: Option<&i32>,
        page_size: Option<&i32>,
        filter: Option<&TaskFilter>,
        user_ids: Option<&Vec<i64>>,
    ) -> Result<Vec<TaskWithUser>, DBAccessError> {
        let mut tx = self.pool.begin().await
        .map_err(|e| DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
            ErrorKey::TaskUserGetByFilterFailed,
            e.to_string()
        ))))?;

        let mut query = format!(
            r#"
                SELECT
                    tasks.task_id,
                    tasks.project_id,
                    tasks.parent_id,
                    tasks.level,
                    tasks.name,
                    tasks.description,
                    tasks.status,
                    tasks.deadline,
                    tasks.created_at,
                    tasks.updated_at,
                    COALESCE((
                        SELECT json_group_array(
                            json_object(
                                'user_id', users.user_id,
                                'username', users.username,
                                'email', users.email
                            )
                        ) FROM users
                        JOIN user_assign ON users.user_id = user_assign.user_id
                        WHERE user_assign.task_id = tasks.task_id
                    ), '[]') AS users
                FROM tasks
            "#
        );

        let mut page_bind_values: Vec<i32> = Vec::new();
        let mut filter_bind_values: Vec<TaskFilterValue> = Vec::new();

        let mut index = 1;

        if filter.is_some() || (user_ids.is_some() && user_ids.unwrap().len() > 0) {
            let unwrapped_filter = match filter {
                Some(filter) => filter,
                None => &TaskFilter::new(),
            };

            match validate_task_filter(unwrapped_filter) {
                Ok(_) => {},
                Err(_) => return Ok(Vec::new()),
            };

            let (where_clause, bind_values) = build_task_where_clause(
                unwrapped_filter,
                user_ids
            );
            query.push_str(&format!(" {}", where_clause));
            index = bind_values.len() + 1;
            filter_bind_values = bind_values;
        }

        query.push_str(" GROUP BY tasks.task_id");
        query.push_str(" ORDER BY tasks.task_id ASC");

        let count = get_tasks_count_with_transaction(&mut tx, filter, user_ids).await?;
        validate_pagination(page, page_size, &count)?;
        if page.is_some() && page_size.is_some() {
            let page = page.unwrap();
            let page_size = page_size.unwrap();
            let offset = (*page - 1) * *page_size;
            let limit = *page_size;
            query.push_str(&format!(" LIMIT ${} OFFSET ${}", index, index + 1));
            page_bind_values.push(limit as i32);
            page_bind_values.push(offset as i32);
        }

        println!("query: {}", query);

        let mut query_builder = sqlx::query_as::<_, TaskWithUser>(&query);

        if !filter_bind_values.is_empty() {
            for (_index, value) in filter_bind_values.iter().enumerate() {
                match value {
                    TaskFilterValue::I64(v) => query_builder = query_builder.bind(v),
                    TaskFilterValue::String(v) => query_builder = query_builder.bind(v),
                }
            }
        }

        if !page_bind_values.is_empty() {
            for v in page_bind_values {
                query_builder = query_builder.bind(v);
            }
        }

        let result = query_builder.fetch_all(&mut *tx).await.map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::TaskUserGetByFilterFailed,
                e.to_string()
            )))
        })?;

        tx.commit().await.map_err(|e| DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
            ErrorKey::TaskUserGetByFilterFailed,
            e.to_string()
        ))))?;

        log::debug!("Get tasks and users by filter: {:?}", result);

        Ok(result)
    }

}

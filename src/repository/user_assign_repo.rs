use crate::enums::TaskLevel;
use crate::errors::db_error::DBAccessError;
use crate::errors::messages::{ErrorKey, get_error_message};
use crate::models::{UserAssign, UserAssignFilter};
use crate::repository::task_repo::get_task_by_id_with_transaction;
use crate::repository::user_repo::get_user_by_id_with_transaction;
use crate::repository::validations::{
    validate_pagination, validate_user_assign_id, validate_user_assign_id_is_none,
    validate_user_assign_task_id, validate_user_assign_user_id,
};
use anyhow::Result;
use sqlx::{Pool, Sqlite, Transaction};
use std::collections::HashMap;

pub struct UserAssignRepository {
    pool: Pool<Sqlite>,
}

#[derive(Debug)]
enum FilterValue {
    I64(i64),
}

impl UserAssignRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    async fn validate_target_user_and_task(
        &self,
        user_assign: &UserAssign,
        tx: &mut Transaction<'_, Sqlite>,
    ) -> Result<(), DBAccessError> {
        get_user_by_id_with_transaction(&user_assign.user_id, tx).await?;
        get_task_by_id_with_transaction(user_assign.task_id, tx).await?;
        Ok(())
    }

    async fn validate_user_assign_to_not_max_level_task(
        &self,
        user_assign: &UserAssign,
        tx: &mut Transaction<'_, Sqlite>,
    ) -> Result<(), DBAccessError> {
        let task = get_task_by_id_with_transaction(user_assign.task_id, tx).await?;
        if task.level != TaskLevel::max_level() as i64 {
            return Err(DBAccessError::ValidationError(get_error_message(
                ErrorKey::UserAssignToNotMaxLevelTask,
                format!("ID = {}", user_assign.task_id),
            )));
        }

        Ok(())
    }

    async fn validate_user_assign_same_user_assign_exists(
        &self,
        user_assign: &UserAssign,
        tx: &mut Transaction<'_, Sqlite>,
    ) -> Result<(), DBAccessError> {
        let user_assigns =
            get_user_assign_by_task_id_with_transaction(user_assign.task_id, tx).await?;
        if user_assigns
            .iter()
            .any(|assign| assign.user_id == user_assign.user_id)
        {
            return Err(DBAccessError::ValidationError(get_error_message(
                ErrorKey::UserAssignSameUserAssignExists,
                format!("ID = {}", user_assign.task_id),
            )));
        }
        Ok(())
    }

    pub async fn create_user_assign(
        &self,
        user_assign: UserAssign,
    ) -> Result<UserAssign, DBAccessError> {
        validate_user_assign_id_is_none(user_assign.user_assign_id)?;
        validate_user_assign_user_id(user_assign.user_id)?;
        validate_user_assign_task_id(user_assign.task_id)?;

        let mut tx = self.pool.begin().await?;

        self.validate_target_user_and_task(&user_assign, &mut tx)
            .await?;
        self.validate_user_assign_to_not_max_level_task(&user_assign, &mut tx)
            .await?;
        self.validate_user_assign_same_user_assign_exists(&user_assign, &mut tx)
            .await?;

        let result = sqlx::query_as!(
            UserAssign,
            r#"
                INSERT INTO user_assign (user_id, task_id)
                VALUES ($1, $2)
                RETURNING user_assign_id, user_id, task_id
            "#,
            user_assign.user_id,
            user_assign.task_id,
        )
        .fetch_one(&mut *tx)
        .await;

        match result {
            Ok(user_assign) => {
                log::info!("User assign created: {:?}", user_assign);
                tx.commit().await.map_err(|e| {
                    DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                        ErrorKey::UserAssignCreateFailed,
                        e.to_string()
                    )))
                })?;
                Ok(user_assign)
            }
            Err(e) => {
                let _ = tx.rollback().await;
                Err(DBAccessError::QueryError(anyhow::anyhow!(
                    get_error_message(ErrorKey::UserAssignCreateFailed, e.to_string())
                )))
            }
        }
    }

    pub async fn get_user_assign_by_id(&self, id: i64) -> Result<UserAssign, DBAccessError> {
        let result = sqlx::query_as!(
            UserAssign,
            r#"
                SELECT user_assign_id, user_id, task_id
                FROM user_assign
                WHERE user_assign_id = $1
            "#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::UserAssignGetByIdFailed,
                e.to_string()
            )))
        })?;
        log::debug!("Get user assign by id: {:?}", result);

        match result {
            Some(user_assign) => Ok(user_assign),
            None => Err(DBAccessError::NotFoundError(get_error_message(
                ErrorKey::UserAssignGetByIdNotFound,
                format!("ID = {}", id),
            ))),
        }
    }

    pub async fn get_all_user_assigns(&self) -> Result<Vec<UserAssign>, DBAccessError> {
        let result = sqlx::query_as!(
            UserAssign,
            r#"
                SELECT user_assign_id, user_id, task_id
                FROM user_assign
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::UserAssignGetAllFailed,
                e.to_string()
            )))
        })?;
        log::debug!("Get all user assigns: {:?}", result);

        Ok(result)
    }

    pub async fn get_user_assigns_count(&self) -> Result<i64, DBAccessError> {
        let result = sqlx::query_scalar!(
            r#"
                SELECT COUNT(*) FROM user_assign
            "#,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::UserAssignGetUserAssignsCountFailed,
                e.to_string()
            )))
        })?;
        log::debug!("Get user assigns count: {:?}", result);

        Ok(result)
    }

    pub async fn get_user_assigns_with_pagination(
        &self,
        page: &i32,
        page_size: &i32,
    ) -> Result<Vec<UserAssign>, DBAccessError> {
        let mut tx = self.pool.begin().await.map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::UserAssignGetAllFailed,
                e.to_string()
            )))
        })?;
        let count = get_user_assigns_count_with_transaction(&mut tx, None).await?;

        validate_pagination(Some(page), Some(page_size), &count)?;
        let offset = (page - 1) * page_size;
        let limit = page_size;

        log::debug!(
            "Get user assigns with pagination: offset: {}, limit: {}",
            offset,
            limit
        );

        let result = sqlx::query_as!(
            UserAssign,
            r#"
                SELECT user_assign_id, user_id, task_id
                FROM user_assign
                ORDER BY user_assign_id
                LIMIT $1 OFFSET $2
            "#,
            limit,
            offset,
        )
        .fetch_all(&mut *tx)
        .await;

        match result {
            Ok(user_assigns) => {
                tx.commit().await.map_err(|e| {
                    DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                        ErrorKey::UserAssignGetAllFailed,
                        e.to_string()
                    )))
                })?;
                Ok(user_assigns)
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
                    get_error_message(ErrorKey::UserAssignGetAllFailed, e.to_string())
                )))
            }
        }
    }

    pub async fn get_user_assigns_by_filter(
        &self,
        filter: Option<&UserAssignFilter>,
        page: Option<&i32>,
        page_size: Option<&i32>,
    ) -> Result<Vec<UserAssign>, DBAccessError> {
        let mut query = String::from(
            r#"
            SELECT user_assign_id, user_id, task_id
            FROM user_assign
        "#,
        );
        let mut page_bind_values: Vec<i32> = Vec::new();
        let mut filter_bind_values: Vec<FilterValue> = Vec::new();

        let mut tx = self.pool.begin().await.map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::UserAssignGetAllFailed,
                e.to_string()
            )))
        })?;

        let mut index = 1;
        if filter.is_some() {
            if validate_filter(filter.as_ref().unwrap()).is_err() {
                return Ok(Vec::new());
            }
            let (where_clause, bind_values) = build_where_clause(filter.as_ref().unwrap());
            query.push_str(&format!(" {}", where_clause));

            index = bind_values.len() + 1;
            filter_bind_values = bind_values;
        }

        let count = get_user_assigns_count_with_transaction(&mut tx, filter).await?;
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

        let mut query_builder = sqlx::query_as::<_, UserAssign>(&query);

        if !filter_bind_values.is_empty() {
            for (_index, value) in filter_bind_values.iter().enumerate() {
                match value {
                    FilterValue::I64(v) => query_builder = query_builder.bind(v),
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
                ErrorKey::UserAssignGetAllFailed,
                e.to_string()
            )))
        })?;

        tx.commit().await.map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::UserAssignGetAllFailed,
                e.to_string()
            )))
        })?;
        log::debug!("Get user assigns by filter: {:?}", result);

        Ok(result)
    }

    pub async fn update_user_assign(
        &self,
        user_assign: UserAssign,
    ) -> Result<UserAssign, DBAccessError> {
        validate_user_assign_user_id(user_assign.user_id)?;
        validate_user_assign_task_id(user_assign.task_id)?;

        if user_assign.user_assign_id.is_none() {
            return Err(DBAccessError::ValidationError(get_error_message(
                ErrorKey::UserIdInvalid,
                format!("ID = {:?}", user_assign.user_assign_id),
            )));
        }

        let mut tx = self.pool.begin().await?;

        self.validate_target_user_and_task(&user_assign, &mut tx)
            .await?;
        self.validate_user_assign_to_not_max_level_task(&user_assign, &mut tx)
            .await?;
        self.validate_user_assign_same_user_assign_exists(&user_assign, &mut tx)
            .await?;

        let result = sqlx::query_as!(
            UserAssign,
            r#"
                UPDATE user_assign
                SET user_id = $1, task_id = $2
                WHERE user_assign_id = $3
                RETURNING user_assign_id, user_id, task_id
            "#,
            user_assign.user_id,
            user_assign.task_id,
            user_assign.user_assign_id,
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::UserAssignUpdateFailed,
                e.to_string()
            )))
        })?;

        tx.commit().await.map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::UserAssignUpdateFailed,
                e.to_string()
            )))
        })?;

        log::info!("Update user assign: {:?}", result);

        match result {
            Some(user_assign) => Ok(user_assign),
            None => Err(DBAccessError::NotFoundError(get_error_message(
                ErrorKey::UserAssignGetByIdNotFound,
                format!("ID = {:?}", user_assign.user_assign_id),
            ))),
        }
    }

    pub async fn delete_user_assign(&self, id: i64) -> Result<(), DBAccessError> {
        validate_user_assign_id(Some(id))?;

        let result = sqlx::query!(
            r#"
                DELETE FROM user_assign
                WHERE user_assign_id = $1
            "#,
            id,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::UserAssignDeleteFailed,
                e.to_string()
            )))
        })?;

        if result.rows_affected() == 0 {
            return Err(DBAccessError::ValidationError(get_error_message(
                ErrorKey::UserAssignDeleteFailedByIdNotFound,
                format!("ID = {}", id),
            )));
        }

        log::info!("Delete user assign: {:?}", result);

        Ok(())
    }
}

pub async fn get_user_assign_by_id_with_transaction(
    id: i64,
    transaction: &mut Transaction<'_, Sqlite>,
) -> Result<UserAssign, DBAccessError> {
    let result = sqlx::query_as!(
        UserAssign,
        r#"
            SELECT user_assign_id, user_id, task_id
            FROM user_assign
            WHERE user_assign_id = $1
        "#,
        id,
    )
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|e| {
        DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
            ErrorKey::UserAssignGetByIdFailed,
            e.to_string()
        )))
    })?;
    log::debug!("Get user assign by id with transaction: {:?}", result);

    match result {
        Some(user_assign) => Ok(user_assign),
        None => Err(DBAccessError::NotFoundError(get_error_message(
            ErrorKey::UserAssignGetByIdNotFound,
            format!("ID = {}", id),
        ))),
    }
}

pub async fn get_user_assign_by_user_id_with_transaction(
    user_id: i64,
    transaction: &mut Transaction<'_, Sqlite>,
) -> Result<Vec<UserAssign>, DBAccessError> {
    let result = sqlx::query_as!(
        UserAssign,
        r#"
            SELECT user_assign_id, user_id, task_id
            FROM user_assign
            WHERE user_id = $1
        "#,
        user_id,
    )
    .fetch_all(&mut **transaction)
    .await
    .map_err(|e| {
        DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
            ErrorKey::UserAssignGetByUserIdFailed,
            e.to_string()
        )))
    })?;
    log::debug!("Get user assign by user id with transaction: {:?}", result);

    Ok(result)
}

pub async fn get_user_assign_by_task_id_with_transaction(
    task_id: i64,
    transaction: &mut Transaction<'_, Sqlite>,
) -> Result<Vec<UserAssign>, DBAccessError> {
    let result = sqlx::query_as!(
        UserAssign,
        r#"
            SELECT user_assign_id, user_id, task_id
            FROM user_assign
            WHERE task_id = $1
        "#,
        task_id,
    )
    .fetch_all(&mut **transaction)
    .await
    .map_err(|e| {
        DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
            ErrorKey::UserAssignGetByTaskIdFailed,
            e.to_string()
        )))
    })?;
    log::debug!("Get user assign by task id with transaction: {:?}", result);

    Ok(result)
}

pub async fn get_user_assign_by_user_id_and_task_id_with_transaction(
    user_id: i64,
    task_id: i64,
    transaction: &mut Transaction<'_, Sqlite>,
) -> Result<UserAssign, DBAccessError> {
    let result = sqlx::query_as!(
        UserAssign,
        r#"
            SELECT user_assign_id, user_id, task_id
            FROM user_assign
            WHERE user_id = $1 AND task_id = $2
        "#,
        user_id,
        task_id,
    )
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|e| {
        DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
            ErrorKey::UserAssignGetByUserIdAndTaskIdFailed,
            e.to_string()
        )))
    })?;
    log::debug!(
        "Get user assign by user id and task id with transaction: {:?}",
        result
    );

    match result {
        Some(user_assign) => Ok(user_assign),
        None => Err(DBAccessError::NotFoundError(get_error_message(
            ErrorKey::UserAssignGetByUserIdAndTaskIdNotFound,
            format!("User ID = {}, Task ID = {}", user_id, task_id),
        ))),
    }
}

pub async fn get_user_assigns_count_with_transaction(
    transaction: &mut Transaction<'_, Sqlite>,
    filter: Option<&UserAssignFilter>,
) -> Result<i64, DBAccessError> {
    let query = r#"
        SELECT COUNT(*) FROM user_assign
    "#;

    let result = match filter {
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
                }
            }

            let count = query_builder
                .fetch_one(&mut **transaction)
                .await
                .map_err(|e| {
                    DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                        ErrorKey::UserAssignGetUserAssignsCountFailed,
                        e.to_string()
                    )))
                })?;
            count
        }
        None => {
            let count = sqlx::query_scalar::<_, i64>(&query)
                .fetch_one(&mut **transaction)
                .await
                .map_err(|e| {
                    DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                        ErrorKey::UserAssignGetUserAssignsCountFailed,
                        e.to_string()
                    )))
                })?;
            count
        }
    };

    log::debug!("Get user assigns count with transaction: {:?}", result);

    Ok(result)
}

fn build_where_clause(filter: &UserAssignFilter) -> (String, Vec<FilterValue>) {
    let mut where_calses = Vec::new();
    let mut bind_values: Vec<FilterValue> = Vec::new();

    let mut index = 1;
    if filter.user_id.is_some() {
        where_calses.push(format!("user_id = ${}", index));
        bind_values.push(FilterValue::I64(filter.user_id.unwrap()));
        index += 1;
    }
    if filter.task_id.is_some() {
        where_calses.push(format!("task_id = ${}", index));
        bind_values.push(FilterValue::I64(filter.task_id.unwrap()));
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

fn validate_filter(filter: &UserAssignFilter) -> Result<()> {
    if filter.user_id.is_some() {
        validate_user_assign_user_id(filter.user_id.unwrap())?;
    }
    if filter.task_id.is_some() {
        validate_user_assign_task_id(filter.task_id.unwrap())?;
    }
    Ok(())
}

pub async fn get_related_task_ids_by_user_ids(
    user_ids: Vec<i64>,
    transaction: &mut Transaction<'_, Sqlite>,
) -> Result<HashMap<i64, Vec<i64>>, DBAccessError> {
    let mut placeholder = Vec::new();
    for i in 1..user_ids.len() + 1 {
        placeholder.push(format!("${}", i));
    }

    let query = match user_ids.len() {
        0 => return Ok(HashMap::new()),
        _ => format!(
            r#"
                SELECT user_assign_id, user_id, task_id FROM user_assign WHERE user_id IN ({})
        "#,
            placeholder.join(",")
        ),
    };
    let mut query_builder = sqlx::query_as::<_, UserAssign>(&query);

    for user_id in user_ids {
        query_builder = query_builder.bind(user_id);
    }

    let result = query_builder
        .fetch_all(&mut **transaction)
        .await
        .map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::UserAssignGetByUserIdFailed,
                e.to_string()
            )))
        })?;

    let mut task_ids_map: HashMap<i64, Vec<i64>> = HashMap::new();
    for user_assign in result {
        task_ids_map
            .entry(user_assign.user_id)
            .or_insert(Vec::new())
            .push(user_assign.task_id);
    }

    Ok(task_ids_map)
}

pub async fn get_related_user_ids_by_task_ids(
    task_ids: Vec<i64>,
    transaction: &mut Transaction<'_, Sqlite>,
) -> Result<HashMap<i64, Vec<i64>>, DBAccessError> {
    let mut placeholder = Vec::new();
    for i in 1..task_ids.len() + 1 {
        placeholder.push(format!("${}", i));
    }

    let query = match task_ids.len() {
        0 => return Ok(HashMap::new()),
        _ => format!(
            r#"
            SELECT user_assign_id, task_id, user_id FROM user_assign WHERE task_id IN ({})
        "#,
            placeholder.join(",")
        ),
    };
    let mut query_builder = sqlx::query_as::<_, UserAssign>(&query);

    for task_id in task_ids {
        query_builder = query_builder.bind(task_id);
    }

    let result = query_builder
        .fetch_all(&mut **transaction)
        .await
        .map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::UserAssignGetByTaskIdFailed,
                e.to_string()
            )))
        })?;

    let mut user_ids_map: HashMap<i64, Vec<i64>> = HashMap::new();
    for user_assign in result {
        user_ids_map
            .entry(user_assign.task_id)
            .or_insert(Vec::new())
            .push(user_assign.user_id);
    }

    Ok(user_ids_map)
}

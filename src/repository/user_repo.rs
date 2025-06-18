use crate::errors::db_error::DBAccessError;
use crate::errors::messages::{ErrorKey, get_error_message};
use crate::models::user::UserFilter;
use crate::models::{User, UserNoPassword};
use crate::repository::validations::{
    validate_pagination, validate_user_email, validate_user_id, validate_user_id_is_none,
    validate_user_name, validate_user_password,
};
use sqlx::{Pool, Sqlite, Transaction};

pub struct UserRepository {
    pool: Pool<Sqlite>,
}

#[derive(Debug)]
enum FilterValue {
    I64(i64),
    String(String),
}

fn users_to_users_no_password(users: Vec<User>) -> Vec<UserNoPassword> {
    users
        .into_iter()
        .map(|user| user.to_user_no_password())
        .collect()
}

fn deduplicate(users: Vec<User>) -> Vec<User> {
    let mut deduplicated_users: Vec<User> = Vec::new();
    for user in users {
        if !deduplicated_users.iter().any(|u| u.user_id == user.user_id) {
            deduplicated_users.push(user);
        }
    }
    deduplicated_users
}

impl UserRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, user: User) -> Result<UserNoPassword, DBAccessError> {
        validate_user_id(user.user_id)?;
        validate_user_id_is_none(user.user_id)?;
        validate_user_name(&user.username)?;
        validate_user_email(&user.email)?;
        validate_user_password(&user.password_hash)?;

        let result = sqlx::query_as!(
            User,
            r#"
                INSERT INTO users (username, email, password_hash)
                VALUES ($1, $2, $3)
                RETURNING user_id, username, email, password_hash
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

        Ok(result.to_user_no_password())
    }

    pub async fn get_user_by_id(&self, id: i64) -> Result<UserNoPassword, DBAccessError> {
        let result = sqlx::query_as!(
            User,
            r#"
                SELECT user_id, username, email, password_hash
                FROM users
                WHERE user_id = $1
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
            Some(user) => Ok(user.to_user_no_password()),
            None => Err(DBAccessError::NotFoundError(get_error_message(
                ErrorKey::UserGetByIdNotFound,
                format!("ID = {}", id),
            ))),
        }
    }

    pub async fn get_user_by_name(&self, name: &str) -> Result<UserNoPassword, DBAccessError> {
        let result = sqlx::query_as!(
            User,
            r#"
                SELECT user_id, username, email, password_hash
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
            Some(user) => Ok(user.to_user_no_password()),
            None => Err(DBAccessError::NotFoundError(get_error_message(
                ErrorKey::UserGetByNameNotFound,
                format!("Name = {}", name),
            ))),
        }
    }

    pub async fn get_all_users(&self) -> Result<Vec<UserNoPassword>, DBAccessError> {
        let result = sqlx::query_as!(
            User,
            r#"
                SELECT user_id, username, email, password_hash
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

        Ok(users_to_users_no_password(result))
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
    ) -> Result<Vec<UserNoPassword>, DBAccessError> {
        let mut tx = self.pool.begin().await.map_err(|e| {
            DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                ErrorKey::ProjectGetAllFailed,
                e.to_string()
            )))
        })?;
        let count = get_users_count_with_transaction(&mut tx, None, None).await?;
        validate_pagination(Some(page), Some(page_size), &count)?;

        let offset = (page - 1) * page_size;
        let limit = page_size;

        if offset as i64 >= count {
            return Err(DBAccessError::NotFoundError(get_error_message(
                ErrorKey::UserGetUsersPaginationNotFound,
                format!("Offset: {}, Count: {}", offset, count),
            )));
        }
        log::debug!(
            "Get users with pagination: offset: {}, limit: {}",
            offset,
            limit
        );

        let result = sqlx::query_as!(
            User,
            r#"
                SELECT user_id, username, email, password_hash
                FROM users
                ORDER BY user_id
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
                Ok(users_to_users_no_password(users))
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

    pub async fn update_user(&self, user: User) -> Result<UserNoPassword, DBAccessError> {
        validate_user_id(user.user_id)?;
        validate_user_name(&user.username)?;
        validate_user_email(&user.email)?;
        validate_user_password(&user.password_hash)?;

        if user.user_id.is_none() {
            return Err(DBAccessError::ValidationError(get_error_message(
                ErrorKey::UserIdInvalid,
                format!("ID = {:?}", user.user_id),
            )));
        }

        let mut tx = self.pool.begin().await?;
        let _ = get_user_by_id_with_transaction(&user.user_id.unwrap(), &mut tx).await?;

        let result = sqlx::query_as!(
            User,
            r#"
                UPDATE users
                SET username = $1, email = $2, password_hash = $3
                WHERE user_id = $4
                RETURNING user_id, username, email, password_hash
            "#,
            user.username,
            user.email,
            user.password_hash,
            user.user_id,
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
            Some(user) => Ok(user.to_user_no_password()),
            None => Err(DBAccessError::NotFoundError(get_error_message(
                ErrorKey::UserGetByIdNotFound,
                format!("ID = {:?}", user.user_id),
            ))),
        }
    }

    pub async fn delete_user(&self, id: i64) -> Result<(), DBAccessError> {
        validate_user_id(Some(id))?;
        let result = sqlx::query!(
            r#"
                DELETE FROM users
                WHERE user_id = $1
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

fn build_where_clause(
    filter: &UserFilter,
    task_ids: Option<&Vec<i64>>,
) -> (String, Vec<FilterValue>) {
    let mut where_calses = Vec::new();
    let mut bind_values: Vec<FilterValue> = Vec::new();

    let mut index = 1;
    if filter.username.is_some() {
        where_calses.push(format!("username = ${}", index));
        bind_values.push(FilterValue::String(
            filter.username.as_ref().unwrap().clone(),
        ));
        index += 1;
    }

    if filter.email.is_some() {
        where_calses.push(format!("email = ${}", index));
        bind_values.push(FilterValue::String(filter.email.as_ref().unwrap().clone()));
        index += 1;
    }

    if task_ids.is_some() {
        let mut id_idx = 0;
        let mut id_placeholders: Vec<String> = Vec::new();
        for id in task_ids.as_ref().unwrap().iter() {
            id_placeholders.push(format!("${}", index + id_idx));
            bind_values.push(FilterValue::I64(*id));
            id_idx += 1;
        }
        where_calses.push(format!(
            "user_assign.task_id IN({})",
            id_placeholders.join(",")
        ));
    }

    if !where_calses.is_empty() {
        (
            format!(" WHERE {}", where_calses.join(" AND ")),
            bind_values,
        )
    } else {
        (String::new(), bind_values)
    }
}

fn validate_filter(filter: &UserFilter) -> Result<(), DBAccessError> {
    if filter.username.is_some() {
        validate_user_name(filter.username.as_ref().unwrap())?;
    }
    if filter.email.is_some() {
        validate_user_email(filter.email.as_ref().unwrap())?;
    }
    Ok(())
}

pub async fn get_user_by_id_with_transaction(
    id: &i64,
    tx: &mut Transaction<'_, Sqlite>,
) -> Result<UserNoPassword, DBAccessError> {
    let result = sqlx::query_as!(
        User,
        r#"
            SELECT user_id, username, email, password_hash
            FROM users
            WHERE user_id = $1
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
        Some(user) => Ok(user.to_user_no_password()),
        None => Err(DBAccessError::NotFoundError(get_error_message(
            ErrorKey::UserGetByIdNotFound,
            format!("ID = {}", id),
        ))),
    }
}

pub async fn get_users_count_with_transaction(
    tx: &mut Transaction<'_, Sqlite>,
    filter: Option<&UserFilter>,
    task_ids: Option<&Vec<i64>>,
) -> Result<i64, DBAccessError> {
    let mut query = String::from(
        r#"
        SELECT COUNT(*) FROM users
    "#,
    );

    if task_ids.is_some() && task_ids.unwrap().len() > 0 {
        query.push_str(
            r#"
            INNER JOIN user_assign ON users.user_id = user_assign.user_id
        "#,
        );
    }

    let result = match filter {
        Some(filter) => {
            if validate_filter(filter).is_err() {
                return Ok(0);
            }

            let (where_clause, bind_values) = build_where_clause(filter, task_ids);
            let query = format!("{} {}", query, where_clause);
            let mut query_builder = sqlx::query_scalar::<_, i64>(&query);

            for (_index, value) in bind_values.iter().enumerate() {
                match value {
                    FilterValue::I64(v) => query_builder = query_builder.bind(v),
                    FilterValue::String(v) => query_builder = query_builder.bind(v),
                }
            }

            let count = query_builder.fetch_one(&mut **tx).await.map_err(|e| {
                DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                    ErrorKey::UserGetUsersCountFailed,
                    e.to_string()
                )))
            })?;
            count
        }
        None => {
            let count = sqlx::query_scalar::<_, i64>(&query)
                .fetch_one(&mut **tx)
                .await
                .map_err(|e| {
                    DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
                        ErrorKey::UserGetUsersCountFailed,
                        e.to_string()
                    )))
                })?;
            count
        }
    };

    log::debug!("Get users count with transaction: {:?}", result);

    Ok(result)
}

pub async fn get_users_with_pagination_with_transaction(
    tx: &mut Transaction<'_, Sqlite>,
    page: Option<&i32>,
    page_size: Option<&i32>,
    filter: Option<&UserFilter>,
    task_ids: Option<&Vec<i64>>,
) -> Result<Vec<UserNoPassword>, DBAccessError> {
    let mut query = String::from(
        r#"
        SELECT users.user_id, users.username, users.email, users.password_hash
        FROM users
    "#,
    );
    let mut page_bind_values: Vec<i32> = Vec::new();
    let mut filter_bind_values: Vec<FilterValue> = Vec::new();

    if task_ids.is_some() && task_ids.unwrap().len() > 0 {
        query.push_str(
            r#"
            INNER JOIN user_assign ON users.user_id = user_assign.user_id
        "#,
        );
    }

    // クエリのバインド値のインデックス
    let mut index = 1;

    // フィルターがある場合
    if filter.is_some() || (task_ids.is_some() && task_ids.unwrap().len() > 0) {
        let unwrapped_filter = match filter {
            Some(filter) => filter,
            None => &UserFilter::new(),
        };
        if validate_filter(unwrapped_filter).is_err() {
            return Ok(Vec::new());
        }
        let (where_clause, bind_values) = build_where_clause(unwrapped_filter, task_ids);
        query.push_str(&format!(" {}", where_clause));

        // クエリのバインド値のインデックスを更新
        index = bind_values.len() + 1;
        filter_bind_values = bind_values;
    }

    query.push_str(" ORDER BY users.user_id ASC");

    // ページングがある場合
    let count = get_users_count_with_transaction(tx, filter, task_ids).await?;
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

    let mut query_builder = sqlx::query_as::<_, User>(&query);

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

    let result = query_builder.fetch_all(&mut **tx).await.map_err(|e| {
        DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
            ErrorKey::UserGetUsersPaginationNotFound,
            e.to_string()
        )))
    })?;

    log::debug!("Get users with pagination: {:?}", result);

    Ok(users_to_users_no_password(deduplicate(result)))
}

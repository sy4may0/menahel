use crate::errors::db_error::DBAccessError;
use crate::errors::messages::{ErrorKey, get_error_message};
use crate::repository::task_repo::get_tasks_with_pagination_with_transaction;
use crate::repository::user_repo::get_users_with_pagination_with_transaction;
use crate::repository::user_assign_repo::get_related_user_ids_by_task_ids;
use crate::repository::user_assign_repo::get_related_task_ids_by_user_ids;
use crate::models::taskwithuser::{FixedUserWithTask, FixedTaskWithUser};
use std::collections::HashMap;
use crate::models::taskwithuser::TaskWithUserFilter;
use crate::models::UserNoPassword;
use crate::models::Task;
use crate::models::UserFilter;
use crate::models::TaskFilter;
use sqlx::{Pool, Sqlite, Transaction};

pub struct TaskUserRepository {
    pool: Pool<Sqlite>,
}

impl TaskUserRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    async fn get_filtered_tasks_map(
        &self, 
        page: Option<&i32>,
        page_size: Option<&i32>,
        filter: Option<&TaskFilter>,
        tx: &mut Transaction<'_, Sqlite>,
    ) -> Result<HashMap<i64, Task>, DBAccessError> {

        let tasks = get_tasks_with_pagination_with_transaction(
            tx, page, page_size, filter,
            None
        ).await?;

        let mut task_user_map: HashMap<i64, Task> = HashMap::new();
        for task in tasks {
            if task.task_id.is_some() {
                task_user_map.insert(task.task_id.unwrap(), task);
            }
        }

        Ok(task_user_map)
    }

    async fn get_filtered_users_map(
        &self,
        page: Option<&i32>,
        page_size: Option<&i32>,
        filter: Option<&UserFilter>,
        tx: &mut Transaction<'_, Sqlite>,
    ) -> Result<HashMap<i64, UserNoPassword>, DBAccessError> {

        let users = get_users_with_pagination_with_transaction(
            tx, page, page_size, filter,
            None
        ).await?;

        let mut user_task_map: HashMap<i64, UserNoPassword> = HashMap::new();
        for user in users {
            if user.user_id.is_some() {
                user_task_map.insert(user.user_id.unwrap(), user);
            }
        }

        Ok(user_task_map)
    }

    fn get_users_by_task_id(
        &self,
        task_id: i64,
        user_map: HashMap<i64, UserNoPassword>,
        user_id_map: HashMap<i64, Vec<i64>>,
    ) -> Vec<UserNoPassword> {
        let user_ids = match user_id_map.get(&task_id) {
            Some(user_ids) => user_ids,
            None => return Vec::new(),
        };

        let mut result: Vec<UserNoPassword> = Vec::new();
        for user_id in user_ids {
            match user_map.get(user_id) {
                Some(user) => result.push(user.clone()),
                None => continue,
            }
        }
        result
    }
    
    fn get_tasks_by_user_id(
        &self,
        user_id: i64,
        task_map: HashMap<i64, Task>,
        task_id_map: HashMap<i64, Vec<i64>>,
    ) -> Vec<Task> {
        let task_ids = match task_id_map.get(&user_id) {
            Some(task_ids) => task_ids,
            None => return Vec::new(),
        };

        let mut result: Vec<Task> = Vec::new();
        for task_id in task_ids {
            match task_map.get(task_id) {
                Some(task) => result.push(task.clone()),
                None => continue,
            }
        }
        result
    }

    // [TODO] 未テスト
    pub async fn get_tasks_and_users_by_filter(
        &self,
        page: Option<&i32>,
        page_size: Option<&i32>,
        filter: Option<&TaskWithUserFilter>,
    ) -> Result<Vec<FixedTaskWithUser>, DBAccessError> {
        let mut tx = self.pool.begin().await
        .map_err(|e| DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
            ErrorKey::TaskUserGetByFilterFailed,
            e.to_string()
        ))))?;

        // フィルターでユーザーを取得
        let user_filter = match filter {
            Some(filter) => filter.build_user_filter(),
            None => None,
        };
        let users = self.get_filtered_users_map(
            None, None, user_filter.as_ref(), &mut tx
        ).await?;

        // A. フィルター+ユーザーID指定でタスクを取得
        let user_ids: Vec<i64> = users.keys().cloned().collect();
        let task_filter = match filter {
            Some(filter) => filter.build_task_filter(),
            None => None,
        };

        let tasks = get_tasks_with_pagination_with_transaction(
            &mut tx, page, page_size, task_filter.as_ref(),
            Some(&user_ids)
        ).await?;

        // B. タスクID指定でユーザーを取得
        let mut task_ids: Vec<i64> = Vec::new();
        for task in &tasks {
            if task.task_id.is_some() {
                task_ids.push(task.task_id.unwrap());
            }
        }
        let user_ids_map = get_related_user_ids_by_task_ids(task_ids, &mut tx).await?;

        // A.とB.の結果を結合
        let mut result: Vec<FixedTaskWithUser> = Vec::new();
        for task in tasks {
            let users = self.get_users_by_task_id(task.task_id.unwrap(), users.clone(), user_ids_map.clone());
            result.push(FixedTaskWithUser {
                task: task,
                users: users,
            });
        }

        Ok(result)
    }

    // [TODO] 未テスト
    pub async fn get_users_and_tasks_by_filter(
        &self,
        page: Option<&i32>,
        page_size: Option<&i32>,
        filter: Option<&TaskWithUserFilter>,
    ) -> Result<Vec<FixedUserWithTask>, DBAccessError> {
        let mut tx = self.pool.begin().await
        .map_err(|e| DBAccessError::QueryError(anyhow::anyhow!(get_error_message(
            ErrorKey::TaskUserGetByFilterFailed,
            e.to_string()
        ))))?;

        // フィルターでタスクを取得
        let task_filter = match filter {
            Some(filter) => filter.build_task_filter(),
            None => None,
        };
        let tasks = self.get_filtered_tasks_map(
            None, None, task_filter.as_ref(), &mut tx
        ).await?;

        // A. フィルター+タスクID指定でユーザーを取得
        let task_ids: Vec<i64> = tasks.keys().cloned().collect();
        let user_filter = match filter {
            Some(filter) => filter.build_user_filter(),
            None => None,
        };

        let users = get_users_with_pagination_with_transaction(
            &mut tx, page, page_size, user_filter.as_ref(),
            Some(&task_ids)
        ).await?;

        // B. ユーザーID指定でタスクを取得
        let mut user_ids: Vec<i64> = Vec::new();
        for user in &users {
            if user.user_id.is_some() {
                user_ids.push(user.user_id.unwrap());
            }
        }
        let task_ids_map = get_related_task_ids_by_user_ids(user_ids, &mut tx).await?;

        // A.とB.の結果を結合
        let mut result: Vec<FixedUserWithTask> = Vec::new();
        for user in users {
            let tasks = self.get_tasks_by_user_id(user.user_id.unwrap(), tasks.clone(), task_ids_map.clone());
            result.push(FixedUserWithTask {
                user: user,
                tasks: tasks,
            });
        }

        Ok(result)
    }

}
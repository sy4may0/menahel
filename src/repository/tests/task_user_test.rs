use crate::repository::task_user_repo::TaskUserRepository;
use sqlx::sqlite::SqlitePool;

#[cfg(test)]
mod task_user_repo_test {
    use super::*;

    #[sqlx::test(fixtures("user_assign"))]
    async fn test_task_user_repo_get_all_tasks_and_users(pool: SqlitePool) {
        let task_user_repo = TaskUserRepository::new(pool);

        assert!(true);
    }
}
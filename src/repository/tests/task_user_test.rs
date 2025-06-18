use crate::models::repository_model::task::TaskFilter;
use crate::repository::task_user_repo::TaskUserRepository;
use sqlx::sqlite::SqlitePool;

#[cfg(test)]
mod task_user_repo_test {

    use super::*;

    #[sqlx::test(fixtures("tasks_user"))]
    async fn test_task_user_repo_get_tasks_and_users_all(pool: SqlitePool) {
        let task_user_repo = TaskUserRepository::new(pool);

        let tasks_and_users = task_user_repo
            .get_tasks_and_users_by_filter(None, None, None, None)
            .await
            .unwrap();

        println!("{:?}", tasks_and_users);
        assert_eq!(tasks_and_users.len(), 8);
        assert_eq!(tasks_and_users[2].users.len(), 3);
    }

    #[sqlx::test(fixtures("tasks_user"))]
    async fn test_task_user_repo_get_tasks_and_users_with_pagination(pool: SqlitePool) {
        let task_user_repo = TaskUserRepository::new(pool);

        let tasks_and_users = task_user_repo
            .get_tasks_and_users_by_filter(Some(&1), Some(&5), None, None)
            .await
            .unwrap();

        assert_eq!(tasks_and_users.len(), 5);

        let tasks_and_users = task_user_repo
            .get_tasks_and_users_by_filter(Some(&2), Some(&5), None, None)
            .await
            .unwrap();

        assert_eq!(tasks_and_users.len(), 3);
    }

    #[sqlx::test(fixtures("tasks_user"))]
    async fn test_task_user_repo_get_tasks_and_users_with_filter(pool: SqlitePool) {
        let task_user_repo = TaskUserRepository::new(pool);
        let mut filter = TaskFilter::new();
        filter.set_level(2);

        let tasks_and_users = task_user_repo
            .get_tasks_and_users_by_filter(None, None, Some(&filter), None)
            .await
            .unwrap();

        assert_eq!(tasks_and_users.len(), 4);

        for task_and_user in tasks_and_users.iter() {
            assert_eq!(task_and_user.level, 2);
        }

        assert_eq!(tasks_and_users[0].users.len(), 3);
    }

    #[sqlx::test(fixtures("tasks_user"))]
    async fn test_task_user_repo_get_tasks_and_users_with_user_ids(pool: SqlitePool) {
        let task_user_repo = TaskUserRepository::new(pool);
        let mut filter = TaskFilter::new();
        filter.set_level(2);

        let tasks_and_users = task_user_repo
            .get_tasks_and_users_by_filter(None, None, None, Some(&vec![1, 2]))
            .await
            .unwrap();

        assert_eq!(tasks_and_users.len(), 4);
        assert_eq!(tasks_and_users[0].users.len(), 3);

        let mut user_ids = vec![];
        for task_and_user in tasks_and_users.iter() {
            let uids = task_and_user
                .users
                .iter()
                .map(|user| user.user_id.unwrap())
                .collect::<Vec<i64>>();
            user_ids.push(uids);
        }

        for uids in user_ids.iter() {
            assert!(uids.contains(&1) || uids.contains(&2));
        }
    }

    #[sqlx::test(fixtures("tasks_user"))]
    async fn test_task_user_repo_get_tasks_and_users_with_full_options(pool: SqlitePool) {
        let task_user_repo = TaskUserRepository::new(pool);
        let mut filter = TaskFilter::new();
        filter.set_level(2);

        let tasks_and_users = task_user_repo
            .get_tasks_and_users_by_filter(Some(&1), Some(&3), Some(&filter), Some(&vec![1, 2]))
            .await
            .unwrap();

        assert_eq!(tasks_and_users.len(), 3);

        for task_and_user in tasks_and_users.iter() {
            assert_eq!(task_and_user.level, 2);
        }

        let mut user_ids = vec![];
        for task_and_user in tasks_and_users.iter() {
            let uids = task_and_user
                .users
                .iter()
                .map(|user| user.user_id.unwrap())
                .collect::<Vec<i64>>();
            user_ids.push(uids);
        }

        for uids in user_ids.iter() {
            assert!(uids.contains(&1) || uids.contains(&2));
        }

        let tasks_and_users = task_user_repo
            .get_tasks_and_users_by_filter(Some(&2), Some(&3), Some(&filter), Some(&vec![1, 2]))
            .await
            .unwrap();

        assert_eq!(tasks_and_users.len(), 1);

        let mut user_ids = vec![];
        for task_and_user in tasks_and_users.iter() {
            let uids = task_and_user
                .users
                .iter()
                .map(|user| user.user_id.unwrap())
                .collect::<Vec<i64>>();
            user_ids.push(uids);
        }

        for uids in user_ids.iter() {
            assert!(uids.contains(&1) || uids.contains(&2));
        }
    }

    #[sqlx::test(fixtures("tasks_user"))]
    async fn test_task_user_repo_get_tasks_and_users_with_invalid_pagination(pool: SqlitePool) {
        let task_user_repo = TaskUserRepository::new(pool);

        let tasks_and_users = task_user_repo
            .get_tasks_and_users_by_filter(Some(&1), Some(&0), None, None)
            .await;

        assert!(tasks_and_users.is_err());

        let tasks_and_users = task_user_repo
            .get_tasks_and_users_by_filter(Some(&10), Some(&20), None, None)
            .await;

        assert!(tasks_and_users.is_err());
    }

    #[sqlx::test(fixtures("tasks_user"))]
    async fn test_task_user_repo_get_tasks_and_users_with_invalid_filter(pool: SqlitePool) {
        let task_user_repo = TaskUserRepository::new(pool);
        let mut filter = TaskFilter::new();
        filter.set_level(10);
        filter.set_created_at_from(1000);
        filter.set_created_at_to(100);

        let tasks_and_users = task_user_repo
            .get_tasks_and_users_by_filter(None, None, Some(&filter), None)
            .await
            .unwrap();

        assert_eq!(tasks_and_users.len(), 0);
    }

    #[sqlx::test(fixtures("tasks_user"))]
    async fn test_task_user_repo_get_tasks_and_users_with_invalid_user_ids(pool: SqlitePool) {
        let task_user_repo = TaskUserRepository::new(pool);

        let tasks_and_users = task_user_repo
            .get_tasks_and_users_by_filter(None, None, None, Some(&vec![1000]))
            .await
            .unwrap();

        assert_eq!(tasks_and_users.len(), 0);
    }
}

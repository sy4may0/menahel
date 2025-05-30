use crate::models::UserAssign;
use crate::repository::user_assign_repo::{
    UserAssignRepository, get_user_assign_by_id_with_transaction,
    get_user_assign_by_task_id_with_transaction, get_user_assign_by_user_id_with_transaction,
};
use sqlx::sqlite::SqlitePool;

#[cfg(test)]
mod user_assign_repo_test {
    use super::*;

    #[sqlx::test(fixtures("user_assign"))]
    async fn test_user_assign_repo_create_user_assign(pool: SqlitePool) {
        let user_assign_repo = UserAssignRepository::new(pool);

        let user_assign = UserAssign {
            id: None,
            user_id: 1,
            task_id: 13,
        };

        let created_user_assign = user_assign_repo
            .create_user_assign(user_assign)
            .await
            .unwrap();
        assert_eq!(created_user_assign.user_id, 1);
        let retrieved_user_assign = user_assign_repo
            .get_user_assign_by_id(created_user_assign.id.unwrap())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(retrieved_user_assign.user_id, 1);
        assert_eq!(retrieved_user_assign.task_id, 13);
    }

    #[sqlx::test(fixtures("user_assign"))]
    async fn test_user_assign_repo_create_user_assign_userid_invalid(pool: SqlitePool) {
        let user_assign_repo = UserAssignRepository::new(pool);

        let user_assign = UserAssign {
            id: None,
            user_id: 0,
            task_id: 13,
        };

        let result = user_assign_repo.create_user_assign(user_assign).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("user_assign"))]
    async fn test_user_assign_repo_create_user_assign_taskid_invalid(pool: SqlitePool) {
        let user_assign_repo = UserAssignRepository::new(pool);

        let user_assign = UserAssign {
            id: None,
            user_id: 1,
            task_id: 0,
        };

        let result = user_assign_repo.create_user_assign(user_assign).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("user_assign"))]
    async fn test_user_assign_repo_create_user_assign_userid_not_found(pool: SqlitePool) {
        let user_assign_repo = UserAssignRepository::new(pool);

        let user_assign = UserAssign {
            id: None,
            user_id: 100,
            task_id: 13,
        };

        let result = user_assign_repo.create_user_assign(user_assign).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("user_assign"))]
    async fn test_user_assign_repo_create_user_assign_taskid_not_found(pool: SqlitePool) {
        let user_assign_repo = UserAssignRepository::new(pool);

        let user_assign = UserAssign {
            id: None,
            user_id: 1,
            task_id: 100,
        };

        let result = user_assign_repo.create_user_assign(user_assign).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("user_assign"))]
    async fn test_user_assign_repo_create_user_assign_not_maxlevel_task(pool: SqlitePool) {
        let user_assign_repo = UserAssignRepository::new(pool);

        let user_assign = UserAssign {
            id: None,
            user_id: 2,
            task_id: 1,
        };

        let result = user_assign_repo.create_user_assign(user_assign).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("user_assign"))]
    async fn test_user_assign_repo_create_user_assign_same_user_assign_exists(pool: SqlitePool) {
        let user_assign_repo = UserAssignRepository::new(pool);

        let user_assign = UserAssign {
            id: None,
            user_id: 1,
            task_id: 11,
        };

        let result = user_assign_repo.create_user_assign(user_assign).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("user_assign"))]
    async fn test_user_assign_repo_get_user_assign_by_id(pool: SqlitePool) {
        let user_assign_repo = UserAssignRepository::new(pool);

        let user_assign = user_assign_repo
            .get_user_assign_by_id(1)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(user_assign.user_id, 1);
        assert_eq!(user_assign.task_id, 3);
    }

    #[sqlx::test(fixtures("user_assign"))]
    async fn test_user_assign_repo_get_user_assign_by_id_not_found(pool: SqlitePool) {
        let user_assign_repo = UserAssignRepository::new(pool);

        let user_assign = user_assign_repo.get_user_assign_by_id(100).await.unwrap();
        assert!(user_assign.is_none());
    }

    #[sqlx::test(fixtures("user_assign"))]
    async fn test_user_assign_repo_get_user_assign_by_task_id(pool: SqlitePool) {
        let user_assign_repo = UserAssignRepository::new(pool);

        let user_assign = user_assign_repo
            .get_user_assign_by_task_id(3)
            .await
            .unwrap();
        assert_eq!(user_assign.len(), 2);
        for user_assign in user_assign {
            assert_eq!(user_assign.task_id, 3);
        }
    }

    #[sqlx::test(fixtures("user_assign"))]
    async fn test_user_assign_repo_get_user_assign_by_task_id_not_found(pool: SqlitePool) {
        let user_assign_repo = UserAssignRepository::new(pool);

        let user_assign = user_assign_repo
            .get_user_assign_by_task_id(100)
            .await
            .unwrap();
        assert_eq!(user_assign.len(), 0);
    }

    #[sqlx::test(fixtures("user_assign"))]
    async fn test_user_assign_repo_get_user_assign_by_user_id(pool: SqlitePool) {
        let user_assign_repo = UserAssignRepository::new(pool);

        let user_assign = user_assign_repo
            .get_user_assign_by_user_id(1)
            .await
            .unwrap();
        assert_eq!(user_assign.len(), 2);
        for user_assign in user_assign {
            assert_eq!(user_assign.user_id, 1);
        }
    }

    #[sqlx::test(fixtures("user_assign"))]
    async fn test_user_assign_repo_get_user_assign_by_user_id_not_found(pool: SqlitePool) {
        let user_assign_repo = UserAssignRepository::new(pool);

        let user_assign = user_assign_repo
            .get_user_assign_by_user_id(100)
            .await
            .unwrap();
        assert_eq!(user_assign.len(), 0);
    }

    #[sqlx::test(fixtures("user_assign"))]
    async fn test_user_assign_repo_get_user_assign_by_user_id_and_task_id(pool: SqlitePool) {
        let user_assign_repo = UserAssignRepository::new(pool);

        let user_assign = user_assign_repo
            .get_user_assign_by_user_id_and_task_id(1, 3)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(user_assign.user_id, 1);
        assert_eq!(user_assign.task_id, 3);
    }

    #[sqlx::test(fixtures("user_assign"))]
    async fn test_user_assign_repo_get_user_assign_by_user_id_and_task_id_not_found(
        pool: SqlitePool,
    ) {
        let user_assign_repo = UserAssignRepository::new(pool);

        let user_assign = user_assign_repo
            .get_user_assign_by_user_id_and_task_id(1, 100)
            .await
            .unwrap();
        assert!(user_assign.is_none());
    }

    #[sqlx::test(fixtures("user_assign"))]
    async fn test_user_assign_repo_get_all_user_assigns(pool: SqlitePool) {
        let user_assign_repo = UserAssignRepository::new(pool);

        let user_assigns = user_assign_repo.get_all_user_assigns().await.unwrap();
        assert_eq!(user_assigns.len(), 4);
    }

    #[sqlx::test(fixtures("user_assign"))]
    async fn test_user_assign_repo_update_user_assign(pool: SqlitePool) {
        let user_assign_repo = UserAssignRepository::new(pool);

        let user_assign = UserAssign {
            id: Some(1),
            user_id: 2,
            task_id: 11,
        };

        let updated_user_assign = user_assign_repo
            .update_user_assign(user_assign)
            .await
            .unwrap();
        assert_eq!(updated_user_assign.user_id, 2);
        assert_eq!(updated_user_assign.task_id, 11);
    }

    #[sqlx::test(fixtures("user_assign"))]
    async fn test_user_assign_repo_update_user_assign_not_found(pool: SqlitePool) {
        let user_assign_repo = UserAssignRepository::new(pool);

        let user_assign = UserAssign {
            id: Some(100),
            user_id: 2,
            task_id: 11,
        };

        let result = user_assign_repo.update_user_assign(user_assign).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("user_assign"))]
    async fn test_user_assign_repo_update_user_assign_userid_invalid(pool: SqlitePool) {
        let user_assign_repo = UserAssignRepository::new(pool);

        let user_assign = UserAssign {
            id: Some(1),
            user_id: 0, // 無効なuser_id
            task_id: 11,
        };

        let result = user_assign_repo.update_user_assign(user_assign).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("user_assign"))]
    async fn test_user_assign_repo_update_user_assign_taskid_invalid(pool: SqlitePool) {
        let user_assign_repo = UserAssignRepository::new(pool);

        let user_assign = UserAssign {
            id: Some(1),
            user_id: 1,
            task_id: 0, // 無効なtask_id
        };

        let result = user_assign_repo.update_user_assign(user_assign).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("user_assign"))]
    async fn test_user_assign_repo_update_user_assign_userid_not_found(pool: SqlitePool) {
        let user_assign_repo = UserAssignRepository::new(pool);

        let user_assign = UserAssign {
            id: Some(1),
            user_id: 100, // 存在しないuser_id
            task_id: 11,
        };

        let result = user_assign_repo.update_user_assign(user_assign).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("user_assign"))]
    async fn test_user_assign_repo_update_user_assign_taskid_not_found(pool: SqlitePool) {
        let user_assign_repo = UserAssignRepository::new(pool);

        let user_assign = UserAssign {
            id: Some(1),
            user_id: 1,
            task_id: 100, // 存在しないtask_id
        };

        let result = user_assign_repo.update_user_assign(user_assign).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("user_assign"))]
    async fn test_user_assign_repo_update_user_assign_not_maxlevel_task(pool: SqlitePool) {
        let user_assign_repo = UserAssignRepository::new(pool);

        let user_assign = UserAssign {
            id: Some(1),
            user_id: 2,
            task_id: 1, // 最大レベルでないタスク
        };

        let result = user_assign_repo.update_user_assign(user_assign).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("user_assign"))]
    async fn test_user_assign_repo_update_user_assign_same_user_assign_exists(pool: SqlitePool) {
        let user_assign_repo = UserAssignRepository::new(pool);

        let user_assign = UserAssign {
            id: Some(1),
            user_id: 2, // 既に同じタスクにアサインされているユーザー
            task_id: 3,
        };

        let result = user_assign_repo.update_user_assign(user_assign).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("user_assign"))]
    async fn test_user_assign_repo_delete_user_assign(pool: SqlitePool) {
        let user_assign_repo = UserAssignRepository::new(pool);
        let result = user_assign_repo.delete_user_assign(1).await;
        assert!(result.is_ok());
    }

    #[sqlx::test(fixtures("user_assign"))]
    async fn test_user_assign_repo_delete_user_assign_not_found(pool: SqlitePool) {
        let user_assign_repo = UserAssignRepository::new(pool);
        let result = user_assign_repo.delete_user_assign(100).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("user_assign"))]
    async fn test_user_assign_repo_get_user_assign_by_id_with_transaction(pool: SqlitePool) {
        let mut tx = pool.begin().await.unwrap();

        let user_assign = get_user_assign_by_id_with_transaction(1, &mut tx)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(user_assign.user_id, 1);
        assert_eq!(user_assign.task_id, 3);
    }

    #[sqlx::test(fixtures("user_assign"))]
    async fn test_user_assign_repo_get_user_assign_by_id_with_transaction_not_found(
        pool: SqlitePool,
    ) {
        let mut tx = pool.begin().await.unwrap();

        let user_assign = get_user_assign_by_id_with_transaction(100, &mut tx)
            .await
            .unwrap();
        assert!(user_assign.is_none());
    }

    #[sqlx::test(fixtures("user_assign"))]
    async fn test_user_assign_repo_get_user_assign_by_task_id_with_transaction(pool: SqlitePool) {
        let mut tx = pool.begin().await.unwrap();

        let user_assign = get_user_assign_by_task_id_with_transaction(3, &mut tx)
            .await
            .unwrap();
        assert_eq!(user_assign.len(), 2);
    }

    #[sqlx::test(fixtures("user_assign"))]
    async fn test_user_assign_repo_get_user_assign_by_task_id_with_transaction_not_found(
        pool: SqlitePool,
    ) {
        let mut tx = pool.begin().await.unwrap();

        let user_assign = get_user_assign_by_task_id_with_transaction(100, &mut tx)
            .await
            .unwrap();
        assert_eq!(user_assign.len(), 0);
    }

    #[sqlx::test(fixtures("user_assign"))]
    async fn test_user_assign_repo_get_user_assign_by_user_id_with_transaction(pool: SqlitePool) {
        let mut tx = pool.begin().await.unwrap();

        let user_assign = get_user_assign_by_user_id_with_transaction(1, &mut tx)
            .await
            .unwrap();
        assert_eq!(user_assign.len(), 2);
    }

    #[sqlx::test(fixtures("user_assign"))]
    async fn test_user_assign_repo_get_user_assign_by_user_id_with_transaction_not_found(
        pool: SqlitePool,
    ) {
        let mut tx = pool.begin().await.unwrap();

        let user_assign = get_user_assign_by_user_id_with_transaction(100, &mut tx)
            .await
            .unwrap();
        assert_eq!(user_assign.len(), 0);
    }
}

use crate::models::Comment;
use crate::repository::comment_repo::{
    CommentRepository, get_comment_by_id_with_transaction,
    get_comment_count_by_task_id_with_transaction, get_comment_count_by_user_id_with_transaction,
    get_comment_count_with_transaction,
};
use sqlx::sqlite::SqlitePool;

#[cfg(test)]
mod comment_repo_test {
    use super::*;

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_create_comment(pool: SqlitePool) {
        let comment_repo = CommentRepository::new(pool);

        let comment = Comment::new(1, 11, "Test Comment 0".to_string());

        let created_comment = comment_repo.create_comment(comment).await.unwrap();
        assert_eq!(created_comment.user_id, 1);
        assert_eq!(created_comment.task_id, 11);
        assert_eq!(created_comment.content, "Test Comment 0");
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_create_comment_with_invalid_user_id(pool: SqlitePool) {
        let comment_repo = CommentRepository::new(pool);

        let comment = Comment::new(0, 11, "Test Comment 0".to_string());

        let result = comment_repo.create_comment(comment).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_create_comment_with_invalid_task_id(pool: SqlitePool) {
        let comment_repo = CommentRepository::new(pool);

        let comment = Comment::new(1, 0, "Test Comment 0".to_string());

        let result = comment_repo.create_comment(comment).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_create_comment_with_empty_content(pool: SqlitePool) {
        let comment_repo = CommentRepository::new(pool);

        let comment = Comment::new(1, 11, "".to_string());

        let result = comment_repo.create_comment(comment).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_create_comment_with_too_long_content(pool: SqlitePool) {
        let comment_repo = CommentRepository::new(pool);

        let comment = Comment::new(1, 11, "a".repeat(2025));

        let result = comment_repo.create_comment(comment).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_create_comment_with_nonexistent_user_id(pool: SqlitePool) {
        let comment_repo = CommentRepository::new(pool);

        let comment = Comment::new(100, 11, "Test Comment 0".to_string());

        let result = comment_repo.create_comment(comment).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_create_comment_with_nonexistent_task_id(pool: SqlitePool) {
        let comment_repo = CommentRepository::new(pool);

        let comment = Comment::new(1, 100, "Test Comment 0".to_string());

        let result = comment_repo.create_comment(comment).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_create_comment_with_not_max_level_task(pool: SqlitePool) {
        let comment_repo = CommentRepository::new(pool);

        let comment = Comment::new(1, 1, "Test Comment 0".to_string());

        let result = comment_repo.create_comment(comment).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_create_comment_to_same_user_and_task(pool: SqlitePool) {
        let comment_repo = CommentRepository::new(pool);

        let comment = Comment::new(1, 3, "Test Comment 0 additional".to_string());

        let result = comment_repo.create_comment(comment).await;
        assert!(result.is_ok());
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_get_comment_by_id(pool: SqlitePool) {
        let comment_repo = CommentRepository::new(pool);

        let comment = comment_repo.get_comment_by_id(1).await.unwrap();
        assert_eq!(comment.user_id, 1);
        assert_eq!(comment.task_id, 3);
        assert_eq!(comment.content, "Test Comment 0");
        assert_eq!(comment.user.user_id, Some(1));
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_get_comment_by_id_not_found(pool: SqlitePool) {
        let comment_repo = CommentRepository::new(pool);

        let result = comment_repo.get_comment_by_id(100).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_get_comment_by_task_id(pool: SqlitePool) {
        let comment_repo = CommentRepository::new(pool);

        let comments = comment_repo.get_comment_by_task_id(3).await.unwrap();
        assert_eq!(comments.len(), 4);
        assert_eq!(comments[0].user_id, 1);
        assert_eq!(comments[0].task_id, 3);
        assert_eq!(comments[0].content, "Test Comment 0");
        assert_eq!(comments[0].user.user_id, Some(1));
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_get_comment_by_task_id_not_found(pool: SqlitePool) {
        let comment_repo = CommentRepository::new(pool);

        let comments = comment_repo.get_comment_by_task_id(100).await.unwrap();
        assert!(comments.is_empty());
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_get_comment_by_user_id(pool: SqlitePool) {
        let comment_repo = CommentRepository::new(pool);

        let comments = comment_repo.get_comment_by_user_id(1).await.unwrap();
        assert_eq!(comments.len(), 6);
        for comment in comments {
            assert_eq!(comment.user_id, 1);
            assert_eq!(comment.user.user_id, Some(1));
        }
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_get_comment_by_user_id_not_found(pool: SqlitePool) {
        let comment_repo = CommentRepository::new(pool);

        let comments = comment_repo.get_comment_by_user_id(100).await.unwrap();
        assert!(comments.is_empty());
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_update_comment(pool: SqlitePool) {
        let comment_repo = CommentRepository::new(pool);

        let comment = Comment {
            comment_id: Some(1),
            user_id: 2,
            task_id: 3,
            content: "Test Comment 0 updated".to_string(),
            created_at: 0,
            updated_at: None,
        };

        let updated_comment = comment_repo.update_comment(comment).await.unwrap();
        assert_eq!(updated_comment.user_id, 2);
        assert_eq!(updated_comment.task_id, 3);
        assert_eq!(updated_comment.content, "Test Comment 0 updated");
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_update_comment_not_found(pool: SqlitePool) {
        let comment_repo = CommentRepository::new(pool);

        let comment = Comment::new(100, 3, "Test Comment 0 updated".to_string());

        let result = comment_repo.update_comment(comment).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_update_comment_with_invalid_user_id(pool: SqlitePool) {
        let comment_repo = CommentRepository::new(pool);

        let comment = Comment {
            comment_id: Some(1),
            user_id: 0,
            task_id: 3,
            content: "Test Comment 0 updated".to_string(),
            created_at: 0,
            updated_at: None,
        };

        let result = comment_repo.update_comment(comment).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_update_comment_with_invalid_task_id(pool: SqlitePool) {
        let comment_repo = CommentRepository::new(pool);

        let comment = Comment {
            comment_id: Some(1),
            user_id: 1,
            task_id: 0,
            content: "Test Comment 0 updated".to_string(),
            created_at: 0,
            updated_at: None,
        };

        let result = comment_repo.update_comment(comment).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_update_comment_with_empty_content(pool: SqlitePool) {
        let comment_repo = CommentRepository::new(pool);

        let comment = Comment {
            comment_id: Some(1),
            user_id: 1,
            task_id: 3,
            content: "".to_string(),
            created_at: 0,
            updated_at: None,
        };

        let result = comment_repo.update_comment(comment).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_update_comment_with_too_long_content(pool: SqlitePool) {
        let comment_repo = CommentRepository::new(pool);

        let comment = Comment {
            comment_id: Some(1),
            user_id: 1,
            task_id: 3,
            content: "a".repeat(2025),
            created_at: 0,
            updated_at: None,
        };

        let result = comment_repo.update_comment(comment).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_update_comment_with_nonexistent_user_id(pool: SqlitePool) {
        let comment_repo = CommentRepository::new(pool);

        let comment = Comment {
            comment_id: Some(1),
            user_id: 100,
            task_id: 3,
            content: "Test Comment 0 updated".to_string(),
            created_at: 0,
            updated_at: None,
        };

        let result = comment_repo.update_comment(comment).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_update_comment_with_nonexistent_task_id(pool: SqlitePool) {
        let comment_repo = CommentRepository::new(pool);

        let comment = Comment {
            comment_id: Some(1),
            user_id: 1,
            task_id: 100,
            content: "Test Comment 0 updated".to_string(),
            created_at: 0,
            updated_at: None,
        };

        let result = comment_repo.update_comment(comment).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_update_comment_with_not_max_level_task(pool: SqlitePool) {
        let comment_repo = CommentRepository::new(pool);

        let comment = Comment {
            comment_id: Some(1),
            user_id: 1,
            task_id: 1,
            content: "Test Comment 0 updated".to_string(),
            created_at: 0,
            updated_at: None,
        };

        let result = comment_repo.update_comment(comment).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_delete_comment(pool: SqlitePool) {
        let comment_repo = CommentRepository::new(pool);

        let result = comment_repo.delete_comment(1).await;
        assert!(result.is_ok());
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_delete_comment_not_found(pool: SqlitePool) {
        let comment_repo = CommentRepository::new(pool);

        let result = comment_repo.delete_comment(100).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_get_comment_by_id_with_transaction(pool: SqlitePool) {
        let mut tx = pool.begin().await.unwrap();

        let comment = get_comment_by_id_with_transaction(1, &mut tx)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(comment.user_id, 1);
        assert_eq!(comment.task_id, 3);
        assert_eq!(comment.content, "Test Comment 0");

        tx.commit().await.unwrap();
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_get_comment_by_id_with_transaction_not_found(pool: SqlitePool) {
        let mut tx = pool.begin().await.unwrap();

        let comment = get_comment_by_id_with_transaction(100, &mut tx)
            .await
            .unwrap();
        assert!(comment.is_none());

        tx.commit().await.unwrap();
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_get_comment_count_with_transaction(pool: SqlitePool) {
        let mut tx = pool.begin().await.unwrap();
        let count = get_comment_count_with_transaction(&mut tx).await.unwrap();
        assert_eq!(count, 12);
        tx.commit().await.unwrap();
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_get_comment_count_by_task_id_with_transaction(pool: SqlitePool) {
        let mut tx = pool.begin().await.unwrap();
        let count = get_comment_count_by_task_id_with_transaction(3, &mut tx)
            .await
            .unwrap();
        assert_eq!(count, 4);
        tx.commit().await.unwrap();
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_get_comment_count_by_user_id_with_transaction(pool: SqlitePool) {
        let mut tx = pool.begin().await.unwrap();
        let count = get_comment_count_by_user_id_with_transaction(1, &mut tx)
            .await
            .unwrap();
        assert_eq!(count, 6);
        tx.commit().await.unwrap();
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_get_comments_with_pagination(pool: SqlitePool) {
        let comment_repo = CommentRepository::new(pool);

        let comments = comment_repo
            .get_comments_with_pagination(&1, &10)
            .await
            .unwrap();
        assert_eq!(comments.len(), 10);

        let comments = comment_repo
            .get_comments_with_pagination(&2, &10)
            .await
            .unwrap();
        assert_eq!(comments.len(), 2);
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_get_comments_with_pagination_not_found(pool: SqlitePool) {
        let comment_repo = CommentRepository::new(pool);

        let comments = comment_repo.get_comments_with_pagination(&3, &10).await;
        assert!(comments.is_err());
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_get_comments_with_pagination_invalid_page(pool: SqlitePool) {
        let comment_repo = CommentRepository::new(pool);

        let comments = comment_repo.get_comments_with_pagination(&0, &2).await;
        assert!(comments.is_err());

        let comments = comment_repo.get_comments_with_pagination(&1, &0).await;
        assert!(comments.is_err());
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_get_comments_with_pagination_by_task_id(pool: SqlitePool) {
        let comment_repo = CommentRepository::new(pool);

        let comments = comment_repo
            .get_comments_with_pagination_by_task_id(3, &1, &3)
            .await
            .unwrap();
        assert_eq!(comments.len(), 3);

        let comments = comment_repo
            .get_comments_with_pagination_by_task_id(3, &2, &3)
            .await
            .unwrap();
        assert_eq!(comments.len(), 1);
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_get_comments_with_pagination_by_task_id_id_not_found(
        pool: SqlitePool,
    ) {
        let comment_repo = CommentRepository::new(pool);

        let comments = comment_repo
            .get_comments_with_pagination_by_task_id(100, &1, &3)
            .await
            .unwrap();
        assert_eq!(comments.len(), 0);
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_get_comments_with_pagination_by_task_id_not_found(pool: SqlitePool) {
        let comment_repo = CommentRepository::new(pool);

        let comments = comment_repo
            .get_comments_with_pagination_by_task_id(11, &10, &10)
            .await;
        assert!(comments.is_err());
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_get_comments_with_pagination_by_task_id_invalid_page(
        pool: SqlitePool,
    ) {
        let comment_repo = CommentRepository::new(pool);

        let comments = comment_repo
            .get_comments_with_pagination_by_task_id(11, &0, &3)
            .await;
        assert!(comments.is_err());

        let comments = comment_repo
            .get_comments_with_pagination_by_task_id(11, &1, &0)
            .await;
        assert!(comments.is_err());
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_get_comments_with_pagination_by_user_id(pool: SqlitePool) {
        let comment_repo = CommentRepository::new(pool);

        let comments = comment_repo
            .get_comments_with_pagination_by_user_id(1, &1, &4)
            .await
            .unwrap();
        assert_eq!(comments.len(), 4);

        let comments = comment_repo
            .get_comments_with_pagination_by_user_id(1, &2, &4)
            .await
            .unwrap();
        assert_eq!(comments.len(), 2);
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_get_comments_with_pagination_by_user_id_id_not_found(
        pool: SqlitePool,
    ) {
        let comment_repo = CommentRepository::new(pool);

        let comments = comment_repo
            .get_comments_with_pagination_by_user_id(100, &1, &3)
            .await
            .unwrap();
        assert_eq!(comments.len(), 0);
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_get_comments_with_pagination_by_user_id_not_found(pool: SqlitePool) {
        let comment_repo = CommentRepository::new(pool);

        let comments = comment_repo
            .get_comments_with_pagination_by_user_id(11, &10, &10)
            .await;
        assert!(comments.is_err());
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_get_comments_with_pagination_by_user_id_invalid_page(
        pool: SqlitePool,
    ) {
        let comment_repo = CommentRepository::new(pool);

        let comments = comment_repo
            .get_comments_with_pagination_by_user_id(11, &0, &3)
            .await;
        assert!(comments.is_err());

        let comments = comment_repo
            .get_comments_with_pagination_by_user_id(11, &1, &0)
            .await;
        assert!(comments.is_err());
    }

    #[sqlx::test(fixtures("comments"))]
    async fn test_comment_repo_get_all_comments(pool: SqlitePool) {
        let comment_repo = CommentRepository::new(pool);
        let comments = comment_repo.get_all_comments().await.unwrap();
        assert_eq!(comments.len(), 12);
    }
}

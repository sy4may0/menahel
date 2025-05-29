use crate::models::User;
use crate::repository::user_repo::{UserRepository, get_user_by_id_with_transaction};
use chrono::Utc;
use sha2::{Digest, Sha256};
use sqlx::sqlite::SqlitePool;

fn build_password() -> String {
    let now = Utc::now().format("%Y%m%d%H%M%S").to_string();
    format!("{:x}", Sha256::digest(now.as_bytes()))
}

#[cfg(test)]
mod user_test {
    use super::*;

    #[sqlx::test]
    async fn test_user_repo_create_user(pool: SqlitePool) {
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let user_repo = UserRepository::new(pool);

        let username = format!("create_test_{}", now);
        let email = format!("create_test_{}@test.com", now);
        let password_hash = build_password();

        // 上の変数は後でassert_eqで使うので、cloneする。
        let user = User {
            id: None,
            username: username.clone(),
            email: email.clone(),
            password_hash: password_hash.clone(),
        };

        let created_user = user_repo.create_user(user).await.unwrap();
        assert_eq!(created_user.username, username);
        assert_eq!(created_user.email, email);
        assert_ne!(created_user.id, None);

        // Result<Option<User>>が返ってくるので、2回unwrapする。
        let retrieved_user = user_repo
            .get_user_by_id(created_user.id.unwrap())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(retrieved_user.username, username);
        assert_eq!(retrieved_user.email, email);
        assert_eq!(retrieved_user.id, created_user.id);
    }

    #[sqlx::test]
    async fn test_user_repo_get_user_by_id(pool: SqlitePool) {
        let user_repo = UserRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();

        let username = format!("get_user_by_id_test_{}", now);
        let email = format!("get_user_by_id_test_{}@test.com", now);
        let password_hash = build_password();

        let user = User {
            id: None,
            username: username.clone(),
            email: email.clone(),
            password_hash: password_hash.clone(),
        };

        let created_user = user_repo.create_user(user).await.unwrap();

        let retrieved_user = user_repo
            .get_user_by_id(created_user.id.unwrap())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(retrieved_user.username, username);
        assert_eq!(retrieved_user.email, email);
        assert_eq!(retrieved_user.id, created_user.id);

        let not_found_user = user_repo.get_user_by_id(114514).await.unwrap();
        assert!(not_found_user.is_none());
    }

    #[sqlx::test]
    async fn test_user_repo_get_user_by_name(pool: SqlitePool) {
        let user_repo = UserRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();

        let username = format!("get_user_by_name_test_{}", now);
        let email = format!("get_user_by_name_test_{}@test.com", now);
        let password_hash = build_password();

        let user = User {
            id: None,
            username: username.clone(),
            email: email.clone(),
            password_hash: password_hash.clone(),
        };

        let created_user = user_repo.create_user(user).await.unwrap();

        let retrieved_user = user_repo
            .get_user_by_name(&username)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(retrieved_user.username, username);
        assert_eq!(retrieved_user.email, email);
        assert_eq!(retrieved_user.id, created_user.id);

        let not_found_user = user_repo
            .get_user_by_name("XXX_SUPER_UNCHI_XXX")
            .await
            .unwrap();
        assert!(not_found_user.is_none());
    }

    #[sqlx::test]
    async fn test_user_repo_get_all_users(pool: SqlitePool) {
        let user_repo = UserRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();

        let user_count = 5;
        let users = (1..=user_count)
            .map(|i| User {
                id: None,
                username: format!("get_all_users_test_{}_{}", now, i),
                email: format!("get_all_users_test_{}_{}@test.com", now, i),
                password_hash: build_password(),
            })
            .collect::<Vec<User>>();

        let mut created_users = Vec::new();
        for user in users.into_iter() {
            let created_user = user_repo.create_user(user).await.unwrap();
            created_users.push(created_user);
        }

        let retrieved_users = user_repo.get_all_users().await.unwrap();
        assert_eq!(retrieved_users.len(), user_count);

        for (i, user) in retrieved_users.iter().enumerate() {
            assert_eq!(user.username, created_users[i].username);
            assert_eq!(user.email, created_users[i].email);
            assert_eq!(user.id, created_users[i].id);
        }
    }

    #[sqlx::test]
    async fn test_user_repo_get_all_users_empty(pool: SqlitePool) {
        let user_repo = UserRepository::new(pool);
        let retrieved_users = user_repo.get_all_users().await.unwrap();
        assert_eq!(retrieved_users.len(), 0);
    }

    #[sqlx::test]
    async fn test_user_repo_update_user(pool: SqlitePool) {
        let user_repo = UserRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();

        let username = format!("update_user_test_{}", now);
        let email = format!("update_user_test_{}@test.com", now);
        let password_hash = build_password();

        let user = User {
            id: None,
            username: username.clone(),
            email: email.clone(),
            password_hash: password_hash.clone(),
        };

        let created_user = user_repo.create_user(user).await.unwrap();

        let updated_user = User {
            id: created_user.id,
            username: username.clone() + "_updated",
            email: email.clone() + "_updated",
            password_hash: build_password(),
        };

        user_repo.update_user(updated_user).await.unwrap();

        let retrieved_user = user_repo
            .get_user_by_id(created_user.id.unwrap())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(retrieved_user.username, username + "_updated");
        assert_eq!(retrieved_user.email, email + "_updated");
        assert_eq!(retrieved_user.id, created_user.id);
    }

    #[sqlx::test]
    async fn test_user_repo_delete_user(pool: SqlitePool) {
        let user_repo = UserRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();

        let username = format!("delete_user_test_{}", now);
        let email = format!("delete_user_test_{}@test.com", now);
        let password_hash = build_password();

        let user = User {
            id: None,
            username: username.clone(),
            email: email.clone(),
            password_hash: password_hash.clone(),
        };

        let created_user = user_repo.create_user(user).await.unwrap();
        let retrieved_user = user_repo
            .get_user_by_id(created_user.id.unwrap())
            .await
            .unwrap();
        assert!(!retrieved_user.is_none());

        user_repo
            .delete_user(created_user.id.unwrap())
            .await
            .unwrap();
        let retrieved_user = user_repo
            .get_user_by_id(created_user.id.unwrap())
            .await
            .unwrap();
        assert!(retrieved_user.is_none());
    }

    #[sqlx::test]
    async fn test_user_repo_create_duplicate_username(pool: SqlitePool) {
        let user_repo = UserRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();

        let username = format!("duplicate_test_{}", now);
        let email1 = format!("duplicate_test_1_{}@test.com", now);
        let email2 = format!("duplicate_test_2_{}@test.com", now);
        let password_hash = build_password();

        let user1 = User {
            id: None,
            username: username.clone(),
            email: email1,
            password_hash: password_hash.clone(),
        };

        let user2 = User {
            id: None,
            username: username.clone(),
            email: email2,
            password_hash: password_hash.clone(),
        };

        user_repo.create_user(user1).await.unwrap();
        let result = user_repo.create_user(user2).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_user_repo_create_duplicate_email(pool: SqlitePool) {
        let user_repo = UserRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();

        let username1 = format!("duplicate_test_1_{}", now);
        let username2 = format!("duplicate_test_2_{}", now);
        let email = format!("duplicate_test_{}@test.com", now);
        let password_hash = build_password();

        let user1 = User {
            id: None,
            username: username1,
            email: email.clone(),
            password_hash: password_hash.clone(),
        };

        let user2 = User {
            id: None,
            username: username2,
            email: email.clone(),
            password_hash: password_hash.clone(),
        };

        user_repo.create_user(user1).await.unwrap();
        let result = user_repo.create_user(user2).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_user_repo_update_nonexistent_user(pool: SqlitePool) {
        let user_repo = UserRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();

        let user = User {
            id: Some(114514),
            username: format!("nonexistent_test_{}", now),
            email: format!("nonexistent_test_{}@test.com", now),
            password_hash: build_password(),
        };

        let result = user_repo.update_user(user).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_user_repo_delete_nonexistent_user(pool: SqlitePool) {
        let user_repo = UserRepository::new(pool);
        let result = user_repo.delete_user(114514).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_user_repo_create_user_with_empty_fields(pool: SqlitePool) {
        let user_repo = UserRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();

        let user = User {
            id: None,
            username: "".to_string(),
            email: format!("empty_test_{}@test.com", now),
            password_hash: build_password(),
        };

        let result = user_repo.create_user(user).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_user_repo_create_user_with_long_fields(pool: SqlitePool) {
        let user_repo = UserRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();

        let long_string = "a".repeat(256);
        let user = User {
            id: None,
            username: long_string.clone(),
            email: format!("long_test_{}@test.com", now),
            password_hash: build_password(),
        };

        let result = user_repo.create_user(user).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_user_repo_create_user_with_special_chars(pool: SqlitePool) {
        let user_repo = UserRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();

        let user = User {
            id: None,
            username: format!("special_test_!@#$%^&*()_ {}", now),
            email: format!("special_test_{}@test.com", now),
            password_hash: build_password(),
        };

        let result = user_repo.create_user(user).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_user_repo_create_user_with_empty_email(pool: SqlitePool) {
        let user_repo = UserRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();

        let user = User {
            id: None,
            username: format!("empty_email_test_{}", now),
            email: "".to_string(),
            password_hash: build_password(),
        };

        let result = user_repo.create_user(user).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_user_repo_create_user_with_invalid_email(pool: SqlitePool) {
        let user_repo = UserRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();

        let user = User {
            id: None,
            username: format!("invalid_email_test_{}", now),
            email: "invalid_email".to_string(),
            password_hash: build_password(),
        };

        let result = user_repo.create_user(user).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_user_repo_create_user_with_long_email(pool: SqlitePool) {
        let user_repo = UserRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();

        let user = User {
            id: None,
            username: format!("long_email_test_{}", now),
            email: format!("{}@{}", "a".repeat(128), "b".repeat(128)),
            password_hash: build_password(),
        };

        let result = user_repo.create_user(user).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_user_repo_create_user_with_invalid_password(pool: SqlitePool) {
        let user_repo = UserRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();

        let user = User {
            id: None,
            username: format!("invalid_password_test_{}", now),
            email: format!("invalid_password_test_{}@test.com", now),
            password_hash: "invalid_password".to_string(),
        };

        let result = user_repo.create_user(user).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_user_repo_create_user_with_empty_password(pool: SqlitePool) {
        let user_repo = UserRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();

        let user = User {
            id: None,
            username: format!("empty_password_test_{}", now),
            email: format!("empty_password_test_{}@test.com", now),
            password_hash: "".to_string(),
        };

        let result = user_repo.create_user(user).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_user_repo_update_user_with_empty_field(pool: SqlitePool) {
        let user_repo = UserRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();

        let user = User {
            id: None,
            username: format!("empty_field_test_{}", now),
            email: format!("empty_field_test_{}@test.com", now),
            password_hash: build_password(),
        };

        let result = user_repo.create_user(user).await.unwrap();

        let updated_user = User {
            id: result.id,
            username: "".to_string(),
            email: format!("empty_field_test_{}@test.com", now),
            password_hash: build_password(),
        };

        let result = user_repo.update_user(updated_user).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_user_repo_update_user_with_long_username(pool: SqlitePool) {
        let user_repo = UserRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();

        let user = User {
            id: None,
            username: format!("long_username_test_{}", now),
            email: format!("long_username_test_{}@test.com", now),
            password_hash: build_password(),
        };

        let result = user_repo.create_user(user).await.unwrap();

        let updated_user = User {
            id: result.id,
            username: "a".repeat(256),
            email: format!("long_username_test_{}@test.com", now),
            password_hash: build_password(),
        };

        let result = user_repo.update_user(updated_user).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_user_repo_update_user_with_special_chars(pool: SqlitePool) {
        let user_repo = UserRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();

        let user = User {
            id: None,
            username: format!("special_chars_test_{}", now),
            email: format!("special_chars_test_{}@test.com", now),
            password_hash: build_password(),
        };

        let result = user_repo.create_user(user).await.unwrap();

        let updated_user = User {
            id: result.id,
            username: format!("special_test_!@#$%^&*()_ {}", now),
            email: format!("special_chars_test_{}@test.com", now),
            password_hash: build_password(),
        };

        let result = user_repo.update_user(updated_user).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_user_repo_update_user_with_invalid_email(pool: SqlitePool) {
        let user_repo = UserRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();

        let user = User {
            id: None,
            username: format!("invalid_email_test_{}", now),
            email: format!("invalid_email_test_{}@test.com", now),
            password_hash: build_password(),
        };

        let result = user_repo.create_user(user).await.unwrap();

        let updated_user = User {
            id: result.id,
            username: format!("invalid_email_test_{}", now),
            email: "invalid_email".to_string(),
            password_hash: build_password(),
        };

        let result = user_repo.update_user(updated_user).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_user_repo_update_user_with_long_email(pool: SqlitePool) {
        let user_repo = UserRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();

        let user = User {
            id: None,
            username: format!("long_email_test_{}", now),
            email: format!("long_email_test_{}@test.com", now),
            password_hash: build_password(),
        };

        let result = user_repo.create_user(user).await.unwrap();

        let updated_user = User {
            id: result.id,
            username: format!("long_email_test_{}", now),
            email: format!("{}@{}", "a".repeat(128), "b".repeat(128)),
            password_hash: build_password(),
        };

        let result = user_repo.update_user(updated_user).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_user_repo_update_user_with_invalid_password(pool: SqlitePool) {
        let user_repo = UserRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();

        let user = User {
            id: None,
            username: format!("invalid_password_test_{}", now),
            email: format!("invalid_password_test_{}@test.com", now),
            password_hash: build_password(),
        };

        let result = user_repo.create_user(user).await.unwrap();

        let updated_user = User {
            id: result.id,
            username: format!("invalid_password_test_{}", now),
            email: format!("invalid_password_test_{}@test.com", now),
            password_hash: "invalid_password".to_string(),
        };

        let result = user_repo.update_user(updated_user).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_user_repo_update_user_with_empty_email(pool: SqlitePool) {
        let user_repo = UserRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();

        let user = User {
            id: None,
            username: format!("empty_email_test_{}", now),
            email: format!("empty_email_test_{}@test.com", now),
            password_hash: build_password(),
        };

        let result = user_repo.create_user(user).await.unwrap();

        let updated_user = User {
            id: result.id,
            username: format!("empty_email_test_{}", now),
            email: "".to_string(),
            password_hash: build_password(),
        };

        let result = user_repo.update_user(updated_user).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_user_repo_update_user_with_empty_password(pool: SqlitePool) {
        let user_repo = UserRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();

        let user = User {
            id: None,
            username: format!("empty_password_test_{}", now),
            email: format!("empty_password_test_{}@test.com", now),
            password_hash: build_password(),
        };

        let result = user_repo.create_user(user).await.unwrap();

        let updated_user = User {
            id: result.id,
            username: format!("empty_password_test_{}", now),
            email: format!("empty_password_test_{}@test.com", now),
            password_hash: "".to_string(),
        };

        let result = user_repo.update_user(updated_user).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_user_repo_create_user_with_negative_id(pool: SqlitePool) {
        let user_repo = UserRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();

        let user = User {
            id: Some(-1),
            username: format!("negative_id_test_{}", now),
            email: format!("negative_id_test_{}@test.com", now),
            password_hash: build_password(),
        };

        let result = user_repo.create_user(user).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_user_repo_create_user_with_zero_id(pool: SqlitePool) {
        let user_repo = UserRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();

        let user = User {
            id: Some(0),
            username: format!("zero_id_test_{}", now),
            email: format!("zero_id_test_{}@test.com", now),
            password_hash: build_password(),
        };

        let result = user_repo.create_user(user).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_user_repo_create_user_with_dot_in_username(pool: SqlitePool) {
        let user_repo = UserRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();

        let user = User {
            id: None,
            username: format!("dot.test.{}", now),
            email: format!("dot_test_{}@test.com", now),
            password_hash: build_password(),
        };

        let result = user_repo.create_user(user).await;
        assert!(result.is_ok());
    }

    #[sqlx::test]
    async fn test_user_repo_create_user_with_underscore_in_username(pool: SqlitePool) {
        let user_repo = UserRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();

        let user = User {
            id: None,
            username: format!("underscore_test_{}", now),
            email: format!("underscore_test_{}@test.com", now),
            password_hash: build_password(),
        };

        let result = user_repo.create_user(user).await;
        assert!(result.is_ok());
    }

    #[sqlx::test]
    async fn test_user_repo_create_user_with_numeric_username(pool: SqlitePool) {
        let user_repo = UserRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();

        let user = User {
            id: None,
            username: format!("1234567890_{}", now),
            email: format!("numeric_test_{}@test.com", now),
            password_hash: build_password(),
        };

        let result = user_repo.create_user(user).await;
        assert!(result.is_ok());
    }

    #[sqlx::test]
    async fn test_user_repo_create_user_with_long_domain(pool: SqlitePool) {
        let user_repo = UserRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();

        let user = User {
            id: None,
            username: format!("long_domain_test_{}", now),
            email: format!("test@{}", "a".repeat(128)),
            password_hash: build_password(),
        };

        let result = user_repo.create_user(user).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_user_repo_create_user_with_long_local_part(pool: SqlitePool) {
        let user_repo = UserRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();

        let user = User {
            id: None,
            username: format!("long_local_test_{}", now),
            email: format!("{}@test.com", "a".repeat(128)),
            password_hash: build_password(),
        };

        let result = user_repo.create_user(user).await;
        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_user_repo_create_user_with_special_chars_in_email(pool: SqlitePool) {
        let user_repo = UserRepository::new(pool);
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();

        let user = User {
            id: None,
            username: format!("special_email_test_{}", now),
            email: format!("test!@#$%^&*()_{}@test.com", now),
            password_hash: build_password(),
        };

        let result = user_repo.create_user(user).await;
        assert!(result.is_err());
    }

    #[sqlx::test(fixtures("user"))]
    async fn test_user_repo_get_user_by_id_with_transaction(pool: SqlitePool) {
        let mut tx = pool.begin().await.unwrap();

        let user = get_user_by_id_with_transaction(1, &mut tx).await.unwrap().unwrap();
        assert_eq!(user.id, Some(1));
    }

    #[sqlx::test(fixtures("user"))]
    async fn test_user_repo_get_user_by_id_with_transaction_not_found(pool: SqlitePool) {
        let mut tx = pool.begin().await.unwrap();

        let user = get_user_by_id_with_transaction(100, &mut tx).await.unwrap();
        assert!(user.is_none());
    }
}

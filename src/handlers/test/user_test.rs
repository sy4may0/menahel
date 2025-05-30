#[cfg(test)]
mod user_handler_test {
    use sqlx::sqlite::SqlitePool;
    use sqlx::migrate::MigrateDatabase;
    use sqlx::sqlite::Sqlite;
    use std::env;
    use crate::origin_dbpool::set_test_db_pool;
    use crate::handlers::user::get_users;
    use actix_web::{test, App};
    use crate::models::UserResponse;
    use crate::models::ErrorResponse;
    use crate::errors::messages::ErrorKey;

    async fn setup_test_db() {
        // テスト用の一時的なデータベースURLを設定
        let test_db_url = "sqlite::memory:";
        unsafe {
            env::set_var("DATABASE_URL", test_db_url);
        }

        // メモリ内データベースを作成
        Sqlite::create_database(test_db_url).await.unwrap();

        // プールを作成
        let pool = SqlitePool::connect(test_db_url).await.unwrap();

        // マイグレーションを実行
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .unwrap();

        sqlx::query_file!("./fixtures/user_test.sql")
            .execute(&pool)
            .await
            .unwrap();

        set_test_db_pool(pool);
    }

    #[actix_web::test]
    async fn test_get_users() {
        setup_test_db().await;

        let app = test::init_service(
            App::new().service(get_users)
        ).await;

        let req = test::TestRequest::get().uri("/users").to_request();
        let res: UserResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
        assert_eq!(res.results.len(), 10);
        assert_eq!(res.count, 10);
    }

    #[actix_web::test]
    async fn test_get_users_with_pagenation() {
        setup_test_db().await;

        let app = test::init_service(
            App::new().service(get_users)
        ).await;

        let req = test::TestRequest::get().uri("/users?page=1&page_size=4").to_request();
        let res: UserResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
        assert_eq!(res.results.len(), 4);
        assert_eq!(res.count, 4);
        let pagination = res.pagination.unwrap();
        assert_eq!(pagination.current_page, 1);
        assert_eq!(pagination.page_size, 4);

        assert_eq!(res.results[0].id, Some(0));
        assert_eq!(res.results[0].username, "Test User 0");
        assert_eq!(res.results[0].email, "test0@example.com");
        assert_eq!(res.results[0].password_hash, "dummy_hash_0");

        assert_eq!(res.results[1].id, Some(1));
        assert_eq!(res.results[2].id, Some(2));
        assert_eq!(res.results[3].id, Some(3));

        let req = test::TestRequest::get().uri("/users?page=2&page_size=4").to_request();
        let res: UserResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
        assert_eq!(res.results.len(), 4);
        assert_eq!(res.count, 4);
        let pagination = res.pagination.unwrap();
        assert_eq!(pagination.current_page, 2);
        assert_eq!(pagination.page_size, 4);

        assert_eq!(res.results[0].id, Some(4));
        assert_eq!(res.results[1].id, Some(5));
        assert_eq!(res.results[2].id, Some(6));
        assert_eq!(res.results[3].id, Some(7));

        let req = test::TestRequest::get().uri("/users?page=3&page_size=4").to_request();
        let res: UserResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
        assert_eq!(res.results.len(), 2);
        assert_eq!(res.count, 2);
        let pagination = res.pagination.unwrap();
        assert_eq!(pagination.current_page, 3);
        assert_eq!(pagination.page_size, 4);

        assert_eq!(res.results[0].id, Some(8));
        assert_eq!(res.results[1].id, Some(9));
    }

    #[actix_web::test]
    async fn test_get_users_with_pagenation_invalid_page() {
        setup_test_db().await;

        let app = test::init_service(
            App::new().service(get_users)
        ).await;

        let req = test::TestRequest::get().uri("/users?page=0&page_size=4").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains(ErrorKey::UserHandlerGetUsersInvalidPage.to_string().as_str()));
    }

    #[actix_web::test]
    async fn test_get_users_with_pagenation_over_page_size() {
        setup_test_db().await;

        let app = test::init_service(
            App::new().service(get_users)
        ).await;

        let req = test::TestRequest::get().uri("/users?page=1&page_size=1000").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains(ErrorKey::UserHandlerGetUsersInvalidPage.to_string().as_str()));
        
    }

    #[actix_web::test]
    async fn test_get_users_with_pagenation_no_page_size() {
        setup_test_db().await;

        let app = test::init_service(
            App::new().service(get_users)
        ).await;

        let req = test::TestRequest::get().uri("/users?page=1").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains(ErrorKey::UserHandlerGetUsersInvalidPage.to_string().as_str()));
    }

    #[actix_web::test]
    async fn test_get_users_with_pagenation_no_page() {
        setup_test_db().await;

        let app = test::init_service(
            App::new().service(get_users)
        ).await;

        let req = test::TestRequest::get().uri("/users?page_size=10").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains(ErrorKey::UserHandlerGetUsersInvalidPage.to_string().as_str()));
    }
}
#[cfg(test)]
mod user_handler_test {
    use crate::handlers::user::{get_users, create_user, update_user, delete_user};
    use actix_web::{test, App, web};
    use crate::models::{User, UserResponse};
    use crate::models::ErrorResponse;
    use crate::errors::messages::ErrorKey;
    use crate::init_logger;
    use crate::handlers::test::utils::setup_test_db;

    #[ctor::ctor]
    fn init() {
        init_logger();

        // Create test_db directory if it doesn't exist
        if !std::path::Path::new("./test_db/user_handler_test").exists() {
            std::fs::create_dir_all("./test_db/user_handler_test").unwrap();
        }

        // Remove all files in the test_db directory
        let files = std::fs::read_dir("./test_db/user_handler_test").unwrap();
        for file in files {
            let path = file.unwrap().path();
            if path.is_file() {
                std::fs::remove_file(path).unwrap();
            }
        }
    }

    #[actix_web::test]
    async fn test_get_users() {
        let pool = setup_test_db("user_handler_test", "test_get_users").await;

        let app = test::init_service(
            App::new().service(get_users).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/users").to_request();
        
        let res: UserResponse = test::call_and_read_body_json(&app, req).await;
        
        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
        assert_eq!(res.results.len(), 10);
        assert_eq!(res.count, 10);
    }

    #[actix_web::test]
    async fn test_get_users_with_pagination() {
        let pool = setup_test_db("user_handler_test", "test_get_users_with_pagination").await;

        let app = test::init_service(
            App::new().service(get_users).app_data(web::Data::new(pool))
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

        assert_eq!(res.results[0].user_id, Some(0));
        assert_eq!(res.results[0].username, "testuser0");
        assert_eq!(res.results[0].email, "test0@example.com");

        assert_eq!(res.results[1].user_id, Some(1));
        assert_eq!(res.results[2].user_id, Some(2));
        assert_eq!(res.results[3].user_id, Some(3));

        let req = test::TestRequest::get().uri("/users?page=2&page_size=4").to_request();
        let res: UserResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
        assert_eq!(res.results.len(), 4);
        assert_eq!(res.count, 4);
        let pagination = res.pagination.unwrap();
        assert_eq!(pagination.current_page, 2);
        assert_eq!(pagination.page_size, 4);

        assert_eq!(res.results[0].user_id, Some(4));
        assert_eq!(res.results[1].user_id, Some(5));
        assert_eq!(res.results[2].user_id, Some(6));
        assert_eq!(res.results[3].user_id, Some(7));

        let req = test::TestRequest::get().uri("/users?page=3&page_size=4").to_request();
        let res: UserResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
        assert_eq!(res.results.len(), 2);
        assert_eq!(res.count, 2);
        let pagination = res.pagination.unwrap();
        assert_eq!(pagination.current_page, 3);
        assert_eq!(pagination.page_size, 4);

        assert_eq!(res.results[0].user_id, Some(8));
        assert_eq!(res.results[1].user_id, Some(9));
    }

    #[actix_web::test]
    async fn test_get_users_with_pagination_invalid_page() {
        let pool = setup_test_db("user_handler_test", "test_get_users_with_pagination_invalid_page").await;

        let app = test::init_service(
            App::new().service(get_users).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/users?page=0&page_size=4").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains(ErrorKey::UserHandlerGetUsersInvalidPage.to_string().as_str()));
    }

    #[actix_web::test]
    async fn test_get_users_with_pagination_over_page_size() {
        let pool = setup_test_db("user_handler_test", "test_get_users_with_pagination_over_page_size").await;

        let app = test::init_service(
            App::new().service(get_users).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/users?page=1&page_size=1000").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains(ErrorKey::UserHandlerGetUsersInvalidPage.to_string().as_str()));
        
    }

    #[actix_web::test]
    async fn test_get_users_by_name() {
        let pool = setup_test_db("user_handler_test", "test_get_users_by_name").await;

        let app = test::init_service(
            App::new().service(get_users).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/users?target=name&name=testuser0").to_request();
        let res: UserResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
        assert_eq!(res.results.len(), 1);
        assert_eq!(res.results[0].username, "testuser0");
        assert_eq!(res.results[0].email, "test0@example.com");
    }

    #[actix_web::test]
    async fn test_get_users_by_id() {
        let pool = setup_test_db("user_handler_test", "test_get_users_by_id").await;

        let app = test::init_service(
            App::new().service(get_users).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/users?target=id&id=0").to_request();
        let res: UserResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
        assert_eq!(res.results.len(), 1);
        assert_eq!(res.results[0].username, "testuser0");
        assert_eq!(res.results[0].email, "test0@example.com");
    }

    #[actix_web::test]
    async fn test_get_users_by_id_invalid_target() {
        let pool = setup_test_db("user_handler_test", "test_get_users_by_id_invalid_target").await;

        let app = test::init_service(
            App::new().service(get_users).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/users?target=invalid&id=0").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_get_users_by_id_no_id() {
        let pool = setup_test_db("user_handler_test", "test_get_users_by_id_no_id").await;

        let app = test::init_service(
            App::new().service(get_users).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/users?target=id").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_get_users_by_name_no_name() {
        let pool = setup_test_db("user_handler_test", "test_get_users_by_name_no_name").await;

        let app = test::init_service(
            App::new().service(get_users).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/users?target=name").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_get_users_by_id_not_exists() {
        let pool = setup_test_db("user_handler_test", "test_get_users_by_id_not_exists").await;

        let app = test::init_service(
            App::new().service(get_users).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/users?target=id&id=1000").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains("NotFound"));
    }

    #[actix_web::test]
    async fn test_get_users_by_name_not_exists() {
        let pool = setup_test_db("user_handler_test", "test_get_users_by_name_not_exists").await;

        let app = test::init_service(
            App::new().service(get_users).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/users?target=name&name=not_exists").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains("NotFound"));
    }

    #[actix_web::test]
    async fn test_get_users_with_pagination_no_page_size() {
        let pool = setup_test_db("user_handler_test", "test_get_users_with_pagination_no_page_size").await;

        let app = test::init_service(
            App::new().service(get_users).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/users?page=1").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains(ErrorKey::UserHandlerGetUsersInvalidPage.to_string().as_str()));
    }

    #[actix_web::test]
    async fn test_get_users_with_pagination_no_page() {
        let pool = setup_test_db("user_handler_test", "test_get_users_with_pagination_no_page").await;

        let app = test::init_service(
            App::new().service(get_users).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/users?page_size=10").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains(ErrorKey::UserHandlerGetUsersInvalidPage.to_string().as_str()));
    }

    #[actix_web::test]
    async fn test_get_user_by_id_invalid_query() {
        let pool = setup_test_db("user_handler_test", "test_get_user_by_id_invalid_query").await;

        let app = test::init_service(
            App::new().service(get_users).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/users?target=id&id=abc").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_create_user() {
        let pool = setup_test_db("user_handler_test", "test_create_user").await;

        let app = test::init_service(
            App::new().service(create_user).service(get_users).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::post()
            .uri("/users")
            .set_json(User {
                user_id: None,
                username: "testuser10".to_string(),
                email: "test10@example.com".to_string(),
                password_hash: "dummy_hash_10".to_string(),
            })
            .to_request();

        let res: UserResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
        assert_eq!(res.results.len(), 1);
        assert_eq!(res.results[0].username, "testuser10");
        assert_eq!(res.results[0].email, "test10@example.com");

        let req = test::TestRequest::get().uri("/users?target=name&name=testuser10").to_request();
        let res: UserResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
        assert_eq!(res.results.len(), 1);
        assert_eq!(res.results[0].username, "testuser10");
        assert_eq!(res.results[0].email, "test10@example.com");
    }

    #[actix_web::test]
    async fn test_create_user_invalid_email() {
        let pool = setup_test_db("user_handler_test", "test_create_user_invalid_email").await;

        let app = test::init_service(
            App::new().service(create_user).service(get_users).app_data(web::Data::new(pool))
        ).await;
        
        let req = test::TestRequest::post()
            .uri("/users")
            .set_json(User {
                user_id: None,
                username: "testuser11".to_string(),
                email: "test11example.com".to_string(),
                password_hash: "dummy_hash_11".to_string(),
            })
            .to_request();

        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_create_exist_username() {
        let pool = setup_test_db("user_handler_test", "test_create_exist_username").await;

        let app = test::init_service(
            App::new().service(create_user).service(get_users).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::post()
            .uri("/users")
            .set_json(User {
                user_id: None,
                username: "testuser9".to_string(),
                email: "test9@example.com".to_string(),
                password_hash: "dummy_hash_9".to_string(),
            })
            .to_request();

        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains("InternalServerError"));
    }

    #[actix_web::test]
    async fn test_create_user_no_param() {
        let pool = setup_test_db("user_handler_test", "test_create_user_no_param").await;

        let app = test::init_service(
            App::new().service(create_user).service(get_users).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::post()
            .uri("/users")
            .to_request();

        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_create_user_empty_param() {
        let pool = setup_test_db("user_handler_test", "test_create_user_empty_param").await;

        let app = test::init_service(
            App::new().service(create_user).service(get_users).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::post()
            .uri("/users")
            .set_json(User {
                user_id: None,
                username: "".to_string(),
                email: "".to_string(),
                password_hash: "".to_string(),
            })
            .to_request();

        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_create_user_duplicate_email() {
        let pool = setup_test_db("user_handler_test", "test_create_user_duplicate_email").await;

        let app = test::init_service(
            App::new().service(create_user).service(get_users).app_data(web::Data::new(pool))
        ).await;
        
        let req = test::TestRequest::post()
            .uri("/users")
            .set_json(User {
                user_id: None,
                username: "testuser12".to_string(),
                email: "test0@example.com".to_string(),
                password_hash: "dummy_hash_12".to_string(),
            })
            .to_request();

        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains("InternalServerError"));
    }

    #[actix_web::test]
    async fn test_update_user() {
        let pool = setup_test_db("user_handler_test", "test_update_user").await;

        let app = test::init_service(
            App::new().service(update_user).service(get_users).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::post()
            .uri("/users/1")
            .set_json(User {
                user_id: Some(1),
                username: "testuser1_x".to_string(),
                email: "test1_x@example.com".to_string(),
                password_hash: "dummy_hash_1_x".to_string(),
            })
            .to_request();

        let res: UserResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
        assert_eq!(res.results.len(), 1);
        assert_eq!(res.results[0].username, "testuser1_x");
        assert_eq!(res.results[0].email, "test1_x@example.com");

        let req = test::TestRequest::get().uri("/users?target=name&name=testuser1_x").to_request();
        let res: UserResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
        assert_eq!(res.results.len(), 1);
        assert_eq!(res.results[0].username, "testuser1_x");
        assert_eq!(res.results[0].email, "test1_x@example.com");
    }

    #[actix_web::test]
    async fn test_update_user_invalid_id() {
        let pool = setup_test_db("user_handler_test", "test_update_user_invalid_id").await;

        let app = test::init_service(
            App::new().service(update_user).service(get_users).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::post()
            .uri("/users/100")
            .set_json(User {
                user_id: Some(100),
                username: "testuser100".to_string(),
                email: "test100@example.com".to_string(),
                password_hash: "dummy_hash_100".to_string(),
            })
            .to_request();

        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains("NotFound"));
    }

    #[actix_web::test]
    async fn test_update_user_invalid_email() {
        let pool = setup_test_db("user_handler_test", "test_update_user_invalid_email").await;

        let app = test::init_service(
            App::new().service(update_user).service(get_users).app_data(web::Data::new(pool))
        ).await;
        
        let req = test::TestRequest::post()
            .uri("/users/1")
            .set_json(User {
                user_id: Some(1),
                username: "testuser1_x".to_string(),
                email: "test1_xexample.com".to_string(),
                password_hash: "dummy_hash_1_x".to_string(),
            })
            .to_request();

        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_update_id_missmatch() {
        let pool = setup_test_db("user_handler_test", "test_update_id_missmatch").await;

        let app = test::init_service(
            App::new().service(update_user).service(get_users).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::post()
            .uri("/users/1")
            .set_json(User {
                user_id: Some(2),
                username: "testuser1_x".to_string(),
                email: "test1_x@example.com".to_string(),
                password_hash: "dummy_hash_1_x".to_string(),
            })
            .to_request();

        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        println!("res: {:?}", res.message);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_update_user_no_param() {
        let pool = setup_test_db("user_handler_test", "test_update_user_no_param").await;

        let app = test::init_service(
            App::new().service(update_user).service(get_users).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::post()
            .uri("/users/1")
            .to_request();

        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_update_user_empty_param() {
        let pool = setup_test_db("user_handler_test", "test_update_user_empty_param").await;

        let app = test::init_service(
            App::new().service(update_user).service(get_users).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::post()
            .uri("/users/1")
            .set_json(User {
                user_id: Some(1),
                username: "".to_string(),
                email: "".to_string(),
                password_hash: "".to_string(),
            })
            .to_request();

        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_update_user_duplicate_username() {
        let pool = setup_test_db("user_handler_test", "test_update_user_duplicate_username").await;

        let app = test::init_service(
            App::new().service(update_user).service(get_users).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::post()
            .uri("/users/2")
            .set_json(User {
                user_id: Some(2),
                username: "testuser0".to_string(),
                email: "test_xxx@example.com".to_string(),
                password_hash: "dummy_hash_1_x".to_string(),
            })
            .to_request();

        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains("InternalServerError"));
    }

    #[actix_web::test]
    async fn test_update_user_duplicate_email() {
        let pool = setup_test_db("user_handler_test", "test_update_user_duplicate_email").await;

        let app = test::init_service(
            App::new().service(update_user).service(get_users).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::post()
            .uri("/users/2")
            .set_json(User {
                user_id: Some(2),
                username: "testuser1_x".to_string(),
                email: "test0@example.com".to_string(),
                password_hash: "dummy_hash_1_x".to_string(),
            })
            .to_request();

        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains("InternalServerError"));
    }

    #[actix_web::test]
    async fn test_update_user_invalid_path() {
        let pool = setup_test_db("user_handler_test", "test_update_user_invalid_path").await;

        let app = test::init_service(
            App::new().service(update_user).service(get_users).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::post().set_json(User {
            user_id: Some(1),
            username: "testuser1_x".to_string(),
            email: "test1_x@example.com".to_string(),
            password_hash: "dummy_hash_1_x".to_string(),
        }).uri("/users/abc").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_delete_user() {
        let pool = setup_test_db("user_handler_test", "test_delete_user").await;

        let app = test::init_service(
            App::new().service(delete_user).service(get_users).app_data(web::Data::new(pool))
        ).await;
        
        let req = test::TestRequest::delete().uri("/users/9").to_request();
        let res: UserResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
        assert_eq!(res.results.len(), 0);
    }

    #[actix_web::test]
    async fn test_delete_user_invalid_path() {
        let pool = setup_test_db("user_handler_test", "test_delete_user_invalid_path").await;

        let app = test::init_service(
            App::new().service(delete_user).service(get_users).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::delete().uri("/users/abc").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_delete_user_not_exists() {
        let pool = setup_test_db("user_handler_test", "test_delete_user_not_exists").await;

        let app = test::init_service(
            App::new().service(delete_user).service(get_users).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::delete().uri("/users/100").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains("NotFound"));
    }
}
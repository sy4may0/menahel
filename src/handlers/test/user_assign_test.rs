#[cfg(test)]
mod user_assign_handler_test {
    use crate::handlers::user_assign::{get_user_assigns, create_user_assign, update_user_assign, delete_user_assign};
    use actix_web::{test, App, web};
    use crate::models::{UserAssign, UserAssignResponse};
    use crate::models::ErrorResponse;
    use crate::errors::messages::ErrorKey;
    use crate::handlers::test::utils::setup_test_db;

    #[ctor::ctor]
    fn init() {
        // Create test_db directory if it doesn't exist
        if !std::path::Path::new("./test_db/user_assign_handler_test").exists() {
            std::fs::create_dir_all("./test_db/user_assign_handler_test").unwrap();
        }

        // Remove all files in the test_db directory
        let files = std::fs::read_dir("./test_db/user_assign_handler_test").unwrap();
        for file in files {
            let path = file.unwrap().path();
            if path.is_file() {
                std::fs::remove_file(path).unwrap();
            }
        }
    }

    #[actix_web::test]
    async fn test_get_user_assigns() {
        let pool = setup_test_db("user_assign_handler_test", "test_get_user_assigns").await;

        let app = test::init_service(
            App::new().service(get_user_assigns).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/userassigns").to_request();

        let res: UserAssignResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
        assert_eq!(res.results.len(), 10);
        assert_eq!(res.count, 10);
    }

    #[actix_web::test]
    async fn test_get_user_assigns_with_pagination() {
        let pool = setup_test_db("user_assign_handler_test", "test_get_user_assigns_with_pagination").await;

        let app = test::init_service(
            App::new().service(get_user_assigns).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/userassigns?page=1&page_size=4").to_request();

        let res: UserAssignResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
        assert_eq!(res.results.len(), 4);
        assert_eq!(res.count, 4);
        let pagination = res.pagination.unwrap();
        assert_eq!(pagination.current_page, 1);
        assert_eq!(pagination.page_size, 4);

        let req = test::TestRequest::get().uri("/userassigns?page=2&page_size=4").to_request();
        let res: UserAssignResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
        assert_eq!(res.results.len(), 4);
        assert_eq!(res.count, 4);
        let pagination = res.pagination.unwrap();
        assert_eq!(pagination.current_page, 2);
        assert_eq!(pagination.page_size, 4);

        let req = test::TestRequest::get().uri("/userassigns?page=3&page_size=4").to_request();
        let res: UserAssignResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
        assert_eq!(res.results.len(), 2);
        assert_eq!(res.count, 2);
        let pagination = res.pagination.unwrap();
        assert_eq!(pagination.current_page, 3);
        assert_eq!(pagination.page_size, 4);

    }

    #[actix_web::test]
    async fn test_get_user_assigns_with_pagination_invalid_page() {
        let pool = setup_test_db("user_assign_handler_test", "test_get_user_assigns_with_pagination_invalid_page").await;

        let app = test::init_service(
            App::new().service(get_user_assigns).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/userassigns?page=0&page_size=4").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains(ErrorKey::UserAssignHandlerGetUserAssignsInvalidPage.to_string().as_str()));
    }

    #[actix_web::test]
    async fn test_get_user_assigns_with_pagination_over_page_size() {
        let pool = setup_test_db("user_assign_handler_test", "test_get_user_assigns_with_pagination_over_page_size").await;

        let app = test::init_service(
            App::new().service(get_user_assigns).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/userassigns?page=1&page_size=101").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_get_user_assigns_by_id() {
        let pool = setup_test_db("user_assign_handler_test", "test_get_user_assigns_by_id").await;

        let app = test::init_service(
            App::new().service(get_user_assigns).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/userassigns?target=id&id=0").to_request();
        let res: UserAssignResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
        assert_eq!(res.results.len(), 1);
        assert_eq!(res.count, 1);
        let user_assign = res.results.first().unwrap();
        assert_eq!(user_assign.id, Some(0));
    }

    #[actix_web::test]
    async fn test_get_user_assigns_by_userid() {
        let pool = setup_test_db("user_assign_handler_test", "test_get_user_assigns_by_userid").await;

        let app = test::init_service(
            App::new().service(get_user_assigns).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/userassigns?target=filter&userid=0").to_request();
        let res: UserAssignResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
        assert_eq!(res.results.len(), 5);
        assert_eq!(res.count, 5);
        for user_assign in res.results {
            assert_eq!(user_assign.user_id, 0);
        }
    }

    #[actix_web::test]
    async fn test_get_user_assigns_by_taskid() {
        let pool = setup_test_db("user_assign_handler_test", "test_get_user_assigns_by_taskid").await;

        let app = test::init_service(
            App::new().service(get_user_assigns).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/userassigns?target=filter&taskid=2").to_request();
        let res: UserAssignResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
        assert_eq!(res.results.len(), 3);
        assert_eq!(res.count, 3);
        for user_assign in res.results {
            assert_eq!(user_assign.task_id, 2);
        }
    }

    #[actix_web::test]
    async fn test_get_user_assigns_by_id_invalid_target() {
        let pool = setup_test_db("user_assign_handler_test", "test_get_user_assigns_by_id_invalid_target").await;

        let app = test::init_service(
            App::new().service(get_user_assigns).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/userassigns?target=invalid&id=0").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_get_user_assigns_by_id_no_id() {
        let pool = setup_test_db("user_assign_handler_test", "test_get_user_assigns_by_id_no_id").await;

        let app = test::init_service(
            App::new().service(get_user_assigns).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/userassigns?target=id").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_get_user_assigns_by_userid_no_filter() {
        let pool = setup_test_db("user_assign_handler_test", "test_get_user_assigns_by_userid_no_userid").await;

        let app = test::init_service(
            App::new().service(get_user_assigns).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/userassigns?target=filter").to_request();
        let res: UserAssignResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
        assert_eq!(res.results.len(), 10);
        assert_eq!(res.count, 10);
    }

    #[actix_web::test]
    async fn test_get_user_assigns_by_filter_and_pagination() {
        let pool = setup_test_db("user_assign_handler_test", "test_get_user_assigns_by_taskid_no_taskid").await;

        let app = test::init_service(
            App::new().service(get_user_assigns).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/userassigns?target=filter&userid=0&page=1&page_size=4").to_request();
        let res: UserAssignResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
        assert_eq!(res.results.len(), 4);
        assert_eq!(res.count, 4);
        let pagination = res.pagination.unwrap();
        assert_eq!(pagination.current_page, 1);
        assert_eq!(pagination.page_size, 4);

        let req = test::TestRequest::get().uri("/userassigns?target=filter&userid=0&page=2&page_size=4").to_request();
        let res: UserAssignResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
        assert_eq!(res.results.len(), 1);
        assert_eq!(res.count, 1);
        let pagination = res.pagination.unwrap();
        assert_eq!(pagination.current_page, 2);
        assert_eq!(pagination.page_size, 4);
    }

    #[actix_web::test]
    async fn test_get_user_assigns_by_id_not_exists() {
        let pool = setup_test_db("user_assign_handler_test", "test_get_user_assigns_by_id_not_exists").await;

        let app = test::init_service(
            App::new().service(get_user_assigns).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/userassigns?target=id&id=100").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("NotFound"));
    }

    #[actix_web::test]
    async fn test_get_user_assigns_by_userid_not_exists() {
        let pool = setup_test_db("user_assign_handler_test", "test_get_user_assigns_by_userid_not_exists").await;

        let app = test::init_service(
            App::new().service(get_user_assigns).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/userassigns?target=filter&userid=100").to_request();
        let res: UserAssignResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
        assert_eq!(res.results.len(), 0);
        assert_eq!(res.count, 0);
    }

    #[actix_web::test]
    async fn test_get_user_assigns_by_taskid_not_exists() {
        let pool = setup_test_db("user_assign_handler_test", "test_get_user_assigns_by_taskid_not_exists").await;

        let app = test::init_service(
            App::new().service(get_user_assigns).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/userassigns?target=filter&taskid=100").to_request();
        let res: UserAssignResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
        assert_eq!(res.results.len(), 0);
        assert_eq!(res.count, 0);
    }

    #[actix_web::test]
    async fn test_get_user_assigns_with_invalid_query() {
        let pool = setup_test_db("user_assign_handler_test", "test_get_user_assigns_with_invalid_query").await;

        let app = test::init_service(
            App::new().service(get_user_assigns).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/userassigns?target=id&id=abc").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }   

    #[actix_web::test]
    async fn test_create_user_assign() {
        let pool = setup_test_db("user_assign_handler_test", "test_create_user_assign").await;

        let app = test::init_service(
            App::new().service(create_user_assign).service(get_user_assigns).app_data(web::Data::new(pool))
        ).await;

        let user_assign = UserAssign {
            id: None,
            user_id: 5,
            task_id: 2,
        };

        let req = test::TestRequest::post().uri("/userassigns").set_json(user_assign).to_request();
        let res: UserAssignResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
        assert_eq!(res.results.len(), 1);
        assert_eq!(res.count, 1);
        let user_assign = res.results.first().unwrap();
        assert_eq!(user_assign.user_id, 5);
        assert_eq!(user_assign.task_id, 2);

        let req = test::TestRequest::get().uri("/userassigns?target=filter&userid=5&taskid=2").to_request();
        let res: UserAssignResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
        assert_eq!(res.results.len(), 1);
        assert_eq!(res.count, 1);
        let user_assign = res.results.first().unwrap();
        assert_eq!(user_assign.user_id, 5);
        assert_eq!(user_assign.task_id, 2);
    }

    #[actix_web::test]
    async fn test_create_user_assign_invalid_userid() {
        let pool = setup_test_db("user_assign_handler_test", "test_create_user_assign_invalid_userid").await;

        let app = test::init_service(
            App::new().service(create_user_assign).service(get_user_assigns).app_data(web::Data::new(pool))
        ).await;

        let user_assign = UserAssign {
            id: None,
            user_id: 100,
            task_id: 2,
        };

        let req = test::TestRequest::post().uri("/userassigns").set_json(user_assign).to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains("NotFound"));
    }

    #[actix_web::test]
    async fn test_create_user_assign_invalid_taskid() {
        let pool = setup_test_db("user_assign_handler_test", "test_create_user_assign_invalid_taskid").await;

        let app = test::init_service(
            App::new().service(create_user_assign).service(get_user_assigns).app_data(web::Data::new(pool))
        ).await;

        let user_assign = UserAssign {
            id: None,
            user_id: 5,
            task_id: 100,
        };

        let req = test::TestRequest::post().uri("/userassigns").set_json(user_assign).to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains("NotFound"));
    }

    #[actix_web::test]
    async fn test_create_user_assign_no_param() {
        let pool = setup_test_db("user_assign_handler_test", "test_create_user_assign_no_param").await;

        let app = test::init_service(
            App::new().service(create_user_assign).service(get_user_assigns).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::post().uri("/userassigns").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_create_user_assign_duplicate_user_task() {
        let pool = setup_test_db("user_assign_handler_test", "test_create_user_assign_duplicate_user_task").await;

        let app = test::init_service(
            App::new().service(create_user_assign).service(get_user_assigns).app_data(web::Data::new(pool))
        ).await;

        let user_assign = UserAssign {
            id: None,
            user_id: 0,
            task_id: 2,
        };

        let req = test::TestRequest::post().uri("/userassigns").set_json(user_assign).to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_create_user_assign_to_major_task() {
        let pool = setup_test_db("user_assign_handler_test", "test_create_user_assign_to_major_task").await;

        let app = test::init_service(
            App::new().service(create_user_assign).service(get_user_assigns).app_data(web::Data::new(pool))
        ).await;

        let user_assign = UserAssign {
            id: None,
            user_id: 5,
            task_id: 0,
        };

        let req = test::TestRequest::post().uri("/userassigns").set_json(user_assign).to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_create_user_assign_to_minor_task() {
        let pool = setup_test_db("user_assign_handler_test", "test_create_user_assign_to_minor_task").await;

        let app = test::init_service(
            App::new().service(create_user_assign).service(get_user_assigns).app_data(web::Data::new(pool))
        ).await;

        let user_assign = UserAssign {
            id: None,
            user_id: 5,
            task_id: 1,
        };

        let req = test::TestRequest::post().uri("/userassigns").set_json(user_assign).to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_update_user_assign() {
        let pool = setup_test_db("user_assign_handler_test", "test_update_user_assign").await;

        let app = test::init_service(
            App::new().service(update_user_assign).service(get_user_assigns).app_data(web::Data::new(pool))
        ).await;

        let user_assign = UserAssign {
            id: Some(0),
            user_id: 5,
            task_id: 2,
        };

        let req = test::TestRequest::post().uri("/userassigns/0").set_json(user_assign).to_request();
        let res: UserAssignResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
        assert_eq!(res.results.len(), 1);
        assert_eq!(res.count, 1);
    }

    #[actix_web::test]
    async fn test_update_user_assign_invalid_json() {
        let pool = setup_test_db("user_assign_handler_test", "test_update_user_assign_invalid_json").await;

        let app = test::init_service(
            App::new().service(update_user_assign).service(get_user_assigns).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::post().uri("/userassigns/0").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_update_user_assign_id_not_exists() {
        let pool = setup_test_db("user_assign_handler_test", "test_update_user_assign_id_not_exists").await;

        let app = test::init_service(
            App::new().service(update_user_assign).service(get_user_assigns).app_data(web::Data::new(pool))
        ).await;

        let user_assign = UserAssign {
            id: Some(100),
            user_id: 5,
            task_id: 2,
        };

        let req = test::TestRequest::post().uri("/userassigns/100").set_json(user_assign).to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains("NotFound"));
    }

    #[actix_web::test]
    async fn test_update_user_assign_id_mismatch() {
        let pool = setup_test_db("user_assign_handler_test", "test_update_user_assign_id_mismatch").await;

        let app = test::init_service(
            App::new().service(update_user_assign).service(get_user_assigns).app_data(web::Data::new(pool))
        ).await;

        let user_assign = UserAssign {
            id: Some(0),
            user_id: 5,
            task_id: 2,
        };

        let req = test::TestRequest::post().uri("/userassigns/1").set_json(user_assign).to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_update_user_assign_invalid_path() {
        let pool = setup_test_db("user_assign_handler_test", "test_update_user_assign_invalid_path").await;

        let app = test::init_service(
            App::new().service(update_user_assign).service(get_user_assigns).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::post().uri("/userassigns/abc").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_delete_user_assign() {
        let pool = setup_test_db("user_assign_handler_test", "test_delete_user_assign").await;

        let app = test::init_service(
            App::new().service(delete_user_assign).service(get_user_assigns).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::delete().uri("/userassigns/0").to_request();
        let res: UserAssignResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
        assert_eq!(res.results.len(), 0);
        assert_eq!(res.count, 0);
    }

    #[actix_web::test]
    async fn test_delete_user_assign_invalid_path() {
        let pool = setup_test_db("user_assign_handler_test", "test_delete_user_assign_invalid_path").await;

        let app = test::init_service(
            App::new().service(delete_user_assign).service(get_user_assigns).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::delete().uri("/userassigns/abc").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }
}

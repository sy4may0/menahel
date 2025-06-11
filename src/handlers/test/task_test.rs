#[cfg(test)]

mod task_handler_test {
    use crate::handlers::task::get_tasks;
    use crate::handlers::task::create_task;
    use crate::handlers::task::update_task;
    use crate::handlers::task::delete_task;
    use actix_web::{test, App, web};
    use crate::models::{Task, TaskResponse};
    use crate::models::ErrorResponse;
    use crate::handlers::test::utils::setup_test_db;

    #[ctor::ctor]
    fn init() {
        if !std::path::Path::new("./test_db/task_handler_test").exists() {
            std::fs::create_dir_all("./test_db/task_handler_test").unwrap();
        }

        let files = std::fs::read_dir("./test_db/task_handler_test").unwrap();
        for file in files {
            let path = file.unwrap().path();
            if path.is_file() {
                std::fs::remove_file(path).unwrap();
            }
        }
    }

    #[actix_web::test]
    async fn test_get_tasks() {
        let pool = setup_test_db("task_handler_test", "test_get_tasks").await;

        let app = test::init_service(
            App::new().service(get_tasks).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/tasks").to_request();

        let res: TaskResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 0);
        assert_eq!(res.results.len(), 11);
        assert_eq!(res.message, "OK");
        assert_eq!(res.count, 11);
        
    }

    #[actix_web::test]
    async fn test_get_tasks_with_pagination() {
        let pool = setup_test_db("task_handler_test", "test_get_tasks_with_pagination").await;

        let app = test::init_service(
            App::new().service(get_tasks).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/tasks?page=1&page_size=4").to_request();
        let res: TaskResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 0);
        assert_eq!(res.results.len(), 4);
        assert_eq!(res.message, "OK");
        assert_eq!(res.count, 4);

        let req = test::TestRequest::get().uri("/tasks?page=3&page_size=4").to_request();
        let res: TaskResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 0);
        assert_eq!(res.results.len(), 3);
        assert_eq!(res.message, "OK");
        assert_eq!(res.count, 3);
        
    }

    #[actix_web::test]
    async fn test_get_tasks_with_pagination_over_page_size() {
        let pool = setup_test_db("task_handler_test", "test_get_tasks_with_pagination_over_page_size").await;

        let app = test::init_service(
            App::new().service(get_tasks).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/tasks?page=1&page_size=1000").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_get_tasks_with_invalid_no_page_or_page_size() {
        let pool = setup_test_db("task_handler_test", "test_get_tasks_with_invalid_pagenation").await;

        let app = test::init_service(
            App::new().service(get_tasks).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/tasks?page=1").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));

        let req = test::TestRequest::get().uri("/tasks?page_size=10").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_get_tasks_with_invalid_pagination_params() {
        let pool = setup_test_db("task_handler_test", "test_get_tasks_with_invalid_pagination_params").await;

        let app = test::init_service(
            App::new().service(get_tasks).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/tasks?page=0&page_size=10").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));

        let req = test::TestRequest::get().uri("/tasks?page=1&page_size=0").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_get_all_tasks_by_target() {
        let pool = setup_test_db("task_handler_test", "test_get_all_tasks_by_target").await;

        let app = test::init_service(
            App::new().service(get_tasks).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/tasks?target=all").to_request();
        let res: TaskResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 0);
        assert_eq!(res.results.len(), 11);
        assert_eq!(res.message, "OK");
        assert_eq!(res.count, 11);

    }

    #[actix_web::test]
    async fn test_get_tasks_by_invalid_target() {
        let pool = setup_test_db("task_handler_test", "test_get_tasks_by_invalid_target").await;

        let app = test::init_service(
            App::new().service(get_tasks).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/tasks?target=invalid").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_get_tasks_by_id() {
        let pool = setup_test_db("task_handler_test", "test_get_tasks_by_id").await;

        let app = test::init_service(
            App::new().service(get_tasks).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/tasks?target=id&id=1").to_request();
        let res: TaskResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 0);
        assert_eq!(res.results.len(), 1);
        assert_eq!(res.message, "OK");
        assert_eq!(res.count, 1);
    }   

    #[actix_web::test]
    async fn test_get_tasks_by_invalid_query() {
        let pool = setup_test_db("task_handler_test", "test_get_tasks_by_invalid_query").await;

        let app = test::init_service(
            App::new().service(get_tasks).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/tasks?target=id&id=abc").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_get_tasks_by_invalid_id() {
        let pool = setup_test_db("task_handler_test", "test_get_tasks_by_invalid_id").await;

        let app = test::init_service(
            App::new().service(get_tasks).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/tasks?target=id&id=-1").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_get_task_by_filter() {
        let pool = setup_test_db("task_handler_test", "test_get_task_by_filter").await;

        let app = test::init_service(
            App::new().service(get_tasks).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/tasks?target=filter&project_id=0&level=2").to_request();
        let res: TaskResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 0);
        assert_eq!(res.results.len(), 6);
        assert_eq!(res.message, "OK");
        assert_eq!(res.count, 6);
        for task in res.results {
            assert_eq!(task.level, 2);
            assert_eq!(task.project_id, 0);
        }
    }

    #[actix_web::test]
    async fn test_get_task_by_filter_with_pagination() {
        let pool = setup_test_db("task_handler_test", "test_get_task_by_filter_with_pagination").await;

        let app = test::init_service(
            App::new().service(get_tasks).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/tasks?target=filter&project_id=0&level=2&page=1&page_size=4").to_request();
        let res: TaskResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 0);
        assert_eq!(res.results.len(), 4);
        assert_eq!(res.message, "OK");
        assert_eq!(res.count, 4);
        for task in res.results {
            assert_eq!(task.level, 2);
            assert_eq!(task.project_id, 0);
        }
    }

    #[actix_web::test]
    async fn test_get_tasks_by_full_filter() {
        let pool = setup_test_db("task_handler_test", "test_get_tasks_by_full_filter").await;

        let app = test::init_service(
            App::new().service(get_tasks).app_data(web::Data::new(pool))
        ).await;
        // (3, 'TestNotStartedTask3', 'TestTask3Description', 2, 1, 0, 2, 1500, 3000, 3000);
        let mut filter_value = Vec::new();
        filter_value.push("project_id=0");
        filter_value.push("parent_id=2");
        filter_value.push("level=2");
        filter_value.push("name=TestNotStartedTask3");
        filter_value.push("description=TestTask3Description");
        filter_value.push("status=1");
        filter_value.push("deadline_from=1500");
        filter_value.push("deadline_to=1500");
        filter_value.push("created_at_from=3000");
        filter_value.push("created_at_to=3000");
        filter_value.push("updated_at_from=3000");
        filter_value.push("updated_at_to=3000");
        let req = test::TestRequest::get().uri(format!("/tasks?target=filter&{}", filter_value.join("&")).as_str()).to_request();
        let res: TaskResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 0);
        assert_eq!(res.results.len(), 1);
        assert_eq!(res.message, "OK");
        assert_eq!(res.count, 1);
        assert_eq!(res.results[0].name, "TestNotStartedTask3");
        assert_eq!(res.results[0].description, Some("TestTask3Description".to_string()));
        assert_eq!(res.results[0].level, 2);
        assert_eq!(res.results[0].status, 1);
        assert_eq!(res.results[0].project_id, 0);
        assert_eq!(res.results[0].parent_id, Some(2));
        assert_eq!(res.results[0].deadline, Some(1500));
        assert_eq!(res.results[0].created_at, 3000);
        assert_eq!(res.results[0].updated_at, Some(3000));
    }

    #[actix_web::test]
    async fn test_get_tasks_by_invalid_filter() {
        let pool = setup_test_db("task_handler_test", "test_get_tasks_by_invalid_filter").await;

        let app = test::init_service(
            App::new().service(get_tasks).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/tasks?target=filter&project_id=-1&level=-1&name=TestNotStartedTask3").to_request();
        let res: TaskResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 0);
        assert_eq!(res.results.len(), 0);
        assert_eq!(res.message, "OK");
        assert_eq!(res.count, 0);
    }

    #[actix_web::test]
    async fn test_create_task() {
        let pool = setup_test_db("task_handler_test", "test_create_task").await;

        let app = test::init_service(
            App::new().service(create_task).service(get_tasks).app_data(web::Data::new(pool))
        ).await;

        let task = Task {
            id: None,
            project_id: 0,
            parent_id: None,
            level: 0,
            name: "CREATE_Major_task".to_string(),
            description: Some("CREATE_Major_task_description".to_string()),
            status: 0,
            deadline: None,
            created_at: 0,
            updated_at: None,
        };

        let req = test::TestRequest::post().uri("/tasks").set_json(task).to_request();
        let res: TaskResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 0);
        assert_eq!(res.results.len(), 1);
        assert_eq!(res.message, "OK");
        assert_eq!(res.count, 1);

        let req = test::TestRequest::get().uri("/tasks?target=id&id=11").to_request();
        let res: TaskResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.results[0].name, "CREATE_Major_task");
        assert_eq!(res.results[0].description, Some("CREATE_Major_task_description".to_string()));
        assert_eq!(res.results[0].level, 0);
        assert_eq!(res.results[0].status, 0);
        assert_eq!(res.results[0].project_id, 0);
        assert_eq!(res.results[0].parent_id, None);
        assert_eq!(res.results[0].deadline, None);
        assert!(res.results[0].created_at >= 0);
    }

    #[actix_web::test]
    async fn test_create_trivial_level_task() {
        let pool = setup_test_db("task_handler_test", "test_create_trivial_level_task").await;

        let app = test::init_service(
            App::new().service(create_task).service(get_tasks).app_data(web::Data::new(pool))
        ).await;

        let task = Task {
            id: None,
            project_id: 0,
            parent_id: Some(1),
            level: 2,
            name: "CREATE_trivial_level_task_1".to_string(),
            description: Some("CREATE_trivial_level_task_1_description".to_string()),
            status: 0,
            deadline: None,
            created_at: 0,
            updated_at: None,
        };

        let req = test::TestRequest::post().uri("/tasks").set_json(task).to_request();
        let res: TaskResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 0);
        assert_eq!(res.results.len(), 1);
        assert_eq!(res.message, "OK");
        assert_eq!(res.count, 1);
        
    }


    #[actix_web::test]
    async fn test_create_task_with_invalid_json() {
        let pool = setup_test_db("task_handler_test", "test_create_task_with_invalid_json").await;

        let app = test::init_service(
            App::new().service(create_task).service(get_tasks).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::post().uri("/tasks").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_create_task_with_invalid_project_id() {
        let pool = setup_test_db("task_handler_test", "test_create_task_with_invalid_project_id").await;

        let app = test::init_service(
            App::new().service(create_task).service(get_tasks).app_data(web::Data::new(pool))
        ).await;

        let task = Task {
            id: None,
            project_id: -1,
            parent_id: None,
            level: 0,
            name: "CREATE_task_with_invalid_project_id".to_string(),
            description: Some("CREATE_task_with_invalid_project_id_description".to_string()),
            status: 0,
            deadline: None,
            created_at: 0,
            updated_at: None,
        };

        let req = test::TestRequest::post().uri("/tasks").set_json(task).to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_create_task_with_project_id_not_exists() {
        let pool = setup_test_db("task_handler_test", "test_create_task_with_project_id_not_exists").await;

        let app = test::init_service(
            App::new().service(create_task).service(get_tasks).app_data(web::Data::new(pool))
        ).await;

        let task = Task {
            id: None,
            project_id: 100,
            parent_id: None,
            level: 0,
            name: "CREATE_task_with_project_id_not_exists".to_string(),
            description: Some("CREATE_task_with_project_id_not_exists_description".to_string()),
            status: 0,
            deadline: None,
            created_at: 0,
            updated_at: None,
        };

        let req = test::TestRequest::post().uri("/tasks").set_json(task).to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_create_task_with_invalid_parent_level() {
        let pool = setup_test_db("task_handler_test", "test_create_task_with_invalid_parent_level").await;

        let app = test::init_service(
            App::new().service(create_task).service(get_tasks).app_data(web::Data::new(pool))
        ).await;

        let task = Task {
            id: None,
            project_id: 0,
            parent_id: Some(2),
            level: 1,
            name: "CREATE_task_with_invalid_parent_level".to_string(),
            description: Some("CREATE_task_with_invalid_parent_level_description".to_string()),
            status: 0,
            deadline: None,
            created_at: 0,
            updated_at: None,
        };

        let req = test::TestRequest::post().uri("/tasks").set_json(task).to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_create_task_with_parent_id_not_exists() {
        let pool = setup_test_db("task_handler_test", "test_create_task_with_parent_id_not_exists").await;

        let app = test::init_service(
            App::new().service(create_task).service(get_tasks).app_data(web::Data::new(pool))
        ).await;

        let task = Task {
            id: None,
            project_id: 0,
            parent_id: Some(100),
            level: 1,
            name: "CREATE_task_with_parent_id_not_exists".to_string(),
            description: Some("CREATE_task_with_parent_id_not_exists_description".to_string()),
            status: 0,
            deadline: None,
            created_at: 0,
            updated_at: None,
        };

        let req = test::TestRequest::post().uri("/tasks").set_json(task).to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("NotFound"));
    }

    #[actix_web::test]
    async fn test_update_task() {
        let pool = setup_test_db("task_handler_test", "test_update_task").await;

        let app = test::init_service(
            App::new().service(update_task).service(get_tasks).app_data(web::Data::new(pool))
        ).await;

        let task = Task {
            id: Some(0),
            project_id: 0,
            parent_id: None,
            level: 0,
            name: "UPDATE_task".to_string(),
            description: Some("UPDATE_task_description".to_string()),
            status: 0,
            deadline: None,
            created_at: 0,
            updated_at: None,
        };

        let req = test::TestRequest::post().uri("/tasks/0").set_json(task).to_request();
        let res: TaskResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 0);
        assert_eq!(res.results.len(), 1);
        assert_eq!(res.message, "OK");
        assert_eq!(res.count, 1);

        let req = test::TestRequest::get().uri("/tasks?target=id&id=0").to_request();
        let res: TaskResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.results[0].name, "UPDATE_task");
        assert_eq!(res.results[0].description, Some("UPDATE_task_description".to_string()));
        assert_eq!(res.results[0].level, 0);
        assert_eq!(res.results[0].status, 0);
        assert_eq!(res.results[0].project_id, 0);
        assert_eq!(res.results[0].parent_id, None);
        assert_eq!(res.results[0].deadline, None);
        assert!(res.results[0].created_at >= 0);
        assert!(res.results[0].updated_at.is_some());
    }

    #[actix_web::test]
    async fn test_update_task_with_invalid_json() {
        let pool = setup_test_db("task_handler_test", "test_update_task_with_invalid_json").await;

        let app = test::init_service(
            App::new().service(update_task).service(get_tasks).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::post().uri("/tasks/0").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_update_task_with_id_not_exists() {
        let pool = setup_test_db("task_handler_test", "test_update_task_with_id_not_exists").await;

        let app = test::init_service(
            App::new().service(update_task).service(get_tasks).app_data(web::Data::new(pool))
        ).await;

        let task = Task {
            id: Some(100),
            project_id: 0,
            parent_id: None,
            level: 0,
            name: "UPDATE_task".to_string(),
            description: Some("UPDATE_task_description".to_string()),
            status: 0,
            deadline: None,
            created_at: 0,
            updated_at: None,
        };

        let req = test::TestRequest::post().uri("/tasks/100").set_json(task).to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("NotFound"));
    }

    #[actix_web::test]
    async fn test_update_task_with_invalid_project_id() {
        let pool = setup_test_db("task_handler_test", "test_update_task_with_invalid_project_id").await;

        let app = test::init_service(
            App::new().service(update_task).service(get_tasks).app_data(web::Data::new(pool))
        ).await;

        let task = Task {
            id: Some(0),
            project_id: -1,
            parent_id: None,
            level: 0,
            name: "UPDATE_task".to_string(),
            description: Some("UPDATE_task_description".to_string()),
            status: 0,
            deadline: None,
            created_at: 0,
            updated_at: None,
        };

        let req = test::TestRequest::post().uri("/tasks/0").set_json(task).to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }


    #[actix_web::test]
    async fn test_update_task_with_invalid_parent_id() {
        let pool = setup_test_db("task_handler_test", "test_update_task_with_invalid_parent_id").await;

        let app = test::init_service(
            App::new().service(update_task).service(get_tasks).app_data(web::Data::new(pool))
        ).await;

        let task = Task {
            id: Some(2),
            project_id: 0,
            parent_id: Some(100),
            level: 2,
            name: "UPDATE_task".to_string(),
            description: Some("UPDATE_task_description".to_string()),
            status: 0,
            deadline: None,
            created_at: 0,
            updated_at: None,
        };

        let req = test::TestRequest::post().uri("/tasks/2").set_json(task).to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("NotFound"));
    }

    #[actix_web::test]
    async fn test_update_task_with_invalid_level() {
        let pool = setup_test_db("task_handler_test", "test_update_task_with_invalid_level").await;

        let app = test::init_service(
            App::new().service(update_task).service(get_tasks).app_data(web::Data::new(pool))
        ).await;

        let task = Task {
            id: Some(2),
            project_id: 0,
            parent_id: Some(1),
            level: 1,
            name: "UPDATE_task".to_string(),
            description: Some("UPDATE_task_description".to_string()),
            status: 0,
            deadline: None,
            created_at: 0,
            updated_at: None,
        };

        let req = test::TestRequest::post().uri("/tasks/2").set_json(task).to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_update_task_with_id_mismatch() {
        let pool = setup_test_db("task_handler_test", "test_update_task_with_id_mismatch").await;

        let app = test::init_service(
            App::new().service(update_task).service(get_tasks).app_data(web::Data::new(pool))
        ).await;

        let task = Task {
            id: Some(0),
            project_id: 0,
            parent_id: None,
            level: 0,
            name: "UPDATE_task".to_string(),
            description: Some("UPDATE_task_description".to_string()),
            status: 0,
            deadline: None,
            created_at: 0,
            updated_at: None,
        };

        let req = test::TestRequest::post().uri("/tasks/1").set_json(task).to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_update_task_with_invalid_path() {
        let pool = setup_test_db("task_handler_test", "test_update_task_with_invalid_path").await;

        let app = test::init_service(
            App::new().service(update_task).service(get_tasks).app_data(web::Data::new(pool))
        ).await;

        let task = Task {
            id: Some(0),
            project_id: 0,
            parent_id: None,
            level: 0,
            name: "UPDATE_task".to_string(),
            description: Some("UPDATE_task_description".to_string()),
            status: 0,
            deadline: None,
            created_at: 0,
            updated_at: None,
        };

        let req = test::TestRequest::post().uri("/tasks/abc").set_json(task).to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_delete_task() {
        let pool = setup_test_db("task_handler_test", "test_delete_task").await;

        let app = test::init_service(
            App::new().service(delete_task).service(get_tasks).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::delete().uri("/tasks/9").to_request();
        let res: TaskResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 0);
        assert_eq!(res.results.len(), 0);
        assert_eq!(res.message, "OK");
        assert_eq!(res.count, 0);
    }

    #[actix_web::test]
    async fn test_delete_task_with_invalid_path() {
        let pool = setup_test_db("task_handler_test", "test_delete_task_with_invalid_path").await;

        let app = test::init_service(
            App::new().service(delete_task).service(get_tasks).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::delete().uri("/tasks/abc").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_delete_task_with_id_not_exists() {
        let pool = setup_test_db("task_handler_test", "test_delete_task_with_id_not_exists").await;

        let app = test::init_service(
            App::new().service(delete_task).service(get_tasks).app_data(web::Data::new(pool))
        ).await;
        
        let req = test::TestRequest::delete().uri("/tasks/100").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("NotFound"));
    }
}
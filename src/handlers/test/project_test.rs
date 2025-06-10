#[cfg(test)]
mod project_handler_test {
    use crate::handlers::project::get_projects;
    use crate::handlers::project::create_project;
    use crate::handlers::project::update_project;
    use crate::handlers::project::delete_project;
    use actix_web::{test,App,web};
    use crate::models::{Project, ProjectResponse};
    use crate::models::ErrorResponse;
    use crate::handlers::test::utils::setup_test_db;

    #[ctor::ctor]
    fn init() {
        if !std::path::Path::new("./test_db/project_handler_test").exists() {
            std::fs::create_dir_all("./test_db/project_handler_test").unwrap();
        }

        let files = std::fs::read_dir("./test_db/project_handler_test").unwrap();
        for file in files {
            let path = file.unwrap().path();
            if path.is_file() {
                std::fs::remove_file(path).unwrap();
            }
        }
    }

    #[actix_web::test]
    async fn test_get_projects() {
        let pool = setup_test_db("project_handler_test", "test_get_projects").await;

        let app = test::init_service(
            App::new().service(get_projects).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/projects").to_request();

        let res: ProjectResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 0);
        assert_eq!(res.results.len(), 10);
        assert_eq!(res.message, "OK");
        assert_eq!(res.count, 10);
    }

    #[actix_web::test]
    async fn test_get_projects_with_pagination() {
        let pool = setup_test_db("project_handler_test", "test_get_projects_with_pagination").await;

        let app = test::init_service(
            App::new().service(get_projects).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/projects?page=1&page_size=4").to_request();
        let res: ProjectResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 0);
        assert_eq!(res.results.len(), 4);
        assert_eq!(res.message, "OK");
        assert_eq!(res.count, 4);
        assert_eq!(res.results[0].name, "TestProject0");
        assert_eq!(res.results[1].name, "TestProject1");
        assert_eq!(res.results[2].name, "TestProject2");
        assert_eq!(res.results[3].name, "TestProject3");

        let req = test::TestRequest::get().uri("/projects?page=2&page_size=4").to_request();
        let res: ProjectResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 0);
        assert_eq!(res.results.len(), 4);
        assert_eq!(res.message, "OK");
        assert_eq!(res.count, 4);
        assert_eq!(res.results[0].name, "TestProject4");
        assert_eq!(res.results[1].name, "TestProject5");
        assert_eq!(res.results[2].name, "TestProject6");
        assert_eq!(res.results[3].name, "TestProject7");

        let req = test::TestRequest::get().uri("/projects?page=3&page_size=4").to_request();
        let res: ProjectResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 0);
        assert_eq!(res.results.len(), 2);
        assert_eq!(res.message, "OK");
        assert_eq!(res.count, 2);
        assert_eq!(res.results[0].name, "TestProject8");
        assert_eq!(res.results[1].name, "TestProject9");
    }

    #[actix_web::test]
    async fn test_get_projects_with_pagination_over_page_size() {
        let pool = setup_test_db("project_handler_test", "test_get_projects_with_pagination_over_page_size").await;

        let app = test::init_service(
            App::new().service(get_projects).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/projects?page=1&page_size=1000").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_get_projects_by_name() {
        let pool = setup_test_db("project_handler_test", "test_get_projects_by_name").await;

        let app = test::init_service(
            App::new().service(get_projects).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/projects?target=name&name=TestProject0").to_request();
        let res: ProjectResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 0);
        assert_eq!(res.results.len(), 1);
        assert_eq!(res.message, "OK");
        assert_eq!(res.count, 1);
        assert_eq!(res.results[0].name, "TestProject0");
    }

    #[actix_web::test]
    async fn test_get_projects_by_id() {
        let pool = setup_test_db("project_handler_test", "test_get_projects_by_id").await;

        let app = test::init_service(
            App::new().service(get_projects).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/projects?target=id&id=0").to_request();
        let res: ProjectResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 0);
        assert_eq!(res.results.len(), 1);
        assert_eq!(res.message, "OK");
        assert_eq!(res.count, 1);
    }

    #[actix_web::test]
    async fn test_get_projects_by_id_no_id() {
        let pool = setup_test_db("project_handler_test", "test_get_projects_by_id_no_id").await;

        let app = test::init_service(
            App::new().service(get_projects).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/projects?target=id").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_get_projects_by_name_no_name() {
        let pool = setup_test_db("project_handler_test", "test_get_projects_by_name_no_name").await;

        let app = test::init_service(
            App::new().service(get_projects).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/projects?target=name").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_get_projects_by_id_not_exists() {
        let pool = setup_test_db("project_handler_test", "test_get_projects_by_id_not_exists").await;

        let app = test::init_service(
            App::new().service(get_projects).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/projects?target=id&id=100").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("NotFound"));
    }

    #[actix_web::test]
    async fn test_get_projects_by_name_not_exists() {
        let pool = setup_test_db("project_handler_test", "test_get_projects_by_name_not_exists").await;

        let app = test::init_service(
            App::new().service(get_projects).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/projects?target=name&name=NotExists").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("NotFound"));
    }

    #[actix_web::test]
    async fn test_get_projects_with_pagination_no_page() {
        let pool = setup_test_db("project_handler_test", "test_get_projects_with_pagination_no_page").await;

        let app = test::init_service(
            App::new().service(get_projects).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/projects?page_size=10").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_get_project_with_invalid_query() {
        let pool = setup_test_db("project_handler_test", "test_get_project_with_invalid_query").await;

        let app = test::init_service(
            App::new().service(get_projects).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::get().uri("/projects?target=id&id=abc").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_create_project() {
        let pool = setup_test_db("project_handler_test", "test_create_project").await;

        let app = test::init_service(
            App::new().service(create_project).service(get_projects).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::post().uri("/projects")
        .set_json(Project {
            id: None,
            name: "TestProject10".to_string(),
        })
        .to_request();
        let res: ProjectResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 0);
        assert_eq!(res.results.len(), 1);
        assert_eq!(res.message, "OK");
        assert_eq!(res.count, 1);
        assert_eq!(res.results[0].name, "TestProject10");

        let req = test::TestRequest::get().uri("/projects?target=name&name=TestProject10").to_request();
        let res: ProjectResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 0);
        assert_eq!(res.results.len(), 1);
        assert_eq!(res.message, "OK");
        assert_eq!(res.count, 1);
        assert_eq!(res.results[0].name, "TestProject10");
    }

    #[actix_web::test]
    async fn test_create_project_name_empty() {
        let pool = setup_test_db("project_handler_test", "test_create_project_name_empty").await;

        let app = test::init_service(
            App::new().service(create_project).service(get_projects).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::post().uri("/projects")
        .set_json(Project {
            id: None,
            name: "".to_string(),
        })
        .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_create_project_exist_name() {
        let pool = setup_test_db("project_handler_test", "test_create_project_exist_name").await;

        let app = test::init_service(
            App::new().service(create_project).service(get_projects).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::post().uri("/projects")
        .set_json(Project {
            id: None,
            name: "TestProject0".to_string(),
        })
        .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("InternalServerError"));
    }

    #[actix_web::test]
    async fn test_create_user_no_param() {
        let pool = setup_test_db("project_handler_test", "test_create_user_no_param").await;

        let app = test::init_service(
            App::new().service(create_project).service(get_projects).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::post().uri("/projects").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_update_project() {
        let pool = setup_test_db("project_handler_test", "test_update_project").await;

        let app = test::init_service(
            App::new().service(update_project).service(get_projects).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::post().uri("/projects/0")
        .set_json(Project {
            id: Some(0),
            name: "TestProject11".to_string(),
        })
        .to_request();
        let res: ProjectResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 0);
        assert_eq!(res.results.len(), 1);
        assert_eq!(res.message, "OK");
        assert_eq!(res.count, 1);
        assert_eq!(res.results[0].name, "TestProject11");

        let req = test::TestRequest::get().uri("/projects?target=id&id=0").to_request();
        let res: ProjectResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 0);
        assert_eq!(res.results.len(), 1);
        assert_eq!(res.message, "OK");
        assert_eq!(res.results[0].name, "TestProject11");
    }

    #[actix_web::test]
    async fn test_update_project_invalid_name() {
        let pool = setup_test_db("project_handler_test", "test_update_project_invalid_name").await;

        let app = test::init_service(
            App::new().service(update_project).service(get_projects).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::post().uri("/projects/0")
        .set_json(Project {
            id: Some(0),
            name: "a".repeat(129),
        })
        .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_update_project_id_mismatch() {
        let pool = setup_test_db("project_handler_test", "test_update_project_id_mismatch").await;

        let app = test::init_service(
            App::new().service(update_project).service(get_projects).app_data(web::Data::new(pool))
        ).await;
        
        let req = test::TestRequest::post().uri("/projects/0")
        .set_json(Project {
            id: Some(1),
            name: "TestProject11".to_string(),
        })
        .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }   

    #[actix_web::test]
    async fn test_update_project_id_not_exists() {
        let pool = setup_test_db("project_handler_test", "test_update_project_id_not_exists").await;

        let app = test::init_service(
            App::new().service(update_project).service(get_projects).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::post().uri("/projects/100")
        .set_json(Project {
            id: Some(100),
            name: "TestProject11".to_string(),
        })
        .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        println!("res: {:?}", res.message);
        assert!(res.message.contains("NotFound"));
    }

    #[actix_web::test]
    async fn test_update_project_duplicate_name() {
        let pool = setup_test_db("project_handler_test", "test_update_project_duplicate_name").await;

        let app = test::init_service(
            App::new().service(update_project).service(get_projects).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::post().uri("/projects/0")
        .set_json(Project {
            id: Some(0),
            name: "TestProject5".to_string(),
        })
        .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("InternalServerError"));
    }

    #[actix_web::test]
    async fn test_update_project_invalid_path() {
        let pool = setup_test_db("project_handler_test", "test_update_project_invalid_path").await;

        let app = test::init_service(
            App::new().service(update_project).service(get_projects).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::post().uri("/projects/abc").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_delete_project() {
        let pool = setup_test_db("project_handler_test", "test_delete_project").await;

        let app = test::init_service(
            App::new().service(delete_project).service(get_projects).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::delete().uri("/projects/9").to_request();
        let res: ProjectResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 0);
        assert_eq!(res.results.len(), 0);
        assert_eq!(res.message, "OK");
        assert_eq!(res.count, 0);
    }

    #[actix_web::test]
    async fn test_delete_project_invalid_path() {
        let pool = setup_test_db("project_handler_test", "test_delete_project_invalid_path").await;

        let app = test::init_service(
            App::new().service(delete_project).service(get_projects).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::delete().uri("/projects/abc").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    #[actix_web::test]
    async fn test_delete_project_not_exists() {
        let pool = setup_test_db("project_handler_test", "test_delete_project_not_exists").await;

        let app = test::init_service(
            App::new().service(delete_project).service(get_projects).app_data(web::Data::new(pool))
        ).await;

        let req = test::TestRequest::delete().uri("/projects/100").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("NotFound"));
    }

}
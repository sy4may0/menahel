#[cfg(test)]
mod handler_root_test {
    use crate::{handlers::root::{health, root}, models::RootResponse};
    use actix_web::{test, App};
    use uuid::Uuid;

    #[actix_web::test]
    async fn test_root() {
        let app = test::init_service(
            App::new().service(root).service(health)
        ).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let res: RootResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.message, "I AM MENAHEL!");
        assert_eq!(res.metadata.as_ref().unwrap().api_version, env!("CARGO_PKG_VERSION"));
    }

    #[actix_web::test]
    async fn test_root_with_request_id() {
        let app = test::init_service(
            App::new().service(root).service(health)
        ).await;
        let uuid = Uuid::new_v4().to_string();
        let req = test::TestRequest::get()
            .uri("/").insert_header(("X-Request-ID", uuid.clone().as_str()))
            .to_request();
        let res: RootResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.message, "I AM MENAHEL!");
        assert_eq!(res.metadata.as_ref().unwrap().request_id, uuid);
    }

    #[actix_web::test]
    async fn test_health() {
        let app = test::init_service(
            App::new().service(root).service(health)
        ).await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let res: RootResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.message, "MENAHEL IS RUNNING!");
        assert_eq!(res.metadata.as_ref().unwrap().api_version, env!("CARGO_PKG_VERSION"));
    }

    #[actix_web::test]
    async fn test_health_with_request_id() {
        let app = test::init_service(
            App::new().service(root).service(health)
        ).await;
        let uuid = Uuid::new_v4().to_string();
        let req = test::TestRequest::get()
            .uri("/health").insert_header(("X-Request-ID", uuid.clone().as_str()))
            .to_request();
        let res: RootResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.message, "MENAHEL IS RUNNING!");
        assert_eq!(res.metadata.as_ref().unwrap().request_id, uuid);
    }
}

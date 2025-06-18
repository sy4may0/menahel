#[cfg(test)]
mod comment_handler_test {
    use crate::handlers::comment::{create_comment, delete_comment, get_comments, update_comment};
    use crate::handlers::test::utils::setup_test_db;
    use crate::models::comment::Comment;
    use crate::models::response_model::ErrorResponse;
    use crate::models::response_model::{CommentResponse, CommentUserResponse};
    use actix_web::{App, test, web};

    #[ctor::ctor]
    fn init() {
        if !std::path::Path::new("./test_db/comment_handler_test").exists() {
            std::fs::create_dir_all("./test_db/comment_handler_test").unwrap();
        }

        let files = std::fs::read_dir("./test_db/comment_handler_test").unwrap();
        for file in files {
            let path = file.unwrap().path();
            if path.is_file() {
                std::fs::remove_file(path).unwrap();
            }
        }
    }

    // [R正常系]
    // - target指定なし
    #[actix_web::test]
    async fn test_get_comments() {
        let pool = setup_test_db("comment_handler_test", "test_get_comments").await;

        let app = test::init_service(
            App::new()
                .service(get_comments)
                .app_data(web::Data::new(pool)),
        )
        .await;

        let req = test::TestRequest::get().uri("/comments").to_request();
        let res: CommentUserResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.results.len(), 14);
        assert_eq!(res.count, 14);
        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
    }

    // - target指定なし + page/page_size指定
    #[actix_web::test]
    async fn test_get_comments_with_pagination() {
        let pool = setup_test_db("comment_handler_test", "test_get_comments_with_pagination").await;

        let app = test::init_service(
            App::new()
                .service(get_comments)
                .app_data(web::Data::new(pool)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/comments?page=1&page_size=10")
            .to_request();
        let res: CommentUserResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.results.len(), 10);
        assert_eq!(res.count, 10);
        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
    }

    // - target: all指定
    #[actix_web::test]
    async fn test_get_comments_with_pagination_all() {
        let pool = setup_test_db(
            "comment_handler_test",
            "test_get_comments_with_pagination_all",
        )
        .await;

        let app = test::init_service(
            App::new()
                .service(get_comments)
                .app_data(web::Data::new(pool)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/comments?page=1&page_size=10&target=all")
            .to_request();
        let res: CommentUserResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.results.len(), 10);
        assert_eq!(res.count, 10);
        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
    }

    // - target: id指定
    #[actix_web::test]
    async fn test_get_comment_by_id() {
        let pool = setup_test_db("comment_handler_test", "test_get_comment_by_id").await;

        let app = test::init_service(
            App::new()
                .service(get_comments)
                .app_data(web::Data::new(pool)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/comments?target=id&id=0")
            .to_request();
        let res: CommentUserResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.results.len(), 1);
        assert_eq!(res.count, 1);
        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
    }
    // - target: task_id指定
    #[actix_web::test]
    async fn test_get_comments_by_task_id() {
        let pool = setup_test_db("comment_handler_test", "test_get_comments_by_task_id").await;

        let app = test::init_service(
            App::new()
                .service(get_comments)
                .app_data(web::Data::new(pool)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/comments?target=task_id&task_id=2")
            .to_request();
        let res: CommentUserResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.results.len(), 4);
        assert_eq!(res.count, 4);
        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
    }

    // - target: task_id指定 + page/page_size指定
    #[actix_web::test]
    async fn test_get_comments_by_task_id_with_pagination() {
        let pool = setup_test_db(
            "comment_handler_test",
            "test_get_comments_by_task_id_with_pagination",
        )
        .await;

        let app = test::init_service(
            App::new()
                .service(get_comments)
                .app_data(web::Data::new(pool)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/comments?target=task_id&task_id=2&page=1&page_size=3")
            .to_request();
        let res: CommentUserResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.results.len(), 3);
        assert_eq!(res.count, 3);
        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");

        let req = test::TestRequest::get()
            .uri("/comments?target=task_id&task_id=2&page=2&page_size=3")
            .to_request();
        let res: CommentUserResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.results.len(), 1);
        assert_eq!(res.count, 1);
        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
    }

    // - target: user_id指定
    #[actix_web::test]
    async fn test_get_comments_by_user_id() {
        let pool = setup_test_db("comment_handler_test", "test_get_comments_by_user_id").await;

        let app = test::init_service(
            App::new()
                .service(get_comments)
                .app_data(web::Data::new(pool)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/comments?target=user_id&user_id=0")
            .to_request();
        let res: CommentUserResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.results.len(), 5);
        assert_eq!(res.count, 5);
        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
    }

    // - target: user_id指定 + page/page_size指定
    #[actix_web::test]
    async fn test_get_comments_by_user_id_with_pagination() {
        let pool = setup_test_db(
            "comment_handler_test",
            "test_get_comments_by_user_id_with_pagination",
        )
        .await;

        let app = test::init_service(
            App::new()
                .service(get_comments)
                .app_data(web::Data::new(pool)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/comments?target=user_id&user_id=0&page=1&page_size=4")
            .to_request();
        let res: CommentUserResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.results.len(), 4);
        assert_eq!(res.count, 4);
        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");

        let req = test::TestRequest::get()
            .uri("/comments?target=user_id&user_id=0&page=2&page_size=4")
            .to_request();
        let res: CommentUserResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.results.len(), 1);
        assert_eq!(res.count, 1);
        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
    }

    // [R異常系]
    // - target指定なし + page/page_size指定(pageまたはpage_sizeが0以下)
    #[actix_web::test]
    async fn test_get_comments_with_pagination_invalid_page() {
        let pool = setup_test_db(
            "comment_handler_test",
            "test_get_comments_with_pagination_invalid_page",
        )
        .await;

        let app = test::init_service(
            App::new()
                .service(get_comments)
                .app_data(web::Data::new(pool)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/comments?page=0&page_size=10")
            .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));

        let req = test::TestRequest::get()
            .uri("/comments?page=1&page_size=0")
            .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    // - target指定なし + page/page_size指定(page_sizeが1000以上)
    #[actix_web::test]
    async fn test_get_comments_with_pagination_page_size_too_long() {
        let pool = setup_test_db(
            "comment_handler_test",
            "test_get_comments_with_pagination_page_size_too_long",
        )
        .await;

        let app = test::init_service(
            App::new()
                .service(get_comments)
                .app_data(web::Data::new(pool)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/comments?page=1&page_size=1000")
            .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    // - target指定なし + pageのみ指定
    #[actix_web::test]
    async fn test_get_comments_with_pagination_page_only() {
        let pool = setup_test_db(
            "comment_handler_test",
            "test_get_comments_with_pagination_page_only",
        )
        .await;

        let app = test::init_service(
            App::new()
                .service(get_comments)
                .app_data(web::Data::new(pool)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/comments?page=1")
            .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    // - target指定なし + page_sizeのみ指定
    #[actix_web::test]
    async fn test_get_comments_with_pagination_page_size_only() {
        let pool = setup_test_db(
            "comment_handler_test",
            "test_get_comments_with_pagination_page_size_only",
        )
        .await;

        let app = test::init_service(
            App::new()
                .service(get_comments)
                .app_data(web::Data::new(pool)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/comments?page_size=10")
            .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    // - target: id指定 + idが無い
    #[actix_web::test]
    async fn test_get_comment_by_id_without_id() {
        let pool = setup_test_db("comment_handler_test", "test_get_comment_by_id_without_id").await;

        let app = test::init_service(
            App::new()
                .service(get_comments)
                .app_data(web::Data::new(pool)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/comments?target=id")
            .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    // - target: id指定 + idが数値でない
    #[actix_web::test]
    async fn test_get_comment_by_id_invalid_id() {
        let pool = setup_test_db("comment_handler_test", "test_get_comment_by_id_invalid_id").await;

        let app = test::init_service(
            App::new()
                .service(get_comments)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/comments?target=id&id=a")
            .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    // - target: id指定 + idが存在しない
    #[actix_web::test]
    async fn test_get_comment_by_id_not_found() {
        let pool = setup_test_db("comment_handler_test", "test_get_comment_by_id_not_found").await;

        let app = test::init_service(
            App::new()
                .service(get_comments)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/comments?target=id&id=1000")
            .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("NotFound"));
    }

    // - target: id指定 + idが0以下
    #[actix_web::test]
    async fn test_get_comment_by_id_invalid_id_negative() {
        let pool = setup_test_db(
            "comment_handler_test",
            "test_get_comment_by_id_invalid_id_negative",
        )
        .await;

        let app = test::init_service(
            App::new()
                .service(get_comments)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/comments?target=id&id=-1")
            .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    // - target: task_id指定 + task_idが無い
    #[actix_web::test]
    async fn test_get_comments_by_task_id_without_task_id() {
        let pool = setup_test_db(
            "comment_handler_test",
            "test_get_comments_by_task_id_without_task_id",
        )
        .await;

        let app = test::init_service(
            App::new()
                .service(get_comments)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/comments?target=task_id")
            .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }
    // - target: task_id指定 + task_idが数値でない
    #[actix_web::test]
    async fn test_get_comments_by_task_id_invalid_task_id() {
        let pool = setup_test_db(
            "comment_handler_test",
            "test_get_comments_by_task_id_invalid_task_id",
        )
        .await;

        let app = test::init_service(
            App::new()
                .service(get_comments)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/comments?target=task_id&task_id=a")
            .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }
    // - target: task_id指定 + task_idが存在しない
    #[actix_web::test]
    async fn test_get_comments_by_task_id_not_found() {
        let pool = setup_test_db(
            "comment_handler_test",
            "test_get_comments_by_task_id_not_found",
        )
        .await;

        let app = test::init_service(
            App::new()
                .service(get_comments)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/comments?target=task_id&task_id=1000")
            .to_request();
        let res: CommentUserResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.results.len(), 0);
        assert_eq!(res.count, 0);
        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
    }
    // - target: task_id指定 + task_idが0以下
    #[actix_web::test]
    async fn test_get_comments_by_task_id_invalid_task_id_negative() {
        let pool = setup_test_db(
            "comment_handler_test",
            "test_get_comments_by_task_id_invalid_task_id_negative",
        )
        .await;

        let app = test::init_service(
            App::new()
                .service(get_comments)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/comments?target=task_id&task_id=-1")
            .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }
    // - target: user_id指定 + user_idが無い
    #[actix_web::test]
    async fn test_get_comments_by_user_id_without_user_id() {
        let pool = setup_test_db(
            "comment_handler_test",
            "test_get_comments_by_user_id_without_user_id",
        )
        .await;

        let app = test::init_service(
            App::new()
                .service(get_comments)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/comments?target=user_id")
            .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }
    // - target: user_id指定 + user_idが数値でない
    #[actix_web::test]
    async fn test_get_comments_by_user_id_invalid_user_id() {
        let pool = setup_test_db(
            "comment_handler_test",
            "test_get_comments_by_user_id_invalid_user_id",
        )
        .await;

        let app = test::init_service(
            App::new()
                .service(get_comments)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/comments?target=user_id&user_id=a")
            .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }
    // - target: user_id指定 + user_idが存在しない
    #[actix_web::test]
    async fn test_get_comments_by_user_id_not_found() {
        let pool = setup_test_db(
            "comment_handler_test",
            "test_get_comments_by_user_id_not_found",
        )
        .await;

        let app = test::init_service(
            App::new()
                .service(get_comments)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/comments?target=user_id&user_id=1000")
            .to_request();
        let res: CommentUserResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.results.len(), 0);
        assert_eq!(res.count, 0);
        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
    }
    // - target: user_id指定 + user_idが0以下
    #[actix_web::test]
    async fn test_get_comments_by_user_id_invalid_user_id_negative() {
        let pool = setup_test_db(
            "comment_handler_test",
            "test_get_comments_by_user_id_invalid_user_id_negative",
        )
        .await;

        let app = test::init_service(
            App::new()
                .service(get_comments)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/comments?target=user_id&user_id=-1")
            .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    // [CUD正常系]
    // - コメント作成
    #[actix_web::test]
    async fn test_create_comment() {
        let pool = setup_test_db("comment_handler_test", "test_create_comment").await;

        let comment = Comment {
            comment_id: None,
            user_id: 1,
            task_id: 5,
            content: "test".to_string(),
            created_at: 0,
            updated_at: None,
        };

        let app = test::init_service(
            App::new()
                .service(create_comment)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/comments")
            .set_json(comment)
            .to_request();
        let res: CommentResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.results.len(), 1);
        assert_eq!(res.count, 1);
        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
    }
    // - コメント更新
    #[actix_web::test]
    async fn test_update_comment() {
        let pool = setup_test_db("comment_handler_test", "test_update_comment").await;

        let comment = Comment {
            comment_id: Some(1),
            user_id: 1,
            task_id: 5,
            content: "test".to_string(),
            created_at: 0,
            updated_at: None,
        };

        let app = test::init_service(
            App::new()
                .service(update_comment)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/comments/1")
            .set_json(comment)
            .to_request();
        let res: CommentResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.results.len(), 1);
        assert_eq!(res.count, 1);
        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
    }
    // - コメント削除
    #[actix_web::test]
    async fn test_delete_comment() {
        let pool = setup_test_db("comment_handler_test", "test_delete_comment").await;

        let app = test::init_service(
            App::new()
                .service(delete_comment)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::delete().uri("/comments/1").to_request();
        let res: CommentResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.results.len(), 0);
        assert_eq!(res.count, 0);
        assert_eq!(res.rc, 0);
        assert_eq!(res.message, "OK");
    }

    // [CUD異常系]
    // - コメント作成: POST JSONデータ無し
    #[actix_web::test]
    async fn test_create_comment_invalid_json() {
        let pool = setup_test_db("comment_handler_test", "test_create_comment_invalid_json").await;

        let app = test::init_service(
            App::new()
                .service(create_comment)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::post().uri("/comments").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    // - コメント作成: POST user_idが-1
    #[actix_web::test]
    async fn test_create_comment_invalid_user_id_negative() {
        let pool = setup_test_db(
            "comment_handler_test",
            "test_create_comment_invalid_user_id_negative",
        )
        .await;

        let comment = Comment {
            comment_id: None,
            user_id: -1,
            task_id: 5,
            content: "test".to_string(),
            created_at: 0,
            updated_at: None,
        };
        let app = test::init_service(
            App::new()
                .service(create_comment)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/comments")
            .set_json(comment)
            .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    // - コメント作成: POST user_idが存在しない
    #[actix_web::test]
    async fn test_create_comment_invalid_user_id_not_found() {
        let pool = setup_test_db(
            "comment_handler_test",
            "test_create_comment_invalid_user_id_not_found",
        )
        .await;

        let comment = Comment {
            comment_id: None,
            user_id: 1000,
            task_id: 5,
            content: "test".to_string(),
            created_at: 0,
            updated_at: None,
        };
        let app = test::init_service(
            App::new()
                .service(create_comment)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/comments")
            .set_json(comment)
            .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("NotFound"));
    }

    // - コメント作成: POST task_idが-1
    #[actix_web::test]
    async fn test_create_comment_invalid_task_id_negative() {
        let pool = setup_test_db(
            "comment_handler_test",
            "test_create_comment_invalid_task_id_negative",
        )
        .await;

        let comment = Comment {
            comment_id: None,
            user_id: 1,
            task_id: -1,
            content: "test".to_string(),
            created_at: 0,
            updated_at: None,
        };
        let app = test::init_service(
            App::new()
                .service(create_comment)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/comments")
            .set_json(comment)
            .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }
    // - コメント作成: POST task_idが存在しない
    #[actix_web::test]
    async fn test_create_comment_invalid_task_id_not_found() {
        let pool = setup_test_db(
            "comment_handler_test",
            "test_create_comment_invalid_task_id_not_found",
        )
        .await;

        let comment = Comment {
            comment_id: None,
            user_id: 1,
            task_id: 1000,
            content: "test".to_string(),
            created_at: 0,
            updated_at: None,
        };
        let app = test::init_service(
            App::new()
                .service(create_comment)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/comments")
            .set_json(comment)
            .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("NotFound"));
    }
    // - コメント作成: POST taskのlevelがmax_levelでない
    #[actix_web::test]
    async fn test_create_comment_invalid_task_level() {
        let pool = setup_test_db(
            "comment_handler_test",
            "test_create_comment_invalid_task_level",
        )
        .await;

        let comment = Comment {
            comment_id: None,
            user_id: 1,
            task_id: 0,
            content: "test".to_string(),
            created_at: 0,
            updated_at: None,
        };
        let app = test::init_service(
            App::new()
                .service(create_comment)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/comments")
            .set_json(comment)
            .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    // - コメント作成: POST contentが2025文字以上
    #[actix_web::test]
    async fn test_create_comment_invalid_content_length() {
        let pool = setup_test_db(
            "comment_handler_test",
            "test_create_comment_invalid_content_length",
        )
        .await;

        let comment = Comment {
            comment_id: None,
            user_id: 1,
            task_id: 5,
            content: "t".repeat(2025),
            created_at: 0,
            updated_at: None,
        };
        let app = test::init_service(
            App::new()
                .service(create_comment)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/comments")
            .set_json(comment)
            .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }
    // - コメント作成: POST contentが空文字
    #[actix_web::test]
    async fn test_create_comment_invalid_content_empty() {
        let pool = setup_test_db(
            "comment_handler_test",
            "test_create_comment_invalid_content_empty",
        )
        .await;

        let comment = Comment {
            comment_id: None,
            user_id: 1,
            task_id: 5,
            content: "".to_string(),
            created_at: 0,
            updated_at: None,
        };
        let app = test::init_service(
            App::new()
                .service(create_comment)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/comments")
            .set_json(comment)
            .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    // - コメント作成: POST comment_idを指定して追加
    #[actix_web::test]
    async fn test_create_comment_invalid_comment_id() {
        let pool = setup_test_db(
            "comment_handler_test",
            "test_create_comment_invalid_comment_id",
        )
        .await;

        let comment = Comment {
            comment_id: Some(20),
            user_id: 1,
            task_id: 5,
            content: "test".to_string(),
            created_at: 0,
            updated_at: None,
        };
        let app = test::init_service(
            App::new()
                .service(create_comment)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/comments")
            .set_json(comment)
            .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    // - コメント更新: POST JSONデータ無し
    #[actix_web::test]
    async fn test_update_comment_invalid_json() {
        let pool = setup_test_db("comment_handler_test", "test_update_comment_invalid_json").await;

        let app = test::init_service(
            App::new()
                .service(update_comment)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::post().uri("/comments/1").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }
    // - コメント更新: POST pathが数値でない
    #[actix_web::test]
    async fn test_update_comment_invalid_path() {
        let pool = setup_test_db("comment_handler_test", "test_update_comment_invalid_path").await;

        let comment = Comment {
            comment_id: Some(1),
            user_id: 1,
            task_id: 5,
            content: "test".to_string(),
            created_at: 0,
            updated_at: None,
        };

        let app = test::init_service(
            App::new()
                .service(update_comment)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/comments/a")
            .set_json(comment)
            .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    // - コメント更新: POST pathが0以下
    #[actix_web::test]
    async fn test_update_comment_invalid_path_negative() {
        let pool = setup_test_db(
            "comment_handler_test",
            "test_update_comment_invalid_path_negative",
        )
        .await;

        let comment = Comment {
            comment_id: Some(-1),
            user_id: 1,
            task_id: 5,
            content: "test".to_string(),
            created_at: 0,
            updated_at: None,
        };
        let app = test::init_service(
            App::new()
                .service(update_comment)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/comments/-1")
            .set_json(comment)
            .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        println!("{}", res.message);
        assert!(res.message.contains("BadRequest"));
    }

    // - コメント更新: POST pathとJSONデータのcomment_idが一致しない
    #[actix_web::test]
    async fn test_update_comment_invalid_comment_id() {
        let pool = setup_test_db(
            "comment_handler_test",
            "test_update_comment_invalid_comment_id",
        )
        .await;

        let comment = Comment {
            comment_id: Some(1),
            user_id: 1,
            task_id: 5,
            content: "test".to_string(),
            created_at: 0,
            updated_at: None,
        };
        let app = test::init_service(
            App::new()
                .service(update_comment)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/comments/2")
            .set_json(comment)
            .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    // - コメント更新: POST pathで指定したidが存在しない。
    #[actix_web::test]
    async fn test_update_comment_invalid_path_not_found() {
        let pool = setup_test_db(
            "comment_handler_test",
            "test_update_comment_invalid_path_not_found",
        )
        .await;

        let comment = Comment {
            comment_id: Some(1000),
            user_id: 1,
            task_id: 5,
            content: "test".to_string(),
            created_at: 0,
            updated_at: None,
        };
        let app = test::init_service(
            App::new()
                .service(update_comment)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/comments/1000")
            .set_json(comment)
            .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        println!("{}", res.message);
        assert!(res.message.contains("NotFound"));
    }

    // - コメント更新: POST user_idが-1
    #[actix_web::test]
    async fn test_update_comment_invalid_user_id_negative() {
        let pool = setup_test_db(
            "comment_handler_test",
            "test_update_comment_invalid_user_id_negative",
        )
        .await;

        let comment = Comment {
            comment_id: Some(1),
            user_id: -1,
            task_id: 5,
            content: "test".to_string(),
            created_at: 0,
            updated_at: None,
        };
        let app = test::init_service(
            App::new()
                .service(update_comment)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/comments/1")
            .set_json(comment)
            .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    // - コメント更新: POST user_idが存在しない
    #[actix_web::test]
    async fn test_update_comment_invalid_user_id_not_found() {
        let pool = setup_test_db(
            "comment_handler_test",
            "test_update_comment_invalid_user_id_not_found",
        )
        .await;

        let comment = Comment {
            comment_id: Some(1),
            user_id: 1000,
            task_id: 5,
            content: "test".to_string(),
            created_at: 0,
            updated_at: None,
        };
        let app = test::init_service(
            App::new()
                .service(update_comment)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/comments/1")
            .set_json(comment)
            .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("NotFound"));
    }

    // - コメント更新: POST taskのlevelがmax_levelでない
    #[actix_web::test]
    async fn test_update_comment_invalid_task_level() {
        let pool = setup_test_db(
            "comment_handler_test",
            "test_update_comment_invalid_task_level",
        )
        .await;

        let comment = Comment {
            comment_id: Some(1),
            user_id: 1,
            task_id: 0,
            content: "test".to_string(),
            created_at: 0,
            updated_at: None,
        };
        let app = test::init_service(
            App::new()
                .service(update_comment)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/comments/1")
            .set_json(comment)
            .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    // - コメント更新: POST task_idが-1
    #[actix_web::test]
    async fn test_update_comment_invalid_task_id_negative() {
        let pool = setup_test_db(
            "comment_handler_test",
            "test_update_comment_invalid_task_id_negative",
        )
        .await;

        let comment = Comment {
            comment_id: Some(1),
            user_id: 1,
            task_id: -1,
            content: "test".to_string(),
            created_at: 0,
            updated_at: None,
        };
        let app = test::init_service(
            App::new()
                .service(update_comment)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/comments/1")
            .set_json(comment)
            .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    // - コメント更新: POST task_idが存在しない
    #[actix_web::test]
    async fn test_update_comment_invalid_task_id_not_found() {
        let pool = setup_test_db(
            "comment_handler_test",
            "test_update_comment_invalid_task_id_not_found",
        )
        .await;

        let comment = Comment {
            comment_id: Some(1),
            user_id: 1,
            task_id: 1000,
            content: "test".to_string(),
            created_at: 0,
            updated_at: None,
        };
        let app = test::init_service(
            App::new()
                .service(update_comment)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/comments/1")
            .set_json(comment)
            .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("NotFound"));
    }

    // - コメント更新: POST contentが2025文字以上
    #[actix_web::test]
    async fn test_update_comment_invalid_content_length() {
        let pool = setup_test_db(
            "comment_handler_test",
            "test_update_comment_invalid_content_length",
        )
        .await;

        let comment = Comment {
            comment_id: Some(1),
            user_id: 1,
            task_id: 5,
            content: "t".repeat(2025),
            created_at: 0,
            updated_at: None,
        };
        let app = test::init_service(
            App::new()
                .service(update_comment)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/comments/1")
            .set_json(comment)
            .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    // - コメント更新: POST contentが空文字
    #[actix_web::test]
    async fn test_update_comment_invalid_content_empty() {
        let pool = setup_test_db(
            "comment_handler_test",
            "test_update_comment_invalid_content_empty",
        )
        .await;

        let comment = Comment {
            comment_id: Some(1),
            user_id: 1,
            task_id: 5,
            content: "".to_string(),
            created_at: 0,
            updated_at: None,
        };
        let app = test::init_service(
            App::new()
                .service(update_comment)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/comments/1")
            .set_json(comment)
            .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }

    // - コメント削除: DELETE pathが数値でない
    #[actix_web::test]
    async fn test_delete_comment_invalid_path() {
        let pool = setup_test_db("comment_handler_test", "test_delete_comment_invalid_path").await;

        let app = test::init_service(
            App::new()
                .service(delete_comment)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::delete().uri("/comments/a").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }
    // - コメント削除: DELETE pathが0以下
    #[actix_web::test]
    async fn test_delete_comment_invalid_path_negative() {
        let pool = setup_test_db(
            "comment_handler_test",
            "test_delete_comment_invalid_path_negative",
        )
        .await;

        let app = test::init_service(
            App::new()
                .service(delete_comment)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::delete().uri("/comments/-1").to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("BadRequest"));
    }
    // - コメント削除: DELETE pathで指定したidが存在しない
    #[actix_web::test]
    async fn test_delete_comment_invalid_path_not_found() {
        let pool = setup_test_db(
            "comment_handler_test",
            "test_delete_comment_invalid_path_not_found",
        )
        .await;

        let app = test::init_service(
            App::new()
                .service(delete_comment)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::delete()
            .uri("/comments/1000")
            .to_request();
        let res: ErrorResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.rc, 1);
        assert!(res.message.contains("NotFound"));
    }
}

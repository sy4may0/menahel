use crate::errors::handler_errors::HandlerError;
use crate::errors::messages::ErrorKey;
use crate::errors::messages::get_error_message;
use crate::handlers::utils::get_request_id;
use crate::handlers::utils::handle_error;
use crate::models::PaginationParams;
use crate::models::repository_model::comment::Comment;
use crate::models::repository_model::comment::CommentWithUser;
use crate::models::response_model::CommentResponse;
use crate::models::response_model::CommentUserResponse;
use crate::models::response_model::ErrorResponse;
use crate::models::response_model::Pagination;
use crate::models::response_model::PaginationStatus;
use crate::models::response_model::ResponseMetadata;
use crate::repository::comment_repo::CommentRepository;
use actix_web::{HttpRequest, HttpResponse, Responder, delete, get, post, web};
use serde::Deserialize;
use sqlx::sqlite::SqlitePool;

enum QueryTarget {
    All,
    Id,
    TaskId,
    UserId,
}

impl QueryTarget {
    fn str_to_enum(s: &str) -> Result<QueryTarget, HandlerError> {
        match s {
            "all" => Ok(QueryTarget::All),
            "id" => Ok(QueryTarget::Id),
            "task_id" => Ok(QueryTarget::TaskId),
            "user_id" => Ok(QueryTarget::UserId),
            _ => Err(HandlerError::BadRequest(get_error_message(
                ErrorKey::CommentHandlerGetCommentInvalidTarget,
                format!("target: {}", s),
            ))),
        }
    }
}

#[derive(Deserialize)]
struct GetCommentsQuery {
    target: Option<String>,
    page: Option<i32>,
    page_size: Option<i32>,
    id: Option<i64>,
    task_id: Option<i64>,
    user_id: Option<i64>,
}

impl GetCommentsQuery {
    fn target(&self) -> Result<QueryTarget, HandlerError> {
        match &self.target {
            Some(target) => QueryTarget::str_to_enum(target),
            None => {
                log::debug!("No target specified. using default target: all");
                Ok(QueryTarget::All)
            }
        }
    }
    fn validate(&self) -> Result<(), HandlerError> {
        let target = self.target()?;

        match target {
            QueryTarget::All => Ok(()),
            QueryTarget::Id => {
                if self.id.is_none() {
                    return Err(HandlerError::BadRequest(get_error_message(
                        ErrorKey::CommentHandlerGetCommentNoIdSpecified,
                        "".to_string(),
                    )));
                }
                Ok(())
            }
            QueryTarget::TaskId => {
                if self.task_id.is_none() {
                    return Err(HandlerError::BadRequest(get_error_message(
                        ErrorKey::CommentHandlerGetCommentNoTaskIdSpecified,
                        "".to_string(),
                    )));
                }
                Ok(())
            }
            QueryTarget::UserId => {
                if self.user_id.is_none() {
                    return Err(HandlerError::BadRequest(get_error_message(
                        ErrorKey::CommentHandlerGetCommentNoUserIdSpecified,
                        "".to_string(),
                    )));
                }
                Ok(())
            }
        }
    }
}

async fn get_comments_with_pagination_all(
    pagination_params: &PaginationParams,
    pool: SqlitePool,
) -> Result<Vec<CommentWithUser>, HandlerError> {
    let comment_repo = CommentRepository::new(pool);

    match pagination_params.status() {
        PaginationStatus::Active => {
            log::debug!(
                "Getting comments with pagination: page: {:?}, page_size: {:?}",
                pagination_params.page(),
                pagination_params.page_size()
            );
            comment_repo
                .get_comments_with_pagination(
                    pagination_params.page().unwrap(),
                    pagination_params.page_size().unwrap(),
                )
                .await
                .map_err(HandlerError::from)
        }
        PaginationStatus::Inactive => {
            log::debug!("Getting all comments");
            comment_repo
                .get_all_comments()
                .await
                .map_err(HandlerError::from)
        }
        PaginationStatus::Error => Err(HandlerError::BadRequest(get_error_message(
            ErrorKey::CommentHandlerGetCommentInvalidPage,
            format!(
                "page: {:?}, page_size: {:?}",
                pagination_params.page(),
                pagination_params.page_size()
            ),
        ))),
    }
}

async fn get_comments_with_pagination_by_task_id(
    pagination_params: &PaginationParams,
    task_id: i64,
    pool: SqlitePool,
) -> Result<Vec<CommentWithUser>, HandlerError> {
    let comment_repo = CommentRepository::new(pool);

    match pagination_params.status() {
        PaginationStatus::Active => {
            log::debug!(
                "Getting comments with pagination by task id: {:?}, page: {:?}, page_size: {:?}",
                task_id,
                pagination_params.page(),
                pagination_params.page_size()
            );
            comment_repo
                .get_comments_with_pagination_by_task_id(
                    task_id,
                    pagination_params.page().unwrap(),
                    pagination_params.page_size().unwrap(),
                )
                .await
                .map_err(HandlerError::from)
        }
        PaginationStatus::Inactive => {
            log::debug!("Getting all comments by task id: {:?}", task_id);
            comment_repo
                .get_comment_by_task_id(task_id)
                .await
                .map_err(HandlerError::from)
        }
        PaginationStatus::Error => Err(HandlerError::BadRequest(get_error_message(
            ErrorKey::CommentHandlerGetCommentInvalidPage,
            format!(
                "page: {:?}, page_size: {:?}",
                pagination_params.page(),
                pagination_params.page_size()
            ),
        ))),
    }
}

async fn get_comments_with_pagination_by_user_id(
    pagination_params: &PaginationParams,
    user_id: i64,
    pool: SqlitePool,
) -> Result<Vec<CommentWithUser>, HandlerError> {
    let comment_repo = CommentRepository::new(pool);

    match pagination_params.status() {
        PaginationStatus::Active => {
            log::debug!(
                "Getting comments with pagination by user id: {:?}, page: {:?}, page_size: {:?}",
                user_id,
                pagination_params.page(),
                pagination_params.page_size()
            );
            comment_repo
                .get_comments_with_pagination_by_user_id(
                    user_id,
                    pagination_params.page().unwrap(),
                    pagination_params.page_size().unwrap(),
                )
                .await
                .map_err(HandlerError::from)
        }
        PaginationStatus::Inactive => {
            log::debug!("Getting all comments by user id: {:?}", user_id);
            comment_repo
                .get_comment_by_user_id(user_id)
                .await
                .map_err(HandlerError::from)
        }
        PaginationStatus::Error => Err(HandlerError::BadRequest(get_error_message(
            ErrorKey::CommentHandlerGetCommentInvalidPage,
            format!(
                "page: {:?}, page_size: {:?}",
                pagination_params.page(),
                pagination_params.page_size()
            ),
        ))),
    }
}

async fn get_comments_with_pagination(
    req: HttpRequest,
    query: GetCommentsQuery,
    pool: SqlitePool,
) -> HttpResponse {
    let metadata = ResponseMetadata::new(get_request_id(&req));

    let mut pagination_params = PaginationParams::new(query.page, query.page_size);
    pagination_params.validate();

    let validated_query = match query.validate() {
        Ok(()) => query,
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return handle_error(e, response);
        }
    };

    let result = match validated_query.target() {
        Ok(QueryTarget::All) => get_comments_with_pagination_all(&pagination_params, pool).await,
        Ok(QueryTarget::TaskId) => {
            get_comments_with_pagination_by_task_id(
                &pagination_params,
                validated_query.task_id.unwrap(),
                pool,
            )
            .await
        }
        Ok(QueryTarget::UserId) => {
            get_comments_with_pagination_by_user_id(
                &pagination_params,
                validated_query.user_id.unwrap(),
                pool,
            )
            .await
        }
        _ => Err(HandlerError::BadRequest(get_error_message(
            ErrorKey::CommentHandlerGetCommentInvalidTarget,
            "".to_string(),
        ))),
    };

    match result {
        Ok(comments) => {
            let len = comments.len() as i64;
            let pagination = match pagination_params.status() {
                PaginationStatus::Active => {
                    let page_size = pagination_params.page_size().unwrap();
                    let page = pagination_params.page().unwrap();
                    Some(Pagination {
                        current_page: *page,
                        page_size: *page_size,
                    })
                }
                _ => None,
            };

            let response = CommentUserResponse::new(comments, len, pagination, Some(metadata));
            log::debug!("Response: {:?}", response);

            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return handle_error(e, response);
        }
    }
}

async fn get_comment_by_id(
    req: HttpRequest,
    query: GetCommentsQuery,
    pool: SqlitePool,
) -> HttpResponse {
    let metadata = ResponseMetadata::new(get_request_id(&req));

    let validated_query = match query.validate() {
        Ok(()) => query,
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return handle_error(e, response);
        }
    };

    let id = match validated_query.target() {
        Ok(QueryTarget::Id) => validated_query.id.unwrap(),
        _ => {
            let e = HandlerError::BadRequest(get_error_message(
                ErrorKey::CommentHandlerGetCommentInvalidTarget,
                "".to_string(),
            ));
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return handle_error(e, response);
        }
    };

    let comment_repo = CommentRepository::new(pool);
    let comment = comment_repo
        .get_comment_by_id(id)
        .await
        .map_err(HandlerError::from);

    match comment {
        Ok(comment) => {
            let response = CommentUserResponse::new(vec![comment], 1, None, Some(metadata));
            log::debug!("Response: {:?}", response);
            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return handle_error(e, response);
        }
    }
}

#[get("/comments")]
pub async fn get_comments(
    req: HttpRequest,
    query: Result<web::Query<GetCommentsQuery>, actix_web::Error>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let query = match query {
        Ok(query) => query.into_inner(),
        Err(e) => {
            let error = HandlerError::BadRequest(get_error_message(
                ErrorKey::CommentHandlerInvalidQuery,
                format!("ActixWebError: {}", e),
            ));
            let response = ErrorResponse::new(error.to_string(), 1, None);
            return handle_error(error, response);
        }
    };

    let target = match query.target() {
        Ok(target) => target,
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, None);
            return handle_error(e, response);
        }
    };

    match target {
        QueryTarget::All => get_comments_with_pagination(req, query, pool.get_ref().clone()).await,
        QueryTarget::Id => get_comment_by_id(req, query, pool.get_ref().clone()).await,
        QueryTarget::TaskId => {
            get_comments_with_pagination(req, query, pool.get_ref().clone()).await
        }
        QueryTarget::UserId => {
            get_comments_with_pagination(req, query, pool.get_ref().clone()).await
        }
    }
}

#[post("/comments")]
pub async fn create_comment(
    req: HttpRequest,
    comment_data: Result<web::Json<Comment>, actix_web::Error>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let metadata = ResponseMetadata::new(get_request_id(&req));

    let comment_data = match comment_data {
        Ok(data) => data,
        Err(e) => {
            let error = HandlerError::BadRequest(get_error_message(
                ErrorKey::CommentHandlerInvalidJsonPost,
                format!("ActixWebError: {}", e),
            ));
            let response = ErrorResponse::new(error.to_string(), 1, Some(metadata));
            return handle_error(error, response);
        }
    };

    let comment_repo = CommentRepository::new(pool.get_ref().clone());
    let comment = comment_repo
        .create_comment(comment_data.into_inner())
        .await
        .map_err(HandlerError::from);

    match comment {
        Ok(comment) => {
            let response = CommentResponse::new(vec![comment], 1, None, Some(metadata));
            log::debug!("Response: {:?}", response);
            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return handle_error(e, response);
        }
    }
}

#[post("/comments/{id}")]
pub async fn update_comment(
    req: HttpRequest,
    comment_data: Result<web::Json<Comment>, actix_web::Error>,
    path: Result<web::Path<i64>, actix_web::Error>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let metadata = ResponseMetadata::new(get_request_id(&req));

    let path = match path {
        Ok(path) => path.into_inner(),
        Err(e) => {
            let error = HandlerError::BadRequest(get_error_message(
                ErrorKey::CommentHandlerInvalidPath,
                format!("ActixWebError: {}", e),
            ));
            let response = ErrorResponse::new(error.to_string(), 1, Some(metadata));
            return handle_error(error, response);
        }
    };

    let comment_data = match comment_data {
        Ok(data) => data,
        Err(e) => {
            let error = HandlerError::BadRequest(get_error_message(
                ErrorKey::CommentHandlerInvalidJsonPost,
                format!("ActixWebError: {}", e),
            ));
            let response = ErrorResponse::new(error.to_string(), 1, Some(metadata));
            return handle_error(error, response);
        }
    };

    if comment_data.comment_id.is_none()
        || (comment_data.comment_id.is_some() && comment_data.comment_id.unwrap() != path)
    {
        let error = HandlerError::BadRequest(get_error_message(
            ErrorKey::CommentHandlerPathAndBodyIdMismatch,
            format!(
                "path_id: {:?}, body_id: {:?}",
                path, comment_data.comment_id
            ),
        ));
        let response = ErrorResponse::new(error.to_string(), 1, Some(metadata));
        return handle_error(error, response);
    }

    let comment_repo = CommentRepository::new(pool.get_ref().clone());
    let comment = comment_repo
        .update_comment(comment_data.into_inner())
        .await
        .map_err(HandlerError::from);

    match comment {
        Ok(comment) => {
            let response = CommentResponse::new(vec![comment], 1, None, Some(metadata));
            log::debug!("Response: {:?}", response);
            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return handle_error(e, response);
        }
    }
}

#[delete("/comments/{id}")]
pub async fn delete_comment(
    req: HttpRequest,
    path: Result<web::Path<i64>, actix_web::Error>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let metadata = ResponseMetadata::new(get_request_id(&req));

    let path = match path {
        Ok(path) => path.into_inner(),
        Err(e) => {
            let error = HandlerError::BadRequest(get_error_message(
                ErrorKey::CommentHandlerInvalidPath,
                format!("ActixWebError: {}", e),
            ));
            let response = ErrorResponse::new(error.to_string(), 1, Some(metadata));
            return handle_error(error, response);
        }
    };

    let comment_repo = CommentRepository::new(pool.get_ref().clone());
    let comment = comment_repo
        .delete_comment(path)
        .await
        .map_err(HandlerError::from);

    match comment {
        Ok(()) => {
            let response = CommentResponse::new(vec![], 0, None, Some(metadata));
            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return handle_error(e, response);
        }
    }
}

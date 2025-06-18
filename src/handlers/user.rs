use crate::errors::handler_errors::HandlerError;
use crate::errors::messages::{ErrorKey, get_error_message};
use crate::handlers::utils::get_request_id;
use crate::handlers::utils::handle_error;
use crate::handlers::utils::hash_password;
use crate::models::PaginationParams;
use crate::models::response_model::ErrorResponse;
use crate::models::response_model::Pagination;
use crate::models::response_model::PaginationStatus;
use crate::models::response_model::ResponseMetadata;
use crate::models::response_model::UserResponse;
use crate::models::{User, UserNoPassword};
use crate::repository::user_repo::*;
use actix_web::{HttpRequest, HttpResponse, Responder, delete, get, post, web};
use serde::Deserialize;
use sqlx::sqlite::SqlitePool;

enum QueryTarget {
    All,
    Name,
    Id,
}

impl QueryTarget {
    fn str_to_enum(s: &str) -> Result<QueryTarget, HandlerError> {
        match s {
            "all" => Ok(QueryTarget::All),
            "name" => Ok(QueryTarget::Name),
            "id" => Ok(QueryTarget::Id),
            _ => Err(HandlerError::BadRequest(get_error_message(
                ErrorKey::UserHandlerGetUsersInvalidTarget,
                format!("target: {:?}", s),
            ))),
        }
    }
}

#[derive(Deserialize, Debug)]
struct GetUsersQuery {
    target: Option<String>,
    page: Option<i32>,
    page_size: Option<i32>,
    name: Option<String>,
    id: Option<i64>,
}

impl GetUsersQuery {
    fn target(&self) -> Result<QueryTarget, HandlerError> {
        match &self.target {
            Some(target) => QueryTarget::str_to_enum(target.as_str()),
            None => {
                log::debug!("No target specified, using default target: All");
                Ok(QueryTarget::All)
            }
        }
    }

    fn validate(&self) -> Result<(), HandlerError> {
        let target = self.target()?;

        match target {
            QueryTarget::All => Ok(()),
            QueryTarget::Name => {
                if self.name.is_none() {
                    return Err(HandlerError::BadRequest(get_error_message(
                        ErrorKey::UserHandlerGetUsersNoNameSpecified,
                        "".to_string(),
                    )));
                }
                Ok(())
            }

            QueryTarget::Id => {
                if self.id.is_none() {
                    return Err(HandlerError::BadRequest(get_error_message(
                        ErrorKey::UserHandlerGetUsersNoIdSpecified,
                        "".to_string(),
                    )));
                }
                Ok(())
            }
        }
    }
}

async fn get_users_with_pagination(
    pagination_params: &PaginationParams,
    pool: SqlitePool,
) -> Result<Vec<UserNoPassword>, HandlerError> {
    let user_repo = UserRepository::new(pool.clone());

    match pagination_params.status() {
        PaginationStatus::Active => {
            // PaginationStatus::Activeの場合は、validate()でpageとpage_sizeがSomeであることが保証されている
            log::debug!(
                "Getting users with pagination: page: {:?}, page_size: {:?}",
                pagination_params.page(),
                pagination_params.page_size()
            );
            user_repo
                .get_users_with_pagination(
                    pagination_params.page().unwrap(),
                    pagination_params.page_size().unwrap(),
                )
                .await
                .map_err(HandlerError::from)
        }
        PaginationStatus::Inactive => {
            log::debug!("Getting all users");
            user_repo.get_all_users().await.map_err(HandlerError::from)
        }
        PaginationStatus::Error => Err(HandlerError::BadRequest(get_error_message(
            ErrorKey::UserHandlerGetUsersInvalidPage,
            format!(
                "page: {:?}, page_size: {:?}",
                pagination_params.page(),
                pagination_params.page_size()
            ),
        ))),
    }
}

async fn get_all_users(req: HttpRequest, query: GetUsersQuery, pool: SqlitePool) -> HttpResponse {
    let metadata = ResponseMetadata::new(get_request_id(&req));

    let mut pagination_params = PaginationParams::new(query.page, query.page_size);
    pagination_params.validate();

    let result = get_users_with_pagination(&pagination_params, pool).await;

    match result {
        Ok(users) => {
            let len = users.len() as i64;
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
            let response = UserResponse::new(users, len, pagination, Some(metadata));
            log::debug!("Response: {:?}", response);
            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return handle_error(e, response);
        }
    }
}

async fn get_user_by_name(
    req: HttpRequest,
    query: GetUsersQuery,
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

    let user_repo = UserRepository::new(pool.clone());
    let user = user_repo
        .get_user_by_name(validated_query.name.clone().unwrap().as_str())
        .await
        .map_err(HandlerError::from);

    match user {
        Ok(user) => {
            let response = UserResponse::new(vec![user], 1, None, Some(metadata));
            log::debug!("Response: {:?}", response);
            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return handle_error(e, response);
        }
    }
}

async fn get_user_by_id(req: HttpRequest, query: GetUsersQuery, pool: SqlitePool) -> HttpResponse {
    let metadata = ResponseMetadata::new(get_request_id(&req));

    let validated_query = match query.validate() {
        Ok(()) => query,
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return HttpResponse::InternalServerError().json(response);
        }
    };

    let user_repo = UserRepository::new(pool.clone());
    let user = user_repo
        .get_user_by_id(validated_query.id.clone().unwrap())
        .await
        .map_err(HandlerError::from);

    match user {
        Ok(user) => {
            let response = UserResponse::new(vec![user], 1, None, Some(metadata));
            log::debug!("Response: {:?}", response);
            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return handle_error(e, response);
        }
    }
}

#[get("/users")]
pub async fn get_users(
    req: HttpRequest,
    query: Result<web::Query<GetUsersQuery>, actix_web::Error>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let query = match query {
        Ok(query) => query.into_inner(),
        Err(e) => {
            let error = HandlerError::BadRequest(get_error_message(
                ErrorKey::UserHandlerInvalidQuery,
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
        QueryTarget::All => get_all_users(req, query, pool.get_ref().clone()).await,
        QueryTarget::Name => get_user_by_name(req, query, pool.get_ref().clone()).await,
        QueryTarget::Id => get_user_by_id(req, query, pool.get_ref().clone()).await,
    }
}

#[post("/users")]
pub async fn create_user(
    req: HttpRequest,
    user_data: Result<web::Json<User>, actix_web::Error>,
    pool: web::Data<SqlitePool>,
) -> HttpResponse {
    let metadata = ResponseMetadata::new(get_request_id(&req));

    let user_data = match user_data {
        Ok(data) => data,
        Err(e) => {
            let error = HandlerError::BadRequest(get_error_message(
                ErrorKey::UserHandlerInvalidJsonPost,
                format!("ActixWebError: {}", e),
            ));
            let response = ErrorResponse::new(error.to_string(), 1, Some(metadata));
            return handle_error(error, response);
        }
    };

    let hashed_password = hash_password(&user_data.password_hash);
    let insert_user = User {
        user_id: None,
        username: user_data.username.clone(),
        email: user_data.email.clone(),
        password_hash: hashed_password,
    };

    let user_repo = UserRepository::new(pool.get_ref().clone());
    let user = user_repo
        .create_user(insert_user)
        .await
        .map_err(HandlerError::from);

    match user {
        Ok(user) => {
            let response = UserResponse::new(vec![user], 1, None, Some(metadata));
            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return handle_error(e, response);
        }
    }
}

#[post("/users/{id}")]
pub async fn update_user(
    req: HttpRequest,
    user_data: Result<web::Json<User>, actix_web::Error>,
    path: Result<web::Path<i64>, actix_web::Error>,
    pool: web::Data<SqlitePool>,
) -> HttpResponse {
    let metadata = ResponseMetadata::new(get_request_id(&req));

    let path = match path {
        Ok(path) => path.into_inner(),
        Err(e) => {
            let error = HandlerError::BadRequest(get_error_message(
                ErrorKey::UserHandlerInvalidPath,
                format!("ActixWebError: {}", e),
            ));
            let response = ErrorResponse::new(error.to_string(), 1, Some(metadata));
            return handle_error(error, response);
        }
    };

    let user_data = match user_data {
        Ok(data) => data,
        Err(e) => {
            let error = HandlerError::BadRequest(get_error_message(
                ErrorKey::UserHandlerInvalidJsonPost,
                format!("ActixWebError: {}", e),
            ));
            let response = ErrorResponse::new(error.to_string(), 1, Some(metadata));
            return handle_error(error, response);
        }
    };

    let path_id = path;

    if user_data.user_id.is_none()
        || (user_data.user_id.is_some() && user_data.user_id.unwrap() != path_id)
    {
        let e = HandlerError::BadRequest(get_error_message(
            ErrorKey::UserHandlerPathAndBodyIdMismatch,
            format!(
                "path.id: {:?}, user_data.id: {:?}",
                path_id, user_data.user_id
            ),
        ));
        let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
        return handle_error(e, response);
    }

    let hashed_password = hash_password(&user_data.password_hash);
    let update_user = User {
        user_id: Some(path_id),
        username: user_data.username.clone(),
        email: user_data.email.clone(),
        password_hash: hashed_password,
    };

    let user_repo = UserRepository::new(pool.get_ref().clone());
    let user = user_repo
        .update_user(update_user)
        .await
        .map_err(HandlerError::from);

    match user {
        Ok(user) => {
            let response = UserResponse::new(vec![user], 1, None, Some(metadata));
            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return handle_error(e, response);
        }
    }
}

#[delete("/users/{id}")]
pub async fn delete_user(
    req: HttpRequest,
    path: Result<web::Path<i64>, actix_web::Error>,
    pool: web::Data<SqlitePool>,
) -> HttpResponse {
    let metadata = ResponseMetadata::new(get_request_id(&req));

    let path = match path {
        Ok(path) => path.into_inner(),
        Err(e) => {
            let error = HandlerError::BadRequest(get_error_message(
                ErrorKey::UserHandlerInvalidPath,
                format!("ActixWebError: {}", e),
            ));
            let response = ErrorResponse::new(error.to_string(), 1, Some(metadata));
            return handle_error(error, response);
        }
    };

    let path_id = path;

    let user_repo = UserRepository::new(pool.get_ref().clone());
    let user = user_repo
        .delete_user(path_id)
        .await
        .map_err(HandlerError::from);

    match user {
        Ok(()) => {
            let response = UserResponse::new(vec![], 0, None, Some(metadata));
            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return handle_error(e, response);
        }
    }
}

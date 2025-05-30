use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use crate::models::response_model::UserResponse;
use crate::models::response_model::ResponseMetadata;
use crate::models::response_model::ErrorResponse;
use crate::models::response_model::Pagination;
use crate::models::response_model::PagenationStatus;
use crate::handlers::utils::get_request_id;
use crate::models::PagenationParams;
use crate::origin_dbpool::get_db_pool;
use crate::repository::user_repo::*;
use crate::errors::handler_errors::HandlerError;
use crate::errors::messages::{get_error_message, ErrorKey};
use crate::models::User;

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
            _ => Err(HandlerError::InvalidRequest(
                get_error_message(ErrorKey::UserHandlerGetUsersInvalidTarget,
                format!("target: {:?}", s)
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
            None => Ok(QueryTarget::All),
        }
    }

    fn validate(&self) -> Result<(), HandlerError> {
        let target = self.target()?;

        match target {
            QueryTarget::All => Ok(()),
            QueryTarget::Name => {
                if self.name.is_none() {
                    return Err(HandlerError::InvalidRequest(
                        get_error_message(ErrorKey::UserHandlerGetUsersNoNameSpecified,
                        format!("target: {:?}", self.target.clone().unwrap())
                    )));
                }
                Ok(())
            }

            QueryTarget::Id => {
                if self.id.is_none() {
                    return Err(HandlerError::InvalidRequest(
                        get_error_message(ErrorKey::UserHandlerGetUsersNoIdSpecified,
                        format!("target: {:?}", self.target.clone().unwrap())
                    )));
                }
                Ok(())
            }
        }
    }
}

async fn get_users_with_pagenation(
    pagenation_params: &PagenationParams
) -> Result<Vec<User>, HandlerError> {
    let user_repo = UserRepository::new(get_db_pool());

    match pagenation_params.status() {
        PagenationStatus::Active => {
            // PagenationStatus::Activeの場合は、validate()でpageとpage_sizeがSomeであることが保証されている
            user_repo.get_users_with_pagenation(
                pagenation_params.page().unwrap(), pagenation_params.page_size().unwrap()
            ).await.map_err(HandlerError::DBAccessError)
        }
        PagenationStatus::Inactive => {
            user_repo.get_all_users().await
                .map_err(HandlerError::DBAccessError)
        }
        PagenationStatus::Error => {
            Err(HandlerError::InvalidRequest(
                get_error_message(ErrorKey::UserHandlerGetUsersInvalidPage,
                format!("page: {:?}, page_size: {:?}", pagenation_params.page(), pagenation_params.page_size())
            )))
        }
    }
}

async fn get_all_users(req: HttpRequest, query: web::Query<GetUsersQuery>) -> HttpResponse {
    let metadata = ResponseMetadata::new(
        get_request_id(&req)
    );

    let mut pagenation_params = PagenationParams::new(query.page, query.page_size);
    pagenation_params.validate();

    let result = get_users_with_pagenation(&pagenation_params).await;

    match result {
        Ok(users) => {
            let len = users.len() as i64;
            let pagenation = match pagenation_params.status() {
                PagenationStatus::Active => {
                    let page_size = pagenation_params.page_size().unwrap();
                    let page = pagenation_params.page().unwrap();
                    Some(Pagination {
                        current_page: *page,
                        page_size: *page_size,
                    })
                }
                _ => None,
            };
            let response = UserResponse::new(users, len, pagenation, Some(metadata));
            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return HttpResponse::InternalServerError().json(response);
        }
    }
}

async fn get_user_by_name(req: HttpRequest, query: web::Query<GetUsersQuery>) -> HttpResponse {
    let metadata = ResponseMetadata::new(
        get_request_id(&req)
    );

    let validated_query = match query.validate() {
        Ok(()) => query,
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return HttpResponse::InternalServerError().json(response);
        }
    };

    let user_repo = UserRepository::new(get_db_pool());
    let user = user_repo.get_user_by_name(
        validated_query.name.clone().unwrap().as_str()
    ).await.map_err(HandlerError::DBAccessError);

    match user {
        Ok(user) => {
            let response = UserResponse::new(vec![user], 1, None, Some(metadata));
            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return HttpResponse::InternalServerError().json(response);
        }
    }
}

async fn get_user_by_id(req: HttpRequest, query: web::Query<GetUsersQuery>) -> HttpResponse {
    let metadata = ResponseMetadata::new(
        get_request_id(&req)
    );

    let validated_query = match query.validate() {
        Ok(()) => query,
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return HttpResponse::InternalServerError().json(response);
        }
    };

    let user_repo = UserRepository::new(get_db_pool());
    let user = user_repo.get_user_by_id(
        validated_query.id.clone().unwrap()
    ).await.map_err(HandlerError::DBAccessError);

    match user {
        Ok(user) => {
            let response = UserResponse::new(vec![user], 1, None, Some(metadata));
            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return HttpResponse::InternalServerError().json(response);
        }
    }
}

#[get("/users")]
pub async fn get_users(req: HttpRequest, query: web::Query<GetUsersQuery>) -> impl Responder {
    let target = match query.target() {
        Ok(target) => target,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ErrorResponse::new(e.to_string(), 1, None));
        }
    };

    match target {
        QueryTarget::All => get_all_users(req, query).await,
        QueryTarget::Name => get_user_by_name(req, query).await,
        QueryTarget::Id => get_user_by_id(req, query).await,
    }
}


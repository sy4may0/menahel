use actix_web::{get, post, delete, web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use crate::models::response_model::UserAssignResponse;
use crate::models::response_model::ResponseMetadata;
use crate::models::response_model::ErrorResponse;
use crate::models::response_model::Pagination;
use crate::models::response_model::PaginationStatus;
use crate::handlers::utils::get_request_id;
use crate::models::PaginationParams;
use crate::repository::user_assign_repo::*;
use crate::errors::handler_errors::HandlerError;
use crate::errors::messages::{get_error_message, ErrorKey};
use crate::models::{UserAssign, UserAssignFilter};
use crate::handlers::utils::handle_error;
use sqlx::sqlite::SqlitePool;

enum QueryTarget {
    All,
    Id,
    Filter,
}

impl QueryTarget {
    fn str_to_enum(s: &str) -> Result<QueryTarget, HandlerError> {
        match s {
            "all" => Ok(QueryTarget::All),
            "id" => Ok(QueryTarget::Id),
            "filter" => Ok(QueryTarget::Filter),
            _ => Err(HandlerError::BadRequest(
                get_error_message(ErrorKey::UserAssignHandlerGetUserAssignsInvalidTarget,
                format!("target: {:?}", s)
            ))),
        }
    }
}

#[derive(Deserialize, Debug)]
struct GetUserAssignsQuery {
    target: Option<String>,
    page: Option<i32>,
    page_size: Option<i32>,
    id: Option<i64>,
    userid: Option<i64>,
    taskid: Option<i64>,
}

impl GetUserAssignsQuery {
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
            QueryTarget::Id => {
                if self.id.is_none() {
                    return Err(HandlerError::BadRequest(
                        get_error_message(ErrorKey::UserAssignHandlerGetUserAssignsNoIdSpecified,
                            "".to_string()
                    )));
                }
                Ok(())
            }
            QueryTarget::Filter => {
                Ok(())
            }
        }
    }

    fn get_user_assign_filter(&self) -> Option<UserAssignFilter> {
        let filter = UserAssignFilter {
            user_id: self.userid,
            task_id: self.taskid,
        };

        match filter.is_empty() {
            true => None,
            false => Some(filter),
        }
    }
}

async fn get_user_assigns_with_pagination(
    pagination_params: &PaginationParams,
    filter: Option<&UserAssignFilter>,
    pool: SqlitePool
) -> Result<Vec<UserAssign>, HandlerError> {
    let user_assign_repo = UserAssignRepository::new(pool.clone());

    match pagination_params.status() {
        PaginationStatus::Active => {
            // PaginationStatus::Activeの場合は、validate()でpageとpage_sizeがSomeであることが保証されている
            log::debug!("Getting user assigns with pagination: page: {:?}, page_size: {:?}", pagination_params.page(), pagination_params.page_size());
            user_assign_repo.get_user_assigns_by_filter(
                filter, pagination_params.page(), pagination_params.page_size()
            ).await.map_err(HandlerError::from)
        }
        PaginationStatus::Inactive => {
            log::debug!("Getting all user assigns");
            user_assign_repo.get_user_assigns_by_filter(
                filter, None, None
            ).await
                .map_err(HandlerError::from)
        }
        PaginationStatus::Error => {
            Err(HandlerError::BadRequest(
                get_error_message(ErrorKey::UserAssignHandlerGetUserAssignsInvalidPage,
                format!("page: {:?}, page_size: {:?}", pagination_params.page(), pagination_params.page_size())
            )))
        }
    }
}

async fn get_all_or_filtered_user_assigns(
    req: HttpRequest, query: GetUserAssignsQuery, pool: SqlitePool
) -> HttpResponse {
    let metadata = ResponseMetadata::new(
        get_request_id(&req)
    );

    let mut pagination_params = PaginationParams::new(query.page, query.page_size);
    pagination_params.validate();

    let result = get_user_assigns_with_pagination(
        &pagination_params, query.get_user_assign_filter().as_ref(), pool
    ).await;

    match result {
        Ok(user_assigns) => {
            let len = user_assigns.len() as i64;
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
            let response = UserAssignResponse::new(user_assigns, len, pagination, Some(metadata));
            log::debug!("Response: {:?}", response);
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return handle_error(e, response);
        }
    }
}

async fn get_user_assign_by_id(req: HttpRequest, query: GetUserAssignsQuery, pool: SqlitePool) -> HttpResponse {
    let metadata = ResponseMetadata::new(
        get_request_id(&req)
    );

    let validated_query = match query.validate() {
        Ok(()) => query,
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return handle_error(e, response);
        }
    };

    let user_assign_repo = UserAssignRepository::new(pool.clone());
    let user_assign = user_assign_repo.get_user_assign_by_id(validated_query.id.unwrap()).await.map_err(HandlerError::from);

    match user_assign {
        Ok(user_assign) => {
            let response = UserAssignResponse::new(vec![user_assign], 1, None, Some(metadata));
            log::debug!("Response: {:?}", response);
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return handle_error(e, response);
        }
    }
}


#[get("/userassigns")]
async fn get_user_assigns(
    req: HttpRequest, 
    query: Result<web::Query<GetUserAssignsQuery>, actix_web::Error>,
    pool: web::Data<SqlitePool>
) -> impl Responder {
    let query = match query {
        Ok(query) => query.into_inner(),
        Err(e) => {
            let error = HandlerError::BadRequest(
                get_error_message(ErrorKey::UserAssignHandlerInvalidQuery,
                format!("ActixWebError: {}", e)
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
        QueryTarget::All => get_all_or_filtered_user_assigns(req, query, pool.get_ref().clone()).await,
        QueryTarget::Id => get_user_assign_by_id(req, query, pool.get_ref().clone()).await,
        QueryTarget::Filter => get_all_or_filtered_user_assigns(req, query, pool.get_ref().clone()).await,
    }
}

#[post("/userassigns")]
async fn create_user_assign(req: HttpRequest, user_assign_data: Result<web::Json<UserAssign>, actix_web::Error>, pool: web::Data<SqlitePool>) -> HttpResponse {
    let metadata = ResponseMetadata::new(
        get_request_id(&req)
    );

    let user_assign_data = match user_assign_data {
        Ok(data) => data,
        Err(e) => {
            let error = HandlerError::BadRequest(
                get_error_message(ErrorKey::UserAssignHandlerInvalidJsonPost,
                format!("ActixWebError: {}", e)
            ));
            let response = ErrorResponse::new(error.to_string(), 1, None);
            return handle_error(error, response);
        }
    };

    let user_assign_repo = UserAssignRepository::new(pool.get_ref().clone());
    let user_assign = user_assign_repo.create_user_assign(user_assign_data.into_inner()).await.map_err(HandlerError::from);

    match user_assign {
        Ok(user_assign) => {
            let response = UserAssignResponse::new(vec![user_assign], 1, None, Some(metadata));
            log::debug!("Response: {:?}", response);
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return handle_error(e, response);
        }
    }
}

#[post("/userassigns/{id}")]
async fn update_user_assign(req: HttpRequest, user_assign_data: Result<web::Json<UserAssign>, actix_web::Error>, path: Result<web::Path<i64>, actix_web::Error>, pool: web::Data<SqlitePool>) -> HttpResponse {
    let metadata = ResponseMetadata::new(
        get_request_id(&req)
    );

    let path = match path {
        Ok(path) => path.into_inner(),
        Err(e) => {
            let error = HandlerError::BadRequest(
                get_error_message(ErrorKey::UserAssignHandlerInvalidPath,
                format!("ActixWebError: {}", e)
            ));
            let response = ErrorResponse::new(error.to_string(), 1, None);
            return handle_error(error, response);
        }
    };

    let user_assign_data = match user_assign_data {
        Ok(data) => data,
        Err(e) => {
            let error = HandlerError::BadRequest(
                get_error_message(ErrorKey::UserAssignHandlerInvalidJsonPost,
                format!("ActixWebError: {}", e)
            ));
            let response = ErrorResponse::new(error.to_string(), 1, None);
            return handle_error(error, response);
        }
    };

    if user_assign_data.user_assign_id.is_none() || (user_assign_data.user_assign_id.is_some() && user_assign_data.user_assign_id.unwrap() != path) {
        let error = HandlerError::BadRequest(
            get_error_message(ErrorKey::UserAssignHandlerPathAndBodyIdMismatch,
            format!("path.id: {:?}, user_assign_data.id: {:?}", path, user_assign_data.user_assign_id)
        ));
        let response = ErrorResponse::new(error.to_string(), 1, None);
        return handle_error(error, response);
    }

    let user_assign_repo = UserAssignRepository::new(pool.get_ref().clone());
    let user_assign = user_assign_repo.update_user_assign(user_assign_data.into_inner()).await.map_err(HandlerError::from);

    match user_assign {
        Ok(user_assign) => {
            let response = UserAssignResponse::new(vec![user_assign], 1, None, Some(metadata));
            log::debug!("Response: {:?}", response);
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return handle_error(e, response);
        }
    }
}

#[delete("/userassigns/{id}")]
async fn delete_user_assign(req: HttpRequest, path: Result<web::Path<i64>, actix_web::Error>, pool: web::Data<SqlitePool>) -> HttpResponse {
    let metadata = ResponseMetadata::new(
        get_request_id(&req)
    );

    let path = match path {
        Ok(path) => path.into_inner(),
        Err(e) => {
            let error = HandlerError::BadRequest(
                get_error_message(ErrorKey::UserAssignHandlerInvalidPath,
                format!("ActixWebError: {}", e)
            ));
            let response = ErrorResponse::new(error.to_string(), 1, None);
            return handle_error(error, response);
        }
    };

    let user_assign_repo = UserAssignRepository::new(pool.get_ref().clone());
    let user_assign = user_assign_repo.delete_user_assign(path).await.map_err(HandlerError::from);

    match user_assign {
        Ok(()) => {
            let response = UserAssignResponse::new(vec![], 0, None, Some(metadata));
            log::debug!("Response: {:?}", response);
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return handle_error(e, response);
        }
    }
}
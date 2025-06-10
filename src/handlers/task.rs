use actix_web::{get, post, delete, web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use crate::models::response_model::TaskResponse;
use crate::models::response_model::ResponseMetadata;
use crate::models::response_model::ErrorResponse;
use crate::models::response_model::Pagination;
use crate::models::response_model::PaginationStatus;
use crate::handlers::utils::get_request_id;
use crate::models::PaginationParams;
use crate::models::repository_model::task::{Task, TaskFilter};
use crate::repository::task_repo::TaskRepository;
use crate::errors::handler_errors::HandlerError;
use crate::errors::messages::{get_error_message, ErrorKey};
use sqlx::sqlite::SqlitePool;
use crate::handlers::utils::handle_error;

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
            _ => Err(HandlerError::BadRequest(get_error_message(ErrorKey::TaskHandlerGetTasksInvalidTarget, s.to_string()))),
        }
    }
}

#[derive(Deserialize, Debug)]
struct GetTasksQuery {
    target: Option<String>,
    page: Option<i32>,
    page_size: Option<i32>,
    id: Option<i64>,
    project_id: Option<i64>,
    parent_id: Option<i64>,
    level: Option<i64>,
    name: Option<String>,
    description: Option<String>,
    status: Option<i64>,
    deadline_from: Option<i64>,
    deadline_to: Option<i64>,
    created_at_from: Option<i64>,
    created_at_to: Option<i64>,
    updated_at_from: Option<i64>,
    updated_at_to: Option<i64>,
}

impl GetTasksQuery {
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
                    return Err(HandlerError::BadRequest(get_error_message(ErrorKey::TaskHandlerGetTasksNoIdSpecified, "".to_string())));
                }
                Ok(())
            }
            QueryTarget::Filter => {
                Ok(())
            }
        }
    }

    fn get_task_filter(&self) -> Option<TaskFilter> {
        let filter = TaskFilter {
            project_id: self.project_id,
            parent_id: self.parent_id,
            level: self.level,
            name: self.name.clone(),
            description: self.description.clone(),
            status: self.status,
            deadline_from: self.deadline_from,
            deadline_to: self.deadline_to,
            created_at_from: self.created_at_from,
            created_at_to: self.created_at_to,
            updated_at_from: self.updated_at_from,
            updated_at_to: self.updated_at_to,
        };

        match filter.is_empty() {
            true => None,
            false => Some(filter),
        }
    }
}

async fn get_tasks_with_pagination (
    pagination_params: &PaginationParams,
    task_filter: Option<&TaskFilter>,
    pool: SqlitePool
) -> Result<Vec<Task>, HandlerError> {
    let task_repo = TaskRepository::new(pool.clone());

    match pagination_params.status() {
        PaginationStatus::Active => {
            task_repo.get_tasks_by_filter(
                task_filter, pagination_params.page(), pagination_params.page_size()
            ).await.map_err(HandlerError::from)
        }
        PaginationStatus::Inactive => {
            task_repo.get_tasks_by_filter(
                task_filter, None, None
            ).await.map_err(HandlerError::from)
        }
        PaginationStatus::Error => {
            Err(HandlerError::BadRequest(get_error_message(ErrorKey::TaskHandlerGetTasksInvalidPage, format!("page: {:?}, page_size: {:?}", pagination_params.page(), pagination_params.page_size()))))
        }
    }
}

// GetTasksQueryにfilterが入っていれば、filterを使ってタスクを取得する
async fn get_all_or_filtered_tasks(
    req: HttpRequest,
    query: GetTasksQuery,
    pool: SqlitePool
) -> HttpResponse {
    let metadata = ResponseMetadata::new(
        get_request_id(&req)
    );

    let mut pagination_params = PaginationParams::new(query.page, query.page_size);
    pagination_params.validate();

    let result = get_tasks_with_pagination(
        &pagination_params, query.get_task_filter().as_ref(), pool
    ).await;

    match result {
        Ok(tasks) => {
            let len = tasks.len() as i64;
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
            let response = TaskResponse::new(tasks, len, pagination, Some(metadata));
            log::debug!("Response: {:?}", response);
            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return handle_error(e, response);
        }
    }
}

async fn get_task_by_id (
    req: HttpRequest,
    query: GetTasksQuery,
    pool: SqlitePool
) -> HttpResponse {
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

    let task_repo = TaskRepository::new(pool.clone());
    let task = task_repo.get_task_by_id(
        validated_query.id.clone().unwrap()
    ).await.map_err(HandlerError::from);

    match task {
        Ok(task) => {
            let response = TaskResponse::new(vec![task], 1, None, Some(metadata));
            log::debug!("Response: {:?}", response);
            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return handle_error(e, response);
        }
    }
}

#[get("/tasks")]
pub async fn get_tasks(
    req: HttpRequest,
    query: Result<web::Query<GetTasksQuery>, actix_web::Error>,
    pool: web::Data<SqlitePool>
) -> impl Responder {
    let query = match query {
        Ok(query) => query.into_inner(),
        Err(e) => {
            let error = HandlerError::BadRequest(
                get_error_message(ErrorKey::TaskHandlerInvalidQuery,
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
        QueryTarget::All => get_all_or_filtered_tasks(req, query, pool.get_ref().clone()).await,
        QueryTarget::Id => get_task_by_id(req, query, pool.get_ref().clone()).await,
        QueryTarget::Filter => get_all_or_filtered_tasks(req, query, pool.get_ref().clone()).await,
    }
}

#[post("/tasks")]
pub async fn create_task(
    req: HttpRequest,
    task_data: Result<web::Json<Task>, actix_web::Error>,
    pool: web::Data<SqlitePool>
) -> HttpResponse {
    let metadata = ResponseMetadata::new(
        get_request_id(&req)
    );

    let task_data = match task_data {
        Ok(data) => data,
        Err(e) => {
            let error = HandlerError::BadRequest(
                get_error_message(ErrorKey::TaskHandlerInvalidJsonPost,
                format!("ActixWebError: {}", e)
            ));
            let response = ErrorResponse::new(error.to_string(), 1, Some(metadata));
            return handle_error(error, response);
        }
    };

    let task_repo = TaskRepository::new(pool.get_ref().clone());
    let task = task_repo.create_task(
        task_data.into_inner()
    ).await.map_err(HandlerError::from);

    match task {
        Ok(task) => {
            let response = TaskResponse::new(vec![task], 1, None, Some(metadata));
            log::debug!("Response: {:?}", response);
            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return handle_error(e, response);
        }
    }
}

#[post("/tasks/{id}")]
pub async fn update_task(
    req: HttpRequest,
    task_data: Result<web::Json<Task>, actix_web::Error>,
    path: Result<web::Path<i64>, actix_web::Error>,
    pool: web::Data<SqlitePool>
) -> HttpResponse {
    let metadata = ResponseMetadata::new(
        get_request_id(&req)
    );

    let path = match path {
        Ok(path) => path.into_inner(),
        Err(e) => {
            let error = HandlerError::BadRequest(
                get_error_message(ErrorKey::TaskHandlerInvalidPath,
                format!("ActixWebError: {}", e)
            ));
            let response = ErrorResponse::new(error.to_string(), 1, Some(metadata));
            return handle_error(error, response);
        }
    };

    let task_data = match task_data {
        Ok(data) => data,
        Err(e) => {
            let error = HandlerError::BadRequest(
                get_error_message(ErrorKey::TaskHandlerInvalidJsonPost,
                format!("ActixWebError: {}", e)
            ));
            let response = ErrorResponse::new(error.to_string(), 1, Some(metadata));
            return handle_error(error, response);
        }
    };

    if task_data.id.is_none() || (task_data.id.is_some() && task_data.id.unwrap() != path) {
        let e = HandlerError::BadRequest(
            get_error_message(ErrorKey::TaskHandlerPathAndBodyIdMismatch,
            format!("path.id: {:?}, task_data.id: {:?}", path, task_data.id)
        ));
        let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
        return handle_error(e, response);
    }

    let task_repo = TaskRepository::new(pool.get_ref().clone());
    let task = task_repo.update_task(
        task_data.into_inner()
    ).await.map_err(HandlerError::from);

    match task {
        Ok(task) => {
            let response = TaskResponse::new(vec![task], 1, None, Some(metadata));
            log::debug!("Response: {:?}", response);
            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return handle_error(e, response);
        }
    }
}

#[delete("/tasks/{id}")]
pub async fn delete_task(
    req: HttpRequest,
    path: Result<web::Path<i64>, actix_web::Error>,
    pool: web::Data<SqlitePool>
) -> HttpResponse {
    let metadata = ResponseMetadata::new(
        get_request_id(&req)
    );

    let path = match path {
        Ok(path) => path.into_inner(),
        Err(e) => {
            let error = HandlerError::BadRequest(
                get_error_message(ErrorKey::TaskHandlerInvalidPath,
                format!("ActixWebError: {}", e)
            ));
            let response = ErrorResponse::new(error.to_string(), 1, Some(metadata));
            return handle_error(error, response);
        }
    };

    let task_repo = TaskRepository::new(pool.get_ref().clone());
    let task = task_repo.delete_task(path).await.map_err(HandlerError::from);

    match task {
        Ok(()) => {
            let response = TaskResponse::new(vec![], 0, None, Some(metadata));
            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return handle_error(e, response);
        }
    }
}
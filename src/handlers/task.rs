use crate::errors::handler_errors::HandlerError;
use crate::errors::messages::{ErrorKey, get_error_message};
use crate::handlers::utils::get_request_id;
use crate::handlers::utils::handle_error;
use crate::models::PaginationParams;
use crate::models::TaskUserResponse;
use crate::models::TaskWithUser;
use crate::models::repository_model::task::{Task, TaskFilter};
use crate::models::response_model::ErrorResponse;
use crate::models::response_model::Pagination;
use crate::models::response_model::PaginationStatus;
use crate::models::response_model::ResponseMetadata;
use crate::models::response_model::TaskResponse;
use crate::repository::task_repo::TaskRepository;
use crate::repository::task_user_repo::TaskUserRepository;
use actix_web::{HttpRequest, HttpResponse, Responder, delete, get, post, web};
use serde::Deserialize;
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
            _ => Err(HandlerError::BadRequest(get_error_message(
                ErrorKey::TaskHandlerGetTasksInvalidTarget,
                s.to_string(),
            ))),
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
    assignee_id: Option<i64>,
    with_user: Option<bool>,
    user_ids: Option<String>,
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
                    return Err(HandlerError::BadRequest(get_error_message(
                        ErrorKey::TaskHandlerGetTasksNoIdSpecified,
                        "".to_string(),
                    )));
                }
                Ok(())
            }
            QueryTarget::Filter => Ok(()),
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
            assignee_id: self.assignee_id,
        };

        match filter.is_empty() {
            true => None,
            false => Some(filter),
        }
    }

    fn get_user_ids(&self) -> Result<Option<Vec<i64>>, HandlerError> {
        match self.user_ids.as_ref() {
            Some(user_ids) => {
                let user_ids_string: Vec<String> =
                    user_ids.split(",").map(|id| id.to_string()).collect();
                let user_ids: Result<Vec<i64>, HandlerError> = user_ids_string
                    .iter()
                    .map(|id| {
                        id.parse::<i64>().map_err(|e| {
                            HandlerError::BadRequest(get_error_message(
                                ErrorKey::TaskHandlerGetUserIdsParseFailed,
                                e.to_string(),
                            ))
                        })
                    })
                    .collect();
                match user_ids {
                    Ok(ids) => Ok(Some(ids)),
                    Err(e) => Err(e),
                }
            }
            None => Ok(None),
        }
    }
}

async fn get_tasks_with_pagination(
    pagination_params: &PaginationParams,
    task_filter: Option<&TaskFilter>,
    pool: SqlitePool,
) -> Result<Vec<Task>, HandlerError> {
    let task_repo = TaskRepository::new(pool.clone());

    match pagination_params.status() {
        PaginationStatus::Active => task_repo
            .get_tasks_by_filter(
                task_filter,
                pagination_params.page(),
                pagination_params.page_size(),
            )
            .await
            .map_err(HandlerError::from),
        PaginationStatus::Inactive => task_repo
            .get_tasks_by_filter(task_filter, None, None)
            .await
            .map_err(HandlerError::from),
        PaginationStatus::Error => Err(HandlerError::BadRequest(get_error_message(
            ErrorKey::TaskHandlerGetTasksInvalidPage,
            format!(
                "page: {:?}, page_size: {:?}",
                pagination_params.page(),
                pagination_params.page_size()
            ),
        ))),
    }
}

async fn get_tasks_with_user_pagination(
    pagination_params: &PaginationParams,
    task_filter: Option<&TaskFilter>,
    pool: SqlitePool,
    user_ids: Option<&Vec<i64>>,
) -> Result<Vec<TaskWithUser>, HandlerError> {
    let task_user_repo = TaskUserRepository::new(pool.clone());

    match pagination_params.status() {
        PaginationStatus::Active => task_user_repo
            .get_tasks_and_users_by_filter(
                pagination_params.page(),
                pagination_params.page_size(),
                task_filter,
                user_ids,
            )
            .await
            .map_err(HandlerError::from),
        PaginationStatus::Inactive => task_user_repo
            .get_tasks_and_users_by_filter(None, None, task_filter, user_ids)
            .await
            .map_err(HandlerError::from),
        PaginationStatus::Error => Err(HandlerError::BadRequest(get_error_message(
            ErrorKey::TaskHandlerGetTasksInvalidPage,
            format!(
                "page: {:?}, page_size: {:?}",
                pagination_params.page(),
                pagination_params.page_size()
            ),
        ))),
    }
}

// GetTasksQueryにfilterが入っていれば、filterを使ってタスクを取得する
async fn get_all_or_filtered_tasks(
    req: HttpRequest,
    query: GetTasksQuery,
    pool: SqlitePool,
    with_user: &bool,
) -> HttpResponse {
    let metadata = ResponseMetadata::new(get_request_id(&req));

    let mut pagination_params = PaginationParams::new(query.page, query.page_size);
    pagination_params.validate();
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

    let user_ids = match query.get_user_ids() {
        Ok(ids) => ids,
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return handle_error(e, response);
        }
    };

    if !*with_user {
        let result =
            get_tasks_with_pagination(&pagination_params, query.get_task_filter().as_ref(), pool)
                .await;

        match result {
            Ok(tasks) => {
                let len = tasks.len() as i64;
                let response = TaskResponse::new(tasks, len, pagination, Some(metadata));
                log::debug!("Response: {:?}", response);
                return HttpResponse::Ok().json(response);
            }
            Err(e) => {
                let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
                return handle_error(e, response);
            }
        }
    } else {
        let result = get_tasks_with_user_pagination(
            &pagination_params,
            query.get_task_filter().as_ref(),
            pool,
            user_ids.as_ref(),
        )
        .await;

        match result {
            Ok(tasks) => {
                let len = tasks.len() as i64;
                let response = TaskUserResponse::new(tasks, len, pagination, Some(metadata));
                log::debug!("Response: {:?}", response);
                return HttpResponse::Ok().json(response);
            }
            Err(e) => {
                let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
                return handle_error(e, response);
            }
        }
    }
}

async fn get_task_or_task_with_user(
    id: i64,
    pool: SqlitePool,
    with_user: &bool,
    metadata: ResponseMetadata,
) -> HttpResponse {
    match !*with_user {
        true => {
            let task_repo = TaskRepository::new(pool.clone());
            let task = task_repo
                .get_task_by_id(id)
                .await
                .map_err(HandlerError::from);

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
        false => {
            let task_user_repo = TaskUserRepository::new(pool.clone());
            let task = task_user_repo
                .get_task_by_id_with_user(id)
                .await
                .map_err(HandlerError::from);

            match task {
                Ok(task) => {
                    let response = TaskUserResponse::new(vec![task], 1, None, Some(metadata));
                    log::debug!("Response: {:?}", response);
                    return HttpResponse::Ok().json(response);
                }
                Err(e) => {
                    let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
                    return handle_error(e, response);
                }
            }
        }
    }
}

async fn get_task_by_id(
    req: HttpRequest,
    query: GetTasksQuery,
    pool: SqlitePool,
    with_user: &bool,
) -> HttpResponse {
    let metadata = ResponseMetadata::new(get_request_id(&req));

    let validated_query = match query.validate() {
        Ok(()) => query,
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return HttpResponse::InternalServerError().json(response);
        }
    };

    get_task_or_task_with_user(
        validated_query.id.clone().unwrap(),
        pool.clone(),
        &with_user,
        metadata,
    )
    .await
}

#[get("/tasks")]
pub async fn get_tasks(
    req: HttpRequest,
    query: Result<web::Query<GetTasksQuery>, actix_web::Error>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let query = match query {
        Ok(query) => query.into_inner(),
        Err(e) => {
            let error = HandlerError::BadRequest(get_error_message(
                ErrorKey::TaskHandlerInvalidQuery,
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

    let with_user = query.with_user.unwrap_or(false);

    match target {
        QueryTarget::All => {
            get_all_or_filtered_tasks(req, query, pool.get_ref().clone(), &with_user).await
        }
        QueryTarget::Id => get_task_by_id(req, query, pool.get_ref().clone(), &with_user).await,
        QueryTarget::Filter => {
            get_all_or_filtered_tasks(req, query, pool.get_ref().clone(), &with_user).await
        }
    }
}

#[post("/tasks")]
pub async fn create_task(
    req: HttpRequest,
    task_data: Result<web::Json<Task>, actix_web::Error>,
    pool: web::Data<SqlitePool>,
) -> HttpResponse {
    let metadata = ResponseMetadata::new(get_request_id(&req));

    let task_data = match task_data {
        Ok(data) => data,
        Err(e) => {
            let error = HandlerError::BadRequest(get_error_message(
                ErrorKey::TaskHandlerInvalidJsonPost,
                format!("ActixWebError: {}", e),
            ));
            let response = ErrorResponse::new(error.to_string(), 1, Some(metadata));
            return handle_error(error, response);
        }
    };

    let task_repo = TaskRepository::new(pool.get_ref().clone());
    let task = task_repo
        .create_task(task_data.into_inner())
        .await
        .map_err(HandlerError::from);

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
    pool: web::Data<SqlitePool>,
) -> HttpResponse {
    let metadata = ResponseMetadata::new(get_request_id(&req));

    let path = match path {
        Ok(path) => path.into_inner(),
        Err(e) => {
            let error = HandlerError::BadRequest(get_error_message(
                ErrorKey::TaskHandlerInvalidPath,
                format!("ActixWebError: {}", e),
            ));
            let response = ErrorResponse::new(error.to_string(), 1, Some(metadata));
            return handle_error(error, response);
        }
    };

    let task_data = match task_data {
        Ok(data) => data,
        Err(e) => {
            let error = HandlerError::BadRequest(get_error_message(
                ErrorKey::TaskHandlerInvalidJsonPost,
                format!("ActixWebError: {}", e),
            ));
            let response = ErrorResponse::new(error.to_string(), 1, Some(metadata));
            return handle_error(error, response);
        }
    };

    if task_data.task_id.is_none()
        || (task_data.task_id.is_some() && task_data.task_id.unwrap() != path)
    {
        let e = HandlerError::BadRequest(get_error_message(
            ErrorKey::TaskHandlerPathAndBodyIdMismatch,
            format!("path.id: {:?}, task_data.id: {:?}", path, task_data.task_id),
        ));
        let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
        return handle_error(e, response);
    }

    let task_repo = TaskRepository::new(pool.get_ref().clone());
    let task = task_repo
        .update_task(task_data.into_inner())
        .await
        .map_err(HandlerError::from);

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
    pool: web::Data<SqlitePool>,
) -> HttpResponse {
    let metadata = ResponseMetadata::new(get_request_id(&req));

    let path = match path {
        Ok(path) => path.into_inner(),
        Err(e) => {
            let error = HandlerError::BadRequest(get_error_message(
                ErrorKey::TaskHandlerInvalidPath,
                format!("ActixWebError: {}", e),
            ));
            let response = ErrorResponse::new(error.to_string(), 1, Some(metadata));
            return handle_error(error, response);
        }
    };

    let task_repo = TaskRepository::new(pool.get_ref().clone());
    let task = task_repo
        .delete_task(path)
        .await
        .map_err(HandlerError::from);

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

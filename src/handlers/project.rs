use serde::Deserialize;
use crate::errors::messages::ErrorKey;
use crate::errors::handler_errors::HandlerError;
use crate::errors::messages::get_error_message;
use crate::models::project::Project;
use crate::models::response_model::Pagination;
use crate::models::response_model::PagenationStatus;
use crate::models::PagenationParams;
use crate::repository::project_repo::ProjectRepository;
use crate::models::response_model::ResponseMetadata;
use crate::models::response_model::ProjectResponse;
use crate::models::response_model::ErrorResponse;
use crate::handlers::utils::get_request_id;
use crate::handlers::utils::handle_error;
use actix_web::{web, HttpResponse, HttpRequest, Responder, get, post};
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
            _ => Err(HandlerError::BadRequest(
                get_error_message(ErrorKey::ProjectHandlerGetProjectsInvalidTarget,
                format!("target: {}", s)
            ))),
        }
    }
}

#[derive(Deserialize)]
struct GetProjectsQuery {
    target: Option<String>,
    page: Option<i32>,
    page_size: Option<i32>,
    name: Option<String>,
    id: Option<i64>,
}

impl GetProjectsQuery {
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
            QueryTarget::Name => {
                if self.name.is_none() {
                    return Err(HandlerError::BadRequest(
                        get_error_message(ErrorKey::ProjectHandlerGetProjectsNoNameSpecified,
                            "".to_string()
                    )));
                }
                Ok(())
            }
            QueryTarget::Id => {
                if self.id.is_none() {
                    return Err(HandlerError::BadRequest(
                        get_error_message(ErrorKey::ProjectHandlerGetProjectsNoIdSpecified,
                            "".to_string()
                    )));
                }
                Ok(())
            }
        }
    }
}

async fn get_projects_with_pagenation(
    pagenation_params: &PagenationParams,
    pool: SqlitePool
) -> Result<Vec<Project>, HandlerError> {
    let project_repo = ProjectRepository::new(pool);

    match pagenation_params.status() {
        PagenationStatus::Active => {
            log::debug!("Getting projects with pagenation: page: {:?}, page_size: {:?}", pagenation_params.page(), pagenation_params.page_size());
            project_repo.get_projects_with_pagenation(
                pagenation_params.page().unwrap(), pagenation_params.page_size().unwrap()
            ).await.map_err(HandlerError::from)
        }
        PagenationStatus::Inactive => {
            log::debug!("Getting all projects");
            project_repo.get_all_projects().await.map_err(HandlerError::from)
        }
        PagenationStatus::Error => {
            Err(HandlerError::BadRequest(
                get_error_message(ErrorKey::ProjectHandlerGetProjectsInvalidPage,
                    format!("page: {:?}, page_size: {:?}", pagenation_params.page(), pagenation_params.page_size())
            )))
        }
    }
}

async fn get_all_projects(
    req: HttpRequest, query: web::Query<GetProjectsQuery>, pool: SqlitePool
) -> HttpResponse {
    let metadata = ResponseMetadata::new(
        get_request_id(&req),
    );

    let mut pagenation_params = PagenationParams::new(
        query.page, query.page_size
    );
    pagenation_params.validate();

    let result = get_projects_with_pagenation(
        &pagenation_params, pool
    ).await;

    match result {
        Ok(projects) => {
            let len = projects.len() as i64;
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
            let response = ProjectResponse::new(projects, len, pagenation, Some(metadata));
            log::debug!("Response: {:?}", response);
            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return handle_error(e, response);
        }
    }
}

async fn get_project_by_name(
    req: HttpRequest, query: web::Query<GetProjectsQuery>, pool: SqlitePool
) -> HttpResponse {
    let metadata = ResponseMetadata::new(
        get_request_id(&req),
    );

    let validated_query = match query.validate() {
        Ok(()) => query,
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return handle_error(e, response);
        }
    };

    let project_repo = ProjectRepository::new(pool);
    let project = project_repo.get_project_by_name(
        validated_query.name.clone().unwrap().as_str()
    ).await.map_err(HandlerError::from);

    match project {
        Ok(project) => {
            let response = ProjectResponse::new(vec![project], 1, None, Some(metadata));
            log::debug!("Response: {:?}", response);
            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return handle_error(e, response);
        }
    }
}

async fn get_project_by_id(
    req: HttpRequest, query: web::Query<GetProjectsQuery>, pool: SqlitePool
) -> HttpResponse {
    let metadata = ResponseMetadata::new(
        get_request_id(&req),
    );

    let validated_query = match query.validate() {
        Ok(()) => query,
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return handle_error(e, response);
        }
    };

    let project_repo = ProjectRepository::new(pool);
    let project = project_repo.get_project_by_id(
        validated_query.id.clone().unwrap()
    ).await.map_err(HandlerError::from);

    match project {
        Ok(project) => {
            let response = ProjectResponse::new(vec![project], 1, None, Some(metadata));
            log::debug!("Response: {:?}", response);
            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return handle_error(e, response);
        }
    }
}

#[get("/projects")]
pub async fn get_projects(
    req: HttpRequest, query: web::Query<GetProjectsQuery>, pool: web::Data<SqlitePool>
) -> impl Responder {
    let target = match query.target() {
        Ok(target) => target,
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, None);
            return handle_error(e, response);
        }
    };

    match target {
        QueryTarget::All => get_all_projects(req, query, pool.get_ref().clone()).await,
        QueryTarget::Name => get_project_by_name(req, query, pool.get_ref().clone()).await,
        QueryTarget::Id => get_project_by_id(req, query, pool.get_ref().clone()).await,
    }
}

#[post("/projects")]
pub async fn create_project(
    req: HttpRequest, project_data: Result<web::Json<Project>, actix_web::Error>, pool: web::Data<SqlitePool>
) -> impl Responder {
    let metadata = ResponseMetadata::new(
        get_request_id(&req),
    );

    let project_data = match project_data {
        Ok(data) => data,
        Err(e) => {
            let error = HandlerError::BadRequest(
                get_error_message(ErrorKey::ProjectHandlerInvalidJsonPost, format!("ActixWebError: {}", e))
            );
            let response = ErrorResponse::new(error.to_string(), 1, Some(metadata));
            return handle_error(error, response);
        }
    };

    let project_repo = ProjectRepository::new(pool.get_ref().clone());
    let project = project_repo.create_project(
        project_data.into_inner()
    ).await.map_err(HandlerError::from);

    match project {
        Ok(project) => {
            let response = ProjectResponse::new(vec![project], 1, None, Some(metadata));
            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return handle_error(e, response);
        }
    }
}

#[post("/projects/{id}")]
pub async fn update_project(
    req: HttpRequest, project_data: Result<web::Json<Project>, actix_web::Error>, path: web::Path<i64>, pool: web::Data<SqlitePool>
) -> HttpResponse {
    let metadata = ResponseMetadata::new(
        get_request_id(&req),
    );

    let project_data = match project_data {
        Ok(data) => data,
        Err(e) => {
            let error = HandlerError::BadRequest(
                get_error_message(ErrorKey::ProjectHandlerInvalidJsonPost, format!("ActixWebError: {}", e))
            );
            let response = ErrorResponse::new(error.to_string(), 1, Some(metadata));
            return handle_error(error, response);
        }
    };

    let path_id = path.into_inner();

    if project_data.id.is_none() || (project_data.id.is_some() && project_data.id.unwrap() != path_id) {
        let error = HandlerError::BadRequest(
            get_error_message(ErrorKey::ProjectHandlerPathAndBodyIdMismatch, 
            format!("path_id: {:?}, body_id: {:?}", path_id, project_data.id))
        );
        let response = ErrorResponse::new(error.to_string(), 1, Some(metadata));
        return handle_error(error, response);
    }

    let project_repo = ProjectRepository::new(pool.get_ref().clone());
    let project = project_repo.update_project(
       project_data.into_inner()
    ).await.map_err(HandlerError::from);

    match project {
        Ok(project) => {
            let response = ProjectResponse::new(vec![project], 1, None, Some(metadata));
            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            let response = ErrorResponse::new(e.to_string(), 1, Some(metadata));
            return handle_error(e, response);
        }
    }
} 

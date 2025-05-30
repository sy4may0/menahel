use actix_web::{get, Responder, web, HttpRequest};
use crate::models::response_model::{RootResponse, ResponseMetadata};
use crate::constants::API_VERSION;
use crate::handlers::utils::get_request_id;

#[get("/")]
async fn root(req: HttpRequest) -> impl Responder {
    let request_id = get_request_id(&req);

    let metadata = ResponseMetadata {
        request_id: request_id.clone(),
        api_version: API_VERSION.to_string(),
    };

    let response = RootResponse::new(
        "I AM MENAHEL!".to_string(), 0, Some(metadata)
    );
    web::Json(response)
        .customize()
        .append_header(("X-Request-ID", request_id))
}

#[get("/health")]
async fn health(req: HttpRequest) -> impl Responder {
    let request_id = get_request_id(&req);

    let metadata = ResponseMetadata {
        request_id: request_id.clone(),
        api_version: API_VERSION.to_string(),
    };
    let response = RootResponse::new(
        "MENAHEL IS RUNNING!".to_string(), 0, Some(metadata)
    );
    web::Json(response)
        .customize()
        .append_header(("X-Request-ID", request_id))
}
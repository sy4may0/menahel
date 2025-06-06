use actix_web::HttpRequest;
use uuid::Uuid;
use crate::errors::handler_errors::HandlerError;
use crate::models::ErrorResponse;
use actix_web::HttpResponse;
use sha2::{Sha256, Digest};


pub fn get_request_id(req: &HttpRequest) -> String {
    // クライアントから送信されたリクエストIDを取得
    req.headers()
        .get("X-Request-ID")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string())
}

pub fn handle_error(e: HandlerError, response: ErrorResponse) -> HttpResponse {
    log::error!("Error: {:?}", e);
    match e {
        HandlerError::NotFound(_) => HttpResponse::NotFound().json(response),
        HandlerError::InternalServerError(_) => HttpResponse::InternalServerError().json(response),
        HandlerError::BadRequest(_) => HttpResponse::BadRequest().json(response),
    }
}

pub fn hash_password(password: &str) -> String {
    let hash = Sha256::digest(password.as_bytes());
    return format!("{:x}", hash);
}
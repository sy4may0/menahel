use actix_web::HttpRequest;
use uuid::Uuid;

pub fn get_request_id(req: &HttpRequest) -> String {
    // クライアントから送信されたリクエストIDを取得
    req.headers()
        .get("X-Request-ID")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string())
}
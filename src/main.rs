use actix_web::{App, HttpServer};
use menahel::handlers::root::{root, health};
use menahel::handlers::user::get_users;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(root)
            .service(health)
            .service(get_users)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
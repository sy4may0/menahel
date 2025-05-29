use actix_web::{App, HttpServer};
use menahel::handlers::root::{root, health};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(root)
            .service(health)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
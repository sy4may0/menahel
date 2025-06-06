use actix_web::{App, HttpServer, web};
use menahel::handlers::root::{root, health};
use menahel::handlers::user::get_users;
use menahel::init_logger;
use sqlx::sqlite::SqlitePoolOptions;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();
    log::info!("Starting server");

    println!("DATABASE_URL: {}", std::env::var("DATABASE_URL").unwrap());

    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect(std::env::var("DATABASE_URL").unwrap().as_str())
        .await
        .unwrap();
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(root)
            .service(health)
            .service(get_users)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
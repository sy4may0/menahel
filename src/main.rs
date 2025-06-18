use actix_web::{App, HttpServer, web};
use menahel::handlers::root::{health, root};
use menahel::handlers::user::{
    get_users,
    create_user,
    update_user,
    delete_user,
};
use menahel::handlers::project::{
    get_projects,
    create_project,
    update_project,
    delete_project,
};
use menahel::handlers::task::{
    get_tasks,
    create_task,
    update_task,
    delete_task,
};
use menahel::handlers::user_assign::{
    get_user_assigns,
    create_user_assign,
    update_user_assign,
    delete_user_assign,
};
use menahel::handlers::comment::{
    get_comments,
    create_comment,
    update_comment,
    delete_comment,
};
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
            .service(create_user)
            .service(update_user)
            .service(delete_user)
            .service(get_projects)
            .service(create_project)
            .service(update_project)
            .service(delete_project)
            .service(get_tasks)
            .service(create_task)
            .service(update_task)
            .service(delete_task)
            .service(get_user_assigns)
            .service(create_user_assign)
            .service(update_user_assign)
            .service(delete_user_assign)
            .service(get_comments)
            .service(create_comment)
            .service(update_comment)
            .service(delete_comment)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}

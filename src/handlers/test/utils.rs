use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::Sqlite;

pub async fn setup_test_db(dir: &str, name: &str) -> SqlitePool {
    let test_db_url = format!("./test_db/{}/{}.db", dir, name);

    Sqlite::create_database(&test_db_url).await.unwrap();

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&test_db_url)
        .await
        .unwrap();

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .unwrap();

    sqlx::query_file!("./fixtures/test_db.sql")
        .execute(&pool)
        .await
        .unwrap();

    pool
}
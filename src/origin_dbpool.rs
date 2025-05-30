use once_cell::sync::Lazy;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::sqlite::SqlitePool;
use tokio::runtime::Runtime;
use std::sync::Mutex;

pub static ORIGIN_DB_POOL: Lazy<SqlitePool> = Lazy::new(|| {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        SqlitePoolOptions::new()
            .max_connections(10)
            .connect(std::env::var("DATABASE_URL").unwrap().as_str())
            .await
            .unwrap()
    })
});

static TEST_DB_POOL: Lazy<Mutex<Option<SqlitePool>>> = Lazy::new(|| Mutex::new(None));

pub fn set_test_db_pool(pool: SqlitePool) {
    let mut test_pool = TEST_DB_POOL.lock().unwrap();
    *test_pool = Some(pool);
}

pub fn get_db_pool() -> SqlitePool {
    if cfg!(test) {
        TEST_DB_POOL.lock().unwrap().as_ref().unwrap().clone()
    } else {
        ORIGIN_DB_POOL.clone()
    }
}
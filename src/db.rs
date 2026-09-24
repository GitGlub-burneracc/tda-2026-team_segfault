use sqlx::sqlite::SqlitePoolOptions;
use sqlx::sqlite::SqlitePool;

pub async fn init_db() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:app.db?mode=rwc")
        .await
        .expect("Failed to create pool");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS team (
            name TEXT NOT NULL,
            contestants TEXT NOT NULL
        )",
    )
    .execute(&pool)
    .await
    .expect("Failed to create table");

    pool
}

pub async fn seed_db(pool: &SqlitePool) {
    sqlx::query(
    "INSERT INTO team (name, contestants) VALUES ('Team Segfault', 4)"
    )
    .execute(pool)
    .await
    .expect("Failed to seed db");
}
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::sqlite::SqlitePool;

pub async fn init_db() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:app.db?mode=rwc")
        .await
        .expect("Failed to create pool");

    sqlx::query("DROP TABLE IF EXISTS stops")
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query(
        "CREATE TABLE stops (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL CHECK (length(name) <= 255),
        image_url TEXT CHECK (length(image_url) <= 255),
        is_transfer BOOLEAN NOT NULL,
        x REAL NOT NULL,
        y REAL NOT NULL,
        wheelchair_accessible BOOLEAN NOT NULL,
        has_shelter BOOLEAN NOT NULL,
        has_bench BOOLEAN NOT NULL,
        has_ticket_machine BOOLEAN NOT NULL,
        has_display BOOLEAN NOT NULL
    )",
    )
    .execute(&pool)
    .await
    .expect("Failed to create table");

    pool
}

pub async fn seed_db(pool: &SqlitePool) {
    sqlx::query(
    "INSERT INTO stops (
        id,
        name,
        image_url,
        is_transfer,
        x,
        y,
        wheelchair_accessible,
        has_shelter,
        has_bench,
        has_ticket_machine,
        has_display
    ) VALUES (
        1,
        'Hlavní nádraží',
        'https://example.com/stops/main-station.jpg',
        1,
        420.5,
        280.25,
        1,
        1,
        1,
        1,
        1
    );"
    )
    .execute(pool)
    .await
    .expect("Failed to seed db");
}
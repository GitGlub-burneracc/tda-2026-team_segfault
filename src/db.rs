use sqlx::sqlite::SqlitePoolOptions;
use sqlx::sqlite::SqlitePool;
use axum::{
    Json,
};
use serde::{Deserialize, Serialize};
use crate::shared;

#[derive(Deserialize, sqlx::FromRow, Serialize)]
pub struct Stop {
    pub id: i32,
    pub name: String,
    pub lines: String,
    pub is_transfer: bool,
    pub transfer_lines: Option<String>,
    pub x: f64,
    pub y: f64,
    pub wheelchair_accessible: bool,
    pub has_shelter: bool,
    pub has_bench: bool,
    pub has_ticket_machine: bool,
    pub has_display: bool,
    pub image_url: Option<String>,
}

#[derive(sqlx::FromRow, Serialize)]
pub struct StopResponse {
    pub id: i32,
    pub name: String,
    pub image_url: Option<String>,
    pub wheelchair_accessible: bool,
    pub has_shelter: bool,
    pub has_ticket_machine: bool,
}

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
            lines TEXT NOT NULL,
            is_transfer BOOLEAN NOT NULL,
            transfer_lines TEXT,
            x REAL NOT NULL,
            y REAL NOT NULL,
            wheelchair_accessible BOOLEAN NOT NULL,
            has_shelter BOOLEAN NOT NULL,
            has_bench BOOLEAN NOT NULL,
            has_ticket_machine BOOLEAN NOT NULL,
            has_display BOOLEAN NOT NULL,
            image_url TEXT CHECK (length(image_url) <= 255)
        )",
    )
    .execute(&pool)
    .await
    .expect("Failed to create table");

    pool
}

pub async fn seed_db(pool: &SqlitePool) {
    let mut reader = csv::Reader::from_path("assets/stops.csv").unwrap();

    for result in reader.deserialize() {
        let stop: Stop = result.unwrap();

        let image_url = format!(
            "assets/stopsImages/{}.png",
            shared::camel_case(&stop.name)
        );

        sqlx::query(
            "INSERT INTO stops (
                id,
                name,
                lines,
                is_transfer,
                transfer_lines,
                x,
                y,
                wheelchair_accessible,
                has_shelter,
                has_bench,
                has_ticket_machine,
                has_display,
                image_url
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(stop.id)
        .bind(&stop.name)
        .bind(&stop.lines)
        .bind(stop.is_transfer)
        .bind(&stop.transfer_lines)
        .bind(stop.x)
        .bind(stop.y)
        .bind(stop.wheelchair_accessible)
        .bind(stop.has_shelter)
        .bind(stop.has_bench)
        .bind(stop.has_ticket_machine)
        .bind(stop.has_display)
        .bind(Some(image_url))
        .execute(pool)
        .await
        .unwrap();
    }
}


pub async fn get_stops(pool: &SqlitePool) -> Json<Vec<StopResponse>> {
    let stops = sqlx::query_as::<_, StopResponse>(
        "SELECT id, name, image_url, wheelchair_accessible, has_shelter, has_ticket_machine FROM stops"
    )
    .fetch_all(pool)
    .await
    .unwrap();

    Json(stops)
}
pub async fn get_single_stop(pool: &SqlitePool, id: u32) -> Json<StopResponse> {
    let stops = sqlx::query_as::<_, StopResponse>(
        "SELECT id, name, image_url, wheelchair_accessible, has_shelter, has_ticket_machine FROM stops WHERE id = ?"
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .unwrap();

    Json(stops)
}
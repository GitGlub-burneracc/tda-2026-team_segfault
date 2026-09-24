use axum::{
    body::Body,
    response::{Html, Response},
    routing::get,
    Json, Router,
};
use serde;
use axum::extract::State;
use sqlx::sqlite::SqlitePool;

#[derive(serde::Serialize, sqlx::FromRow)]
pub struct Team {
    name: String,
    contestants: String
}

pub async fn teamdb(State(pool): State<SqlitePool>,) -> Json<Vec<Team>> {
    let teams = sqlx::query_as::<_, Team>(
        "SELECT name, contestants FROM team"
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    Json(teams)
}
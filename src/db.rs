use axum::http::StatusCode;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::sqlite::SqlitePool;
use axum::{
    Json,
};
use axum::extract::rejection::JsonRejection;
use serde::{Deserialize, Serialize};
use crate::shared;


#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct ErrorResponse {
    pub e: String
}


#[derive(Deserialize, sqlx::FromRow, Serialize)]
pub struct Stop {
    pub id: u32,
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

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct StopResponse {
    #[serde(default)]
    pub id: u32,
    pub name: String,
    pub image_url: Option<String>,
    pub wheelchair_accessible: bool,
    pub has_shelter: bool,
    pub has_ticket_machine: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StopInput {
    pub name: String,
    pub image_url: Option<String>,
    pub wheelchair_accessible: bool,
    pub has_shelter: bool,
    pub has_ticket_machine: bool,
}

impl StopInput {
    fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("name must not be empty".to_string());
        }

        if self.name.len() > 255 {
            return Err("name must not exceed 255 characters".to_string());
        }

        if let Some(image_url) = &self.image_url {
            if image_url.len() > 255 {
                return Err("image_url must not exceed 255 characters".to_string());
            }

            if !image_url.starts_with('/') {
                return Err("image_url must be a valid relative URL".to_string());
            }
        }

        Ok(())
    }
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
            lines TEXT,
            is_transfer BOOLEAN,
            transfer_lines TEXT,
            x REAL,
            y REAL,
            wheelchair_accessible BOOLEAN NOT NULL,
            has_shelter BOOLEAN NOT NULL,
            has_bench BOOLEAN,
            has_ticket_machine BOOLEAN NOT NULL,
            has_display BOOLEAN,
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
            "/assets/stopsImages/{}.png",
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


// API FUNCTIONS

pub async fn get_stops(pool: &SqlitePool) -> Json<Vec<StopResponse>> {
    let stops = sqlx::query_as::<_, StopResponse>(
        "SELECT id, name, image_url, wheelchair_accessible, has_shelter, has_ticket_machine FROM stops"
    )
    .fetch_all(pool)
    .await
    .unwrap();

    Json(stops)
}

pub fn check_id(strid: String) -> Result<u32, (StatusCode, Json<ErrorResponse>)> {
    let id: u32 = match strid.parse() {
        Ok(id) if id > 0 => id,
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    e: "invalid stop id".to_string(),
                }),
            ));
        }
    };
    Ok(id)
}

pub async fn get_single_stop(pool: &SqlitePool, id: String) -> Result<(StatusCode, Json<StopResponse>), (StatusCode, Json<ErrorResponse>)> {
    let id = check_id(id)?;
    let stop = sqlx::query_as::<_, StopResponse>(
        "SELECT id, name, image_url, wheelchair_accessible, has_shelter, has_ticket_machine FROM stops WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .unwrap();

    match stop {
        Some(stop) => Ok((StatusCode::OK, Json(stop))),
        None => Err((StatusCode::NOT_FOUND, Json(ErrorResponse{e: format!("stop with id {id} not found")})))
    }
}

fn parse_stop_input(
    input: Result<Json<StopInput>, JsonRejection>,
) -> Result<StopInput, (StatusCode, Json<ErrorResponse>)> {
    let Json(stop) = match input {
        Ok(input) => input,
        Err(error) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    e: error.to_string(),
                }),
            ));
        }
    };

    if let Err(error) = stop.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse { e: error }),
        ));
    }

    Ok(stop)
}

pub async fn create_stop(pool: &SqlitePool, input: Result<Json<StopInput>, JsonRejection>) -> Result<(StatusCode, Json<StopResponse>), (StatusCode, Json<ErrorResponse>)> {
    let stop = parse_stop_input(input)?;

    let result = sqlx::query(
        "INSERT INTO stops (
            name,
            image_url,
            wheelchair_accessible,
            has_shelter,
            has_ticket_machine
        ) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(&stop.name)
        .bind(&stop.image_url)
        .bind(stop.wheelchair_accessible)
        .bind(stop.has_shelter)
        .bind(stop.has_ticket_machine)
        .execute(pool)
        .await
        .unwrap();

    let stop = StopResponse {
        id: result.last_insert_rowid() as u32,
        name: stop.name,
        image_url: stop.image_url,
        wheelchair_accessible: stop.wheelchair_accessible,
        has_shelter: stop.has_shelter,
        has_ticket_machine: stop.has_ticket_machine,
    };

    Ok((StatusCode::CREATED, Json(stop)))
}

pub async fn update_stop(pool: &SqlitePool, id: String, input: Result<Json<StopInput>, JsonRejection>) -> Result<(StatusCode, Json<StopResponse>), (StatusCode, Json<ErrorResponse>)> {
    let id = check_id(id)?;
    let stop = parse_stop_input(input)?;

    let result = sqlx::query(
        "UPDATE stops SET
            name = ?,
            image_url = ?,
            wheelchair_accessible = ?,
            has_shelter = ?,
            has_ticket_machine = ?
        WHERE id = ?"
    )
    .bind(&stop.name)
    .bind(&stop.image_url)
    .bind(stop.wheelchair_accessible)
    .bind(stop.has_shelter)
    .bind(stop.has_ticket_machine)
    .bind(id)
    .execute(pool)
    .await
    .unwrap();

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                e: format!("stop with id {id} not found"),
            }),
        ));
    }

    let stop = StopResponse {
        id: id,
        name: stop.name,
        image_url: stop.image_url,
        wheelchair_accessible: stop.wheelchair_accessible,
        has_shelter: stop.has_shelter,
        has_ticket_machine: stop.has_ticket_machine,
    };


    Ok((StatusCode::OK, Json(stop)))
}

pub async fn delete_stop(pool: &SqlitePool, id: String) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let id = check_id(id)?;
    let result = sqlx::query(
        "DELETE FROM stops
         WHERE id = ?"
    )
    .bind(id)
    .execute(pool)
    .await
    .unwrap();

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                e: format!("stop with id {id} not found"),
            }),
        ));
    }

    Ok(StatusCode::NO_CONTENT)
}
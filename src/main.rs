use sqlx::sqlite::SqlitePool;
use axum::{
    routing::get,
    Router,
    extract::Path
};

use crate::shared::serve_file;
mod db;
mod shared;



#[tokio::main]
async fn main() {
    let pool: &'static SqlitePool =
    Box::leak(Box::new(db::init_db().await));
    db::seed_db(&pool).await;

    let app = Router::new()
        .route("/", get(|| serve_file("erd/index/html.html".to_string(), "text/html")))
        .route("/api/v1/stops", get(|| db::get_stops(pool)))
        .route("/api/v1/stops/{id}", get(|Path(id): Path<u32>| db::get_single_stop(pool, id)))
        .route("/{page}/",        get(|Path(page): Path<String>| shared::serve_file(format!("erd/{page}/wasm_bg.wasm"), "text/html")))
        .route("/{page}/erd.css", get(|Path(page): Path<String>| shared::serve_file(format!("erd/{page}/erd.css"), "text/css")))
        .route("/{page}/wasm.js", get(|Path(page): Path<String>| shared::serve_file(format!("erd/{page}/wasm.js"), "text/javascript")))
        .route("/{page}/bg.wasm", get(|Path(page): Path<String>| shared::serve_file(format!("erd/{page}/wasm_bg.wasm"), "application/wasm")));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

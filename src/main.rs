use sqlx::sqlite::SqlitePool;
use axum::{
    routing::{get, post, put, delete},
    Router,
    extract::Path,
    Json, 
    http::HeaderMap,
};
use axum::extract::rejection::JsonRejection;

use crate::shared::serve_file;
mod db;
mod shared;
use tower_http::services::ServeDir;


#[tokio::main]
async fn main() {
    let pool: &'static SqlitePool =
    Box::leak(Box::new(db::init_db().await));
    db::seed_db(&pool).await;

    let app = Router::new()
        .route("/",                  get(|| serve_file("erd/index/html.html".to_string(), "text/html")))
        
        .route("/api/v1/stops",      get(|| db::get_stops(pool)))
        .route("/api/v1/stops",      post(move|headers: HeaderMap, input: Result<Json<db::StopInput>, JsonRejection>| async move {match shared::check_auth(&headers) {Ok(()) => db::create_stop(pool, input).await,Err(error) => { Err(error) },}}))
        .route("/api/v1/stops/{id}", get(|Path(id): Path<String>| db::get_single_stop(pool, id)))
        .route("/api/v1/stops/{id}", put(move|headers: HeaderMap, Path(id): Path<String>, input: Result<Json<db::StopInput>, JsonRejection>| async move {match shared::check_auth(&headers) {Ok(()) => db::update_stop(pool, id, input).await,Err(error) => { Err(error) },}}))
        .route("/api/v1/stops/{id}", delete(move|headers: HeaderMap, Path(id): Path<String>| async move {match shared::check_auth(&headers) {Ok(()) => db::delete_stop(pool, id).await,Err(error) => { Err(error) },}}))
        
        .nest_service("/assets", ServeDir::new("assets"))
        .nest_service("/doc", ServeDir::new("erd"))
        
        .route("/stops/{id}",        get(|| serve_file("erd/stop-detail/html.html".to_string(), "text/html")))
        .route("/{page}", get(shared::serve_page));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

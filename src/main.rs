use axum::response::Html;
use axum::routing::get;
use axum::Router;
use ezrustdom::compile_pages;

#[tokio::main]
async fn main() {
    compile_pages();

    let app = Router::new()
        .route("/", get(|| async {
            Html(include_str!("../erd/index/html.html"))
        }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
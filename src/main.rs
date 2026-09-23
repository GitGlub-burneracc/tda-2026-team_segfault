// Starts the local web server and serves the project's main HTML page.
use axum::{
    body::Body,
    response::{Html, Response},
    routing::get,
    Json, Router,
};
use ezrustdom::compile_pages;
use serde_json::json;
use std::fs;

#[tokio::main]
async fn main() {
    // Prepare generated pages before accepting browser requests.
    compile_pages();

    // Register the home route and embed the HTML file into the compiled binary.
    let app = Router::new()
        .route("/", get(|| async {
            Html(include_str!("../erd/index/html.html"))
        }))
        .route("/api/v1/health", get(|| async {
            Json(json!({
                "status": "ok"
            }))
        }))
        .route("/erd.css", get(|| async {
            Response::builder()
                .header("content-type", "text/css")
                .body(Body::from(
                    fs::read("erd/index/erd.css")
                        .expect("failed to read erd.css"),
                ))
                .unwrap()
        }))
        .route("/wasm.js", get(|| async {
            Response::builder()
                .header("content-type", "text/javascript")
                .body(Body::from(
                    fs::read("erd/index/wasm.js")
                        .expect("failed to read wasm.js"),
                ))
                .unwrap()
        }))
        .route("/index_bg.wasm", get(|| async {
            Response::builder()
                .header("content-type", "application/wasm")
                .body(Body::from(
                    fs::read("erd/index/wasm_bg.wasm")
                        .expect("failed to read wasm_bg.wasm"),
                ))
                .unwrap()
        }));


    // Listen only on this computer at http://127.0.0.1:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    // Keep serving requests until the server exits or encounters an error.
    axum::serve(listener, app).await.unwrap();
}

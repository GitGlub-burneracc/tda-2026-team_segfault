// Starts the local web server and serves the project's main HTML page.
use axum::response::Html;
use axum::routing::get;
use axum::Router;
use ezrustdom::compile_pages;

#[tokio::main]
async fn main() {
    // Prepare generated pages before accepting browser requests.
    compile_pages();

    // Register the home route and embed the HTML file into the compiled binary.
    let app = Router::new()
        .route("/", get(|| async {
            Html(include_str!("../erd/index/html.html"))
        }));

    // Listen only on this computer at http://127.0.0.1:3000.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    // Keep serving requests until the server exits or encounters an error.
    axum::serve(listener, app).await.unwrap();
}

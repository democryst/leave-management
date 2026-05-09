use axum::{routing::get, Router};

mod auth_middleware;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(|| async { "API Gateway" }));

    let addr = "0.0.0.0:8080";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("🚀 API Gateway running on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

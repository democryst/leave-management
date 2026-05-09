use axum::{routing::get, Router};

mod internal;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/health", get(|| async { "Policy Service OK" }));

    let addr = "0.0.0.0:8083";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("🚀 Policy Service running on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

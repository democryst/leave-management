use axum::{routing::get, Router};

mod internal;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/health", get(|| async { "Staff Service OK" }));

    let addr = "0.0.0.0:8081";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("🚀 Staff Service running on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

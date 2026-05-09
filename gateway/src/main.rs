use axum::{
    routing::get,
    Router,
    middleware,
};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

mod auth_middleware;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // 1. Build routes with Auth Middleware
    // In a real proxy, we'd use something like 'tower-reverse-proxy' or 'hyper'
    // For this demonstration, we'll route to placeholder handlers that simulate the backend calls
    let api_routes = Router::new()
        .route("/staff/:id", get(|| async { "Staff Details (IST Protected)" }))
        .route("/leave/apply", get(|| async { "Leave Application (IST Protected)" }))
        .layer(middleware::from_fn(auth_middleware::auth_middleware));

    let app = Router::new()
        .route("/health", get(|| async { "Gateway OK" }))
        .nest("/api/v1", api_routes)
        .layer(TraceLayer::new_for_http());

    // 2. Start Gateway
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("🚀 API Gateway running on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

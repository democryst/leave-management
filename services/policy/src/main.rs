use axum::{routing::get, Router};
use std::sync::Arc;
use crate::internal::adapters::repository::SqlxPolicyRepository;
use crate::internal::adapters::handler::get_policy;
use sqlx::postgres::PgPoolOptions;

mod internal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // 1. Initialize DB
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/leave_management".to_string());
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url).await?;

    // 2. Initialize Dependencies
    let repo = Arc::new(SqlxPolicyRepository::new(pool));

    // 3. Compose Application
    let app = Router::new()
        .route("/health", get(|| async { "Policy Service OK" }))
        .route("/api/v1/policies/leave-types/:id", get(get_policy))
        .with_state(repo);

    // 4. Start Server
    let addr = "0.0.0.0:8083";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("🚀 Policy Service running on {}", addr);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

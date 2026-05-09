use axum::{routing::get, Router};
use std::sync::Arc;
use crate::internal::adapters::repository::SqlxStaffRepository;
use crate::internal::adapters::handler::get_staff;
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

    // 2. Initialize Dependencies (Ports/Adapters)
    let repo = Arc::new(SqlxStaffRepository::new(pool));

    // 3. Compose Application
    let app = Router::new()
        .route("/health", get(|| async { "Staff Service OK" }))
        .route("/api/v1/staff/:id", get(get_staff))
        .with_state(repo);

    // 4. Start Server
    let addr = "0.0.0.0:8081";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("🚀 Staff Service running on {}", addr);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

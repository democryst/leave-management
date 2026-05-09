use axum::{routing::get, Router, Json, extract::Path};
use std::sync::Arc;
use crate::internal::adapters::repository::SqlxStaffRepository;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

use crate::internal::core::ports::StaffRepository;

mod internal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // 1. Initialize DB
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/leave_management".to_string());
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url).await?;

    // 2. Initialize Repository
    let repo = Arc::new(SqlxStaffRepository::new(pool));

    // 3. Build Router
    let app = Router::new()
        .route("/health", get(|| async { "Staff Service OK" }))
        .route("/api/v1/staff/:id", get({
            let repo = repo.clone();
            move |Path(id): Path<Uuid>| {
                let repo = repo.clone();
                async move {
                    match repo.find_by_id(id).await {
                        Ok(Some(staff)) => (axum::http::StatusCode::OK, Json(serde_json::json!(staff))),
                        Ok(None) => (axum::http::StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Staff not found"}))),
                        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e})))
                    }
                }
            }
        }));

    // 4. Start Server
    let addr = "0.0.0.0:8081";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("🚀 Staff Service running on {}", addr);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

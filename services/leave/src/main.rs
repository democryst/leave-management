use axum::{routing::post, Router};
use std::sync::Arc;
use crate::internal::core::services::LeaveApplicationService;
use crate::internal::adapters::repository::SqlxLeaveRepository;
use crate::internal::adapters::gateway::policy_client::PolicyServiceClient;
use crate::internal::adapters::handler::apply_leave;
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

    // 2. Initialize Adapters
    let repo = Arc::new(SqlxLeaveRepository::new(pool));
    let policy_client = Arc::new(PolicyServiceClient::new(
        std::env::var("POLICY_SERVICE_URL").unwrap_or_else(|_| "http://policy:8083".to_string())
    ));

    // 3. Initialize Core Service
    let leave_service = Arc::new(LeaveApplicationService::new(repo, policy_client));

    // 4. Build Router
    let app = Router::new()
        .route("/api/v1/leave/requests", post(apply_leave))
        .with_state(leave_service);

    // 5. Start Server
    let addr = "0.0.0.0:8082";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("🚀 Leave Service running on {}", addr);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

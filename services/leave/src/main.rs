use axum::{routing::post, Router, Json};
use std::sync::Arc;
use crate::internal::core::services::LeaveApplicationService;
use crate::internal::adapters::repository::SqlxLeaveRepository;
use crate::internal::adapters::gateway::policy_client::PolicyServiceClient;
use sqlx::postgres::PgPoolOptions;

mod internal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize DB
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url).await?;

    // 2. Initialize Adapters
    let repo = Arc::new(SqlxLeaveRepository::new(pool));
    let policy_client = Arc::new(PolicyServiceClient::new(
        std::env::var("POLICY_SERVICE_URL").unwrap_or_else(|_| "http://policy:8080".to_string())
    ));

    // 3. Initialize Core Service
    let leave_service = Arc::new(LeaveApplicationService::new(repo, policy_client));

    // 4. Build Router (Demo Endpoint)
    let app = Router::new()
        .route("/api/v1/leave/requests", post(move |Json(payload)| {
            let service = leave_service.clone();
            async move {
                // In a real app, this would call the service and handle errors
                format!("Processed request for staff {}", "ST-123")
            }
        }));

    // 5. Start Server
    let addr = "0.0.0.0:8080".parse().unwrap();
    println!("🚀 Leave Service running on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

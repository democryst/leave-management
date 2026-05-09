use axum::{routing::post, Router, Json};
use std::sync::Arc;
use crate::internal::core::services::LeaveApplicationService;
use crate::internal::adapters::repository::SqlxLeaveRepository;
use crate::internal::adapters::gateway::policy_client::PolicyServiceClient;
use sqlx::postgres::PgPoolOptions;
use serde::Deserialize;
use bigdecimal::BigDecimal;

mod internal;

#[derive(Deserialize)]
struct CreateLeaveRequest {
    staff_id: String,
    leave_type_id: String,
    start_date: String,
    end_date: String,
    reason: String,
    days: BigDecimal,
}

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
        .route("/api/v1/leave/requests", post(move |Json(payload): Json<CreateLeaveRequest>| {
            let service = leave_service.clone();
            async move {
                let token = "system-token"; // Placeholder for auth logic
                match service.apply_for_leave(
                    &payload.staff_id,
                    &payload.leave_type_id,
                    &payload.start_date,
                    &payload.end_date,
                    payload.reason,
                    payload.days,
                    token
                ).await {
                    Ok(req) => (axum::http::StatusCode::CREATED, Json(serde_json::to_value(req).unwrap())),
                    Err(e) => (axum::http::StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": e.to_string() })))
                }
            }
        }));

    // 5. Start Server
    let addr = "0.0.0.0:8082";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("🚀 Leave Service running on {}", addr);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

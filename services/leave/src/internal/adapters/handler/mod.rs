use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;
use serde::Deserialize;
use bigdecimal::BigDecimal;
use crate::internal::core::services::LeaveApplicationService;

#[derive(Deserialize)]
pub struct CreateLeaveRequest {
    pub staff_id: String,
    pub leave_type_id: String,
    pub start_date: String,
    pub end_date: String,
    pub reason: String,
    pub days: BigDecimal,
}

pub async fn apply_leave(
    State(service): State<Arc<LeaveApplicationService>>,
    Json(payload): Json<CreateLeaveRequest>,
) -> impl IntoResponse {
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
        Ok(req) => (StatusCode::CREATED, Json(serde_json::to_value(req).unwrap())),
        Err(e) => (StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": e.to_string() })))
    }
}

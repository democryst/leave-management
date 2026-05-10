use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;
use serde::Deserialize;
use validator::Validate;
use crate::internal::core::ports::LeaveService;
use crate::internal::core::domain::{LeaveRequest, LeaveStatus};
use uuid::Uuid;
use chrono::{NaiveDate, Utc};

#[derive(Deserialize, Validate)]
pub struct CreateLeaveRequest {
    pub staff_id: Uuid,
    pub leave_type_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    
    #[validate(length(max = 500, message = "reason too long"))]
    pub reason: String,
}

pub async fn apply_leave(
    State(service): State<Arc<dyn LeaveService>>,
    Json(payload): Json<CreateLeaveRequest>,
) -> impl IntoResponse {
    // 1. Validate DTO
    if let Err(e) = payload.validate() {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": e.to_string() }))).into_response();
    }

    // 2. Map to Domain
    let request = LeaveRequest {
        id: Uuid::now_v7(),
        staff_id: payload.staff_id,
        leave_type_id: payload.leave_type_id,
        start_date: payload.start_date,
        end_date: payload.end_date,
        status: LeaveStatus::Pending,
        reason: payload.reason,
        approver_id: None,
        created_at: Utc::now(),
    };

    // 3. Execute Business Logic
    match service.submit_request(request).await {
        Ok(req) => (StatusCode::CREATED, Json(serde_json::to_value(req).unwrap())).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": e }))).into_response()
    }
}

pub async fn get_pending_approvals(
    State(service): State<Arc<dyn LeaveService>>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    let user_id_str = headers.get("X-User-Id").and_then(|h: &axum::http::HeaderValue| h.to_str().ok());
    
    if let Some(user_id) = user_id_str.and_then(|s| Uuid::parse_str(s).ok()) {
        match service.get_approver_queue(user_id).await {
            Ok(requests) => (StatusCode::OK, Json(serde_json::to_value(requests).unwrap())).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e }))).into_response()
        }
    } else {
        (StatusCode::UNAUTHORIZED, "Missing identity").into_response()
    }
}

use axum::{extract::{State, Query}, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;
use crate::internal::core::ports::PolicyService;
use serde::Deserialize;
use chrono::NaiveDate;

#[derive(Deserialize)]
pub struct HolidayQuery {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

pub async fn list_policies(
    State(service): State<Arc<dyn PolicyService>>,
) -> impl IntoResponse {
    match service.list_leave_types().await {
        Ok(policies) => (StatusCode::OK, Json(serde_json::json!(policies))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e})))
    }
}

pub async fn get_holidays(
    State(service): State<Arc<dyn PolicyService>>,
    Query(query): Query<HolidayQuery>,
) -> impl IntoResponse {
    match service.get_holidays_in_range(query.start, query.end).await {
        Ok(holidays) => (StatusCode::OK, Json(serde_json::json!(holidays))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e})))
    }
}

pub async fn get_policy(
    State(service): State<Arc<dyn PolicyService>>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
) -> impl IntoResponse {
    match service.get_leave_type(id).await {
        Ok(Some(policy)) => (StatusCode::OK, Json(serde_json::json!(policy))),
        Ok(None) => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Policy not found"}))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e})))
    }
}

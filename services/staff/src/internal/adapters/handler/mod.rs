use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;
use uuid::Uuid;
use crate::internal::core::ports::StaffRepository;

pub async fn get_staff(
    State(repo): State<Arc<dyn StaffRepository>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match repo.find_by_id(id).await {
        Ok(Some(staff)) => (StatusCode::OK, Json(serde_json::json!(staff))),
        Ok(None) => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Staff not found"}))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e})))
    }
}

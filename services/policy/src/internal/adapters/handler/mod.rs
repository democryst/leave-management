use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;
use uuid::Uuid;
use crate::internal::core::ports::PolicyRepository;

pub async fn get_policy(
    State(repo): State<Arc<dyn PolicyRepository>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match repo.get_leave_type_by_id(id).await {
        Ok(Some(lt)) => (StatusCode::OK, Json(serde_json::json!(lt))),
        Ok(None) => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Policy not found"}))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e})))
    }
}

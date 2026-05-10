use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;
use crate::internal::core::ports::PolicyService;

pub async fn list_policies(
    State(service): State<Arc<dyn PolicyService>>,
) -> impl IntoResponse {
    match service.list_leave_types().await {
        Ok(policies) => (StatusCode::OK, Json(serde_json::json!(policies))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e})))
    }
}

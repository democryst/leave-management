use axum::{extract::{Path, State}, http::{header, StatusCode}, response::IntoResponse, Json};
use std::sync::Arc;
use uuid::Uuid;
use crate::internal::core::ports::StaffService;

pub async fn get_staff(
    State(service): State<Arc<dyn StaffService>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match service.get_staff_by_id(id).await {
        Ok(Some(staff)) => (StatusCode::OK, Json(serde_json::json!(staff))),
        Ok(None) => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Staff not found"}))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e})))
    }
}

pub async fn get_profile(
    State(service): State<Arc<dyn StaffService>>,
    headers: header::HeaderMap,
) -> impl IntoResponse {
    // Extract User ID from X-User-Id header injected by Gateway
    let user_id = headers.get("X-User-Id")
        .and_then(|h| h.to_str().ok());

    let id_str = match user_id {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Missing user identity"}))).into_response(),
    };

    let id = match Uuid::parse_str(id_str) {
        Ok(u) => u,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "Invalid user ID"}))).into_response(),
    };

    match service.get_staff_by_id(id).await {
        Ok(Some(staff)) => (StatusCode::OK, Json(serde_json::json!(staff))).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Staff not found"}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e}))).into_response()
    }
}

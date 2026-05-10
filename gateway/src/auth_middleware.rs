use axum::{
    extract::Request,
    middleware::Next,
    http::{header, StatusCode},
    response::Response,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Validation, Header, Algorithm};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
    pub iat: usize,
    pub iss: Option<String>,
}

const INTERNAL_ISSUER: &str = "leave-management-gateway";

/// ADR-001: Gateway Auth Middleware
/// Validates Edge JWT -> Generates Internal IST (RS256) -> Injects Header
pub async fn auth_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // 1. Extract Edge JWT from Authorization header
    let auth_header = req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .filter(|h| h.starts_with("Bearer "))
        .map(|h| &h[7..]);

    let edge_token = auth_header.ok_or(StatusCode::UNAUTHORIZED)?;

    // 2. Validate Edge JWT
    // CRITICAL: No hardcoded fallback. Must fail if EDGE_JWT_SECRET is missing.
    let edge_secret = env::var("EDGE_JWT_SECRET").map_err(|_| {
        tracing::error!("EDGE_JWT_SECRET not set");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let mut validation = Validation::default();
    validation.validate_exp = true;
    
    let token_data = decode::<Claims>(
        edge_token,
        &DecodingKey::from_secret(edge_secret.as_bytes()),
        &validation
    ).map_err(|e| {
        tracing::warn!("Edge JWT validation failed: {}", e);
        StatusCode::UNAUTHORIZED
    })?;

    // 3. Generate Internal Service Token (IST) per ADR-001 (RS256)
    let ist = generate_internal_token(&token_data.claims)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 4. Inject IST and User Identity into headers for downstream services
    req.headers_mut().insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&format!("Bearer {}", ist))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    );
    
    req.headers_mut().insert(
        "X-User-Id",
        header::HeaderValue::from_str(&token_data.claims.sub)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    );

    req.headers_mut().insert(
        "X-User-Role",
        header::HeaderValue::from_str(&token_data.claims.role)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    );

    Ok(next.run(req).await)
}

fn generate_internal_token(claims: &Claims) -> Result<String, jsonwebtoken::errors::Error> {
    // CRITICAL: No hardcoded fallback. Must fail if IST_PRIVATE_KEY is missing.
    let private_key_pem = env::var("IST_PRIVATE_KEY").map_err(|_| {
        tracing::error!("IST_PRIVATE_KEY not set");
        jsonwebtoken::errors::Error::from(jsonwebtoken::errors::ErrorKind::InvalidRsaKey("Missing IST_PRIVATE_KEY".to_string()))
    })?;

    // IST uses RS256 per ADR-001
    let header = Header::new(Algorithm::RS256);

    // IST should have its own short-lived expiration
    let now = chrono::Utc::now().timestamp() as usize;
    let ist_claims = Claims {
        sub: claims.sub.clone(),
        role: claims.role.clone(),
        exp: now + 900, // IST valid for 15 minutes
        iat: now,
        iss: Some(INTERNAL_ISSUER.to_string()),
    };

    encode(
        &header,
        &ist_claims,
        &EncodingKey::from_rsa_pem(private_key_pem.as_bytes())?
    )
}

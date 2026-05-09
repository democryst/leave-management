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

    // 2. Validate Edge JWT (Assuming HS256 for the Edge for now)
    let edge_secret = env::var("EDGE_JWT_SECRET").unwrap_or_else(|_| "placeholder_secret".to_string());
    let mut validation = Validation::default();
    validation.validate_exp = true;
    
    let token_data = decode::<Claims>(
        edge_token,
        &DecodingKey::from_secret(edge_secret.as_bytes()),
        &validation
    ).map_err(|_| StatusCode::UNAUTHORIZED)?;

    // 3. Generate Internal Service Token (IST) per ADR-001 (RS256)
    let ist = generate_internal_token(&token_data.claims)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 4. Inject IST into Authorization header for downstream services
    req.headers_mut().insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&format!("Bearer {}", ist))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    );

    Ok(next.run(req).await)
}

fn generate_internal_token(claims: &Claims) -> Result<String, jsonwebtoken::errors::Error> {
    let private_key_pem = env::var("IST_PRIVATE_KEY")
        .unwrap_or_else(|_| "---BEGIN RSA PRIVATE KEY---\n...placeholder...\n---END RSA PRIVATE KEY---".to_string());

    // IST uses RS256 per ADR-001
    let header = Header::new(Algorithm::RS256);

    let ist_claims = Claims {
        sub: claims.sub.clone(),
        role: claims.role.clone(),
        exp: claims.exp,
        iat: claims.iat,
        iss: Some(INTERNAL_ISSUER.to_string()),
    };

    encode(
        &header,
        &ist_claims,
        &EncodingKey::from_rsa_pem(private_key_pem.as_bytes())?
    )
}

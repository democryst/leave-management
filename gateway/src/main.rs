use axum::{
    routing::get,
    Router,
    middleware,
    extract::{Path, State},
    http::{header, Method},
    response::IntoResponse,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use reqwest::Client;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use opentelemetry::{KeyValue, trace::TracerProvider as _};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{trace as sdktrace, Resource};

mod auth_middleware;

struct AppState {
    client: Client,
    staff_service_url: String,
    leave_service_url: String,
    policy_service_url: String,
}

fn init_tracing() {
    let otlp_endpoint = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
        .unwrap_or_else(|_| "http://localhost:4317".to_string());

    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint(otlp_endpoint)
        .build()
        .expect("Failed to create exporter");

    let provider = sdktrace::SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(
            Resource::builder().with_attributes(vec![KeyValue::new("service.name", "api-gateway")]).build()
        )
        .build();

    let tracer = provider.tracer("api-gateway");

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .init();
    
    opentelemetry::global::set_tracer_provider(provider);
}

#[tokio::main]
async fn main() {
    init_tracing();

    let state = Arc::new(AppState {
        client: Client::new(),
        staff_service_url: std::env::var("STAFF_SERVICE_URL").unwrap_or_else(|_| "http://localhost:8081".to_string()),
        leave_service_url: std::env::var("LEAVE_SERVICE_URL").unwrap_or_else(|_| "http://localhost:8082".to_string()),
        policy_service_url: std::env::var("POLICY_SERVICE_URL").unwrap_or_else(|_| "http://localhost:8083".to_string()),
    });

    let api_routes = Router::new()
        // Staff Routes
        .route("/staff/profile", get(proxy_staff_profile))
        .route("/staff/:id", get(proxy_staff_by_id))
        
        // Admin Staff Routes
        .nest("/admin", Router::new()
            .route("/staff/register", axum::routing::post(proxy_staff_register))
            .route("/staff/terminate/:id", axum::routing::post(proxy_staff_terminate))
            .layer(middleware::from_fn(auth_middleware::admin_only_middleware))
        )
        
        // Leave Routes
        .route("/leave/requests", get(proxy_leave_requests).post(proxy_leave_requests))
        
        // Policy Routes
        .route("/policy/leave-types", get(proxy_policy_leave_types))
        
        .layer(middleware::from_fn(auth_middleware::auth_middleware))
        .with_state(state);

    let app = Router::new()
        .route("/health", get(|| async { "Gateway OK" }))
        .nest("/api/v1", api_routes)
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("🚀 API Gateway running on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

// Proxies
async fn proxy_staff_profile(
    State(state): State<Arc<AppState>>,
    headers: header::HeaderMap,
) -> impl IntoResponse {
    let url = format!("{}/api/v1/staff/profile", state.staff_service_url);
    forward_request(&state.client, &url, headers, Method::GET, None).await
}

async fn proxy_staff_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    headers: header::HeaderMap,
) -> impl IntoResponse {
    let url = format!("{}/api/v1/staff/{}", state.staff_service_url, id);
    forward_request(&state.client, &url, headers, Method::GET, None).await
}

async fn proxy_staff_register(
    State(state): State<Arc<AppState>>,
    headers: header::HeaderMap,
    body: axum::body::Bytes,
) -> impl IntoResponse {
    let url = format!("{}/api/v1/staff/register", state.staff_service_url);
    forward_request(&state.client, &url, headers, Method::POST, Some(body)).await
}

async fn proxy_staff_terminate(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    headers: header::HeaderMap,
) -> impl IntoResponse {
    let url = format!("{}/api/v1/staff/terminate/{}", state.staff_service_url, id);
    forward_request(&state.client, &url, headers, Method::POST, None).await
}

async fn proxy_leave_requests(
    method: Method,
    State(state): State<Arc<AppState>>,
    headers: header::HeaderMap,
    body: Option<axum::body::Bytes>,
) -> impl IntoResponse {
    let url = format!("{}/api/v1/leave/requests", state.leave_service_url);
    forward_request(&state.client, &url, headers, method, body).await
}

async fn proxy_policy_leave_types(
    State(state): State<Arc<AppState>>,
    headers: header::HeaderMap,
) -> impl IntoResponse {
    let url = format!("{}/api/v1/policies", state.policy_service_url);
    forward_request(&state.client, &url, headers, Method::GET, None).await
}

async fn forward_request(
    client: &Client, 
    url: &str, 
    headers: header::HeaderMap, 
    method: Method,
    body: Option<axum::body::Bytes>,
) -> impl IntoResponse {
    let mut req = client.request(method, url);
    
    // Forward headers
    for (name, value) in headers.iter() {
        if name != header::HOST {
            req = req.header(name, value);
        }
    }

    if let Some(b) = body {
        req = req.body(b);
    }

    match req.send().await {
        Ok(resp) => {
            let status = resp.status().as_u16();
            let body = resp.bytes().await.unwrap_or_default();
            let mut response = (axum::http::StatusCode::from_u16(status).unwrap(), body).into_response();
            
            // Forward back some headers if needed (e.g. content-type)
            // For now, assume JSON
            response.headers_mut().insert(header::CONTENT_TYPE, header::HeaderValue::from_static("application/json"));
            
            response
        },
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}

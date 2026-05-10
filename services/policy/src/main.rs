use axum::{routing::get, Router};
use std::sync::Arc;
use sqlx::postgres::PgPoolOptions;
use crate::internal::adapters::repository::SqlxPolicyRepository;
use crate::internal::adapters::handler::list_policies;
use crate::internal::core::services::PolicyServiceImpl;
use crate::internal::core::ports::PolicyService;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use opentelemetry::{KeyValue, trace::TracerProvider as _};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{trace as sdktrace, Resource};

mod internal;

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
            Resource::builder().with_attributes(vec![KeyValue::new("service.name", "policy-service")]).build()
        )
        .build();

    let tracer = provider.tracer("policy-service");

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .init();
    
    opentelemetry::global::set_tracer_provider(provider);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    // 1. Initialize DB
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://policy_user:secret@localhost:5434/policy_database".to_string());
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url).await?;

    // 2. Initialize Dependencies
    let repo = Arc::new(SqlxPolicyRepository::new(pool));
    let service: Arc<dyn PolicyService> = Arc::new(PolicyServiceImpl::new(repo));

    // 3. Compose Application
    let app = Router::new()
        .route("/health", get(|| async { "Policy Service OK" }))
        .route("/api/v1/policies", get(list_policies))
        .with_state(service);

    // 4. Start Server
    let addr = "0.0.0.0:8083";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("🚀 Policy Service running on {}", addr);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

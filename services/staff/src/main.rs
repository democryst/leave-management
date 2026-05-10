use axum::{routing::get, Router};
use std::sync::Arc;
use sqlx::postgres::PgPoolOptions;
use crate::internal::adapters::repository::SqlxStaffRepository;
use crate::internal::adapters::handler::get_staff;
use crate::internal::core::services::StaffServiceImpl;
use crate::internal::core::ports::StaffService;
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
            Resource::builder().with_attributes(vec![KeyValue::new("service.name", "staff-service")]).build()
        )
        .build();

    let tracer = provider.tracer("staff-service");

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
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://staff_user:secret@localhost:5432/staff_database".to_string());
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url).await?;

    // 2. Initialize Dependencies (Ports/Adapters)
    let repo = Arc::new(SqlxStaffRepository::new(pool));
    let service: Arc<dyn StaffService> = Arc::new(StaffServiceImpl::new(repo));

    // 3. Compose Application
    let app = Router::new()
        .route("/health", get(|| async { "Staff Service OK" }))
        .route("/api/v1/staff/profile", get(crate::internal::adapters::handler::get_profile))
        .route("/api/v1/staff/delegations", get(crate::internal::adapters::handler::get_delegations))
        .route("/api/v1/staff/delegations/check/:delegatee/:delegator", get(crate::internal::adapters::handler::check_delegation))
        .route("/api/v1/staff/:id", get(get_staff))
        .with_state(service);

    // 4. Start Server
    let addr = "0.0.0.0:8081";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("🚀 Staff Service running on {}", addr);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

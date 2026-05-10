use axum::{routing::post, Router};
use std::sync::Arc;
use crate::internal::core::services::LeaveServiceImpl;
use crate::internal::core::ports::LeaveService;
use crate::internal::adapters::repository::SqlxLeaveRepository;
use crate::internal::adapters::gateway::policy_client::PolicyServiceClient;
use crate::internal::adapters::handler::apply_leave;
use sqlx::postgres::PgPoolOptions;
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
            Resource::builder().with_attributes(vec![KeyValue::new("service.name", "leave-service")]).build()
        )
        .build();

    let tracer = provider.tracer("leave-service");

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
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://leave_user:secret@localhost:5433/leave_database".to_string());
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url).await?;

    // 2. Initialize Adapters
    let repo = Arc::new(SqlxLeaveRepository::new(pool));
    let policy_client = Arc::new(PolicyServiceClient::new(
        std::env::var("POLICY_SERVICE_URL").unwrap_or_else(|_| "http://localhost:8083".to_string())
    ));

    // 3. Initialize Core Service
    let leave_service: Arc<dyn LeaveService> = Arc::new(LeaveServiceImpl::new(repo, policy_client));

    // 4. Build Router
    let app = Router::new()
        .route("/api/v1/leave/requests", post(apply_leave))
        .with_state(leave_service);

    // 5. Start Server
    let addr = "0.0.0.0:8082";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("🚀 Leave Service running on {}", addr);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

# ADR-002: Distributed Tracing & PII Masking

## Context
Full visibility into request lifecycles is required, but we must prevent Staff PII from appearing in traces and logs.

## Decision
We will enforce OpenTelemetry (OTel) across all agents with the following rules:
1. **Stack:** Use OpenTelemetry SDK v0.31 with OTLP gRPC export.
2. **Propagation:** Gateway initializes the `trace_id` and `span_id`. All downstream calls must carry these in the `traceparent` header.
3. **Frontend Sync:** Next.js frontend uses `@opentelemetry/instrumentation-fetch` to propagate browser traces to the Gateway.
4. **PII Masking:** Every domain entity in Rust must implement a `Mask` trait. PII is masked before being logged or added as span attributes in the `service` layer.
5. **Instrumentation:** Services use the `tracing` crate with `tracing-opentelemetry` and `opentelemetry-otlp`.

## Consequences
- **Positive:** Root cause analysis across services becomes trivial.
- **Positive:** Visual confirmation of request flow via Jaeger.
- **Positive:** **gemma-sec** can programmatically verify PII compliance.
- **Negative:** Minor performance overhead for telemetry spans.

## Standards
- **Header:** `traceparent` (W3C standard).
- **Masking Pattern:** `email -> e***l@domain.com`, `name -> J*** D***`.
- **Collector:** All signals go to `otel-collector:4317` (gRPC) or `4318` (HTTP).

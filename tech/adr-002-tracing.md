# ADR-002: Distributed Tracing & PII Masking

## Context
Full visibility into request lifecycles is required, but we must prevent Staff PII from appearing in traces and logs.

## Decision
We will enforce OpenTelemetry (OTel) across all agents with the following rules:
1. **Propagation:** Gateway initializes the `trace_id` and `span_id`. All downstream calls must carry these in the `traceparent` header.
2. **PII Masking:** Every domain entity in Rust must implement a `Mask` trait.
3. **Instrumentation:** Services will use the `tracing` crate with `tracing-opentelemetry`.

## Consequences
- **Positive:** Root cause analysis across services becomes trivial.
- **Positive:** **gemma-sec** can programmatically verify PII compliance.
- **Negative:** Minor performance overhead for telemetry spans.

## Standards
- **Header:** `traceparent` (W3C standard).
- **Masking Pattern:** `email -> e***l@domain.com`, `name -> J*** D***`.

# skill.md — Institutional Memory & Patterns

## [Architectural] — Supervisor Multi-Agent Pattern
**Discovered:** 2026-05-09
**Context:** Creating a complex Leave Management Microservices suite using a mix of a Supervisor AI (Antigravity) and localized specialist agents (Gemma).
**Pattern:**
1. **Root Supervisor:** Maintains global `CLAUDE.md` for orchestration, SDLC gates, and cross-cutting standards (OTel, Auth).
2. **Project-Local Manuals:** Each sub-directory (`web/`, `gateway/`, `services/*/`) contains a specialized `CLAUDE.md` tailored to a specific Gemma specialist's persona.
3. **Context Isolation:** Local agents are restricted to their domain directories to prevent "God Service" leakage and maintain DDD boundaries.
4. **Unified Observability:** OTel `trace_id` is the common thread required in all localized manuals to ensure cross-service visibility.
**Gotcha:** Supervisor must audit inter-service contracts (gRPC/REST) early in the `/tech` phase to prevent breaking changes during parallel agent execution.

## [HASH: 6cff285d20a1b54622521fb8a854e3cc235ad2d1a8523344f3868aca3264397e] | [REF: CLAUDE.md v7.0] | 2026-05-10 | [PARENT: Distributed PII Masking]
## [Security] — Distributed PII Masking
**Discovered:** 2026-05-09
**Verified:** 2026-05-10 (Implemented in Staff, Leave, Policy services)
**Pattern:** Mandatory implementation of a `Mask` trait in the Rust `domain` layer. The Service layer must call `.mask()` before returning data to Adapters.
**Gotcha:** Ensure `password_hash` is always masked even in internal logs.

## [Technical] — Rust v1.8x / Axum 0.7 / SQLx 0.8 Hardening
**Discovered:** 2026-05-09
**Context:** Migrating microservices to modern Axum/SQLx stacks.
**Pattern:**
1. **Axum 0.7:** Use `tokio::net::TcpListener::bind` + `axum::serve` for server initialization.
2. **SQLx 0.8:** Prefer `BigDecimal` with the `serde` feature enabled for numeric precision; `rust_decimal` requires explicit `db-postgres` features and often suffers from trait bound inference issues in macros.
3. **Module Tree:** Always expose `internal` submodules via `mod.rs` in `core` and `adapters` to maintain hexagonal visibility.
4. **OTel v2.x (Web):** Use `resourceFromAttributes` and inject span processors via the constructor's `spanProcessors` array.
**Gotcha:** SQLx `query!` macros require a live database connection or `.sqlx` data during `cargo check`. Use non-macro `query_as::<Postgres, T>` for environment-agnostic validation.

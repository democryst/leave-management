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

## [HASH: aba76f3004c605c401c4c14e979ac4f19d6db2d8fddb45793685304f1d8d1bd4] | [REF: CLAUDE.md v8.0] | 2026-05-10 | [PARENT: f1e2d3a4]
### [Procedural] — Sovereign Agentic Commander Protocol
**Institutionalized:** 2026-05-10
**Context:** Upgrading the agentic workflow to v8.0 standards for high-autonomy engineering.
**Pattern:**
1. **Sovereignty**: Zero-Trust/Zero-Magic paradigm. All operations are local-first.
2. **XFV (Execution Fact Verification)**: Mandatory terminal proof in `5_EXECUTION_LOG.txt`.
3. **KHP (Knowledge Hash Protocol)**: Insights are hashed and linked in `INTERNAL/SKILLS.md`.
4. **CWSP**: Context Window Survival Protocol for token efficiency.

## [HASH: be4e26ce6504222629b139535f29910a307040f7d5448378875ee9a69527ec56] | [REF: FR-4.2] | 2026-05-10 | [PARENT: aba76f30]
## [Architectural] — Delegatee-Aware Authorization
**Discovered:** 2026-05-10
**Context:** Cross-service authorization where one user acts on behalf of another (Delegation).
**Pattern:** 
1. The **Target Service** (e.g., Leave) receives a request from a Delegatee.
2. The **Target Service** queries the **Identity/Staff Service** via a specialized check API (`/delegations/check/:delegatee/:delegator`).
3. Authorization is granted if the delegation is active, preserving DDD boundaries (Target Service doesn't need to know *how* delegations are stored).
**Gotcha:** Always propagate the `system-token` for service-to-service checks to bypass user-level permission loops.

## [Architectural] — Policy-Driven State Transitions
**Discovered:** 2026-05-10
**Context:** Implementing "Auto-Approval" logic in the Leave Management System.
**Pattern:**
1. The **Primary Service** (Leave) fetches "Rule Metadata" (e.g., `auto_approve`) from the **Policy Service** during the creation phase.
2. If the policy flag is set, the **Primary Service** immediately transitions the entity to the final state (`Approved`) and assigns a "System User" ID (`Uuid::nil()`) as the approver.
3. This decouples business rule definitions (Policy) from workflow execution (Leave).
**Gotcha:** Ensure atomic updates to both the entity status and the side-effects (e.g., balance deduction) to prevent partial approvals.

## [Technical] — Rust v1.8x / Axum 0.7 / SQLx 0.8 Hardening
... (remaining content)
**Discovered:** 2026-05-09
**Context:** Migrating microservices to modern Axum/SQLx stacks.
**Pattern:**
1. **Axum 0.7:** Use `tokio::net::TcpListener::bind` + `axum::serve` for server initialization.
2. **SQLx 0.8:** Prefer `BigDecimal` with the `serde` feature enabled for numeric precision; `rust_decimal` requires explicit `db-postgres` features and often suffers from trait bound inference issues in macros.
3. **Module Tree:** Always expose `internal` submodules via `mod.rs` in `core` and `adapters` to maintain hexagonal visibility.
4. **OTel v2.x (Web):** Use `resourceFromAttributes` and inject span processors via the constructor's `spanProcessors` array.
**Gotcha:** SQLx `query!` macros require a live database connection or `.sqlx` data during `cargo check`. Use non-macro `query_as::<Postgres, T>` for environment-agnostic validation.

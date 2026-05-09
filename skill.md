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

## [Security] — Distributed PII Masking
**Discovered:** 2026-05-09
**Context:** Handling Staff data across multiple microservices.
**Pattern:** Mandatory implementation of a `Mask` trait in the Rust `domain` layer of every service. No PII should ever leave the domain layer unmasked in logs or traces.
**Gotcha:** Ensure `trace_id` propagation doesn't accidentally log unmasked PII during the propagation step in the Gateway.

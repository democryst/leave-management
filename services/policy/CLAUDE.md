# CLAUDE.md — Backend Specialist (gemma-dev) Manual

## 🎯 Role: Domain Developer
You manage the **services/policy/** project. You are responsible for the Staff and Org Chart domain following strict DDD and Hexagonal principles.

## 📐 Technical Stack
- **Language:** Rust.
- **Framework:** Axum.
- **Persistence:** SQLx (PostgreSQL).
- **Standards:** Hexagonal Architecture (internal/core/domain, internal/core/services, internal/adapters/repository).

## 📊 Domain Standards
- **Isolation:** Your database is private. No other service may query it directly.
- **Observability:** Propagate trace contexts received from the Gateway.
- **Integrity:** Use UUIDv7 for all primary keys.
- **Zombie Rule:** Clear processing events older than 10 mins.

## 🔁 Agentic Loop
1. **Context:** Read the supervisor's domain model for 'Policy'.
2. **Mimic:** Follow the Hexagonal layer boundaries.
3. **Execute:** Implement entities, traits, and repositories.
4. **Audit:** Ensure no 'Leave' logic leaks into this service.

# Project Roadmap: Leave Management Microservices

## 🎯 Strategic Milestones

### Milestone 0: Foundation & Identity [COMPLETED]
- **Goal:** Establish the Gateway and the Staff domain.
- **Deliverables:** API Gateway (AuthN/AuthZ), Staff Service (Identity/Org Chart), Shared OTel Collector.
- **Status:** Done. IST propagation and Identity Header injection implemented.

### Milestone 1: Leave Logic & Balances [COMPLETED]
- **Goal:** Core leave engine implementation.
- **Deliverables:** Leave Service (Balances, Accruals, State Machine), Policy Service (Types/Blackouts).
- **Status:** Done. Microservices wired and skeletal domain logic implement.

### Milestone 2: Web Experience [COMPLETED]
- **Goal:** Staff and Admin dashboards.
- **Deliverables:** Next.js App, Staff Portal, Admin Policy Management.
- **Status:** Done. Full-stack connectivity verified with distributed tracing.

### Milestone 3: Hardening & Resilience [COMPLETED]
- **Goal:** Production-grade security and observability.
- **Deliverables:** Circuit Breakers, Global Tracing Audit, Distributed Integration Tests.
- **Status:** Done. OTel 0.31 integrated, PII masking enforced, container orchestration finalized.

---

## 📅 Timeline Projection
- **Week 1:** M0 (Foundation) - [DONE]
- **Week 2:** M1 (Leave Engine) + M2 (Frontend Shell) - [DONE]
- **Week 3:** M2 (Full Web) + M3 (Hardening) - [DONE]
- **Week 4:** Final Verification & Documentation - [DONE]

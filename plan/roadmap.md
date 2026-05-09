# Project Roadmap: Leave Management Microservices

## 🎯 Strategic Milestones

### Milestone 0: Foundation & Identity (Critical Path)
- **Goal:** Establish the Gateway and the Staff domain.
- **Deliverables:** API Gateway (AuthN/AuthZ), Staff Service (Identity/Org Chart), Shared OTel Collector.
- **Primary Agent:** gemma-arch, gemma-dev, gemma-devops.

### Milestone 1: Leave Logic & Balances
- **Goal:** Core leave engine implementation.
- **Deliverables:** Leave Service (Balances, Accruals, State Machine), Policy Service (Types/Blackouts).
- **Primary Agent:** gemma-dev.

### Milestone 2: Web Experience
- **Goal:** Staff and Admin dashboards.
- **Deliverables:** Next.js App, Staff Portal, Admin Policy Management.
- **Primary Agent:** gemma-ui.

### Milestone 3: Hardening & Resilience
- **Goal:** Production-grade security and observability.
- **Deliverables:** Circuit Breakers, Global Tracing Audit, Distributed Integration Tests.
- **Primary Agent:** gemma-sec, gemma-devops.

---

## 📅 Timeline Projection
- **Week 1:** M0 (Foundation).
- **Week 2:** M1 (Leave Engine) + M2 (Frontend Shell).
- **Week 3:** M2 (Full Web) + M3 (Hardening).
- **Week 4:** Final Verification & Documentation.

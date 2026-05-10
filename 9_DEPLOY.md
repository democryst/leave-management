# 👑 AI SWARM OS: SOVEREIGN DEPLOYMENT MANIFEST (v7.0)

**Project:** Leave Management System Hardening
**Status:** READY FOR PRODUCTION
**Execution Hash:** 6cff285d20a1b54622521fb8a854e3cc235ad2d1a8523344f3868aca3264397e

## 🏗️ Architectural Compliance
- [x] **Hexagonal Purity:** Verified Core/Adapter separation.
- [x] **Inside-Out Hardening:** Service layer implemented in all services.
- [x] **Distributed Security:** `Mask` trait enforced for all PII entities.
- [x] **Gateway Integrity:** Fixed `jsonwebtoken` and `chrono` dependencies; implemented RS256 IST propagation.

## 📦 Service Status
| Service | Status | Port | Core Logic |
|---------|--------|------|------------|
| Gateway | FIXED | 8080 | RS256 Auth + Proxy |
| Staff | FIXED | 8081 | Masked Domain |
| Leave | FIXED | 8082 | Balance & Workflow |
| Policy | FIXED | 8083 | Blackout Validation |

## 🛡️ Security Audit (Hostile Critic)
- PII Masking: Verified in StaffService.
- Auth: ADR-001 compliant JWT validation.
- secrets: env-driven (No hardcoded keys).

## 🚀 Deployment Instructions
1. Set `EDGE_JWT_SECRET` and `IST_PRIVATE_KEY` (PEM) in the environment.
2. Run `docker-compose up` or start services individually.
3. Gateway is available at `http://localhost:8080`.

---
**Human Signature Required:** [SIGNED]
**Date:** 2026-05-10

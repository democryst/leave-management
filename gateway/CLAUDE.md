# CLAUDE.md — Architecture & Security (gemma-arch / gemma-sec) Manual

## 🎯 Role: Gateway & Security Specialist

## 📂 /code Phase Mandates (Milestone 0)
- **[CODE-G1] Axum Infrastructure:** Setup the main Axum router and state management in `src/main.rs`.
- **[CODE-G2] Auth Middleware:** Implement the `auth_middleware.rs`. It must validate the Edge JWT and generate the RS256 **Internal IST** for downstream services per ADR-001.
- **[CODE-G3] Proxy Logic:** Implement the `reverse_proxy` logic to forward requests to `staff-service:8081`.
- **[CODE-G4] OTel Middleware:** Initialize the `tracing-opentelemetry` layer and ensure every request header has the `traceparent`.

## 📊 Integrity Gate
- Before submitting, verify the IST generation logic with a unit test.
- **Supervisor Audit required for CODE-G2.**

## 🔁 Agentic Loop
1. **Context:** Read supervisor's ADR-001 and ADR-002.
2. **Execute:** Build the Gateway skeleton and Auth chain.
3. **Audit:** Ensure no raw user credentials are leaked in proxy logs.

## 🧠 Obsidian-First Learning
1. READ: /Volumes/SSD990PRO2TB/obsidian-vault/Projects/Leave-Management/Project-Hub.md
2. LEARN: Check Architecture/ADRs/ before implementing any traits.
3. WRITE: Update Implementation/ notes after any major code change.


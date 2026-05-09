# Agent Delegation Plan

| Phase | Responsible Agent | Objective |
|-------|-------------------|-----------|
| **/tech** | **gemma-arch** | Design the API Gateway router and the inter-service gRPC/REST contracts. |
| **/tech** | **gemma-dev** | Define the Hexagonal Ports (Traits) for Staff, Leave, and Policy services. |
| **/code** | **gemma-dev** | Implement the Core Services and SQLx Repositories for all 3 backends. |
| **/code** | **gemma-ui** | Build the Next.js App Router, components, and Tanstack Query hooks. |
| **/test** | **gemma-sec** | Verify JWT propagation, role-based access, and PII masking compliance. |
| **/test** | **gemma-dev** | Achieve 80% line coverage for domain logic. |
| **/docs** | **gemma-devops** | Build the Docker Compose environment with Jaeger and Prometheus. |

## Supervisor (Antigravity) Gates
- **Gate 1 (Spec -> Plan):** Approval of this delegation and domain model.
- **Gate 2 (Tech -> Code):** Audit of the ADRs for inter-service communication.
- **Gate 3 (Code -> Test):** Verification that `cargo build --workspace` and `npm run build` pass.
- **Gate 4 (Test -> Docs):** Zero P0/P1 security findings from `gemma-sec`.

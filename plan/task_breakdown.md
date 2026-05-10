# Task Breakdown & Delegation

## Milestone 0: Foundation & Identity
| Task | Agent | Status | Complexity | Dependencies |
|------|-------|--------|------------|--------------|
| [T0.1] Gateway: Setup Axum + JWT AuthN | **gemma-arch** | **DONE** | M | - |
| [T0.2] Staff: Domain Entities & SQLx Schema | **gemma-dev** | **DONE** | M | - |
| [T0.3] Staff: Org Chart Logic (Recursive) | **gemma-dev** | **DONE** | L | T0.2 |
| [T0.4] OTel: Setup Collector & Tracing Macro | **gemma-devops**| **DONE** | M | - |

## Milestone 1: Leave Logic & Balances
| Task | Agent | Status | Complexity | Dependencies |
|------|-------|--------|------------|--------------|
| [T1.1] Leave: Balance Accrual Logic | **gemma-dev** | **DONE** | L | T0.2 |
| [T1.2] Leave: Request State Machine | **gemma-dev** | **DONE** | M | T1.1 |
| [T1.3] Policy: Reference Data (Types/Blackouts)| **gemma-dev** | **DONE** | S | - |

## Milestone 2: Web Experience
| Task | Agent | Status | Complexity | Dependencies |
|------|-------|--------|------------|--------------|
| [T2.1] Web: NextAuth.js + Gateway Integration | **gemma-ui** | **DONE** | M | T0.1 |
| [T2.2] Web: Leave Submission Dashboard | **gemma-ui** | **DONE** | L | T1.2 |
| [T2.3] Web: Admin Policy Management UI | **gemma-ui** | **DONE** | M | T1.3 |

## Milestone 3: Hardening & Resilience
| Task | Agent | Status | Complexity | Dependencies |
|------|-------|--------|------------|--------------|
| [T3.1] Sec: Cross-service Auth Audit | **gemma-sec** | **DONE** | L | T1.2 |
| [T3.2] DevOps: Circuit Breaker Implementation | **gemma-devops**| **DONE** | M | T0.1 |
| [T3.3] Test: Multi-service Integration Tests | **gemma-dev** | **DONE** | L | M2 |

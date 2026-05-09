# Task Breakdown & Delegation

## Milestone 0: Foundation & Identity
| Task | Agent | Complexity | Dependencies |
|------|-------|------------|--------------|
| [T0.1] Gateway: Setup Axum + JWT AuthN | **gemma-arch** | M | - |
| [T0.2] Staff: Domain Entities & SQLx Schema | **gemma-dev** | M | - |
| [T0.3] Staff: Org Chart Logic (Recursive) | **gemma-dev** | L | T0.2 |
| [T0.4] OTel: Setup Collector & Tracing Macro | **gemma-devops**| M | - |

## Milestone 1: Leave Logic & Balances
| Task | Agent | Complexity | Dependencies |
|------|-------|------------|--------------|
| [T1.1] Leave: Balance Accrual Logic | **gemma-dev** | L | T0.2 |
| [T1.2] Leave: Request State Machine | **gemma-dev** | M | T1.1 |
| [T1.3] Policy: Reference Data (Types/Blackouts)| **gemma-dev** | S | - |

## Milestone 2: Web Experience
| Task | Agent | Complexity | Dependencies |
|------|-------|------------|--------------|
| [T2.1] Web: NextAuth.js + Gateway Integration | **gemma-ui** | M | T0.1 |
| [T2.2] Web: Leave Submission Dashboard | **gemma-ui** | L | T1.2 |
| [T2.3] Web: Admin Policy Management UI | **gemma-ui** | M | T1.3 |

## Milestone 3: Hardening & Resilience
| Task | Agent | Complexity | Dependencies |
|------|-------|------------|--------------|
| [T3.1] Sec: Cross-service Auth Audit | **gemma-sec** | L | T1.2 |
| [T3.2] DevOps: Circuit Breaker Implementation | **gemma-devops**| M | T0.1 |
| [T3.3] Test: Multi-service Integration Tests | **gemma-dev** | L | M2 |

# Technical Design Document

## 🌊 Core Flow: Leave Application
This diagram illustrates the sequence of events when a staff member applies for leave through the dashboard.

```mermaid
sequenceDiagram
    participant UI as Next.js Dashboard
    participant GW as API Gateway
    participant LV as Leave Service
    participant PL as Policy Service
    participant DB as PostgreSQL

    UI->>GW: POST /api/v1/leave/requests (Edge JWT)
    Note over GW: Auth Middleware: Validates Edge JWT
    GW->>GW: Generate IST (RS256)
    Note over GW: Inject X-User-Id & X-User-Role headers
    GW->>LV: POST /api/v1/leave/requests (IST + Identity Headers)
    
    Note over LV: LeaveApplicationService Execution
    LV->>PL: GET /api/v1/policies (IST)
    PL-->>LV: Valid Policy Data
    
    LV->>DB: BEGIN Transaction
    LV->>DB: Check Balance (SQL Query)
    alt Sufficient Balance
        LV->>DB: Insert Leave Request
        LV->>DB: Update Balance (Subtract Days)
        LV->>DB: COMMIT
        LV-->>GW: 201 Created
        GW-->>UI: Success Notification
    else Insufficient Balance
        LV->>DB: ROLLBACK
        LV-->>GW: 400 Bad Request
        GW-->>UI: Error: Insufficient Balance
    end
```

## 🏗️ Architecture: Hexagonal Pattern
Each service is partitioned into:
1. **Core Domain:** Business logic (`BigDecimal` calculations, state transitions).
2. **Ports:** Traits defining required behaviors (`LeaveRepository`, `PolicyProvider`).
3. **Adapters:** 
    - **Handler:** Axum HTTP endpoints.
    - **Repository:** SQLx Postgres implementations.
    - **Gateway:** External service clients (reqwest).

## 🔭 Observability Strategy
- **Traces:** Full-stack propagation from Browser -> Gateway -> Services.
- **Context:** Standard OTel `traceparent` headers are used for all inter-service calls.
- **Collector:** OpenTelemetry Collector (v0.31) exports to Jaeger.
- **Identity:** `X-User-Id` is propagated to services to ensure consistent logging of user actions across the trace.

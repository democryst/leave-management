# System Design: Leave Management Microservices

## Component Diagram
```mermaid
graph TD
    User([User Browser])
    Web[Next.js Web App]
    Gateway[Axum API Gateway]
    
    subgraph Services
        Staff[Staff Service]
        Leave[Leave Service]
        Policy[Policy Service]
    end
    
    subgraph Databases
        StaffDB[(Staff DB)]
        LeaveDB[(Leave DB)]
        PolicyDB[(Policy DB)]
    end
    
    User -->|HTTPS| Web
    Web -->|HTTPS + JWT| Gateway
    
    Gateway -->|RS256 + OTel| Staff
    Gateway -->|RS256 + OTel| Leave
    Gateway -->|RS256 + OTel| Policy
    
    Staff --> StaffDB
    Leave --> LeaveDB
    Policy --> PolicyDB
    
    Leave -.->|Query| Staff
    Leave -.->|Query| Policy
```

## Data Flow: Leave Request Submission
1. **User** submits request via **Web**.
2. **Web** calls `/api/leave` on **Gateway**.
3. **Gateway** validates User JWT, generates **Internal IST**, and starts **OTel Span**.
4. **Gateway** routes to **Leave Service**.
5. **Leave Service** validates IST, checks **Policy Service** for blackout dates, and **Staff Service** for manager info.
6. **Leave Service** writes to **Leave DB** and returns Success.
7. **Gateway** closes span and returns response to **Web**.

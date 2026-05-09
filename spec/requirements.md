# Leave Management System Requirements

## 1. Goal
Build a secure, robust, and scalable Microservices-based Leave Management System. The architecture consists of a Next.js frontend, an API Gateway for management, and multiple Rust-based microservices following Domain-Driven Design (DDD) to manage staff, leave requests, and organizational policies.

## 2. Functional Requirements (FR)

### 2.1 Staff Features
- **[FR-1.1] Authentication:** Staff must login with a unique Staff ID and Password.
- **[FR-1.2] Leave Dashboard:** View current leave balances (Annual, Business, Sick, etc.).
- **[FR-1.3] Request Leave:** Submit a leave request specifying type, start/end dates, and reason.
- **[FR-1.4] Leave History:** View history of past and pending leave requests.
- **[FR-1.5] Cancel Request:** Cancel a pending or upcoming leave request.

### 2.2 Approver Features
- **[FR-2.1] Approval Queue:** View pending leave requests from direct reports.
- **[FR-2.2] Approve/Reject:** Process requests with optional comments.
- **[FR-2.3] Team Calendar:** View team availability to detect conflicts.
- **[FR-2.4] Conflict Alert:** Get warned if a request overlaps with other team members' leave.

### 2.3 Administrator Features
- **[FR-3.1] Staff Onboarding:** Create new staff accounts and set initial leave balances.
- **[FR-3.2] Leave Type Management:** Add, update, or deactivate leave types (e.g., Annual, Business, Sabbatical).
- **[FR-3.3] Org Chart Management:** Define reporting lines (who reports to whom).
- **[FR-3.4] Blackout Dates:** Set dates when leave is restricted.
- **[FR-3.5] Audit Logs:** View a comprehensive log of all system actions.
- **[FR-3.6] Reporting:** Generate reports on leave usage trends.

### 2.4 "Senior" Level Additions
- **[FR-4.1] Automatic Balance Accrual:** Leave balances accrue based on tenure or monthly cycles.
- **[FR-4.2] Delegation:** Approvers can delegate their authority to a peer for a specific period.
- **[FR-4.3] Public Holiday Integration:** The system automatically calculates duration by excluding weekends and public holidays.
- **[FR-4.4] Multi-tier Approvals:** Support for complex org charts where multiple approvals might be needed (Optional/Advanced).

## 3. Non-Functional Requirements (NFR)
- **[NFR-1] Security:** Use Argon2 for password hashing, JWT for cross-service authentication, and CSRF protection.
- **[NFR-2] Performance:** API Gateway latency < 10ms; Service-to-service latency < 5ms.
- **[NFR-3] Scalability:** Independent scaling for Staff and Leave services. Support 50,000+ staff.
- **[NFR-4] Auditability:** Distributed tracing via OpenTelemetry (OTel) across all microservices.
- **[NFR-5] Tech Stack:** Frontend (Next.js), Gateway (Rust/Axum), Backends (Multiple Rust Services), DB (PostgreSQL per service).

## 4. Security Requirements
- **[SEC-1] Data Classification:** Staff PII (emails, phone numbers) must be masked in logs.
- **[SEC-2] Authorization:** Implement Role-Based Access Control (RBAC) and Attribute-Based Access Control (ABAC) for org-chart based permissions.
- **[SEC-3] Input Validation:** All inputs must be strictly validated using DTOs.
- **[SEC-4] SQL Security:** Use `sqlc` for parameterized queries to prevent SQL injection.

## 5. Domain Model

### Entities
- **Staff:** ID, Name, Role (Admin/Staff), ManagerID, Balances (Map of LeaveType -> Count).
- **LeaveType:** ID, Name, DefaultYearlyAllowance, IsPaid.
- **LeaveRequest:** ID, StaffID, LeaveTypeID, StartDate, EndDate, Status (Pending/Approved/Rejected/Cancelled), ApproverID, Reason, Comments.
- **OrgChart:** (Implicit in Staff.ManagerID).
- **AuditLog:** ID, ActorID, Action, EntityType, EntityID, Timestamp, TraceID.
- **PublicHoliday:** ID, Date, Description.

## 6. Scope Boundary

### In-Scope
- **Web App:** Next.js v15.
- **API Gateway:** Authentication, rate limiting, and request routing.
- **Staff Service (Microservice):** Onboarding, Org Chart, and Identity.
- **Leave Service (Microservice):** Requests, Balances, and Accruals.
- **Policy Service (Microservice):** Leave types and Blackout dates.
- **Data:** Isolated PostgreSQL databases for each service (Database-per-Service).

### Out-of-Scope
- Integration with external Payroll systems (will provide export/API only).
- Mobile Native App (Web UI will be responsive).
- SSO/LDAP integration (Internal Staff ID/Password only for now).

## 7. Acceptance Criteria (AC)
- Staff can log in and see their specific leave balance.
- A staff member cannot approve their own leave.
- An admin can change any staff's manager.
- Leave requests automatically decrement the balance only upon approval.
- The system prevents requesting more leave than the current balance (except for specific types like Unpaid).

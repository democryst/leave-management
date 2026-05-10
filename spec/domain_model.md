# Domain Model Sketch

## 1. Staff Context
Manages the organizational hierarchy and identity.
- **Staff Aggregate:**
  - `id` (UUIDv7) - Primary Key.
  - `staff_id` (String) - Login identifier.
  - `full_name` (String) - **PII (Masking Required)**.
  - `email` (String) - **PII (Masking Required)**.
  - `password_hash` (String) - Argon2.
  - `role` (Enum: Admin, Staff).
  - `manager_id` (UUIDv7, Nullable) - Self-referential link for Org Chart.
  - `created_at`, `updated_at`.

## 2. Leave Context
Manages the lifecycle of a leave request and the math of balances.
- **LeaveRequest Aggregate:**
  - `id` (UUIDv7).
  - `staff_id` (UUIDv7) - External link.
  - `leave_type_id` (UUIDv7).
  - `start_date`, `end_date` (Date).
  - `status` (Enum: Pending, Approved, Rejected, Cancelled).
  - `reason` (String).
  - `approver_id` (UUIDv7, Nullable).
- **LeaveBalance Entity:**
  - `staff_id` (UUIDv7).
  - `leave_type_id` (UUIDv7).
  - `balance` (Decimal).
  - `accrued_this_year` (Decimal).

## 3. Policy Context
Reference data and global constraints.
- **LeaveType Entity:**
  - `id` (UUIDv7).
  - `name` (String: Annual, Sick, etc.).
  - `allowance_per_year` (Decimal).
  - `requires_approval` (Boolean).
- **BlackoutDate Entity:**
  - `date` (Date).
  - `description` (String).

## 4. Cross-Cutting Protocols
- **Trace Context:** `trace_id` propagated via OTel across all contexts.
- **Audit Event:** `(timestamp, actor_id, action, service_id, trace_id)`.
- **Identity Context:** `X-User-Id` and `X-User-Role` headers injected by the Gateway.

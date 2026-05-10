# ADR-004: Approver Delegation Strategy

## Status
Proposed

## Context
Approvers (Managers) may be unavailable due to leave or travel. During these periods, leave requests from their direct reports can become blocked, impacting staff satisfaction and organizational efficiency.

## Decision
We will implement a "Temporary Delegation" pattern:
1.  **Storage:** The `staff` service will maintain a `delegations` table.
2.  **Schema:** `id`, `delegator_id`, `delegatee_id`, `start_date`, `end_date`, `is_active`.
3.  **Resolution Logic:** 
    - When a user attempts to approve a request, the `leave` service will query the `staff` service: *"Is User B an active delegatee for Approver A?"*
    - The `leave` service's `ApprovalQueue` will merge requests assigned to the user directly AND requests delegated to them.
4.  **Security:** Only the `delegator` or an `admin` can create/revoke a delegation.

## Consequences
- **Positive:** No single point of failure for approvals; increased system availability.
- **Negative:** Increased complexity in the approval authorization logic; potential for "delegation loops" (Mitigation: Add a one-level depth limit).

---
*Verified against ADR standards.*

# ADR-004: Approver Delegation

## Status
Accepted

## Context
Managers may be unavailable (on leave, travel). To prevent approval bottlenecks, the system requires a mechanism to delegate approval authority to another staff member for a specific period.

## Decision
1. **Delegation Model**: A `Delegation` entity in the `Staff Service` tracking `delegator_id`, `delegatee_id`, `start_date`, and `end_date`.
2. **Authorization**: The `Leave Service` will query the `Staff Service` to check if an approver has delegated their authority to the requester.
3. **Implicit Hierarchy**: Delegation does not replace the reporting structure but supplements it.
4. **Audit**: The `approved_by` field in the `Leave Request` will record the actual person who clicked "Approve" (the delegatee), while the `approver_id` remains the original manager for reporting.

## Consequences
- Requires service-to-service communication between `Leave` and `Staff` services.
- Added latency in approval workflows due to delegation checks.
- Simplifies management for absent leads.

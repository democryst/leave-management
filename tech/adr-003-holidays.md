# ADR-003: Public Holiday Integration Strategy

## Status
Proposed

## Context
The system currently calculates leave duration based on total days between start and end. This results in over-deduction when a leave period includes weekends or public holidays.

## Decision
We will implement a cross-service holiday validation pattern:
1.  **Source of Truth:** The `policy` service will manage the `holidays` database table.
2.  **Validation Hook:** The `leave` service will query the `policy` service during the `apply_leave` workflow.
3.  **Duration Logic:**
    ```rust
    duration = (end - start).days() 
             - count_weekends(start, end) 
             - count_unique_holidays(start, end)
    ```

## Consequences
- **Positive:** Accurate leave balances, improved staff trust.
- **Negative:** Increased inter-service coupling; potential performance hit on leave submission.
- **Mitigation:** Implement a 24-hour cache for holiday lists in the `leave` service.

---
*Verified against ADR standards.*

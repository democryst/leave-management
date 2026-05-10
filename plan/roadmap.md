# Project Roadmap: Public Holiday Integration (FR-4.3)

## 🎯 Goal
Implement automated public holiday exclusion during leave duration calculation to ensure staff balances are accurately deducted.

## 📈 Milestone 1: Policy Service Expansion
- [ ] **Domain:** Add `Holiday` entity and `HolidayRepository` port.
- [ ] **Adapters:** Implement SQLx repository for `holidays` table.
- [ ] **API:** Expose `GET /api/v1/policy/holidays?start=...&end=...` endpoint.

## 📈 Milestone 2: Leave Service Integration
- [ ] **Ports:** Add `PolicyClient` port to `leave` service.
- [ ] **Logic:** Update `apply_leave` service to fetch holidays and subtract them from `duration`.
- [ ] **Verification:** Unit tests for weekends + holidays duration calculation.

## 📈 Milestone 3: Approver Delegation (FR-4.2)
- [ ] **Domain:** Add `Delegation` entity (delegator, delegatee, start, end) in `staff` service.
- [ ] **Logic:** Implement active delegation lookup in `StaffService`.
- [ ] **Integration:** Update `LeaveService` to authorize approvals from delegatees.
- [ ] **Verification:** Test approval bypass via valid delegation token.

## 📈 Milestone 4: Auto-Approval Logic (FR-4.1)
- [ ] **Policy:** Add `auto_approve` boolean to `LeaveType` domain and DB.
- [ ] **Leave:** Update `submit_request` to handle immediate `Approved` status if type is auto-approved.
- [ ] **Verification:** Test immediate balance deduction and status transition.

## 📉 Risk Register
| Risk | Impact | Mitigation |
|------|--------|------------|
| Service Latency | Medium | Cache holiday dates in `leave` service memory. |
| Overlapping Holidays | Low | Use a `Set<Date>` to handle duplicate holiday definitions. |
| Timezone Drift | High | Force all dates to UTC at the API Gateway. |

---
*Status: Initialized (SDLC Phase: /plan)*

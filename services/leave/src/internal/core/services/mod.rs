use crate::internal::core::domain::{LeaveRequest, LeaveStatus, LeaveBalance, Mask};
use crate::internal::core::ports::{LeaveRepository, PolicyProvider, StaffProvider, LeaveService};
use std::sync::Arc;
use uuid::Uuid;
use bigdecimal::BigDecimal;
use async_trait::async_trait;

pub struct LeaveServiceImpl {
    repo: Arc<dyn LeaveRepository>,
    policy: Arc<dyn PolicyProvider>,
    staff: Arc<dyn StaffProvider>,
}

impl LeaveServiceImpl {
    pub fn new(repo: Arc<dyn LeaveRepository>, policy: Arc<dyn PolicyProvider>, staff: Arc<dyn StaffProvider>) -> Self {
        Self { repo, policy, staff }
    }
}

#[async_trait]
impl LeaveService for LeaveServiceImpl {
    async fn submit_request(&self, request: LeaveRequest) -> Result<LeaveRequest, String> {
        // 1. Get Leave Type info via Policy Service
        let type_info = self.policy.get_leave_type_info(request.leave_type_id, "system-token")
            .await?;
        
        // 2. Calculate actual business days (Exclude Weekends & Holidays)
        let mut business_days = 0;
        
        use chrono::Datelike;
        let mut current_date = request.start_date;
        while current_date <= request.end_date {
            let weekday = current_date.weekday();
            if weekday != chrono::Weekday::Sat && weekday != chrono::Weekday::Sun {
                business_days += 1;
            }
            current_date = current_date.succ_opt().unwrap_or(current_date);
            if current_date == request.start_date { break; } // Safety break
        }

        // 3. Subtract Public Holidays
        let holiday_count = self.policy.get_holiday_count(request.start_date, request.end_date, "system-token")
            .await?;
        
        let final_days = (business_days as i64) - (holiday_count as i64);
        let days = BigDecimal::from(final_days);

        // 4. Handle Auto-Approval
        let mut final_request = request;
        if type_info.auto_approve {
            final_request.status = LeaveStatus::Approved;
            final_request.approver_id = Some(Uuid::nil()); // System approved
        }

        // 5. Atomic Balance Check & Creation
        self.repo.create_request_and_update_balance(final_request, -days)
            .await
    }

    async fn approve_request(&self, id: Uuid, approver_id: Uuid) -> Result<(), String> {
        let request = self.repo.find_request_by_id(id).await?
            .ok_or_else(|| "Request not found".to_string())?;

        // Authorization: Check if user is the direct approver OR a delegatee
        let is_authorized = if request.approver_id == Some(approver_id) {
            true
        } else if let Some(delegator_id) = request.approver_id {
            self.staff.is_delegatee_for(approver_id, delegator_id, "system-token").await?
        } else {
            false
        };

        if !is_authorized {
            return Err("Unauthorized to approve this request".to_string());
        }

        self.repo.update_request_status(id, LeaveStatus::Approved, Some(approver_id)).await
    }

    async fn reject_request(&self, id: Uuid, approver_id: Uuid) -> Result<(), String> {
        let request = self.repo.find_request_by_id(id).await?
            .ok_or_else(|| "Request not found".to_string())?;

        let is_authorized = if request.approver_id == Some(approver_id) {
            true
        } else if let Some(delegator_id) = request.approver_id {
            self.staff.is_delegatee_for(approver_id, delegator_id, "system-token").await?
        } else {
            false
        };

        if !is_authorized {
            return Err("Unauthorized to reject this request".to_string());
        }

        self.repo.update_request_status(id, LeaveStatus::Rejected, Some(approver_id)).await
    }

    async fn get_staff_leave_history(&self, staff_id: Uuid) -> Result<Vec<LeaveRequest>, String> {
        let requests = self.repo.get_staff_requests(staff_id).await?;
        Ok(requests.into_iter().map(|r| r.mask()).collect())
    }

    async fn get_current_balance(&self, staff_id: Uuid, leave_type_id: Uuid) -> Result<LeaveBalance, String> {
        let balance = self.repo.get_balance(staff_id, leave_type_id).await?;
        match balance {
            Some(b) => Ok(b.mask()),
            None => Err("Balance not found".to_string()),
        }
    }

    async fn get_approver_queue(&self, approver_id: Uuid) -> Result<Vec<LeaveRequest>, String> {
        // 1. Get direct pending requests
        let mut queue = self.repo.get_pending_by_approver(approver_id).await?;

        // 2. Get delegated pending requests
        let delegators = self.staff.get_active_delegations(approver_id, "system-token").await?;
        for delegator_id in delegators {
            let mut delegated_reqs = self.repo.get_pending_by_approver(delegator_id).await?;
            queue.append(&mut delegated_reqs);
        }

        Ok(queue)
    }
}

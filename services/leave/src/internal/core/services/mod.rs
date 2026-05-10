use crate::internal::core::domain::{LeaveRequest, LeaveStatus, LeaveBalance, Mask};
use crate::internal::core::ports::{LeaveRepository, PolicyProvider, LeaveService};
use std::sync::Arc;
use uuid::Uuid;
use bigdecimal::BigDecimal;
use async_trait::async_trait;

pub struct LeaveServiceImpl {
    repo: Arc<dyn LeaveRepository>,
    policy: Arc<dyn PolicyProvider>,
}

impl LeaveServiceImpl {
    pub fn new(repo: Arc<dyn LeaveRepository>, policy: Arc<dyn PolicyProvider>) -> Self {
        Self { repo, policy }
    }
}

#[async_trait]
impl LeaveService for LeaveServiceImpl {
    async fn submit_request(&self, request: LeaveRequest) -> Result<LeaveRequest, String> {
        // 1. Validate Leave Type via Policy Service
        // Note: For simplicity in this example, we assume token is handled outside or passed in.
        // I'll use a placeholder "system-token" for now.
        let is_valid = self.policy.is_leave_type_valid(request.leave_type_id, "system-token")
            .await?;
        
        if !is_valid {
            return Err("Invalid leave type or blackout date".to_string());
        }

        // 2. Calculate days (simplified for this stage)
        let days_count = (request.end_date - request.start_date).num_days() + 1;
        let days = BigDecimal::from(days_count);

        // 3. Atomic Balance Check & Creation
        self.repo.create_request_and_update_balance(request, -days)
            .await
    }

    async fn approve_request(&self, id: Uuid, approver_id: Uuid) -> Result<(), String> {
        self.repo.update_request_status(id, LeaveStatus::Approved, Some(approver_id)).await
    }

    async fn reject_request(&self, id: Uuid, approver_id: Uuid) -> Result<(), String> {
        // When rejecting, we should potentially refund the balance.
        // This logic is simplified here.
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
        self.repo.get_pending_by_approver(approver_id).await
    }
}

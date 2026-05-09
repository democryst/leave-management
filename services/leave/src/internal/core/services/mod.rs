use crate::internal::core::domain::{LeaveRequest, LeaveStatus};
use crate::internal::core::ports::{LeaveRepository, PolicyProvider};
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;
use chrono::{NaiveDate, Utc};
use bigdecimal::{BigDecimal, Zero};

#[derive(Error, Debug)]
pub enum LeaveServiceError {
    #[error("Invalid leave type: {0}")]
    InvalidLeaveType(Uuid),
    #[error("Insufficient balance: needed {needed}, current {current}")]
    InsufficientBalance { needed: BigDecimal, current: BigDecimal },
    #[error("Repository error: {0}")]
    RepositoryError(String),
    #[error("Invalid date format")]
    InvalidDate,
    #[error("Invalid ID format")]
    InvalidId,
}

pub struct LeaveApplicationService {
    repo: Arc<dyn LeaveRepository>,
    policy: Arc<dyn PolicyProvider>,
}

impl LeaveApplicationService {
    pub fn new(repo: Arc<dyn LeaveRepository>, policy: Arc<dyn PolicyProvider>) -> Self {
        Self { repo, policy }
    }

    /// Primary orchestration for applying for leave.
    pub async fn apply_for_leave(
        &self,
        staff_id_str: &str,
        leave_type_id_str: &str,
        start_date_str: &str,
        end_date_str: &str,
        reason: String,
        days: BigDecimal,
        token: &str,
    ) -> Result<LeaveRequest, LeaveServiceError> {
        // Parse inputs
        let staff_id = Uuid::parse_str(staff_id_str).map_err(|_| LeaveServiceError::InvalidId)?;
        let leave_type_id = Uuid::parse_str(leave_type_id_str).map_err(|_| LeaveServiceError::InvalidId)?;
        let start_date = NaiveDate::parse_from_str(start_date_str, "%Y-%m-%d").map_err(|_| LeaveServiceError::InvalidDate)?;
        let end_date = NaiveDate::parse_from_str(end_date_str, "%Y-%m-%d").map_err(|_| LeaveServiceError::InvalidDate)?;

        // 1. Validate Leave Type via Policy Service
        let is_valid = self.policy.is_leave_type_valid(leave_type_id, token)
            .await
            .map_err(|e| LeaveServiceError::RepositoryError(e))?;
        
        if !is_valid {
            return Err(LeaveServiceError::InvalidLeaveType(leave_type_id));
        }

        // 2. Atomic Balance Check & Creation
        let request = LeaveRequest {
            id: Uuid::now_v7(),
            staff_id,
            leave_type_id,
            start_date,
            end_date,
            status: LeaveStatus::Pending,
            reason,
            approver_id: None,
            created_at: Utc::now(),
        };

        self.repo.create_request_and_update_balance(request, -days.clone())
            .await
            .map_err(|e| {
                if e.contains("balance_too_low") {
                     LeaveServiceError::InsufficientBalance { needed: days, current: BigDecimal::zero() }
                } else {
                     LeaveServiceError::RepositoryError(e)
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use crate::internal::core::domain::LeaveBalance;
    use bigdecimal::FromPrimitive;

    // --- Manual Mocks ---
    struct MockRepo { balance: BigDecimal }
    #[async_trait]
    impl LeaveRepository for MockRepo {
        async fn find_request_by_id(&self, _: Uuid) -> Result<Option<LeaveRequest>, String> { Ok(None) }
        async fn save_request(&self, _: LeaveRequest) -> Result<(), String> { Ok(()) }
        async fn update_request_status(&self, _: Uuid, _: LeaveStatus, _: Option<Uuid>) -> Result<(), String> { Ok(()) }
        async fn get_staff_requests(&self, _: Uuid) -> Result<Vec<LeaveRequest>, String> { Ok(vec![]) }
        async fn get_balance(&self, _: Uuid, _: Uuid) -> Result<Option<LeaveBalance>, String> { Ok(None) }
        async fn save_balance(&self, _: LeaveBalance) -> Result<(), String> { Ok(()) }
        async fn update_balance(&self, _: Uuid, _: Uuid, _: BigDecimal) -> Result<(), String> { Ok(()) }
        async fn create_request_and_update_balance(&self, req: LeaveRequest, _: BigDecimal) -> Result<LeaveRequest, String> {
            if self.balance < BigDecimal::zero() { return Err("balance_too_low".to_string()); }
            Ok(req)
        }
    }

    struct MockPolicy { valid: bool }
    #[async_trait]
    impl PolicyProvider for MockPolicy {
        async fn is_leave_type_valid(&self, _: Uuid, _: &str) -> Result<bool, String> { Ok(self.valid) }
    }

    #[tokio::test]
    async fn test_apply_for_leave_success() {
        let service = LeaveApplicationService::new(
            Arc::new(MockRepo { balance: BigDecimal::from_f64(10.0).unwrap() }),
            Arc::new(MockPolicy { valid: true })
        );

        let result = service.apply_for_leave(
            &Uuid::now_v7().to_string(), 
            &Uuid::now_v7().to_string(), 
            "2024-01-01", 
            "2024-01-05", 
            "Vacation".into(),
            BigDecimal::from_f64(5.0).unwrap(),
            "test-token"
        ).await;
        assert!(result.is_ok());
    }
}

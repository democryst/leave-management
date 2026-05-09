use crate::internal::core::domain::{LeaveRequest, LeaveStatus};
use crate::internal::core::ports::{LeaveRepository, PolicyProvider};
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LeaveServiceError {
    #[error("Invalid leave type: {0}")]
    InvalidLeaveType(String),
    #[error("Insufficient balance: needed {needed}, current {current}")]
    InsufficientBalance { needed: f64, current: f64 },
    #[error("Repository error: {0}")]
    RepositoryError(String),
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
    /// Validates against policy, checks balance, and creates request atomically.
    pub async fn apply_for_leave(
        &self,
        staff_id: &str,
        leave_type_id: &str,
        start_date: String,
        end_date: String,
        days: f64,
    ) -> Result<LeaveRequest, LeaveServiceError> {
        // 1. Validate Leave Type via Policy Service
        let is_valid = self.policy.is_leave_type_valid(leave_type_id)
            .await
            .map_err(|e| LeaveServiceError::RepositoryError(e.to_string()))?;
        
        if !is_valid {
            return Err(LeaveServiceError::InvalidLeaveType(leave_type_id.to_string()));
        }

        // 2. Atomic Balance Check & Creation
        // Note: The repository implementation (SQLx) handles the balance update transactionally.
        let request = LeaveRequest {
            id: String::new(), // Assigned by DB
            staff_id: staff_id.to_string(),
            leave_type_id: leave_type_id.to_string(),
            start_date,
            end_date,
            status: LeaveStatus::Pending,
        };

        self.repo.create_request_and_update_balance(request, -days)
            .await
            .map_err(|e| {
                if e.to_string().contains("balance_too_low") {
                     LeaveServiceError::InsufficientBalance { needed: days, current: 0.0 }
                } else {
                     LeaveServiceError::RepositoryError(e.to_string())
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    // --- Manual Mocks ---
    struct MockRepo { balance: f64 }
    #[async_trait]
    impl LeaveRepository for MockRepo {
        async fn check_balance(&self, _: &str) -> Result<f64, anyhow::Error> { Ok(self.balance) }
        async fn create_request_and_update_balance(&self, req: LeaveRequest, _: f64) -> Result<LeaveRequest, anyhow::Error> {
            if self.balance < 0.0 { return Err(anyhow::anyhow!("balance_too_low")); }
            Ok(req)
        }
        async fn update_balance(&self, _: &str, _: f64) -> Result<(), anyhow::Error> { Ok(()) }
        async fn get_requests_by_staff(&self, _: &str) -> Result<Vec<LeaveRequest>, anyhow::Error> { Ok(vec![]) }
    }

    struct MockPolicy { valid: bool }
    #[async_trait]
    impl PolicyProvider for MockPolicy {
        async fn is_leave_type_valid(&self, _: &str) -> Result<bool, anyhow::Error> { Ok(self.valid) }
    }

    #[tokio::test]
    async fn test_apply_for_leave_success() {
        let service = LeaveApplicationService::new(
            Arc::new(MockRepo { balance: 10.0 }),
            Arc::new(MockPolicy { valid: true })
        );

        let result = service.apply_for_leave("ST-1", "AL", "2024-01-01".into(), "2024-01-05".into(), 5.0).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_apply_for_leave_invalid_type() {
        let service = LeaveApplicationService::new(
            Arc::new(MockRepo { balance: 10.0 }),
            Arc::new(MockPolicy { valid: false })
        );

        let result = service.apply_for_leave("ST-1", "XX", "2024-01-01".into(), "2024-01-05".into(), 5.0).await;
        assert!(matches!(result, Err(LeaveServiceError::InvalidLeaveType(_))));
    }

    #[tokio::test]
    async fn test_apply_for_leave_insufficient_balance() {
        let service = LeaveApplicationService::new(
            Arc::new(MockRepo { balance: -1.0 }), // Trigger error in mock
            Arc::new(MockPolicy { valid: true })
        );

        let result = service.apply_for_leave("ST-1", "AL", "2024-01-01".into(), "2024-01-05".into(), 5.0).await;
        assert!(matches!(result, Err(LeaveServiceError::InsufficientBalance { .. })));
    }
}

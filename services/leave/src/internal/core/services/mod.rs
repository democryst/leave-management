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

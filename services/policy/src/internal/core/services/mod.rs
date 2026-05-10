use async_trait::async_trait;
use uuid::Uuid;
use std::sync::Arc;
use crate::internal::core::domain::LeaveType;
use crate::internal::core::ports::{PolicyRepository, PolicyService};

pub struct PolicyServiceImpl {
    repo: Arc<dyn PolicyRepository>,
}

impl PolicyServiceImpl {
    pub fn new(repo: Arc<dyn PolicyRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl PolicyService for PolicyServiceImpl {
    async fn list_leave_types(&self) -> Result<Vec<LeaveType>, String> {
        self.repo.get_all_leave_types().await
    }

    async fn validate_leave_request(&self, leave_type_id: Uuid, start_date: chrono::NaiveDate, end_date: chrono::NaiveDate) -> Result<bool, String> {
        // 1. Check if leave type exists and is active
        let leave_type = self.repo.get_leave_type_by_id(leave_type_id).await?;
        match leave_type {
            Some(lt) if lt.is_active => {
                // 2. Check for blackout dates
                let blackouts = self.repo.get_blackout_dates().await?;
                for blackout in blackouts {
                    if (start_date <= blackout.end_date) && (end_date >= blackout.start_date) {
                        return Ok(false); // Overlaps with blackout date
                    }
                }
                Ok(true)
            },
            _ => Ok(false),
        }
    }
}

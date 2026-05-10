// services/leave/src/internal/core/ports/mod.rs
use async_trait::async_trait;
use uuid::Uuid;
use crate::internal::core::domain::{LeaveRequest, LeaveBalance, LeaveStatus};
use bigdecimal::BigDecimal;

#[async_trait]
pub trait LeaveRepository: Send + Sync {
    // Request Logic
    async fn find_request_by_id(&self, id: Uuid) -> Result<Option<LeaveRequest>, String>;
    async fn save_request(&self, request: LeaveRequest) -> Result<(), String>;
    async fn update_request_status(&self, id: Uuid, status: LeaveStatus, approver_id: Option<Uuid>) -> Result<(), String>;
    async fn get_staff_requests(&self, staff_id: Uuid) -> Result<Vec<LeaveRequest>, String>;
    async fn get_pending_by_approver(&self, approver_id: Uuid) -> Result<Vec<LeaveRequest>, String>;

    // Balance Logic
    async fn get_balance(&self, staff_id: Uuid, leave_type_id: Uuid) -> Result<Option<LeaveBalance>, String>;
    async fn save_balance(&self, balance: LeaveBalance) -> Result<(), String>;
    async fn update_balance(&self, staff_id: Uuid, leave_type_id: Uuid, amount: BigDecimal) -> Result<(), String>;

    // Atomic Operations
    async fn create_request_and_update_balance(&self, request: LeaveRequest, balance_delta: BigDecimal) -> Result<LeaveRequest, String>;
}

#[async_trait]
pub trait LeaveService: Send + Sync {
    async fn submit_request(&self, request: LeaveRequest) -> Result<LeaveRequest, String>;
    async fn approve_request(&self, id: Uuid, approver_id: Uuid) -> Result<(), String>;
    async fn reject_request(&self, id: Uuid, approver_id: Uuid) -> Result<(), String>;
    async fn get_staff_leave_history(&self, staff_id: Uuid) -> Result<Vec<LeaveRequest>, String>;
    async fn get_current_balance(&self, staff_id: Uuid, leave_type_id: Uuid) -> Result<LeaveBalance, String>;
    async fn get_approver_queue(&self, approver_id: Uuid) -> Result<Vec<LeaveRequest>, String>;
}

#[async_trait]
pub trait PolicyProvider: Send + Sync {
    /// Verifies if a given leave type ID is valid by calling the Policy Service.
    async fn is_leave_type_valid(&self, leave_type_id: Uuid, token: &str) -> Result<bool, String>;
    
    /// Returns the number of public holidays between two dates.
    async fn get_holiday_count(&self, start: chrono::NaiveDate, end: chrono::NaiveDate, token: &str) -> Result<usize, String>;

    /// Returns the metadata for a specific leave type.
    async fn get_leave_type_info(&self, leave_type_id: Uuid, token: &str) -> Result<LeaveTypeInfo, String>;
}

#[derive(serde::Deserialize)]
pub struct LeaveTypeInfo {
    pub id: Uuid,
    pub name: String,
    pub auto_approve: bool,
}

#[async_trait]
pub trait StaffProvider: Send + Sync {
    /// Checks if a user is an active delegatee for a delegator.
    async fn is_delegatee_for(&self, delegatee_id: Uuid, delegator_id: Uuid, token: &str) -> Result<bool, String>;
    
    /// Returns a list of active delegations for a delegatee.
    async fn get_active_delegations(&self, delegatee_id: Uuid, token: &str) -> Result<Vec<Uuid>, String>;
}

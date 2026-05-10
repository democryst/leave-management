// services/policy/src/internal/core/ports/mod.rs
use async_trait::async_trait;
use uuid::Uuid;
use crate::internal::core::domain::{LeaveType, BlackoutDate, Holiday};
use chrono::NaiveDate;

#[async_trait]
pub trait PolicyRepository: Send + Sync {
    async fn get_all_leave_types(&self) -> Result<Vec<LeaveType>, String>;
    async fn get_leave_type_by_id(&self, id: Uuid) -> Result<Option<LeaveType>, String>;
    async fn save_leave_type(&self, leave_type: LeaveType) -> Result<(), String>;
    async fn get_blackout_dates(&self) -> Result<Vec<BlackoutDate>, String>;
    async fn add_blackout_date(&self, blackout: BlackoutDate) -> Result<(), String>;
    async fn get_holidays(&self, start: NaiveDate, end: NaiveDate) -> Result<Vec<Holiday>, String>;
    async fn add_holiday(&self, holiday: Holiday) -> Result<(), String>;
}

#[async_trait]
pub trait PolicyService: Send + Sync {
    async fn list_leave_types(&self) -> Result<Vec<LeaveType>, String>;
    async fn get_leave_type(&self, id: Uuid) -> Result<Option<LeaveType>, String>;
    async fn validate_leave_request(&self, leave_type_id: Uuid, start_date: NaiveDate, end_date: NaiveDate) -> Result<bool, String>;
    async fn get_holidays_in_range(&self, start: NaiveDate, end: NaiveDate) -> Result<Vec<Holiday>, String>;
}

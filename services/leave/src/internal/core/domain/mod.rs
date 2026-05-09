// services/leave/src/internal/core/domain/mod.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc, NaiveDate};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LeaveStatus {
    Pending,
    Approved,
    Rejected,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveRequest {
    pub id: Uuid,
    pub staff_id: Uuid,
    pub leave_type_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub status: LeaveStatus,
    pub reason: String,
    pub approver_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveBalance {
    pub staff_id: Uuid,
    pub leave_type_id: Uuid,
    pub balance: Decimal,
    pub accrued_this_year: Decimal,
    pub updated_at: DateTime<Utc>,
}

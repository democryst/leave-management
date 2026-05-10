// services/leave/src/internal/core/domain/mod.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc, NaiveDate};

pub trait Mask {
    fn mask(&self) -> Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "text", rename_all = "lowercase")]
pub enum LeaveStatus {
    Pending,
    Approved,
    Rejected,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
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

impl Mask for LeaveRequest {
    fn mask(&self) -> Self {
        self.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct LeaveBalance {
    pub staff_id: Uuid,
    pub leave_type_id: Uuid,
    pub balance: BigDecimal,
    pub accrued_this_year: BigDecimal,
    pub updated_at: DateTime<Utc>,
}

impl Mask for LeaveBalance {
    fn mask(&self) -> Self {
        self.clone()
    }
}
pub mod accrual;

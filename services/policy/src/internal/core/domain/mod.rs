// services/policy/src/internal/core/domain/mod.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveType {
    pub id: Uuid,
    pub name: String,
    pub allowance_per_year: Decimal,
    pub requires_approval: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlackoutDate {
    pub date: chrono::NaiveDate,
    pub description: String,
}

// services/policy/src/internal/core/domain/mod.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};

pub trait Mask {
    fn mask(&self) -> Self;
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct LeaveType {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub default_days: f64,
    pub auto_approve: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Mask for LeaveType {
    fn mask(&self) -> Self {
        self.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BlackoutDate {
    pub id: Uuid,
    pub name: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl Mask for BlackoutDate {
    fn mask(&self) -> Self {
        self.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Holiday {
    pub id: Uuid,
    pub name: String,
    pub date: NaiveDate,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl Mask for Holiday {
    fn mask(&self) -> Self {
        self.clone()
    }
}

pub mod rules;

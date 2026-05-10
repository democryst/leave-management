// services/staff/src/internal/core/domain/mod.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub trait Mask {
    fn mask(&self) -> Self;
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text", rename_all = "lowercase")]
pub enum StaffRole {
    Admin,
    Manager,
    Staff,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Staff {
    pub id: Uuid,
    pub staff_id: String,
    pub full_name: String,
    pub email: String,
    pub password_hash: String,
    pub role: StaffRole,
    pub manager_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Mask for Staff {
    fn mask(&self) -> Self {
        Self {
            password_hash: "[MASKED]".to_string(),
            ..self.clone()
        }
    }
}

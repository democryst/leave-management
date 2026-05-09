use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "staff_role", rename_all = "lowercase")]
pub enum StaffRole {
    Admin,
    Staff,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// ADR-002: Distributed Tracing & PII Masking
pub trait Mask {
    fn masked(&self) -> Self;
}

impl Mask for Staff {
    fn masked(&self) -> Self {
        Self {
            full_name: mask_name(&self.full_name),
            email: mask_email(&self.email),
            password_hash: "[MASKED]".to_string(),
            ..self.clone()
        }
    }
}

fn mask_name(name: &str) -> String {
    let parts: Vec<&str> = name.split_whitespace().collect();
    parts.iter()
        .map(|p| {
            if p.len() <= 2 {
                p.to_string()
            } else {
                format!("{}***{}", &p[..1], &p[p.len()-1..])
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn mask_email(email: &str) -> String {
    if let Some((user, domain)) = email.split_once('@') {
        if user.len() <= 2 {
            format!("{}***@{}", user, domain)
        } else {
            format!("{}***{}@{}", &user[..1], &user[user.len()-1..], domain)
        }
    } else {
        "***@***".to_string()
    }
}

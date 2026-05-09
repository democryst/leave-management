use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::internal::core::domain::{LeaveType, BlackoutDate};
use crate::internal::core::ports::PolicyRepository;

/// Concrete implementation of the PolicyRepository trait using SQLx.
pub struct SqlxPolicyRepository {
    pool: PgPool,
}

impl SqlxPolicyRepository {
    /// Creates a new repository instance with a database connection pool.
    pub fn new(pool: PgPool) -> Self {
        SqlxPolicyRepository { pool }
    }
}

#[async_trait]
impl PolicyRepository for SqlxPolicyRepository {
    /// Retrieves all defined leave types.
    async fn get_all_leave_types(&self) -> Result<Vec<LeaveType>, String> {
        let result = sqlx::query_as!(
            LeaveType,
            r#"SELECT id, name, allowance_per_year, requires_approval FROM leave_types"#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Database error fetching leave types: {}", e))?;

        Ok(result)
    }

    /// Retrieves a single leave type by its UUID.
    async fn get_leave_type_by_id(&self, id: Uuid) -> Result<Option<LeaveType>, String> {
        let result = sqlx::query_as!(
            LeaveType,
            r#"SELECT id, name, allowance_per_year, requires_approval FROM leave_types WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Database error fetching leave type by ID {}: {}", id, e))?;

        Ok(result)
    }

    /// Saves (updates/upserts) a leave type record based on its ID.
    async fn save_leave_type(&self, leave_type: LeaveType) -> Result<(), String> {
        sqlx::query!(
            r#"
            INSERT INTO leave_types (id, name, allowance_per_year, requires_approval)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (id) DO UPDATE
            SET
                name = EXCLUDED.name,
                allowance_per_year = EXCLUDED.allowance_per_year,
                requires_approval = EXCLUDED.requires_approval
            "#,
            leave_type.id,
            &leave_type.name,
            leave_type.allowance_per_year,
            leave_type.requires_approval
        )
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Database error saving leave type: {}", e))?;

        Ok(())
    }

    /// Retrieves all defined blackout dates.
    async fn get_blackout_dates(&self) -> Result<Vec<BlackoutDate>, String> {
        let result = sqlx::query_as!(
            BlackoutDate,
            r#"SELECT date, description FROM blackout_dates ORDER BY date"#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Database error fetching blackout dates: {}", e))?;

        Ok(result)
    }

    /// Adds a new blackout date (upserts if date exists).
    async fn add_blackout_date(&self, blackout: BlackoutDate) -> Result<(), String> {
        sqlx::query!(
            r#"
            INSERT INTO blackout_dates (date, description)
            VALUES ($1, $2)
            ON CONFLICT (date) DO UPDATE SET description = EXCLUDED.description
            "#,
            blackout.date,
            &blackout.description
        )
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Database error adding blackout date: {}", e))?;

        Ok(())
    }
}

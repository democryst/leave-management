use async_trait::async_trait;
use sqlx::{PgPool, Postgres};
use uuid::Uuid;
use crate::internal::core::domain::{LeaveType, BlackoutDate, Holiday};
use crate::internal::core::ports::PolicyRepository;
use chrono::NaiveDate;

#[derive(Clone)]
pub struct SqlxPolicyRepository {
    pool: PgPool,
}

impl SqlxPolicyRepository {
    pub fn new(pool: PgPool) -> Self {
        SqlxPolicyRepository { pool }
    }
}

#[async_trait]
impl PolicyRepository for SqlxPolicyRepository {
    async fn get_leave_type_by_id(&self, id: Uuid) -> Result<Option<LeaveType>, String> {
        let leave_type = sqlx::query_as::<Postgres, LeaveType>(
            r#"SELECT id, name, description, default_days, auto_approve, is_active, created_at, updated_at 
               FROM leave_types WHERE id = $1"#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(leave_type)
    }

    async fn get_all_leave_types(&self) -> Result<Vec<LeaveType>, String> {
        let types = sqlx::query_as::<Postgres, LeaveType>(
            r#"SELECT id, name, description, default_days, auto_approve, is_active, created_at, updated_at 
               FROM leave_types ORDER BY name ASC"#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(types)
    }

    async fn save_leave_type(&self, leave_type: LeaveType) -> Result<(), String> {
        sqlx::query(
            r#"INSERT INTO leave_types (id, name, description, default_days, auto_approve, is_active, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
               ON CONFLICT (id) DO UPDATE SET
                   name = EXCLUDED.name,
                   description = EXCLUDED.description,
                   default_days = EXCLUDED.default_days,
                   auto_approve = EXCLUDED.auto_approve,
                   is_active = EXCLUDED.is_active,
                   updated_at = EXCLUDED.updated_at"#
        )
        .bind(leave_type.id)
        .bind(leave_type.name)
        .bind(leave_type.description)
        .bind(leave_type.default_days)
        .bind(leave_type.auto_approve)
        .bind(leave_type.is_active)
        .bind(leave_type.created_at)
        .bind(leave_type.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn get_blackout_dates(&self) -> Result<Vec<BlackoutDate>, String> {
        let dates = sqlx::query_as::<Postgres, BlackoutDate>(
            r#"SELECT id, name, start_date, end_date, description, created_at 
               FROM blackout_dates ORDER BY start_date ASC"#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(dates)
    }

    async fn add_blackout_date(&self, blackout: BlackoutDate) -> Result<(), String> {
        sqlx::query(
            r#"INSERT INTO blackout_dates (id, name, start_date, end_date, description, created_at)
               VALUES ($1, $2, $3, $4, $5, $6)"#
        )
        .bind(blackout.id)
        .bind(blackout.name)
        .bind(blackout.start_date)
        .bind(blackout.end_date)
        .bind(blackout.description)
        .bind(blackout.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn get_holidays(&self, start: NaiveDate, end: NaiveDate) -> Result<Vec<Holiday>, String> {
        let holidays = sqlx::query_as::<Postgres, Holiday>(
            r#"SELECT id, name, date, description, created_at 
               FROM holidays WHERE date BETWEEN $1 AND $2 ORDER BY date ASC"#
        )
        .bind(start)
        .bind(end)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(holidays)
    }

    async fn add_holiday(&self, holiday: Holiday) -> Result<(), String> {
        sqlx::query(
            r#"INSERT INTO holidays (id, name, date, description, created_at)
               VALUES ($1, $2, $3, $4, $5)"#
        )
        .bind(holiday.id)
        .bind(holiday.name)
        .bind(holiday.date)
        .bind(holiday.description)
        .bind(holiday.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }
}

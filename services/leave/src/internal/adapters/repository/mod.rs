use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use rust_decimal::Decimal;
use crate::internal::core::domain::{LeaveRequest, LeaveBalance, LeaveStatus};
use crate::internal::core::ports::LeaveRepository;

/// Concrete implementation of the LeaveRepository trait using SQLx.
pub struct SqlxLeaveRepository {
    pool: PgPool,
}

impl SqlxLeaveRepository {
    /// Creates a new repository instance with a database connection pool.
    pub fn new(pool: PgPool) -> Self {
        SqlxLeaveRepository { pool }
    }
}

#[async_trait]
impl LeaveRepository for SqlxLeaveRepository {
    /// Finds a leave request by its unique ID.
    async fn find_request_by_id(&self, id: Uuid) -> Result<Option<LeaveRequest>, String> {
        let request = sqlx::query_as!(
            LeaveRequest,
            r#"SELECT id, staff_id, leave_type_id, start_date, end_date, status as "status: LeaveStatus", reason, approver_id, created_at 
               FROM leave_requests WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Database error finding request by ID {}: {}", id, e))?;

        Ok(request)
    }

    /// Saves a new leave request record.
    async fn save_request(&self, request: LeaveRequest) -> Result<(), String> {
        sqlx::query!(
            r#"INSERT INTO leave_requests (id, staff_id, leave_type_id, start_date, end_date, status, reason, approver_id, created_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#,
            request.id,
            request.staff_id,
            request.leave_type_id,
            request.start_date,
            request.end_date,
            request.status as LeaveStatus,
            request.reason,
            request.approver_id,
            request.created_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Database error saving request: {}", e))?;

        Ok(())
    }

    /// Updates the status of a leave request (State Machine transition).
    async fn update_request_status(&self, id: Uuid, status: LeaveStatus, approver_id: Option<Uuid>) -> Result<(), String> {
        sqlx::query!(
            r#"UPDATE leave_requests SET status = $1, approver_id = $2 WHERE id = $3"#,
            status as LeaveStatus,
            approver_id,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Database error updating status for ID {}: {}", id, e))?;

        Ok(())
    }

    /// Retrieves all leave requests for a specific staff member.
    async fn get_staff_requests(&self, staff_id: Uuid) -> Result<Vec<LeaveRequest>, String> {
        let requests = sqlx::query_as!(
            LeaveRequest,
            r#"SELECT id, staff_id, leave_type_id, start_date, end_date, status as "status: LeaveStatus", reason, approver_id, created_at 
               FROM leave_requests WHERE staff_id = $1 ORDER BY created_at DESC"#,
            staff_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Database error fetching staff requests: {}", e))?;

        Ok(requests)
    }

    /// Retrieves a specific leave balance for a staff member.
    async fn get_balance(&self, staff_id: Uuid, leave_type_id: Uuid) -> Result<Option<LeaveBalance>, String> {
        let balance = sqlx::query_as!(
            LeaveBalance,
            r#"SELECT staff_id, leave_type_id, balance, accrued_this_year, updated_at 
               FROM leave_balances WHERE staff_id = $1 AND leave_type_id = $2"#,
            staff_id,
            leave_type_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Database error fetching balance: {}", e))?;

        Ok(balance)
    }

    /// Saves (upserts) a staff member's leave balance.
    async fn save_balance(&self, balance: LeaveBalance) -> Result<(), String> {
        sqlx::query!(
            r#"INSERT INTO leave_balances (staff_id, leave_type_id, balance, accrued_this_year, updated_at)
               VALUES ($1, $2, $3, $4, $5)
               ON CONFLICT (staff_id, leave_type_id) DO UPDATE SET
                   balance = EXCLUDED.balance,
                   accrued_this_year = EXCLUDED.accrued_this_year,
                   updated_at = EXCLUDED.updated_at"#,
            balance.staff_id,
            balance.leave_type_id,
            balance.balance,
            balance.accrued_this_year,
            balance.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Database error saving balance: {}", e))?;

        Ok(())
    }

    /// Atomic balance adjustment to prevent race conditions during concurrent requests.
    async fn update_balance(&self, staff_id: Uuid, leave_type_id: Uuid, amount: Decimal) -> Result<(), String> {
        sqlx::query!(
            r#"UPDATE leave_balances SET balance = balance + $1, updated_at = NOW() 
               WHERE staff_id = $2 AND leave_type_id = $3"#,
            amount,
            staff_id,
            leave_type_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Database error updating balance atomically: {}", e))?;

        Ok(())
    }
}

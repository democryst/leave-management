use async_trait::async_trait;
use sqlx::{PgPool, Postgres};
use uuid::Uuid;
use bigdecimal::BigDecimal;
use crate::internal::core::domain::{LeaveRequest, LeaveBalance, LeaveStatus};
use crate::internal::core::ports::LeaveRepository;

#[derive(Clone)]
pub struct SqlxLeaveRepository {
    pool: PgPool,
}

impl SqlxLeaveRepository {
    pub fn new(pool: PgPool) -> Self {
        SqlxLeaveRepository { pool }
    }
}

#[async_trait]
impl LeaveRepository for SqlxLeaveRepository {
    async fn find_request_by_id(&self, id: Uuid) -> Result<Option<LeaveRequest>, String> {
        let request = sqlx::query_as::<Postgres, LeaveRequest>(
            r#"SELECT id, staff_id, leave_type_id, start_date, end_date, status as "status: LeaveStatus", reason, approver_id, created_at 
               FROM leave_requests WHERE id = $1"#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(request)
    }

    async fn save_request(&self, request: LeaveRequest) -> Result<(), String> {
        sqlx::query(
            r#"INSERT INTO leave_requests (id, staff_id, leave_type_id, start_date, end_date, status, reason, approver_id, created_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#
        )
        .bind(request.id)
        .bind(request.staff_id)
        .bind(request.leave_type_id)
        .bind(request.start_date)
        .bind(request.end_date)
        .bind(request.status)
        .bind(request.reason)
        .bind(request.approver_id)
        .bind(request.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn update_request_status(&self, id: Uuid, status: LeaveStatus, approver_id: Option<Uuid>) -> Result<(), String> {
        sqlx::query(
            r#"UPDATE leave_requests SET status = $1, approver_id = $2 WHERE id = $3"#
        )
        .bind(status)
        .bind(approver_id)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn get_staff_requests(&self, staff_id: Uuid) -> Result<Vec<LeaveRequest>, String> {
        let requests = sqlx::query_as::<Postgres, LeaveRequest>(
            r#"SELECT id, staff_id, leave_type_id, start_date, end_date, status as "status: LeaveStatus", reason, approver_id, created_at 
               FROM leave_requests WHERE staff_id = $1 ORDER BY created_at DESC"#
        )
        .bind(staff_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(requests)
    }

    async fn get_pending_by_approver(&self, approver_id: Uuid) -> Result<Vec<LeaveRequest>, String> {
        let requests = sqlx::query_as::<Postgres, LeaveRequest>(
            r#"SELECT id, staff_id, leave_type_id, start_date, end_date, status as "status: LeaveStatus", reason, approver_id, created_at 
               FROM leave_requests WHERE approver_id = $1 AND status = 'Pending' ORDER BY created_at ASC"#
        )
        .bind(approver_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(requests)
    }

    async fn get_balance(&self, staff_id: Uuid, leave_type_id: Uuid) -> Result<Option<LeaveBalance>, String> {
        let balance = sqlx::query_as::<Postgres, LeaveBalance>(
            r#"SELECT staff_id, leave_type_id, balance, accrued_this_year, updated_at 
               FROM leave_balances WHERE staff_id = $1 AND leave_type_id = $2"#
        )
        .bind(staff_id)
        .bind(leave_type_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(balance)
    }

    async fn save_balance(&self, balance: LeaveBalance) -> Result<(), String> {
        sqlx::query(
            r#"INSERT INTO leave_balances (staff_id, leave_type_id, balance, accrued_this_year, updated_at)
               VALUES ($1, $2, $3, $4, $5)
               ON CONFLICT (staff_id, leave_type_id) DO UPDATE SET
                   balance = EXCLUDED.balance,
                   accrued_this_year = EXCLUDED.accrued_this_year,
                   updated_at = EXCLUDED.updated_at"#
        )
        .bind(balance.staff_id)
        .bind(balance.leave_type_id)
        .bind(balance.balance)
        .bind(balance.accrued_this_year)
        .bind(balance.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn update_balance(&self, staff_id: Uuid, leave_type_id: Uuid, amount: BigDecimal) -> Result<(), String> {
        sqlx::query(
            r#"UPDATE leave_balances SET balance = balance + $1, updated_at = NOW() 
               WHERE staff_id = $2 AND leave_type_id = $3"#
        )
        .bind(amount)
        .bind(staff_id)
        .bind(leave_type_id)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn create_request_and_update_balance(&self, request: LeaveRequest, balance_delta: BigDecimal) -> Result<LeaveRequest, String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        // 1. Check current balance
        let balance = sqlx::query_scalar::<Postgres, BigDecimal>(
            "SELECT balance FROM leave_balances WHERE staff_id = $1 AND leave_type_id = $2"
        )
        .bind(request.staff_id)
        .bind(request.leave_type_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        if balance + &balance_delta < BigDecimal::from(0) {
            return Err("balance_too_low".to_string());
        }

        // 2. Save Request
        sqlx::query(
            r#"INSERT INTO leave_requests (id, staff_id, leave_type_id, start_date, end_date, status, reason, approver_id, created_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#
        )
        .bind(request.id)
        .bind(request.staff_id)
        .bind(request.leave_type_id)
        .bind(request.start_date)
        .bind(request.end_date)
        .bind(request.status)
        .bind(request.reason.clone())
        .bind(request.approver_id)
        .bind(request.created_at)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        // 3. Update Balance
        sqlx::query(
            "UPDATE leave_balances SET balance = balance + $1, updated_at = NOW() WHERE staff_id = $2 AND leave_type_id = $3"
        )
        .bind(balance_delta)
        .bind(request.staff_id)
        .bind(request.leave_type_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(request)
    }
}

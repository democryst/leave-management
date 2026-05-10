use async_trait::async_trait;
use sqlx::{PgPool, Postgres};
use uuid::Uuid;
use crate::internal::core::domain::{Staff, Delegation};
use crate::internal::core::ports::StaffRepository;

/// The concrete implementation of the StaffRepository trait using SQLx.
#[derive(Clone)]
pub struct SqlxStaffRepository {
    pool: PgPool,
}

impl SqlxStaffRepository {
    pub fn new(pool: PgPool) -> Self {
        SqlxStaffRepository { pool }
    }
}

#[async_trait]
impl StaffRepository for SqlxStaffRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Staff>, String> {
        let staff = sqlx::query_as::<Postgres, Staff>(
            r#"SELECT id, staff_id, full_name, email, password_hash, role as "role: StaffRole", manager_id, created_at, updated_at 
               FROM staff WHERE id = $1"#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Database error finding staff by ID: {}", e))?;

        Ok(staff)
    }

    async fn find_by_staff_id(&self, staff_id: &str) -> Result<Option<Staff>, String> {
        let staff = sqlx::query_as::<Postgres, Staff>(
            r#"SELECT id, staff_id, full_name, email, password_hash, role as "role: StaffRole", manager_id, created_at, updated_at 
               FROM staff WHERE staff_id = $1"#
        )
        .bind(staff_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Database error finding staff by staff ID: {}", e))?;

        Ok(staff)
    }

    async fn save(&self, staff: Staff) -> Result<(), String> {
        sqlx::query(
            r#"INSERT INTO staff (id, staff_id, full_name, email, password_hash, role, manager_id, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
               ON CONFLICT (id) DO UPDATE SET
                   staff_id = EXCLUDED.staff_id,
                   full_name = EXCLUDED.full_name,
                   email = EXCLUDED.email,
                   password_hash = EXCLUDED.password_hash,
                   role = EXCLUDED.role,
                   manager_id = EXCLUDED.manager_id,
                   updated_at = EXCLUDED.updated_at"#
        )
        .bind(staff.id)
        .bind(staff.staff_id)
        .bind(staff.full_name)
        .bind(staff.email)
        .bind(staff.password_hash)
        .bind(staff.role)
        .bind(staff.manager_id)
        .bind(staff.created_at)
        .bind(staff.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Database error saving staff: {}", e))?;

        Ok(())
    }

    async fn get_reports(&self, manager_id: Uuid) -> Result<Vec<Staff>, String> {
        let reports = sqlx::query_as::<Postgres, Staff>(
            r#"SELECT id, staff_id, full_name, email, password_hash, role as "role: StaffRole", manager_id, created_at, updated_at 
               FROM staff WHERE manager_id = $1"#
        )
        .bind(manager_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Database error fetching reports: {}", e))?;

        Ok(reports)
    }

    async fn terminate(&self, id: Uuid) -> Result<(), String> {
        sqlx::query("UPDATE staff SET terminated_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn get_approver_chain(&self, staff_id: Uuid) -> Result<Vec<Staff>, String> {
        // Recursive CTE for Org Chart resolution
        let chain = sqlx::query_as::<Postgres, Staff>(
            r#"WITH RECURSIVE org_chain AS (
                SELECT id, staff_id, full_name, email, password_hash, role, manager_id, created_at, updated_at
                FROM staff WHERE id = $1
                UNION ALL
                SELECT s.id, s.staff_id, s.full_name, s.email, s.password_hash, s.role, s.manager_id, s.created_at, s.updated_at
                FROM staff s
                INNER JOIN org_chain oc ON s.id = oc.manager_id
            )
            SELECT id, staff_id, full_name, email, password_hash, role as "role: StaffRole", manager_id, created_at, updated_at
            FROM org_chain WHERE id != $1"#
        )
        .bind(staff_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(chain)
    }

    async fn save_delegation(&self, delegation: Delegation) -> Result<(), String> {
        sqlx::query(
            r#"INSERT INTO delegations (id, delegator_id, delegatee_id, start_date, end_date, is_active, created_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7)"#
        )
        .bind(delegation.id)
        .bind(delegation.delegator_id)
        .bind(delegation.delegatee_id)
        .bind(delegation.start_date)
        .bind(delegation.end_date)
        .bind(delegation.is_active)
        .bind(delegation.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn get_active_delegations_for_delegatee(&self, delegatee_id: Uuid) -> Result<Vec<Delegation>, String> {
        let delegations = sqlx::query_as::<Postgres, Delegation>(
            r#"SELECT id, delegator_id, delegatee_id, start_date, end_date, is_active, created_at 
               FROM delegations 
               WHERE delegatee_id = $1 AND is_active = TRUE 
               AND CURRENT_DATE BETWEEN start_date AND end_date"#
        )
        .bind(delegatee_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(delegations)
    }

    async fn is_delegatee_for(&self, delegatee_id: Uuid, delegator_id: Uuid) -> Result<bool, String> {
        let count = sqlx::query_scalar::<Postgres, i64>(
            r#"SELECT COUNT(*) FROM delegations 
               WHERE delegatee_id = $1 AND delegator_id = $2 AND is_active = TRUE 
               AND CURRENT_DATE BETWEEN start_date AND end_date"#
        )
        .bind(delegatee_id)
        .bind(delegator_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(count > 0)
    }
}

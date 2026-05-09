use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::internal::core::domain::{Staff, StaffRole};
use crate::internal::core::ports::StaffRepository;

/// The concrete implementation of the StaffRepository trait using SQLx.
#[derive(Clone)]
pub struct SqlxStaffRepository {
    pool: PgPool,
}

impl SqlxStaffRepository {
    /// Creates a new repository instance with a database connection pool.
    pub fn new(pool: PgPool) -> Self {
        SqlxStaffRepository { pool }
    }
}

#[async_trait]
impl StaffRepository for SqlxStaffRepository {
    /// Finds a staff member by their primary key ID.
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Staff>, String> {
        let staff = sqlx::query_as!(
            Staff,
            r#"SELECT id, staff_id, full_name, email, password_hash, role as "role: StaffRole", manager_id, created_at, updated_at 
               FROM staff WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Database error finding staff by ID: {}", e))?;

        Ok(staff)
    }

    /// Finds a staff member by their unique staff ID.
    async fn find_by_staff_id(&self, staff_id: &str) -> Result<Option<Staff>, String> {
        let staff = sqlx::query_as!(
            Staff,
            r#"SELECT id, staff_id, full_name, email, password_hash, role as "role: StaffRole", manager_id, created_at, updated_at 
               FROM staff WHERE staff_id = $1"#,
            staff_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Database error finding staff by staff ID: {}", e))?;

        Ok(staff)
    }

    /// Saves (updates/upserts) a staff member record.
    async fn save(&self, staff: Staff) -> Result<(), String> {
        // Using an ON CONFLICT upsert to handle both new and existing staff
        sqlx::query!(
            r#"INSERT INTO staff (id, staff_id, full_name, email, password_hash, role, manager_id, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
               ON CONFLICT (id) DO UPDATE SET
                   staff_id = EXCLUDED.staff_id,
                   full_name = EXCLUDED.full_name,
                   email = EXCLUDED.email,
                   password_hash = EXCLUDED.password_hash,
                   role = EXCLUDED.role,
                   manager_id = EXCLUDED.manager_id,
                   updated_at = EXCLUDED.updated_at"#,
            staff.id,
            staff.staff_id,
            staff.full_name,
            staff.email,
            staff.password_hash,
            staff.role as StaffRole,
            staff.manager_id,
            staff.created_at,
            staff.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Database error saving staff: {}", e))?;

        Ok(())
    }

    /// Retrieves all direct reports for a given manager.
    async fn get_reports(&self, manager_id: Uuid) -> Result<Vec<Staff>, String> {
        let reports = sqlx::query_as!(
            Staff,
            r#"SELECT id, staff_id, full_name, email, password_hash, role as "role: StaffRole", manager_id, created_at, updated_at 
               FROM staff WHERE manager_id = $1"#,
            manager_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Database error fetching reports: {}", e))?;

        Ok(reports)
    }
}

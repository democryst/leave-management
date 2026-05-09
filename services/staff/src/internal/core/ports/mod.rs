use async_trait::async_trait;
use uuid::Uuid;
use crate::internal::core::domain::Staff;

#[async_trait]
pub trait StaffRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Staff>, String>;
    async fn find_by_staff_id(&self, staff_id: &str) -> Result<Option<Staff>, String>;
    async fn save(&self, staff: Staff) -> Result<(), String>;
    async fn get_reports(&self, manager_id: Uuid) -> Result<Vec<Staff>, String>;
}

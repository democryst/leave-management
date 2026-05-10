use async_trait::async_trait;
use uuid::Uuid;
use crate::internal::core::domain::{Staff, Delegation};

#[async_trait]
pub trait StaffRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Staff>, String>;
    async fn find_by_staff_id(&self, staff_id: &str) -> Result<Option<Staff>, String>;
    async fn save(&self, staff: Staff) -> Result<(), String>;
    async fn get_reports(&self, manager_id: Uuid) -> Result<Vec<Staff>, String>;
    async fn get_approver_chain(&self, staff_id: Uuid) -> Result<Vec<Staff>, String>;
    async fn terminate(&self, id: Uuid) -> Result<(), String>;
    
    // Delegation
    async fn save_delegation(&self, delegation: Delegation) -> Result<(), String>;
    async fn get_active_delegations_for_delegatee(&self, delegatee_id: Uuid) -> Result<Vec<Delegation>, String>;
    async fn is_delegatee_for(&self, delegatee_id: Uuid, delegator_id: Uuid) -> Result<bool, String>;
}

#[async_trait]
pub trait StaffService: Send + Sync {
    async fn get_staff_by_id(&self, id: Uuid) -> Result<Option<Staff>, String>;
    async fn register_staff(&self, staff: Staff) -> Result<(), String>;
    async fn get_manager_reports(&self, manager_id: Uuid) -> Result<Vec<Staff>, String>;
    async fn resolve_approver_chain(&self, staff_id: Uuid) -> Result<Vec<Staff>, String>;
    async fn terminate_staff(&self, id: Uuid) -> Result<(), String>;
    
    // Delegation
    async fn create_delegation(&self, delegation: Delegation) -> Result<(), String>;
    async fn check_delegation(&self, delegatee_id: Uuid, delegator_id: Uuid) -> Result<bool, String>;
    async fn list_active_delegations(&self, delegatee_id: Uuid) -> Result<Vec<Delegation>, String>;
}

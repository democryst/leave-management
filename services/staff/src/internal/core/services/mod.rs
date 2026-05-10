use async_trait::async_trait;
use uuid::Uuid;
use std::sync::Arc;
use crate::internal::core::domain::{Staff, Mask};
use crate::internal::core::ports::{StaffRepository, StaffService};

pub struct StaffServiceImpl {
    repo: Arc<dyn StaffRepository>,
}

impl StaffServiceImpl {
    pub fn new(repo: Arc<dyn StaffRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl StaffService for StaffServiceImpl {
    async fn get_staff_by_id(&self, id: Uuid) -> Result<Option<Staff>, String> {
        let staff = self.repo.find_by_id(id).await?;
        if let Some(ref s) = staff {
            tracing::info!("Found staff member: {:?}", s.mask());
        }
        Ok(staff.map(|s| s.mask()))
    }

    async fn register_staff(&self, staff: Staff) -> Result<(), String> {
        // Here we could add validation or password hashing logic
        self.repo.save(staff).await
    }

    async fn get_manager_reports(&self, manager_id: Uuid) -> Result<Vec<Staff>, String> {
        let reports = self.repo.get_reports(manager_id).await?;
        Ok(reports.into_iter().map(|s| s.mask()).collect())
    }

    async fn resolve_approver_chain(&self, staff_id: Uuid) -> Result<Vec<Staff>, String> {
        let mut chain = Vec::new();
        let mut current_id = staff_id;

        // Limiting to 5 levels to prevent infinite loops in bad data
        for _ in 0..5 {
            if let Some(staff) = self.repo.find_by_id(current_id).await? {
                if let Some(manager_id) = staff.manager_id {
                    if let Some(manager) = self.repo.find_by_id(manager_id).await? {
                        chain.push(manager.mask());
                        current_id = manager_id;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        Ok(chain)
    }

    async fn terminate_staff(&self, id: Uuid) -> Result<(), String> {
        self.repo.terminate(id).await
    }

    async fn create_delegation(&self, delegation: crate::internal::core::domain::Delegation) -> Result<(), String> {
        self.repo.save_delegation(delegation).await
    }

    async fn check_delegation(&self, delegatee_id: Uuid, delegator_id: Uuid) -> Result<bool, String> {
        self.repo.is_delegatee_for(delegatee_id, delegator_id).await
    }

    async fn list_active_delegations(&self, delegatee_id: Uuid) -> Result<Vec<crate::internal::core::domain::Delegation>, String> {
        self.repo.get_active_delegations_for_delegatee(delegatee_id).await
    }
}

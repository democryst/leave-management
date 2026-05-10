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
}

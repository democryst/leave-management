use async_trait::async_trait;
use reqwest::{Client, StatusCode};
use uuid::Uuid;
use crate::internal::core::ports::StaffProvider;
use serde::Deserialize;

pub struct StaffServiceClient {
    http_client: Client,
    base_url: String,
}

impl StaffServiceClient {
    pub fn new(base_url: String) -> Self {
        StaffServiceClient {
            http_client: Client::new(),
            base_url,
        }
    }
}

#[derive(Deserialize)]
struct DelegationResponse {
    delegator_id: Uuid,
}

#[derive(Deserialize)]
struct CheckResponse {
    is_active: bool,
}

#[async_trait]
impl StaffProvider for StaffServiceClient {
    async fn is_delegatee_for(&self, delegatee_id: Uuid, delegator_id: Uuid, token: &str) -> Result<bool, String> {
        let url = format!("{}/api/v1/staff/delegations/check/{}/{}", self.base_url, delegatee_id, delegator_id);

        let response = self.http_client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if response.status() != StatusCode::OK {
            return Ok(false);
        }

        let res: CheckResponse = response.json().await.map_err(|e| e.to_string())?;
        Ok(res.is_active)
    }

    async fn get_active_delegations(&self, delegatee_id: Uuid, token: &str) -> Result<Vec<Uuid>, String> {
        let url = format!("{}/api/v1/staff/delegations", self.base_url);

        let response = self.http_client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("X-User-Id", delegatee_id.to_string())
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if response.status() != StatusCode::OK {
            return Ok(vec![]);
        }

        let delegations: Vec<DelegationResponse> = response.json().await.map_err(|e| e.to_string())?;
        Ok(delegations.into_iter().map(|d| d.delegator_id).collect())
    }
}

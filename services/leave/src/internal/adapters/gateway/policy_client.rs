use async_trait::async_trait;
use reqwest::{Client, StatusCode};
use uuid::Uuid;
use crate::internal::core::ports::PolicyProvider;

/// Concrete implementation of PolicyProvider using a REST client.
pub struct PolicyServiceClient {
    http_client: Client,
    base_url: String,
}

impl PolicyServiceClient {
    /// Creates a new PolicyServiceClient instance.
    pub fn new(base_url: String) -> Self {
        PolicyServiceClient {
            http_client: Client::new(),
            base_url,
        }
    }
}

#[async_trait]
impl PolicyProvider for PolicyServiceClient {
    /// Calls the Policy Service to validate a leave type ID.
    async fn is_leave_type_valid(&self, leave_type_id: Uuid, token: &str) -> Result<bool, String> {
        let url = format!("{}/api/v1/policy/leave-types/{}", self.base_url, leave_type_id);

        let response = self.http_client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("HTTP error calling Policy Service: {}", e))?;

        match response.status() {
            StatusCode::OK => Ok(true),
            StatusCode::NOT_FOUND => Ok(false),
            status => {
                let error_msg = response.text().await.unwrap_or_default();
                Err(format!("Policy Service error ({}): {}", status, error_msg))
            }
        }
    }
}

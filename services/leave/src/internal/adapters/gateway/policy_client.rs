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

    async fn get_holiday_count(&self, start: chrono::NaiveDate, end: chrono::NaiveDate, token: &str) -> Result<usize, String> {
        let url = format!("{}/api/v1/policy/holidays", self.base_url);

        let response = self.http_client
            .get(&url)
            .query(&[("start", start.to_string()), ("end", end.to_string())])
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("HTTP error calling Policy Service: {}", e))?;

        if response.status() != StatusCode::OK {
            return Err(format!("Policy Service error: {}", response.status()));
        }

        let holidays: Vec<serde_json::Value> = response.json().await
            .map_err(|e| format!("Failed to parse holidays: {}", e))?;

        Ok(holidays.len())
    }

    async fn get_leave_type_info(&self, leave_type_id: Uuid, token: &str) -> Result<crate::internal::core::ports::LeaveTypeInfo, String> {
        let url = format!("{}/api/v1/policy/leave-types/{}", self.base_url, leave_type_id);

        let response = self.http_client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("HTTP error calling Policy Service: {}", e))?;

        if response.status() != StatusCode::OK {
            return Err(format!("Policy Service error: {}", response.status()));
        }

        let info: crate::internal::core::ports::LeaveTypeInfo = response.json().await
            .map_err(|e| format!("Failed to parse leave type info: {}", e))?;

        Ok(info)
    }
}

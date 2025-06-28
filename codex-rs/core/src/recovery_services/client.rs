//! REST API client for Recovery Services (Azure Backup).

use crate::error::{CodexErr, Result};
use crate::recovery_services::models::*;
use serde_json::{json, Value};

/// REST API client for Recovery Services
pub struct RecoveryServicesClient {
    /// Azure subscription ID
    subscription_id: String,
    /// Resource group name
    resource_group: String,
    /// Vault name
    vault_name: String,
    /// Access token for authentication
    access_token: String,
    /// HTTP client
    client: reqwest::Client,
}

impl RecoveryServicesClient {
    /// Create a new Recovery Services REST client
    pub fn new(
        subscription_id: String,
        resource_group: String,
        vault_name: String,
        access_token: String,
    ) -> Self {
        let client = reqwest::Client::new();
        Self {
            subscription_id,
            resource_group,
            vault_name,
            access_token,
            client,
        }
    }

    /// Get the base URL for Recovery Services API
    fn get_base_url(&self) -> String {
        format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}",
            self.subscription_id, self.resource_group, self.vault_name
        )
    }

    /// Execute a GET request to the Recovery Services API
    async fn get_request(&self, endpoint: &str) -> Result<Value> {
        let url = format!("{}{}", self.get_base_url(), endpoint);
        tracing::debug!("GET request to: {}", url);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Content-Type", "application/json")
            .query(&[("api-version", "2021-12-01")])
            .send()
            .await
            .map_err(|e| CodexErr::Other(format!("Failed to send GET request: {}", e)))?;

        self.handle_response(response).await
    }

    /// Execute a POST request to the Recovery Services API
    async fn post_request(&self, endpoint: &str, body: Value) -> Result<Value> {
        let url = format!("{}{}", self.get_base_url(), endpoint);
        tracing::debug!("POST request to: {}", url);
        tracing::debug!("Request body: {}", serde_json::to_string_pretty(&body).unwrap_or_default());

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Content-Type", "application/json")
            .query(&[("api-version", "2021-12-01")])
            .json(&body)
            .send()
            .await
            .map_err(|e| CodexErr::Other(format!("Failed to send POST request: {}", e)))?;

        self.handle_response(response).await
    }

    /// Execute a PUT request to the Recovery Services API
    async fn put_request(&self, endpoint: &str, body: Value) -> Result<Value> {
        let url = format!("{}{}", self.get_base_url(), endpoint);
        tracing::debug!("PUT request to: {}", url);
        tracing::debug!("Request body: {}", serde_json::to_string_pretty(&body).unwrap_or_default());

        let response = self
            .client
            .put(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Content-Type", "application/json")
            .query(&[("api-version", "2021-12-01")])
            .json(&body)
            .send()
            .await
            .map_err(|e| CodexErr::Other(format!("Failed to send PUT request: {}", e)))?;

        self.handle_response(response).await
    }

    /// Execute a DELETE request to the Recovery Services API
    async fn delete_request(&self, endpoint: &str) -> Result<Value> {
        let url = format!("{}{}", self.get_base_url(), endpoint);
        tracing::debug!("DELETE request to: {}", url);

        let response = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Content-Type", "application/json")
            .query(&[("api-version", "2021-12-01")])
            .send()
            .await
            .map_err(|e| CodexErr::Other(format!("Failed to send DELETE request: {}", e)))?;

        self.handle_response(response).await
    }

    /// Handle HTTP response and parse JSON
    async fn handle_response(&self, response: reqwest::Response) -> Result<Value> {
        let status = response.status();
        tracing::debug!("Response status: {}", status);

        if status.is_success() {
            let response_text = response.text().await.map_err(|e| {
                CodexErr::Other(format!("Failed to read response text: {}", e))
            })?;

            if response_text.is_empty() {
                return Ok(json!({}));
            }

            serde_json::from_str(&response_text).map_err(|e| {
                tracing::error!("Failed to parse JSON response: {}", e);
                tracing::debug!("Response text: {}", response_text);
                CodexErr::Other(format!("Failed to parse response: {}", e))
            })
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            tracing::error!("API request failed with status: {}", status);
            tracing::error!("Error response: {}", error_text);

            Err(CodexErr::Other(format!(
                "Recovery Services API request failed: {} - {}",
                status, error_text
            )))
        }
    }

    /// List Recovery Services vaults (requires subscription-level access)
    pub async fn list_vaults(&self) -> Result<Vec<VaultInfo>> {
        let url = format!(
            "https://management.azure.com/subscriptions/{}/providers/Microsoft.RecoveryServices/vaults",
            self.subscription_id
        );

        tracing::debug!("Listing vaults with URL: {}", url);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Content-Type", "application/json")
            .query(&[("api-version", "2021-12-01")])
            .send()
            .await
            .map_err(|e| CodexErr::Other(format!("Failed to list vaults: {}", e)))?;

        let json_response = self.handle_response(response).await?;
        
        if let Some(vaults_array) = json_response.get("value").and_then(|v| v.as_array()) {
            let mut vaults = Vec::new();
            for vault_json in vaults_array {
                if let Ok(vault) = serde_json::from_value::<VaultInfo>(vault_json.clone()) {
                    vaults.push(vault);
                }
            }
            Ok(vaults)
        } else {
            Ok(Vec::new())
        }
    }

    /// Get vault properties
    pub async fn get_vault_properties(&self) -> Result<VaultInfo> {
        let response = self.get_request("").await?;
        serde_json::from_value(response).map_err(|e| {
            CodexErr::Other(format!("Failed to parse vault properties: {}", e))
        })
    }

    /// List backup containers (registered VMs)
    pub async fn list_backup_containers(&self) -> Result<Vec<BackupContainer>> {
        let endpoint = "/backupFabrics/Azure/protectionContainers";
        let response = self.get_request(endpoint).await?;
        
        if let Some(containers_array) = response.get("value").and_then(|v| v.as_array()) {
            let mut containers = Vec::new();
            for container_json in containers_array {
                if let Ok(container) = serde_json::from_value::<BackupContainer>(container_json.clone()) {
                    containers.push(container);
                }
            }
            Ok(containers)
        } else {
            Ok(Vec::new())
        }
    }

    /// Register VM for backup
    pub async fn register_vm(&self, vm_resource_id: &str, workload_type: WorkloadType) -> Result<Value> {
        let container_name = self.generate_container_name(vm_resource_id);
        let endpoint = format!("/backupFabrics/Azure/protectionContainers/{}", container_name);
        
        let body = json!({
            "properties": {
                "containerType": "VMAppContainer",
                "sourceResourceId": vm_resource_id,
                "registrationStatus": "Registered",
                "workloadType": workload_type.to_string(),
                "backupManagementType": "AzureWorkload"
            }
        });

        self.put_request(&endpoint, body).await
    }

    /// List protectable items
    pub async fn list_protectable_items(&self, workload_type: Option<WorkloadType>) -> Result<Vec<ProtectableItem>> {
        let mut endpoint = "/backupProtectableItems".to_string();
        
        if let Some(wl_type) = workload_type {
            endpoint.push_str(&format!("?$filter=backupManagementType eq 'AzureWorkload' and workloadType eq '{}'", wl_type));
        }

        let response = self.get_request(&endpoint).await?;
        
        if let Some(items_array) = response.get("value").and_then(|v| v.as_array()) {
            let mut items = Vec::new();
            for item_json in items_array {
                if let Ok(item) = serde_json::from_value::<ProtectableItem>(item_json.clone()) {
                    items.push(item);
                }
            }
            Ok(items)
        } else {
            Ok(Vec::new())
        }
    }

    /// List protected items
    pub async fn list_protected_items(&self, workload_type: Option<WorkloadType>) -> Result<Vec<ProtectedItem>> {
        let mut endpoint = "/backupProtectedItems".to_string();
        
        if let Some(wl_type) = workload_type {
            endpoint.push_str(&format!("?$filter=backupManagementType eq 'AzureWorkload' and workloadType eq '{}'", wl_type));
        }

        let response = self.get_request(&endpoint).await?;
        
        if let Some(items_array) = response.get("value").and_then(|v| v.as_array()) {
            let mut items = Vec::new();
            for item_json in items_array {
                if let Ok(item) = serde_json::from_value::<ProtectedItem>(item_json.clone()) {
                    items.push(item);
                }
            }
            Ok(items)
        } else {
            Ok(Vec::new())
        }
    }

    /// List backup jobs
    pub async fn list_backup_jobs(&self, filter: Option<&str>) -> Result<Vec<BackupJob>> {
        let mut endpoint = "/backupJobs".to_string();
        
        if let Some(filter_str) = filter {
            endpoint.push_str(&format!("?$filter={}", filter_str));
        }

        let response = self.get_request(&endpoint).await?;
        
        if let Some(jobs_array) = response.get("value").and_then(|v| v.as_array()) {
            let mut jobs = Vec::new();
            for job_json in jobs_array {
                if let Ok(job) = serde_json::from_value::<BackupJob>(job_json.clone()) {
                    jobs.push(job);
                }
            }
            Ok(jobs)
        } else {
            Ok(Vec::new())
        }
    }

    /// List backup policies
    pub async fn list_backup_policies(&self, workload_type: Option<WorkloadType>) -> Result<Vec<BackupPolicy>> {
        let mut endpoint = "/backupPolicies".to_string();
        
        if let Some(wl_type) = workload_type {
            endpoint.push_str(&format!("?$filter=backupManagementType eq 'AzureWorkload' and workloadType eq '{}'", wl_type));
        }

        let response = self.get_request(&endpoint).await?;
        
        if let Some(policies_array) = response.get("value").and_then(|v| v.as_array()) {
            let mut policies = Vec::new();
            for policy_json in policies_array {
                if let Ok(policy) = serde_json::from_value::<BackupPolicy>(policy_json.clone()) {
                    policies.push(policy);
                }
            }
            Ok(policies)
        } else {
            Ok(Vec::new())
        }
    }

    /// Generate container name from VM resource ID
    fn generate_container_name(&self, vm_resource_id: &str) -> String {
        // Extract VM name from resource ID
        let vm_name = vm_resource_id.split('/').last().unwrap_or("unknown");
        format!("vmappcontainer;compute;{};{}", self.resource_group, vm_name)
    }

    /// Test basic connectivity
    pub async fn test_connectivity(&self) -> Result<()> {
        tracing::info!("Testing Recovery Services API connectivity...");
        
        let _vault_info = self.get_vault_properties().await?;
        
        tracing::info!("Recovery Services API connectivity test successful");
        Ok(())
    }
}
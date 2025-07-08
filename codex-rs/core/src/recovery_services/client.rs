//! REST API client for Recovery Services (Azure Backup).

use crate::error::{CodexErr, Result};
use crate::recovery_services::models::*;
use serde_json::{json, Value};
use chrono;

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
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))  // 30 second timeout per request
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
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
    pub async fn get_request(&self, endpoint: &str) -> Result<Value> {
        let url = format!("{}{}", self.get_base_url(), endpoint);
        tracing::debug!("GET request to: {}", url);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Content-Type", "application/json")
            .query(&[("api-version", "2025-02-01")])
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
            .query(&[("api-version", "2025-02-01")])
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
            .query(&[("api-version", "2016-06-01")])
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

    /// Handle HTTP response and parse JSON, including async operations
    async fn handle_response(&self, response: reqwest::Response) -> Result<Value> {
        let status = response.status();
        tracing::debug!("Response status: {}", status);

        // Check for async operation (202 Accepted)
        if status == 202 {
            let location_header = response.headers().get("location")
                .and_then(|h| h.to_str().ok())
                .map(|s| s.to_string());
            
            let azure_async_operation = response.headers().get("azure-asyncoperation")
                .and_then(|h| h.to_str().ok())
                .map(|s| s.to_string());

            let response_text = response.text().await.map_err(|e| {
                CodexErr::Other(format!("Failed to read response text: {}", e))
            })?;

            let mut result = if response_text.is_empty() {
                json!({})
            } else {
                serde_json::from_str(&response_text).unwrap_or_else(|_| json!({}))
            };

            // Add async operation tracking information
            result["async_operation"] = json!({
                "status": "accepted",
                "location_header": location_header,
                "azure_async_operation": azure_async_operation,
                "message": "Operation accepted and is running asynchronously. Use the location header to track progress."
            });

            return Ok(result);
        }

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

        tracing::info!("Listing vaults with URL: {}", url);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Content-Type", "application/json")
            // Use the latest stable API version
            .query(&[("api-version", "2025-02-01")])
            .send()
            .await
            .map_err(|e| CodexErr::Other(format!("Failed to list vaults: {}", e)))?;

        let json_response = self.handle_response(response).await?;
        tracing::debug!("Vault list response: {:?}", json_response);
        
        if let Some(vaults_array) = json_response.get("value").and_then(|v| v.as_array()) {
            let mut vaults = Vec::new();
            for vault_json in vaults_array {
                // Extract resource group and subscription ID from the vault ID
                let mut vault_data = vault_json.clone();
                
                // Get the ID string first to avoid borrowing issues
                let id_string = vault_data.get("id").and_then(|v| v.as_str()).map(|s| s.to_string());
                
                if let Some(id) = id_string {
                    // Parse subscription ID from ID: /subscriptions/{sub}/resourceGroups/{rg}/providers/Microsoft.RecoveryServices/vaults/{name}
                    if let Some(sub_start) = id.find("/subscriptions/") {
                        let sub_part = &id[sub_start + 14..]; // Skip "/subscriptions/"
                        if let Some(sub_end) = sub_part.find('/') {
                            let subscription_id = &sub_part[..sub_end];
                            vault_data["subscription_id"] = serde_json::Value::String(subscription_id.to_string());
                        }
                    }
                    
                    // Parse resource group from ID
                    if let Some(rg_start) = id.find("/resourceGroups/") {
                        let rg_part = &id[rg_start + 16..]; // Skip "/resourceGroups/"
                        if let Some(rg_end) = rg_part.find('/') {
                            let resource_group = &rg_part[..rg_end];
                            vault_data["resource_group"] = serde_json::Value::String(resource_group.to_string());
                        }
                    }
                }
                
                match serde_json::from_value::<VaultInfo>(vault_data) {
                    Ok(vault) => vaults.push(vault),
                    Err(e) => {
                        tracing::warn!("Failed to parse vault info: {:?}, error: {}", vault_json, e);
                    }
                }
            }
            tracing::info!("Found {} vaults in subscription", vaults.len());
            Ok(vaults)
        } else {
            tracing::warn!("No 'value' array found in response: {:?}", json_response);
            Ok(Vec::new())
        }
    }

    /// Get vault properties
    pub async fn get_vault_properties(&self) -> Result<VaultInfo> {
        let mut response = self.get_request("").await?;
        
        // Extract resource group and subscription ID from the vault ID if not already present
        let id_string = response.get("id").and_then(|v| v.as_str()).map(|s| s.to_string());
        
        if let Some(id) = id_string {
            // Parse subscription ID from ID: /subscriptions/{sub}/resourceGroups/{rg}/providers/Microsoft.RecoveryServices/vaults/{name}
            if response.get("subscription_id").is_none() {
                if let Some(sub_start) = id.find("/subscriptions/") {
                    let sub_part = &id[sub_start + 14..]; // Skip "/subscriptions/"
                    if let Some(sub_end) = sub_part.find('/') {
                        let subscription_id = &sub_part[..sub_end];
                        response["subscription_id"] = serde_json::Value::String(subscription_id.to_string());
                    }
                }
            }
            
            // Parse resource group from ID if not already present
            if response.get("resource_group").is_none() {
                if let Some(rg_start) = id.find("/resourceGroups/") {
                    let rg_part = &id[rg_start + 16..]; // Skip "/resourceGroups/"
                    if let Some(rg_end) = rg_part.find('/') {
                        let resource_group = &rg_part[..rg_end];
                        response["resource_group"] = serde_json::Value::String(resource_group.to_string());
                    }
                }
            }
        }
        
        // If we still don't have resource_group, use the client's known resource group
        if response.get("resource_group").is_none() {
            response["resource_group"] = serde_json::Value::String(self.resource_group.clone());
        }
        
        // If we still don't have subscription_id, use the client's known subscription ID
        if response.get("subscription_id").is_none() {
            response["subscription_id"] = serde_json::Value::String(self.subscription_id.clone());
        }
        
        serde_json::from_value(response).map_err(|e| {
            CodexErr::Other(format!("Failed to parse vault properties: {}", e))
        })
    }

    /// List backup containers (registered VMs)
    pub async fn list_backup_containers(&self) -> Result<Vec<BackupContainer>> {
        // Try multiple backup management types since the endpoint requires this parameter
        let backup_management_types = vec!["AzureIaasVM", "AzureWorkload", "AzureStorage", "MAB"];
        let mut all_containers = Vec::new();
        
        for backup_type in &backup_management_types {
            let endpoint = format!("/backupProtectionContainers?$filter=backupManagementType eq '{}'", backup_type);
            
            tracing::debug!("Listing backup containers with endpoint: {} for type: {}", endpoint, backup_type);
            
            match self.get_request(&endpoint).await {
                Ok(response) => {
                    if let Some(containers_array) = response.get("value").and_then(|v| v.as_array()) {
                        for container_json in containers_array {
                            match serde_json::from_value::<BackupContainer>(container_json.clone()) {
                                Ok(container) => all_containers.push(container),
                                Err(e) => {
                                    tracing::warn!("Failed to parse container info: {:?}, error: {}", container_json, e);
                                }
                            }
                        }
                        tracing::info!("Found {} backup containers for type {}", containers_array.len(), backup_type);
                    }
                },
                Err(e) => {
                    tracing::debug!("Failed to list containers for backup type {}: {}", backup_type, e);
                    // Continue with next backup type instead of failing completely
                }
            }
        }
        
        tracing::info!("Found total {} backup containers across all types", all_containers.len());
        Ok(all_containers)
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
        // First, get all containers to find protectable items within them
        let containers = self.list_backup_containers().await?;
        let mut all_items = Vec::new();
        
        // If no containers, try the direct endpoint as fallback
        if containers.is_empty() {
            let mut endpoint = "/backupProtectableItems".to_string();
            
            if let Some(wl_type) = workload_type {
                endpoint.push_str(&format!("?$filter=backupManagementType eq 'AzureWorkload' and workloadType eq '{}'", wl_type));
            } else {
                endpoint.push_str("?$filter=backupManagementType eq 'AzureWorkload'");
            }

            let response = self.get_request(&endpoint).await?;
            
            if let Some(items_array) = response.get("value").and_then(|v| v.as_array()) {
                for item_json in items_array {
                    if let Ok(item) = serde_json::from_value::<ProtectableItem>(item_json.clone()) {
                        all_items.push(item);
                    }
                }
            }
        } else {
            // List protectable items for each container
            for container in containers {
                let container_name = container.name;
                let mut endpoint = format!("/backupFabrics/Azure/protectionContainers/{}/protectableItems", container_name);
                
                if let Some(wl_type) = &workload_type {
                    endpoint.push_str(&format!("?$filter=backupManagementType eq 'AzureWorkload' and workloadType eq '{}'", wl_type));
                } else {
                    endpoint.push_str("?$filter=backupManagementType eq 'AzureWorkload'");
                }

                match self.get_request(&endpoint).await {
                    Ok(response) => {
                        if let Some(items_array) = response.get("value").and_then(|v| v.as_array()) {
                            for item_json in items_array {
                                if let Ok(item) = serde_json::from_value::<ProtectableItem>(item_json.clone()) {
                                    all_items.push(item);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        tracing::warn!("Failed to list protectable items for container {}: {}", container_name, e);
                    }
                }
            }
        }
        
        Ok(all_items)
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

    /// Get vault configuration
    pub async fn get_vault_config(&self) -> Result<Value> {
        // Use the correct endpoint for vault configuration with preview API version
        let endpoint = "/backupconfig/vaultconfig";
        
        // Override API version for this specific endpoint that requires preview version
        let url = format!("{}{}", self.get_base_url(), endpoint);
        tracing::debug!("GET request to vault config: {}", url);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Content-Type", "application/json")
            .query(&[("api-version", "2025-02-28-preview")])  // Use preview API version
            .send()
            .await
            .map_err(|e| CodexErr::Other(format!("Failed to send GET request: {}", e)))?;

        self.handle_response(response).await
    }

    /// Update vault configuration
    pub async fn update_vault_config(&self, enhanced_security: bool, soft_delete_enabled: bool) -> Result<Value> {
        let endpoint = "/backupconfig";
        let body = json!({
            "properties": {
                "enhancedSecurityState": if enhanced_security { "Enabled" } else { "Disabled" },
                "softDeleteFeatureState": if soft_delete_enabled { "Enabled" } else { "Disabled" }
            }
        });
        
        self.put_request(endpoint, body).await
    }

    /// Get specific backup job details
    pub async fn get_backup_job(&self, job_id: &str) -> Result<BackupJob> {
        let endpoint = format!("/backupJobs/{}", job_id);
        let response = self.get_request(&endpoint).await?;
        
        serde_json::from_value(response).map_err(|e| {
            CodexErr::Other(format!("Failed to parse backup job: {}", e))
        })
    }

    /// Cancel a backup job
    pub async fn cancel_backup_job(&self, job_id: &str) -> Result<Value> {
        let endpoint = format!("/backupJobs/{}/cancel", job_id);
        self.post_request(&endpoint, json!({})).await
    }

    /// List recovery points for a protected item
    pub async fn list_recovery_points(&self, container_name: &str, protected_item_name: &str, filter: Option<&str>) -> Result<Vec<RecoveryPoint>> {
        let mut endpoint = format!("/backupFabrics/Azure/protectionContainers/{}/protectedItems/{}/recoveryPoints", container_name, protected_item_name);
        
        if let Some(filter_str) = filter {
            endpoint.push_str(&format!("?$filter={}", filter_str));
        }

        let response = self.get_request(&endpoint).await?;
        
        if let Some(points_array) = response.get("value").and_then(|v| v.as_array()) {
            let mut points = Vec::new();
            for point_json in points_array {
                if let Ok(point) = serde_json::from_value::<RecoveryPoint>(point_json.clone()) {
                    points.push(point);
                }
            }
            Ok(points)
        } else {
            Ok(Vec::new())
        }
    }

    /// Get specific recovery point details
    pub async fn get_recovery_point(&self, container_name: &str, protected_item_name: &str, recovery_point_id: &str) -> Result<RecoveryPoint> {
        let endpoint = format!("/backupFabrics/Azure/protectionContainers/{}/protectedItems/{}/recoveryPoints/{}", 
                              container_name, protected_item_name, recovery_point_id);
        let response = self.get_request(&endpoint).await?;
        
        serde_json::from_value(response).map_err(|e| {
            CodexErr::Other(format!("Failed to parse recovery point: {}", e))
        })
    }

    /// Trigger backup for a protected item
    pub async fn trigger_backup(&self, container_name: &str, protected_item_name: &str, retention_days: Option<u32>) -> Result<Value> {
        let endpoint = format!("/backupFabrics/Azure/protectionContainers/{}/protectedItems/{}/backup", 
                              container_name, protected_item_name);
        
        let expiry_time = if let Some(days) = retention_days {
            let expiry = chrono::Utc::now() + chrono::Duration::days(days as i64);
            expiry.to_rfc3339()
        } else {
            let expiry = chrono::Utc::now() + chrono::Duration::days(30);
            expiry.to_rfc3339()
        };

        let body = json!({
            "properties": {
                "objectType": "IaasVMBackupRequest",
                "recoveryPointExpiryTimeInUTC": expiry_time
            }
        });
        
        self.post_request(&endpoint, body).await
    }

    /// Restore VM from recovery point
    pub async fn restore_vm(&self, container_name: &str, protected_item_name: &str, recovery_point_id: &str, 
                           restore_type: &str, target_vm_name: Option<&str>, target_resource_group: Option<&str>) -> Result<Value> {
        let endpoint = format!("/backupFabrics/Azure/protectionContainers/{}/protectedItems/{}/recoveryPoints/{}/restore", 
                              container_name, protected_item_name, recovery_point_id);
        
        let mut body = json!({
            "properties": {
                "objectType": "IaasVMRestoreRequest",
                "recoveryPointId": recovery_point_id,
                "recoveryType": restore_type,
                "sourceResourceId": format!("/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines/{}", 
                                          self.subscription_id, self.resource_group, protected_item_name.split(';').last().unwrap_or("unknown"))
            }
        });

        if restore_type == "AlternateLocation" {
            if let (Some(target_vm), Some(target_rg)) = (target_vm_name, target_resource_group) {
                body["properties"]["targetVirtualMachineName"] = json!(target_vm);
                body["properties"]["targetResourceGroupId"] = json!(format!("/subscriptions/{}/resourceGroups/{}", 
                                                                           self.subscription_id, target_rg));
            }
        }
        
        self.post_request(&endpoint, body).await
    }

    /// Enable protection for a VM
    pub async fn enable_protection(&self, container_name: &str, protected_item_name: &str, policy_name: &str, vm_resource_id: &str) -> Result<Value> {
        let endpoint = format!("/backupFabrics/Azure/protectionContainers/{}/protectedItems/{}", 
                              container_name, protected_item_name);
        
        let body = json!({
            "properties": {
                "protectedItemType": "Microsoft.Compute/virtualMachines",
                "sourceResourceId": vm_resource_id,
                "policyId": format!("{}/backupPolicies/{}", self.get_base_url(), policy_name),
                "protectionState": "ProtectionEnabled"
            }
        });
        
        self.put_request(&endpoint, body).await
    }

    /// Disable protection for a VM
    pub async fn disable_protection(&self, container_name: &str, protected_item_name: &str, delete_backup_data: bool) -> Result<Value> {
        let endpoint = format!("/backupFabrics/Azure/protectionContainers/{}/protectedItems/{}", 
                              container_name, protected_item_name);
        
        if delete_backup_data {
            self.delete_request(&endpoint).await
        } else {
            let body = json!({
                "properties": {
                    "protectionState": "ProtectionStopped"
                }
            });
            self.put_request(&endpoint, body).await
        }
    }

    /// Disable workload protection (for databases like SAP ASE) using DELETE request
    pub async fn disable_workload_protection(&self, container_name: &str, protected_item_name: &str) -> Result<Value> {
        let endpoint = format!("/backupFabrics/Azure/protectionContainers/{}/protectedItems/{}", 
                              container_name, protected_item_name);
        
        // Use a custom DELETE request with the correct API version for workload protection
        let url = format!("{}{}", self.get_base_url(), endpoint);
        tracing::debug!("DELETE request to: {}", url);

        let response = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .query(&[("api-version", "2018-01-10")])  // Use the API version from SAP ASE example
            .send()
            .await
            .map_err(|e| CodexErr::Other(format!("Failed to send DELETE request: {}", e)))?;

        self.handle_response(response).await
    }

    /// Create backup policy
    pub async fn create_backup_policy(&self, policy_name: &str, schedule_type: &str, retention_days: u32) -> Result<Value> {
        let endpoint = format!("/backupPolicies/{}", policy_name);
        
        let schedule_time = "2023-01-01T01:00:00Z";
        
        let body = json!({
            "properties": {
                "backupManagementType": "AzureIaasVM",
                "schedulePolicy": {
                    "schedulePolicyType": "SimpleSchedulePolicy",
                    "scheduleRunFrequency": schedule_type,
                    "scheduleRunTimes": [schedule_time],
                    "scheduleWeeklyFrequency": 0
                },
                "retentionPolicy": {
                    "retentionPolicyType": "LongTermRetentionPolicy",
                    "dailySchedule": {
                        "retentionTimes": [schedule_time],
                        "retentionDuration": {
                            "count": retention_days,
                            "durationType": "Days"
                        }
                    }
                },
                "timeZone": "UTC"
            }
        });
        
        self.put_request(&endpoint, body).await
    }

    /// Get backup policy details
    pub async fn get_backup_policy(&self, policy_name: &str) -> Result<BackupPolicy> {
        let endpoint = format!("/backupPolicies/{}", policy_name);
        let response = self.get_request(&endpoint).await?;
        
        serde_json::from_value(response).map_err(|e| {
            CodexErr::Other(format!("Failed to parse backup policy: {}", e))
        })
    }

    /// Generate container name from VM details
    pub fn generate_vm_container_name(&self, vm_resource_group: &str, vm_name: &str) -> String {
        format!("iaasvmcontainer;iaasvmcontainerv2;{};{}", vm_resource_group, vm_name)
    }

    /// Generate protected item name from VM details
    pub fn generate_vm_protected_item_name(&self, vm_resource_group: &str, vm_name: &str) -> String {
        format!("vm;iaasvmcontainerv2;{};{}", vm_resource_group, vm_name)
    }

    /// Generate VM resource ID
    pub fn generate_vm_resource_id(&self, vm_resource_group: &str, vm_name: &str) -> String {
        format!("/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines/{}", 
                self.subscription_id, vm_resource_group, vm_name)
    }

    /// List protectable items for workload discovery
    pub async fn list_protectable_items_for_workload(&self, workload_type: &str, server_name: Option<&str>) -> Result<Vec<Value>> {
        let mut endpoint = format!("/backupProtectableItems?$filter=backupManagementType eq 'AzureWorkload' and workloadType eq '{}'", workload_type);
        
        if let Some(server) = server_name {
            endpoint.push_str(&format!(" and serverName eq '{}'", server));
        }

        let response = self.get_request(&endpoint).await?;
        
        if let Some(items_array) = response.get("value").and_then(|v| v.as_array()) {
            Ok(items_array.clone())
        } else {
            Ok(Vec::new())
        }
    }

    /// Register VM for workload backup
    pub async fn register_vm_for_workload(&self, vm_resource_id: &str, workload_type: &str) -> Result<Value> {
        // Extract VM name and resource group from resource ID for container name
        let parts: Vec<&str> = vm_resource_id.split('/').collect();
        if parts.len() < 9 {
            return Err(CodexErr::Other("Invalid VM resource ID format".to_string()));
        }
        
        let vm_resource_group = parts[4];
        let vm_name = parts[8];
        
        // Generate container name for workload backup
        let container_name = format!("VMAppContainer;Compute;{};{}", vm_resource_group, vm_name);
        let endpoint = format!("/backupFabrics/Azure/protectionContainers/{}", container_name);
        
        let body = json!({
            "properties": {
                "sourceResourceId": vm_resource_id,
                "workloadType": workload_type,
                "backupManagementType": "AzureWorkload",
                "containerType": "VMAppContainer"
            }
        });
        
        tracing::info!("Registering VM for workload backup: PUT {}", endpoint);
        tracing::info!("Request body: {}", serde_json::to_string_pretty(&body).unwrap_or_default());
        
        self.put_request(&endpoint, body).await
    }

    /// Re-register container for workload
    pub async fn reregister_container(&self, container_name: &str, workload_type: &str) -> Result<Value> {
        let endpoint = format!("/backupFabrics/Azure/protectionContainers/{}/inquire", container_name);
        
        let body = json!({
            "properties": {
                "workloadType": workload_type,
                "backupManagementType": "AzureWorkload"
            }
        });
        
        self.post_request(&endpoint, body).await
    }

    /// Unregister container
    pub async fn unregister_container(&self, container_name: &str) -> Result<Value> {
        let endpoint = format!("/backupFabrics/Azure/protectionContainers/{}", container_name);
        self.delete_request(&endpoint).await
    }

    /// Unregister workload container (for VMs registered for workload backup like SAP ASE)
    pub async fn unregister_workload_container(&self, container_name: &str, vm_name: &str, vm_resource_group: &str, workload_type: &str) -> Result<Value> {
        let endpoint = format!("/backupFabrics/Azure/protectionContainers/{}", container_name);
        
        // Build the request body as shown in SAP ASE API documentation
        let vm_resource_id = format!("/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines/{}", 
                                   self.subscription_id, vm_resource_group, vm_name);
        
        let body = json!({
            "Id": format!("/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupFabrics/Azure/protectionContainers/{}", 
                         self.subscription_id, self.resource_group, self.vault_name, container_name),
            "name": container_name,
            "type": "",
            "properties": {
                "containerType": "VMAppContainer",
                "friendlyName": vm_name,
                "backupManagementType": "AzureWorkload",
                "sourceResourceId": vm_resource_id,
                "workloadType": workload_type
            },
            "SubscriptionId": self.subscription_id
        });
        
        // Use a custom DELETE request with body and correct API version for workload unregistration
        let url = format!("{}{}", self.get_base_url(), endpoint);
        tracing::debug!("DELETE request to: {}", url);
        tracing::debug!("Request body: {}", serde_json::to_string_pretty(&body).unwrap_or_default());

        let response = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .query(&[("api-version", "2018-01-10")])  // Use the API version from SAP ASE example
            .json(&body)  // Include the request body
            .send()
            .await
            .map_err(|e| CodexErr::Other(format!("Failed to send DELETE request: {}", e)))?;

        self.handle_response(response).await
    }

    /// Create workload-specific backup policy
    pub async fn create_workload_backup_policy(&self, policy_name: &str, workload_type: &str, policy_config: Value) -> Result<Value> {
        let endpoint = format!("/backupPolicies/{}", policy_name);
        
        let mut body = json!({
            "properties": {
                "backupManagementType": "AzureWorkload",
                "workLoadType": workload_type,
                "settings": {
                    "timeZone": "UTC",
                    "issqlcompression": false,
                    "isCompression": false
                }
            }
        });

        // Merge the provided policy configuration
        if let Some(properties) = body.get_mut("properties") {
            if let Some(config_obj) = policy_config.as_object() {
                for (key, value) in config_obj {
                    properties[key] = value.clone();
                }
            }
        }
        
        self.put_request(&endpoint, body).await
    }

    /// Enable protection for database
    pub async fn enable_database_protection(&self, container_name: &str, protectable_item_name: &str, policy_name: &str, workload_type: &str) -> Result<Value> {
        let endpoint = format!("/backupFabrics/Azure/protectionContainers/{}/protectedItems/{}", 
                              container_name, protectable_item_name);
        
        let body = json!({
            "properties": {
                "protectedItemType": format!("{}Database", workload_type),
                "policyId": format!("{}/backupPolicies/{}", self.get_base_url(), policy_name),
                "protectionState": "ProtectionEnabled",
                "workloadType": format!("{}Database", workload_type)
            }
        });
        
        self.put_request(&endpoint, body).await
    }

    /// Trigger database backup
    pub async fn trigger_database_backup(&self, container_name: &str, protected_item_name: &str, backup_type: &str, retention_date: Option<&str>) -> Result<Value> {
        let endpoint = format!("/backupFabrics/Azure/protectionContainers/{}/protectedItems/{}/backup", 
                              container_name, protected_item_name);
        
        let mut body = json!({
            "properties": {
                "objectType": "AzureWorkloadBackupRequest",
                "backupType": backup_type
            }
        });

        if let Some(retention) = retention_date {
            body["properties"]["recoveryPointExpiryTimeInUTC"] = json!(retention);
        }
        
        self.post_request(&endpoint, body).await
    }

    /// Restore database to original location
    pub async fn restore_database_original(&self, container_name: &str, protected_item_name: &str, recovery_point_id: &str, log_point_in_time: Option<&str>) -> Result<Value> {
        let endpoint = format!("/backupFabrics/Azure/protectionContainers/{}/protectedItems/{}/recoveryPoints/{}/restore", 
                              container_name, protected_item_name, recovery_point_id);
        
        let mut body = json!({
            "properties": {
                "objectType": "AzureWorkloadRestoreRequest",
                "recoveryType": "OriginalLocation",
                "recoveryPointId": recovery_point_id
            }
        });

        if let Some(log_time) = log_point_in_time {
            body["properties"]["pointInTime"] = json!(log_time);
        }
        
        self.post_request(&endpoint, body).await
    }

    /// Restore database to alternate location
    pub async fn restore_database_alternate(&self, container_name: &str, protected_item_name: &str, recovery_point_id: &str, 
                                          target_server: &str, target_database: &str, log_point_in_time: Option<&str>) -> Result<Value> {
        let endpoint = format!("/backupFabrics/Azure/protectionContainers/{}/protectedItems/{}/recoveryPoints/{}/restore", 
                              container_name, protected_item_name, recovery_point_id);
        
        let mut body = json!({
            "properties": {
                "objectType": "AzureWorkloadRestoreRequest",
                "recoveryType": "AlternateLocation",
                "recoveryPointId": recovery_point_id,
                "targetInfo": {
                    "overwriteOption": "Overwrite",
                    "containerId": format!("/subscriptions/{}/resourceGroups/{}/providers/Microsoft.RecoveryServices/vaults/{}/backupFabrics/Azure/protectionContainers/VMAppContainer;compute;{};{}", 
                                         self.subscription_id, self.resource_group, self.vault_name, self.resource_group, target_server),
                    "databaseName": target_database
                }
            }
        });

        if let Some(log_time) = log_point_in_time {
            body["properties"]["pointInTime"] = json!(log_time);
        }
        
        self.post_request(&endpoint, body).await
    }

    /// Restore database as files
    pub async fn restore_database_as_files(&self, container_name: &str, protected_item_name: &str, recovery_point_id: &str, 
                                         target_container: &str, file_path: &str, log_point_in_time: Option<&str>) -> Result<Value> {
        let endpoint = format!("/backupFabrics/Azure/protectionContainers/{}/protectedItems/{}/recoveryPoints/{}/restore", 
                              container_name, protected_item_name, recovery_point_id);
        
        let mut body = json!({
            "properties": {
                "objectType": "AzureWorkloadRestoreRequest",
                "recoveryType": "RestoreAsFiles",
                "recoveryPointId": recovery_point_id,
                "targetInfo": {
                    "containerId": target_container,
                    "targetDirectoryForFileRestore": file_path
                }
            }
        });

        if let Some(log_time) = log_point_in_time {
            body["properties"]["pointInTime"] = json!(log_time);
        }
        
        self.post_request(&endpoint, body).await
    }

    /// Generate recovery configuration for database restore
    pub async fn generate_recovery_config(&self, container_name: &str, protected_item_name: &str, recovery_point_name: &str,
                                         restore_mode: &str, target_server: Option<&str>, target_database: Option<&str>,
                                         log_point_in_time: Option<&str>, file_path: Option<&str>) -> Result<Value> {
        let mut endpoint = format!("/backupFabrics/Azure/protectionContainers/{}/protectedItems/{}/recoveryPoints/{}/provisionInstantItemRecovery", 
                                  container_name, protected_item_name, recovery_point_name);
        
        let mut query_params = vec![
            ("restoreMode".to_string(), restore_mode.to_string())
        ];

        if let Some(server) = target_server {
            query_params.push(("targetServerName".to_string(), server.to_string()));
        }

        if let Some(database) = target_database {
            query_params.push(("targetItemName".to_string(), database.to_string()));
        }

        if let Some(log_time) = log_point_in_time {
            query_params.push(("logPointInTime".to_string(), log_time.to_string()));
        }

        if let Some(path) = file_path {
            query_params.push(("filepath".to_string(), path.to_string()));
        }

        if !query_params.is_empty() {
            endpoint.push('?');
            endpoint.push_str(&query_params.iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&"));
        }

        let body = json!({
            "properties": {
                "objectType": "ILRRequestProperties"
            }
        });
        
        self.post_request(&endpoint, body).await
    }

    /// Track async operation status using location header
    pub async fn track_async_operation(&self, location_url: &str) -> Result<Value> {
        tracing::info!("Tracking async operation: {}", location_url);
        
        let response = self
            .client
            .get(location_url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| CodexErr::Other(format!("Failed to track async operation: {}", e)))?;

        let mut result = self.handle_response(response).await?;
        
        // Azure async operations can return different response formats
        // Sometimes the status is in the root, sometimes in properties
        // Let's normalize the response to make it easier to parse
        if result.get("status").is_none() {
            // Check if status is in properties
            if let Some(properties) = result.get("properties") {
                if let Some(status) = properties.get("status") {
                    result["status"] = status.clone();
                } else if let Some(provisioning_state) = properties.get("provisioningState") {
                    result["status"] = provisioning_state.clone();
                }
            }
        }
        
        // Add additional debugging information
        result["_debug"] = json!({
            "location_url": location_url,
            "response_structure": {
                "has_status": result.get("status").is_some(),
                "has_properties": result.get("properties").is_some(),
                "has_error": result.get("error").is_some(),
                "top_level_keys": result.as_object().map(|obj| obj.keys().collect::<Vec<_>>()).unwrap_or_default()
            }
        });
        
        Ok(result)
    }

    /// Check if an async operation is complete and get final result
    pub async fn wait_for_async_operation(&self, location_url: &str, max_wait_seconds: u64) -> Result<Value> {
        let start_time = std::time::Instant::now();
        let max_duration = std::time::Duration::from_secs(max_wait_seconds);
        
        loop {
            let status_result = self.track_async_operation(location_url).await?;
            
            // Check if operation is complete - use the same logic as track_async_operation
            let status = status_result.get("status").and_then(|s| s.as_str())
                .or_else(|| status_result.get("properties").and_then(|p| p.get("status")).and_then(|s| s.as_str()))
                .or_else(|| status_result.get("properties").and_then(|p| p.get("provisioningState")).and_then(|s| s.as_str()));
            
            if let Some(status) = status {
                match status {
                    "Succeeded" | "Success" => {
                        tracing::info!("Async operation completed successfully");
                        return Ok(status_result);
                    },
                    "Failed" | "Error" => {
                        tracing::error!("Async operation failed: {:?}", status_result);
                        return Err(CodexErr::Other(format!("Async operation failed: {:?}", status_result)));
                    },
                    "InProgress" | "Running" | "Accepted" => {
                        tracing::debug!("Async operation still in progress...");
                        // Continue waiting
                    },
                    _ => {
                        tracing::warn!("Unknown async operation status: {}", status);
                    }
                }
            } else {
                tracing::debug!("No status found in async operation response: {:?}", status_result);
            }
            
            // Check timeout
            if start_time.elapsed() > max_duration {
                tracing::warn!("Async operation tracking timed out after {} seconds", max_wait_seconds);
                return Ok(json!({
                    "status": "timeout",
                    "message": format!("Operation tracking timed out after {} seconds. Operation may still be running.", max_wait_seconds),
                    "last_known_status": status_result
                }));
            }
            
            // Wait before next check
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    }

    /// Refresh containers (discovery operation) - triggers discovery of eligible containers
    pub async fn refresh_containers(&self, fabric_name: Option<&str>) -> Result<Value> {
        let fabric = fabric_name.unwrap_or("Azure");
        let endpoint = format!("/backupFabrics/{}/refreshContainers", fabric);
        
        tracing::info!("Triggering container discovery for fabric: {}", fabric);
        
        // This is a POST request with empty body to trigger the discovery operation
        let body = json!({});
        self.post_request(&endpoint, body).await
    }

    /// List protectable containers - containers that can be registered but aren't yet
    pub async fn list_protectable_containers(&self, fabric_name: Option<&str>, backup_management_type: Option<&str>) -> Result<Vec<Value>> {
        let fabric = fabric_name.unwrap_or("Azure");
        let mut endpoint = format!("/backupFabrics/{}/protectableContainers", fabric);
        
        // Add filter for backup management type if specified
        if let Some(backup_type) = backup_management_type {
            endpoint.push_str(&format!("?$filter=backupManagementType eq '{}'", backup_type));
        }
        
        tracing::info!("Listing protectable containers for fabric: {}, backup type: {:?}", fabric, backup_management_type);
        
        let response = self.get_request(&endpoint).await?;
        
        if let Some(containers_array) = response.get("value").and_then(|v| v.as_array()) {
            tracing::info!("Found {} protectable containers", containers_array.len());
            Ok(containers_array.clone())
        } else {
            tracing::warn!("No 'value' array found in protectable containers response: {:?}", response);
            Ok(Vec::new())
        }
    }

    /// List protectable items (workloads/databases that can be protected) - new version
    pub async fn list_protectable_items_new(&self, workload_type: Option<&str>, backup_management_type: Option<&str>) -> Result<Vec<Value>> {
        let mut endpoint = "/backupProtectableItems".to_string();
        
        // Build filter - Azure API requires backupManagementType filter
        let mut filters = Vec::new();
        
        // Add backup management type filter (required)
        if let Some(backup_type) = backup_management_type {
            filters.push(format!("backupManagementType eq '{}'", backup_type));
        } else {
            // Default to AzureWorkload if not specified
            filters.push("backupManagementType eq 'AzureWorkload'".to_string());
        }
        
        // Add workload type filter if specified
        if let Some(workload) = workload_type {
            filters.push(format!("workloadType eq '{}'", workload));
        }
        
        if !filters.is_empty() {
            endpoint.push_str(&format!("?$filter={}", filters.join(" and ")));
        }
        
        tracing::info!("Listing protectable items, workload type: {:?}, backup management type: {:?}", workload_type, backup_management_type);
        tracing::info!("Using endpoint: {}", endpoint);
        
        // Use a custom request with the correct API version for this endpoint
        let url = format!("{}{}", self.get_base_url(), endpoint);
        tracing::debug!("GET request to: {}", url);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Content-Type", "application/json")
            .query(&[("api-version", "2018-01-10")])  // Use the API version from the example
            .send()
            .await
            .map_err(|e| CodexErr::Other(format!("Failed to send GET request: {}", e)))?;

        let json_response = self.handle_response(response).await?;
        
        if let Some(items_array) = json_response.get("value").and_then(|v| v.as_array()) {
            tracing::info!("Found {} protectable items", items_array.len());
            Ok(items_array.clone())
        } else {
            tracing::warn!("No 'value' array found in protectable items response: {:?}", json_response);
            Ok(Vec::new())
        }
    }

    /// List workload items (registered/protected workloads)
    pub async fn list_workload_items(&self, workload_type: Option<&str>) -> Result<Vec<Value>> {
        let mut endpoint = "/backupWorkloadItems".to_string();
        
        // Add filter for workload type if specified
        if let Some(workload) = workload_type {
            endpoint.push_str(&format!("?$filter=workloadType eq '{}'", workload));
        }
        
        tracing::info!("Listing workload items, workload type: {:?}", workload_type);
        
        let response = self.get_request(&endpoint).await?;
        
        if let Some(items_array) = response.get("value").and_then(|v| v.as_array()) {
            tracing::info!("Found {} workload items", items_array.len());
            Ok(items_array.clone())
        } else {
            tracing::warn!("No 'value' array found in workload items response: {:?}", response);
            Ok(Vec::new())
        }
    }

    /// Discover databases for a specific workload type using the inquire endpoint
    /// This matches the curl example: POST .../inquire?$filter=workloadType eq 'SAPAseDatabase'
    pub async fn inquire_workload_databases(&self, container_name: &str, workload_type: &str) -> Result<Value> {
        let endpoint = format!("/backupFabrics/Azure/protectionContainers/{}/inquire", container_name);
        
        tracing::info!("Inquiring workload databases for container: {}, workload type: {}", container_name, workload_type);
        
        // Build the URL with query parameters
        let url = format!("{}{}", self.get_base_url(), endpoint);
        
        // Create an empty JSON body for the POST request
        let body = json!({});
        
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Content-Type", "application/json")
            .header("Content-Length", "2") // Empty JSON object "{}" has length 2
            .query(&[
                ("api-version", "2018-01-10"),
                ("$filter", &format!("workloadType eq '{}'", workload_type))
            ])
            .json(&body) // This will automatically set Content-Length
            .send()
            .await
            .map_err(|e| CodexErr::Other(format!("Failed to send inquire request: {}", e)))?;

        self.handle_response(response).await
    }

    /// Enable workload protection with custom body
    pub async fn enable_workload_protection(&self, container_name: &str, protected_item_name: &str, _policy_name: &str, body: &Value) -> Result<Value> {
        let endpoint = format!("/backupFabrics/Azure/protectionContainers/{}/protectedItems/{}", 
                              container_name, protected_item_name);
        
        // Use a custom PUT request with the correct API version for workload protection
        let url = format!("{}{}", self.get_base_url(), endpoint);
        tracing::debug!("PUT request to: {}", url);
        tracing::debug!("Request body: {}", serde_json::to_string_pretty(body).unwrap_or_default());

        let response = self
            .client
            .put(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .query(&[("api-version", "2018-01-10")])  // Use the API version from the example
            .json(body)
            .send()
            .await
            .map_err(|e| CodexErr::Other(format!("Failed to send PUT request: {}", e)))?;

        self.handle_response(response).await
    }

    /// Trigger workload backup with custom body
    pub async fn trigger_workload_backup(&self, container_name: &str, protected_item_name: &str, body: &Value) -> Result<Value> {
        let endpoint = format!("/backupFabrics/Azure/protectionContainers/{}/protectedItems/{}/backup", 
                              container_name, protected_item_name);
        
        // Use a custom POST request with the correct API version for workload backup
        let url = format!("{}{}", self.get_base_url(), endpoint);
        tracing::debug!("POST request to: {}", url);
        tracing::debug!("Request body: {}", serde_json::to_string_pretty(body).unwrap_or_default());

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .query(&[("api-version", "2021-12-01")])  // Use newer API version for backup operations
            .json(body)
            .send()
            .await
            .map_err(|e| CodexErr::Other(format!("Failed to send POST request: {}", e)))?;

        self.handle_response(response).await
    }

}
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
        let response = self.get_request("").await?;
        serde_json::from_value(response).map_err(|e| {
            CodexErr::Other(format!("Failed to parse vault properties: {}", e))
        })
    }

    /// List backup containers (registered VMs)
    pub async fn list_backup_containers(&self) -> Result<Vec<BackupContainer>> {
        let endpoint = "/backupFabrics/Azure/protectionContainers";
        
        tracing::debug!("Listing backup containers with endpoint: {}", endpoint);
        
        let response = self.get_request(endpoint).await?;
        
        if let Some(containers_array) = response.get("value").and_then(|v| v.as_array()) {
            let mut containers = Vec::new();
            for container_json in containers_array {
                match serde_json::from_value::<BackupContainer>(container_json.clone()) {
                    Ok(container) => containers.push(container),
                    Err(e) => {
                        tracing::warn!("Failed to parse container info: {:?}, error: {}", container_json, e);
                    }
                }
            }
            tracing::info!("Found {} backup containers", containers.len());
            Ok(containers)
        } else {
            tracing::warn!("No 'value' array found in containers response: {:?}", response);
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
        let endpoint = "/backupconfig";
        self.get_request(endpoint).await
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
        let endpoint = "/backupFabrics/Azure/protectionContainers";
        
        let body = json!({
            "properties": {
                "containerType": "VMAppContainer",
                "sourceResourceId": vm_resource_id,
                "workloadType": workload_type,
                "backupManagementType": "AzureWorkload"
            }
        });
        
        self.post_request(endpoint, body).await
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
}
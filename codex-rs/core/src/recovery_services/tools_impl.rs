//! Implementation of Recovery Services (Azure Backup) tool functions.

use serde_json::{json, Value};
use std::sync::Arc;
use std::collections::HashMap;

use crate::recovery_services::client::RecoveryServicesClient;
use crate::recovery_services::models::*;
use crate::config_types::RecoveryServicesConfig;
use crate::error::{CodexErr, Result};

/// Implementation of Recovery Services tools
pub struct RecoveryServicesTools {
    pub clients: HashMap<String, Arc<RecoveryServicesClient>>,
    pub config: RecoveryServicesConfig,
}

impl RecoveryServicesTools {
    /// Create a new instance of Recovery Services tools
    pub async fn new(config: &RecoveryServicesConfig) -> Result<Self> {
        use crate::recovery_services::auth::RecoveryServicesAuthHandler;
        
        // Get codex home directory for OAuth token storage
        let codex_home = dirs::home_dir()
            .ok_or_else(|| CodexErr::Other("Could not determine home directory".to_string()))?
            .join(".codex");
        
        // Create auth handler using OAuth
        let auth = RecoveryServicesAuthHandler::from_oauth(&codex_home).await?;
        
        let mut clients = HashMap::new();
        
        // Add default vault client
        if !config.subscription_id.is_empty() && !config.resource_group.is_empty() && !config.vault_name.is_empty() {
            let access_token = match &auth.auth {
                crate::recovery_services::auth::RecoveryServicesAuth::OAuth(token) => token.clone(),
                crate::recovery_services::auth::RecoveryServicesAuth::None => {
                    return Err(CodexErr::Other("No authentication provided".to_string()));
                }
            };
            
            let client = RecoveryServicesClient::new(
                config.subscription_id.clone(),
                config.resource_group.clone(),
                config.vault_name.clone(),
                access_token.clone()
            );
            
            // Test connectivity
            if let Err(e) = client.test_connectivity().await {
                tracing::warn!("Recovery Services client connectivity test failed: {}", e);
            }
            
            clients.insert("default".to_string(), Arc::new(client));
            clients.insert(config.vault_name.clone(), Arc::new(RecoveryServicesClient::new(
                config.subscription_id.clone(),
                config.resource_group.clone(),
                config.vault_name.clone(),
                access_token
            )));
        }
        
        // Add clients for additional vaults
        for (vault_alias, vault_config) in &config.vaults {
            let subscription_id = vault_config.subscription_id.as_ref().unwrap_or(&config.subscription_id);
            let resource_group = vault_config.resource_group.as_ref().unwrap_or(&config.resource_group);
            
            let access_token = match &auth.auth {
                crate::recovery_services::auth::RecoveryServicesAuth::OAuth(token) => token.clone(),
                crate::recovery_services::auth::RecoveryServicesAuth::None => {
                    return Err(CodexErr::Other("No authentication provided".to_string()));
                }
            };
            
            let client = Arc::new(RecoveryServicesClient::new(
                subscription_id.clone(),
                resource_group.clone(),
                vault_config.name.clone(),
                access_token
            ));
            clients.insert(vault_alias.clone(), client.clone());
            clients.insert(vault_config.name.clone(), client);
        }
            
        Ok(Self {
            clients,
            config: config.clone(),
        })
    }
    
    /// Get the appropriate client for a vault
    fn get_client(&self, vault_name: Option<&str>) -> Result<Arc<RecoveryServicesClient>> {
        let vault_key = vault_name.unwrap_or("default");
        
        self.clients.get(vault_key)
            .or_else(|| self.clients.get("default"))
            .cloned()
            .ok_or_else(|| CodexErr::Other(format!("No client found for vault: {}", vault_key)))
    }

    /// List Recovery Services vaults
    pub async fn list_vaults(&self, args: Value) -> Result<Value> {
        // Get subscription ID from args or use default from config
        let subscription_id = args["subscription_id"].as_str()
            .filter(|s| !s.is_empty())
            .unwrap_or(&self.config.subscription_id);
        
        // Get resource group from args (optional filter)
        let resource_group_filter = args["resource_group"].as_str()
            .filter(|s| !s.is_empty());
        
        tracing::info!("Listing Recovery Services vaults for subscription: {}, resource group filter: {:?}", 
                      subscription_id, resource_group_filter);
        
        // Get authentication token
        use crate::recovery_services::auth::RecoveryServicesAuthHandler;
        
        // Get codex home directory for OAuth token storage
        let codex_home = dirs::home_dir()
            .ok_or_else(|| CodexErr::Other("Could not determine home directory".to_string()))?
            .join(".codex");
        
        // Create auth handler using OAuth
        let auth = RecoveryServicesAuthHandler::from_oauth(&codex_home).await?;
        
        let access_token = match &auth.auth {
            crate::recovery_services::auth::RecoveryServicesAuth::OAuth(token) => token.clone(),
            crate::recovery_services::auth::RecoveryServicesAuth::None => {
                return Err(CodexErr::Other("No authentication provided".to_string()));
            }
        };
        
        // Create a client for the specified subscription
        // Use empty strings for resource group and vault name since we're listing at subscription level
        let client = Arc::new(RecoveryServicesClient::new(
            subscription_id.to_string(),
            String::new(),  // Empty resource group - we'll filter later
            String::new(),  // Empty vault name
            access_token
        ));
        
        tracing::info!("Created client for subscription: {}", subscription_id);
        
        // Get all vaults in the subscription
        let all_vaults = client.list_vaults().await?;
        
        // Filter by resource group if specified
        let filtered_vaults: Vec<_> = if let Some(rg_filter) = resource_group_filter {
            all_vaults.into_iter()
                .filter(|vault| {
                    vault.resource_group.eq_ignore_ascii_case(rg_filter)
                })
                .collect()
        } else {
            all_vaults
        };
        
        // Group vaults by resource group for better organization
        let mut vaults_by_rg: std::collections::HashMap<String, Vec<&VaultInfo>> = std::collections::HashMap::new();
        for vault in &filtered_vaults {
            vaults_by_rg.entry(vault.resource_group.clone()).or_insert_with(Vec::new).push(vault);
        }
        
        Ok(json!({
            "vaults": filtered_vaults,
            "vaults_by_resource_group": vaults_by_rg,
            "total_count": filtered_vaults.len(),
            "filter": {
                "subscription_id": subscription_id,
                "resource_group": resource_group_filter
            },
            "config_defaults": {
                "default_subscription_id": self.config.subscription_id,
                "default_resource_group": self.config.resource_group,
                "default_vault_name": self.config.vault_name
            }
        }))
    }

    /// Test connection to Recovery Services
    pub async fn test_connection(&self, args: Value) -> Result<Value> {
        let vault_name = args["vault_name"].as_str();
        let client = self.get_client(vault_name)?;
        
        let vault_key = vault_name.unwrap_or("default");
        tracing::info!("Testing Recovery Services connection for vault: {}", vault_key);
        
        let mut results = json!({
            "vault": vault_key,
            "tests": []
        });
        
        let mut tests = Vec::new();
        
        // Test 1: Basic connectivity - test vault properties
        tracing::info!("Testing basic connectivity...");
        let basic_connectivity_test = match client.get_vault_properties().await {
            Ok(_) => json!({
                "test": "basic_connectivity",
                "status": "success",
                "message": "Successfully connected to Recovery Services vault"
            }),
            Err(e) => json!({
                "test": "basic_connectivity", 
                "status": "failed",
                "error": format!("Connection failed: {}", e)
            })
        };
        tests.push(basic_connectivity_test);
        
        // Test 2: List protected items
        tracing::info!("Testing protected items listing...");
        let protected_items_test = match client.list_protected_items(None).await {
            Ok(items) => {
                json!({
                    "test": "list_protected_items",
                    "status": "success",
                    "message": format!("Successfully listed {} protected items", items.len())
                })
            },
            Err(e) => json!({
                "test": "list_protected_items",
                "status": "failed", 
                "error": format!("Failed to list protected items: {}", e)
            })
        };
        tests.push(protected_items_test);
        
        // Test 3: List backup containers
        tracing::info!("Testing container listing permissions...");
        let containers_test = match client.list_backup_containers().await {
            Ok(containers) => json!({
                "test": "list_containers",
                "status": "success",
                "message": format!("Successfully listed {} backup containers", containers.len())
            }),
            Err(e) => json!({
                "test": "list_containers",
                "status": "failed", 
                "error": format!("Failed to list containers: {}", e)
            })
        };
        tests.push(containers_test);
        
        // Test 4: List policies
        tracing::info!("Testing policy listing permissions...");
        let policies_test = match client.list_backup_policies(None).await {
            Ok(policies) => json!({
                "test": "list_policies",
                "status": "success",
                "message": format!("Successfully listed {} backup policies", policies.len())
            }),
            Err(e) => json!({
                "test": "list_policies",
                "status": "failed", 
                "error": format!("Failed to list policies: {}", e)
            })
        };
        tests.push(policies_test);
        
        // Test 5: Get vault configuration
        tracing::info!("Testing vault configuration access...");
        let vault_config_test = match client.get_vault_config().await {
            Ok(_) => json!({
                "test": "vault_config",
                "status": "success",
                "message": "Successfully retrieved vault configuration"
            }),
            Err(e) => json!({
                "test": "vault_config", 
                "status": "failed",
                "error": format!("Failed to get vault configuration: {}", e)
            })
        };
        tests.push(vault_config_test);
        
        results["tests"] = json!(tests);
        
        // Summary
        let failed_tests: Vec<_> = tests.iter()
            .filter(|test| test["status"] == "failed")
            .collect();
            
        results["summary"] = json!({
            "total_tests": tests.len(),
            "passed": tests.len() - failed_tests.len(),
            "failed": failed_tests.len(),
            "overall_status": if failed_tests.is_empty() { "healthy" } else { "issues_detected" }
        });
        
        // Add configuration information to help with debugging
        results["configuration"] = json!({
            "subscription_id": self.config.subscription_id,
            "resource_group": self.config.resource_group,
            "vault_name": self.config.vault_name,
            "additional_vaults": self.config.vaults.keys().collect::<Vec<_>>()
        });
        
        // Add vault details if available
        if let Ok(vault_details) = client.get_vault_properties().await {
            results["vault_details"] = json!(vault_details);
        }
        
        Ok(results)
    }

    /// Register VM for backup
    pub async fn register_vm(&self, args: Value) -> Result<Value> {
        let vm_resource_id = args["vm_resource_id"].as_str().ok_or_else(|| {
            CodexErr::Other("vm_resource_id parameter is required".to_string())
        })?;
        
        // Default to VM workload type if not specified
        let workload_type_str = args["workload_type"].as_str().unwrap_or("VM");
        
        // Map the workload type string to the enum
        let workload_type = match workload_type_str.to_uppercase().as_str() {
            "VM" => WorkloadType::VM,
            "FILEFOLDER" => WorkloadType::FileFolder,
            "AZURESQLDB" => WorkloadType::AzureSqlDb,
            "SQLDB" => WorkloadType::SqlDb,
            "EXCHANGE" => WorkloadType::Exchange,
            "SHAREPOINT" => WorkloadType::Sharepoint,
            "VMWAREVM" => WorkloadType::VMwareVM,
            "SYSTEMSTATE" => WorkloadType::SystemState,
            "CLIENT" => WorkloadType::Client,
            "GENERICDATASOURCE" => WorkloadType::GenericDataSource,
            "SQLDATABASE" => WorkloadType::SqlDatabase,
            "AZUREFILESHARE" => WorkloadType::AzureFileShare,
            "SAPHANADATABASE" | "SAPHANA" => WorkloadType::SapHanaDatabase,
            "SAPASEDATABASE" | "SAPASE" => WorkloadType::SapAseDatabase,
            "SAPHANADBINSTANCE" => WorkloadType::SapHanaDbInstance,
            "ANYDATABASE" => WorkloadType::AnyDatabase,
            _ => return Err(CodexErr::Other(format!(
                "Unsupported workload type: {}. Supported types are: VM, FileFolder, AzureSqlDb, SqlDb, Exchange, Sharepoint, VMwareVM, SystemState, Client, GenericDataSource, SqlDatabase, AzureFileShare, SapHanaDatabase, SapAseDatabase, SapHanaDbInstance, AnyDatabase", 
                workload_type_str
            ))),
        };
        
        // Get backup management type from args or default to appropriate value based on workload
        let backup_management_type = match args["backup_management_type"].as_str() {
            Some(bmt) => bmt,
            None => match workload_type {
                WorkloadType::VM => "AzureIaasVM",
                WorkloadType::AzureFileShare => "AzureStorage",
                WorkloadType::AzureSqlDb => "AzureSql",
                _ => "AzureWorkload", // Default for most workload types
            }
        };
        
        let vault_name = args["vault_name"].as_str();
        let client = self.get_client(vault_name)?;
        
        tracing::info!("Registering VM {} for {} backup with management type {}", 
                      vm_resource_id, workload_type_str, backup_management_type);
        
        // For standard VM backup
        if workload_type == WorkloadType::VM && backup_management_type == "AzureIaasVM" {
            // Extract VM name and resource group from resource ID
            let parts: Vec<&str> = vm_resource_id.split('/').collect();
            if parts.len() < 9 {
                return Err(CodexErr::Other("Invalid VM resource ID format".to_string()));
            }
            
            let vm_resource_group = parts[4];
            let vm_name = parts[8];
            
            // Generate container name for the response
            let container_name = client.generate_vm_container_name(vm_resource_group, vm_name);
            
            // Use the register_vm method for VM registration
            let result = client.register_vm(vm_resource_id, workload_type).await?;
            
            return Ok(json!({
                "success": true,
                "message": format!("Successfully registered VM {} for standard Azure VM backup", vm_name),
                "vm_resource_id": vm_resource_id,
                "vm_name": vm_name,
                "vm_resource_group": vm_resource_group,
                "workload_type": "VM",
                "backup_management_type": "AzureIaasVM",
                "container_name": container_name,
                "result": result
            }));
        }
        
        // For workload-specific backup (SQL, SAP HANA, etc.)
        let result = client.register_vm_for_workload(vm_resource_id, workload_type_str).await?;
        
        Ok(json!({
            "success": true,
            "message": format!("Successfully registered VM {} for {} backup", vm_resource_id, workload_type_str),
            "vm_resource_id": vm_resource_id,
            "workload_type": workload_type_str,
            "backup_management_type": backup_management_type,
            "result": result
        }))
    }

    /// List protectable items
    pub async fn list_protectable_items(&self, args: Value) -> Result<Value> {
        let workload_type = if let Some(wl_str) = args["workload_type"].as_str() {
            match wl_str {
                "SAPHANA" => Some(WorkloadType::SapHanaDatabase),
                "SQLDataBase" => Some(WorkloadType::SqlDatabase),
                _ => return Err(CodexErr::Other("workload_type must be 'SAPHANA' or 'SQLDataBase'".to_string())),
            }
        } else {
            None
        };
        
        let server_name = args["server_name"].as_str();
        let vault_name = args["vault_name"].as_str();
        let client = self.get_client(vault_name)?;
        
        tracing::info!("Listing protectable items for workload: {:?}, server: {:?}", workload_type, server_name);
        
        let items = client.list_protectable_items(workload_type.clone()).await?;
        
        // Filter by server name if provided
        let filtered_items: Vec<_> = if let Some(server) = server_name {
            items.into_iter()
                .filter(|item| item.properties.server_name == server)
                .collect()
        } else {
            items
        };
        
        Ok(json!({
            "protectable_items": filtered_items,
            "total_count": filtered_items.len(),
            "filter": {
                "workload_type": workload_type,
                "server_name": server_name
            }
        }))
    }

    /// List protected items
    pub async fn list_protected_items(&self, args: Value) -> Result<Value> {
        // Get vault name
        let vault_name = args["vault_name"].as_str();
        let client = self.get_client(vault_name)?;
        
        // Get backup management type (AzureIaasVM, AzureWorkload, AzureStorage, etc.)
        let backup_management_type = args["backup_management_type"].as_str().unwrap_or("AzureIaasVM");
        
        // Get workload type with enhanced support for all types
        let workload_type_str = args["workload_type"].as_str();
        let workload_type = if let Some(wl_str) = workload_type_str {
            match wl_str.to_uppercase().as_str() {
                "VM" => Some(WorkloadType::VM),
                "FILEFOLDER" => Some(WorkloadType::FileFolder),
                "AZURESQLDB" => Some(WorkloadType::AzureSqlDb),
                "SQLDB" => Some(WorkloadType::SqlDb),
                "EXCHANGE" => Some(WorkloadType::Exchange),
                "SHAREPOINT" => Some(WorkloadType::Sharepoint),
                "VMWAREVM" => Some(WorkloadType::VMwareVM),
                "SYSTEMSTATE" => Some(WorkloadType::SystemState),
                "CLIENT" => Some(WorkloadType::Client),
                "GENERICDATASOURCE" => Some(WorkloadType::GenericDataSource),
                "SQLDATABASE" | "SQL" => Some(WorkloadType::SqlDatabase),
                "AZUREFILESHARE" => Some(WorkloadType::AzureFileShare),
                "SAPHANADATABASE" | "SAPHANA" => Some(WorkloadType::SapHanaDatabase),
                "SAPASEDATABASE" | "SAPASE" => Some(WorkloadType::SapAseDatabase),
                "SAPHANADBINSTANCE" => Some(WorkloadType::SapHanaDbInstance),
                "ANYDATABASE" => Some(WorkloadType::AnyDatabase),
                _ => return Err(CodexErr::Other(format!(
                    "Unsupported workload type: {}. Supported types are: VM, FileFolder, AzureSqlDb, SqlDb, Exchange, Sharepoint, VMwareVM, SystemState, Client, GenericDataSource, SqlDatabase, AzureFileShare, SapHanaDatabase, SapAseDatabase, SapHanaDbInstance, AnyDatabase", 
                    wl_str
                ))),
            }
        } else {
            None
        };
        
        // Get server name for filtering
        let server_name = args["server_name"].as_str();
        
        // Get container name for direct filtering if provided
        let container_name = args["container_name"].as_str();
        
        tracing::info!("Listing protected items for backup management type: {}, workload: {:?}, server: {:?}", 
                      backup_management_type, workload_type_str, server_name);
        
        // Build direct API query with filters for more reliable results
        let mut endpoint = "/backupProtectedItems?".to_string();
        let mut filters = Vec::new();
        
        // Add backup management type filter
        filters.push(format!("backupManagementType eq '{}'", backup_management_type));
        
        // Add workload type filter if specified
        if let Some(wl_str) = workload_type_str {
            filters.push(format!("workloadType eq '{}'", wl_str));
        }
        
        // Add filters to endpoint
        endpoint.push_str(&format!("$filter={}", filters.join(" and ")));
        
        // Use the list_protected_items method with workload filter
        let items = client.list_protected_items(workload_type).await?;
        
        // Filter by server name if provided
        let filtered_items: Vec<_> = if let Some(server) = server_name {
            items.into_iter()
                .filter(|item| item.properties.server_name.contains(server))
                .collect()
        } else {
            items
        };
        
        // Build response
        let response = json!({
            "protected_items": filtered_items,
            "total_count": filtered_items.len(),
            "filter": {
                "backup_management_type": backup_management_type,
                "workload_type": workload_type_str,
                "server_name": server_name,
                "container_name": container_name
            }
        });
        
        Ok(response)
    }

    /// List backup jobs
    pub async fn list_backup_jobs(&self, args: Value) -> Result<Value> {
        let vault_name = args["vault_name"].as_str();
        let client = self.get_client(vault_name)?;
        
        // Build filter string
        let mut filters = Vec::new();
        
        if let Some(status) = args["status_filter"].as_str() {
            filters.push(format!("status eq '{}'", status));
        }
        
        if let Some(operation) = args["operation_filter"].as_str() {
            filters.push(format!("operation eq '{}'", operation));
        }
        
        if let Some(hours) = args["time_range_hours"].as_f64() {
            let start_time = chrono::Utc::now() - chrono::Duration::hours(hours as i64);
            filters.push(format!("startTime ge {}", start_time.format("%Y-%m-%dT%H:%M:%SZ")));
        }
        
        let filter_str = if filters.is_empty() {
            None
        } else {
            Some(filters.join(" and "))
        };
        
        tracing::info!("Listing backup jobs with filter: {:?}", filter_str);
        
        let jobs = client.list_backup_jobs(filter_str.as_deref()).await?;
        
        Ok(json!({
            "backup_jobs": jobs,
            "total_count": jobs.len(),
            "filter": filter_str
        }))
    }

    /// List backup policies
    pub async fn list_policies(&self, args: Value) -> Result<Value> {
        let workload_type = if let Some(wl_str) = args["workload_type"].as_str() {
            match wl_str {
                "SAPHANA" => Some(WorkloadType::SapHanaDatabase),
                "SQLDataBase" => Some(WorkloadType::SqlDatabase),
                _ => return Err(CodexErr::Other("workload_type must be 'SAPHANA' or 'SQLDataBase'".to_string())),
            }
        } else {
            None
        };
        
        let vault_name = args["vault_name"].as_str();
        let client = self.get_client(vault_name)?;
        
        tracing::info!("Listing backup policies for workload: {:?}", workload_type);
        
        let policies = client.list_backup_policies(workload_type.clone()).await?;
        
        Ok(json!({
            "backup_policies": policies,
            "total_count": policies.len(),
            "filter": {
                "workload_type": workload_type
            }
        }))
    }

    /// Clear authentication cache
    pub async fn clear_auth_cache(&self, _args: Value) -> Result<Value> {
        use crate::recovery_services::auth::RecoveryServicesOAuthHandler;
        
        // Get codex home directory
        let codex_home = dirs::home_dir()
            .ok_or_else(|| CodexErr::Other("Could not determine home directory".to_string()))?
            .join(".codex");
        
        let oauth_handler = RecoveryServicesOAuthHandler::new(&codex_home);
        oauth_handler.logout().await?;
        
        Ok(json!({
            "success": true,
            "message": "Recovery Services authentication cache cleared. Next operation will prompt for re-authentication."
        }))
    }

    /// Check VM registration status
    pub async fn check_registration_status(&self, args: Value) -> Result<Value> {
        let vm_name = args["vm_name"].as_str().ok_or_else(|| {
            CodexErr::Other("vm_name parameter is required".to_string())
        })?;
        
        // Get resource group if provided, otherwise use the default from config
        let vm_resource_group = args["vm_resource_group"].as_str();
        
        let vault_name = args["vault_name"].as_str();
        let client = self.get_client(vault_name)?;
        
        tracing::info!("Checking registration status for VM: {} in resource group: {:?}", vm_name, vm_resource_group);
        
        // Approach 1: If we have resource group, try direct VM container check
        if let Some(rg) = vm_resource_group {
            tracing::info!("Attempting direct VM container check for {}/{}", rg, vm_name);
            
            // Generate possible container names for this VM
            let possible_containers = vec![
                // Standard VM container format
                format!("iaasvmcontainer;iaasvmcontainerv2;{};{}", rg, vm_name),
                // Workload container format  
                format!("VMAppContainer;Compute;{};{}", rg, vm_name),
                // Alternative formats
                format!("vmappcontainer;compute;{};{}", rg, vm_name),
            ];
            
            for container_name in &possible_containers {
                tracing::debug!("Checking for container: {}", container_name);
                let endpoint = format!("/backupFabrics/Azure/protectionContainers/{}", container_name);
                
                match client.get_request(&endpoint).await {
                    Ok(container_response) => {
                        tracing::info!("Found container: {}", container_name);
                        if let Some(properties) = container_response.get("properties") {
                            let registration_status = properties.get("registrationStatus")
                                .and_then(|s| s.as_str()).unwrap_or("Unknown");
                            let health_status = properties.get("healthStatus")
                                .and_then(|s| s.as_str()).unwrap_or("Unknown");
                            let container_type = properties.get("containerType")
                                .and_then(|s| s.as_str()).unwrap_or("Unknown");
                            
                            let is_registered = registration_status.eq_ignore_ascii_case("Registered");
                            let is_workload = container_type.eq_ignore_ascii_case("VMAppContainer");
                            
                            return Ok(json!({
                                "vm_name": vm_name,
                                "vm_resource_group": rg,
                                "registration_status": if is_registered { "Registered" } else { "Not Registered" },
                                "health_status": health_status,
                                "container_type": container_type,
                                "container_name": container_name,
                                "backup_management_type": if is_workload { "AzureWorkload" } else { "AzureIaasVM" },
                                "workload_backup": is_workload,
                                "container_details": container_response,
                                "message": format!("VM '{}' is {} in this vault", vm_name, 
                                                 if is_registered { "registered" } else { "not registered" }),
                                "detection_method": "direct_container_check"
                            }));
                        }
                    },
                    Err(_) => {
                        tracing::debug!("Container {} not found", container_name);
                        // Continue to next container name
                    }
                }
            }
        }
        
        // Approach 2: List all registered VMs across all workload types and find our VM
        tracing::info!("Direct check failed, listing all registered VMs to find: {}", vm_name);
        
        let all_containers_endpoints = vec![
            "/backupProtectionContainers?$filter=backupManagementType eq 'AzureWorkload'",
            "/backupProtectionContainers?$filter=backupManagementType eq 'AzureIaasVM'", 
            "/backupProtectionContainers", // All containers
        ];
        
        for endpoint in &all_containers_endpoints {
            tracing::debug!("Querying endpoint: {}", endpoint);
            match client.get_request(endpoint).await {
                Ok(response) => {
                    if let Some(containers_array) = response.get("value").and_then(|v| v.as_array()) {
                        tracing::info!("Found {} containers via {}", containers_array.len(), endpoint);
                        
                        for container_json in containers_array {
                            if let Some(name) = container_json.get("name").and_then(|n| n.as_str()) {
                                if let Some(properties) = container_json.get("properties") {
                                    if let Some(friendly_name) = properties.get("friendlyName").and_then(|f| f.as_str()) {
                                        
                                        // Check if this container matches our VM
                                        let vm_matches = friendly_name.eq_ignore_ascii_case(vm_name) ||
                                                       name.to_lowercase().contains(&vm_name.to_lowercase());
                                        
                                        // If we have resource group, also check if it matches
                                        let rg_matches = if let Some(rg) = vm_resource_group {
                                            name.to_lowercase().contains(&rg.to_lowercase())
                                        } else {
                                            true // If no RG specified, don't filter by RG
                                        };
                                        
                                        if vm_matches && rg_matches {
                                            tracing::info!("Found matching VM container: name='{}', friendly_name='{}'", name, friendly_name);
                                            
                                            let registration_status = properties.get("registrationStatus")
                                                .and_then(|s| s.as_str()).unwrap_or("Unknown");
                                            let health_status = properties.get("healthStatus")
                                                .and_then(|s| s.as_str()).unwrap_or("Unknown");
                                            let container_type = properties.get("containerType")
                                                .and_then(|s| s.as_str()).unwrap_or("Unknown");
                                            
                                            let is_registered = registration_status.eq_ignore_ascii_case("Registered");
                                            let is_workload = container_type.eq_ignore_ascii_case("VMAppContainer");
                                            
                                            let mut response = json!({
                                                "vm_name": vm_name,
                                                "registration_status": if is_registered { "Registered" } else { "Not Registered" },
                                                "health_status": health_status,
                                                "container_type": container_type,
                                                "container_name": name,
                                                "backup_management_type": if is_workload { "AzureWorkload" } else { "AzureIaasVM" },
                                                "workload_backup": is_workload,
                                                "container_details": container_json,
                                                "message": format!("VM '{}' is {} in this vault", vm_name, 
                                                                 if is_registered { "registered" } else { "not registered" }),
                                                "detection_method": "comprehensive_search"
                                            });
                                            
                                            if let Some(rg) = vm_resource_group {
                                                response["vm_resource_group"] = json!(rg);
                                            }
                                            
                                            return Ok(response);
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                Err(e) => {
                    tracing::warn!("Failed to query {}: {}", endpoint, e);
                }
            }
        }
        
        // Approach 3: Final fallback - check protected items
        tracing::info!("Container search failed, checking protected items as final fallback...");
        let workload_types = vec![
            Some(WorkloadType::VM),
            Some(WorkloadType::SapHanaDatabase),
            Some(WorkloadType::SqlDatabase),
            Some(WorkloadType::AnyDatabase),
            None // Check all types
        ];
        
        for workload_type in workload_types {
            tracing::debug!("Checking protected items for workload type: {:?}", workload_type);
            match client.list_protected_items(workload_type.clone()).await {
                Ok(items) => {
                    tracing::debug!("Found {} protected items for workload type {:?}", items.len(), workload_type);
                    for item in &items {
                        tracing::debug!("Checking protected item: friendly_name='{}'", item.properties.friendly_name);
                        
                        // Check if this protected item matches our VM name
                        let vm_matches = item.properties.friendly_name.eq_ignore_ascii_case(vm_name) ||
                                       item.properties.friendly_name.contains(vm_name);
                        
                        if vm_matches {
                            tracing::info!("Found VM in protected items: {}", item.properties.friendly_name);
                            return Ok(json!({
                                "vm_name": vm_name,
                                "registration_status": "Registered",
                                "protection_status": "Protected",
                                "workload_type": workload_type,
                                "protected_item_details": item,
                                "message": format!("VM '{}' is registered and protected in this vault", vm_name),
                                "detection_method": "protected_items_search"
                            }));
                        }
                    }
                },
                Err(e) => {
                    tracing::warn!("Failed to query protected items for workload type {:?}: {}", workload_type, e);
                }
            }
        }
        
        // If we get here, the VM is not registered
        let mut response = json!({
            "vm_name": vm_name,
            "registration_status": "Not Registered",
            "message": format!("VM '{}' is not registered for backup in this vault", vm_name),
            "suggestion": "Use 'recovery_services_register_vm' to register this VM for backup"
        });
        
        // Add resource group if provided
        if let Some(rg) = vm_resource_group {
            response["vm_resource_group"] = json!(rg);
        }
        
        Ok(response)
    }

    /// Re-register VM for backup
    pub async fn reregister_vm(&self, vault_name: &str, vm_name: &str, vm_resource_group: &str) -> Result<Value> {
        let client = self.get_client(Some(vault_name))?;
        
        // Generate VM resource ID
        let vm_resource_id = client.generate_vm_resource_id(vm_resource_group, vm_name);
        
        // Re-register using the workload registration method
        let result = client.register_vm_for_workload(&vm_resource_id, "VM").await?;
        
        Ok(json!({
            "vm_name": vm_name,
            "vm_resource_group": vm_resource_group,
            "vault_name": vault_name,
            "operation": "reregister",
            "status": "initiated",
            "result": result
        }))
    }

    /// Unregister VM from backup
    pub async fn unregister_vm(&self, vault_name: &str, vm_name: &str, vm_resource_group: &str) -> Result<Value> {
        let client = self.get_client(Some(vault_name))?;
        
        // Generate container name for the VM
        let container_name = client.generate_vm_container_name(vm_resource_group, vm_name);
        
        // Unregister the container
        let result = client.unregister_container(&container_name).await?;
        
        Ok(json!({
            "vm_name": vm_name,
            "vm_resource_group": vm_resource_group,
            "vault_name": vault_name,
            "container_name": container_name,
            "operation": "unregister",
            "status": "initiated",
            "result": result
        }))
    }

    /// Create backup policy
    pub async fn create_policy(&self, vault_name: &str, policy_name: &str, schedule_type: &str, retention_days: u32) -> Result<Value> {
        let client = self.get_client(Some(vault_name))?;
        
        let result = client.create_backup_policy(policy_name, schedule_type, retention_days).await?;
        
        Ok(json!({
            "vault_name": vault_name,
            "policy_name": policy_name,
            "schedule_type": schedule_type,
            "retention_days": retention_days,
            "operation": "create_policy",
            "status": "created",
            "result": result
        }))
    }

    /// Get backup policy details
    pub async fn get_policy_details(&self, vault_name: &str, policy_name: &str) -> Result<Value> {
        let client = self.get_client(Some(vault_name))?;
        
        let policy = client.get_backup_policy(policy_name).await?;
        
        Ok(json!({
            "vault_name": vault_name,
            "policy_name": policy_name,
            "policy_details": policy
        }))
    }

    /// Enable protection for a VM
    pub async fn enable_protection(&self, vault_name: &str, vm_name: &str, vm_resource_group: &str, policy_name: &str) -> Result<Value> {
        let client = self.get_client(Some(vault_name))?;
        
        // Generate names for the VM
        let container_name = client.generate_vm_container_name(vm_resource_group, vm_name);
        let protected_item_name = client.generate_vm_protected_item_name(vm_resource_group, vm_name);
        let vm_resource_id = client.generate_vm_resource_id(vm_resource_group, vm_name);
        
        let result = client.enable_protection(&container_name, &protected_item_name, policy_name, &vm_resource_id).await?;
        
        Ok(json!({
            "vm_name": vm_name,
            "vm_resource_group": vm_resource_group,
            "vault_name": vault_name,
            "policy_name": policy_name,
            "container_name": container_name,
            "protected_item_name": protected_item_name,
            "operation": "enable_protection",
            "status": "initiated",
            "result": result
        }))
    }

    /// Disable protection for a VM
    pub async fn disable_protection(&self, vault_name: &str, vm_name: &str, vm_resource_group: &str, delete_backup_data: Option<bool>) -> Result<Value> {
        let client = self.get_client(Some(vault_name))?;
        
        // Generate names for the VM
        let container_name = client.generate_vm_container_name(vm_resource_group, vm_name);
        let protected_item_name = client.generate_vm_protected_item_name(vm_resource_group, vm_name);
        
        let delete_data = delete_backup_data.unwrap_or(false);
        let result = client.disable_protection(&container_name, &protected_item_name, delete_data).await?;
        
        Ok(json!({
            "vm_name": vm_name,
            "vm_resource_group": vm_resource_group,
            "vault_name": vault_name,
            "container_name": container_name,
            "protected_item_name": protected_item_name,
            "delete_backup_data": delete_data,
            "operation": "disable_protection",
            "status": "initiated",
            "result": result
        }))
    }

    /// Trigger backup for a VM
    pub async fn trigger_backup(&self, vault_name: &str, vm_name: &str, vm_resource_group: &str, retention_days: Option<u32>) -> Result<Value> {
        let client = self.get_client(Some(vault_name))?;
        
        // Generate names for the VM
        let container_name = client.generate_vm_container_name(vm_resource_group, vm_name);
        let protected_item_name = client.generate_vm_protected_item_name(vm_resource_group, vm_name);
        
        let result = client.trigger_backup(&container_name, &protected_item_name, retention_days).await?;
        
        Ok(json!({
            "vm_name": vm_name,
            "vm_resource_group": vm_resource_group,
            "vault_name": vault_name,
            "container_name": container_name,
            "protected_item_name": protected_item_name,
            "retention_days": retention_days.unwrap_or(30),
            "operation": "trigger_backup",
            "status": "initiated",
            "result": result
        }))
    }

    /// Get backup job status
    pub async fn get_job_status(&self, vault_name: &str, job_id: &str) -> Result<Value> {
        let client = self.get_client(Some(vault_name))?;
        
        let job = client.get_backup_job(job_id).await?;
        
        Ok(json!({
            "vault_name": vault_name,
            "job_id": job_id,
            "job_details": job
        }))
    }

    /// Get backup summary for the vault
    pub async fn get_backup_summary(&self, vault_name: &str, workload_type: Option<&str>) -> Result<Value> {
        let client = self.get_client(Some(vault_name))?;
        
        // Get protected items
        let workload_filter = workload_type.and_then(|wt| match wt {
            "VM" => Some(WorkloadType::VM),
            "SqlDb" => Some(WorkloadType::SqlDb),
            "SqlDatabase" => Some(WorkloadType::SqlDatabase),
            "SapHanaDatabase" => Some(WorkloadType::SapHanaDatabase),
            _ => None,
        });
        
        let protected_items = client.list_protected_items(workload_filter).await?;
        
        // Get recent backup jobs
        let recent_jobs = client.list_backup_jobs(Some("startTime ge datetime'2024-01-01T00:00:00Z'")).await?;
        
        // Calculate summary statistics
        let total_protected_items = protected_items.len();
        let successful_jobs = recent_jobs.iter().filter(|job| job.properties.status == "Completed").count();
        let failed_jobs = recent_jobs.iter().filter(|job| job.properties.status == "Failed").count();
        let in_progress_jobs = recent_jobs.iter().filter(|job| job.properties.status == "InProgress").count();
        
        Ok(json!({
            "vault_name": vault_name,
            "workload_type": workload_type.unwrap_or("All"),
            "summary": {
                "total_protected_items": total_protected_items,
                "recent_jobs": {
                    "total": recent_jobs.len(),
                    "successful": successful_jobs,
                    "failed": failed_jobs,
                    "in_progress": in_progress_jobs
                }
            },
            "protected_items": protected_items,
            "recent_jobs": recent_jobs
        }))
    }

    /// List recovery points for a VM
    pub async fn list_recovery_points(&self, vault_name: &str, vm_name: &str, vm_resource_group: &str, filter: Option<&str>) -> Result<Value> {
        let client = self.get_client(Some(vault_name))?;
        
        // Generate names for the VM
        let container_name = client.generate_vm_container_name(vm_resource_group, vm_name);
        let protected_item_name = client.generate_vm_protected_item_name(vm_resource_group, vm_name);
        
        let recovery_points = client.list_recovery_points(&container_name, &protected_item_name, filter).await?;
        
        Ok(json!({
            "vm_name": vm_name,
            "vm_resource_group": vm_resource_group,
            "vault_name": vault_name,
            "container_name": container_name,
            "protected_item_name": protected_item_name,
            "filter": filter.unwrap_or("none"),
            "recovery_points": recovery_points
        }))
    }

    /// Restore VM to original location
    pub async fn restore_original_location(&self, vault_name: &str, vm_name: &str, vm_resource_group: &str, recovery_point_id: &str) -> Result<Value> {
        let client = self.get_client(Some(vault_name))?;
        
        // Generate names for the VM
        let container_name = client.generate_vm_container_name(vm_resource_group, vm_name);
        let protected_item_name = client.generate_vm_protected_item_name(vm_resource_group, vm_name);
        
        let result = client.restore_vm(&container_name, &protected_item_name, recovery_point_id, "OriginalLocation", None, None).await?;
        
        Ok(json!({
            "vm_name": vm_name,
            "vm_resource_group": vm_resource_group,
            "vault_name": vault_name,
            "recovery_point_id": recovery_point_id,
            "restore_type": "OriginalLocation",
            "container_name": container_name,
            "protected_item_name": protected_item_name,
            "operation": "restore_original_location",
            "status": "initiated",
            "result": result
        }))
    }

    /// Restore VM to alternate location
    pub async fn restore_alternate_location(&self, vault_name: &str, vm_name: &str, vm_resource_group: &str, 
                                          recovery_point_id: &str, target_vm_name: &str, target_resource_group: &str) -> Result<Value> {
        let client = self.get_client(Some(vault_name))?;
        
        // Generate names for the VM
        let container_name = client.generate_vm_container_name(vm_resource_group, vm_name);
        let protected_item_name = client.generate_vm_protected_item_name(vm_resource_group, vm_name);
        
        let result = client.restore_vm(&container_name, &protected_item_name, recovery_point_id, 
                                     "AlternateLocation", Some(target_vm_name), Some(target_resource_group)).await?;
        
        Ok(json!({
            "vm_name": vm_name,
            "vm_resource_group": vm_resource_group,
            "vault_name": vault_name,
            "recovery_point_id": recovery_point_id,
            "restore_type": "AlternateLocation",
            "target_vm_name": target_vm_name,
            "target_resource_group": target_resource_group,
            "container_name": container_name,
            "protected_item_name": protected_item_name,
            "operation": "restore_alternate_location",
            "status": "initiated",
            "result": result
        }))
    }

    /// Restore VM as files
    pub async fn restore_as_files(&self, vault_name: &str, vm_name: &str, vm_resource_group: &str, 
                                 recovery_point_id: &str, target_storage_account: &str, target_container: &str) -> Result<Value> {
        let client = self.get_client(Some(vault_name))?;
        
        // Generate names for the VM
        let container_name = client.generate_vm_container_name(vm_resource_group, vm_name);
        let protected_item_name = client.generate_vm_protected_item_name(vm_resource_group, vm_name);
        
        // Use the existing restore_vm method with RestoreDisks type
        let result = client.restore_vm(&container_name, &protected_item_name, recovery_point_id, "RestoreDisks", None, None).await?;
        
        Ok(json!({
            "vm_name": vm_name,
            "vm_resource_group": vm_resource_group,
            "vault_name": vault_name,
            "recovery_point_id": recovery_point_id,
            "restore_type": "RestoreAsFiles",
            "target_storage_account": target_storage_account,
            "target_container": target_container,
            "container_name": container_name,
            "protected_item_name": protected_item_name,
            "operation": "restore_as_files",
            "status": "initiated",
            "result": result
        }))
    }
    
    /// Bridge method for MCP server: Get job details (maps to get_job_status)
    pub async fn get_job_details(&self, vault_name: &str, job_id: &str) -> Result<Value> {
        // Reuse the existing get_job_status implementation
        self.get_job_status(vault_name, job_id).await
    }
    
    /// Bridge method for MCP server: Restore files (maps to restore_as_files)
    pub async fn restore_files(&self, vault_name: &str, vm_name: &str, vm_resource_group: &str, 
                             recovery_point_id: &str, file_paths: Vec<String>, target_storage_account: &str) -> Result<Value> {
        // Create a default container name
        let target_container = "restored-files";
        
        // Call the existing restore_as_files implementation
        let mut result = self.restore_as_files(vault_name, vm_name, vm_resource_group, 
                                             recovery_point_id, target_storage_account, target_container).await?;
        
        // Add file paths to the result
        if let Some(obj) = result.as_object_mut() {
            obj.insert("file_paths".to_string(), json!(file_paths));
        }
        
        Ok(result)
    }
    
    /// Bridge method for MCP server: Consolidated restore_vm with type parameter
    pub async fn restore_vm(&self, vault_name: &str, vm_name: &str, vm_resource_group: &str,
                          recovery_point_id: &str, restore_type: &str, 
                          target_vm_name: Option<&str>, target_resource_group: Option<&str>) -> Result<Value> {
        match restore_type {
            "OriginalLocation" => {
                self.restore_original_location(vault_name, vm_name, vm_resource_group, recovery_point_id).await
            },
            "AlternateLocation" => {
                if let (Some(target_vm), Some(target_rg)) = (target_vm_name, target_resource_group) {
                    self.restore_alternate_location(vault_name, vm_name, vm_resource_group, 
                                                  recovery_point_id, target_vm, target_rg).await
                } else {
                    Err(CodexErr::Other("Target VM name and resource group are required for alternate location restore".to_string()))
                }
            },
            "RestoreDisks" => {
                // For disk restore, we'll use a default storage account name based on the VM name
                let target_storage_account = format!("{}storage", vm_name.to_lowercase().replace('-', ""));
                let target_container = "restored-disks";
                
                self.restore_as_files(vault_name, vm_name, vm_resource_group, 
                                     recovery_point_id, &target_storage_account, target_container).await
            },
            _ => {
                Err(CodexErr::Other(format!("Unsupported restore type: {}", restore_type)))
            }
        }
    }
    
    /// Get backup summary for a specific VM
    pub async fn get_vm_backup_summary(&self, vault_name: &str, vm_name: &str, vm_resource_group: &str) -> Result<Value> {
        let client = self.get_client(Some(vault_name))?;
        
        // Generate names for the VM
        let container_name = client.generate_vm_container_name(vm_resource_group, vm_name);
        let protected_item_name = client.generate_vm_protected_item_name(vm_resource_group, vm_name);
        
        // Get recovery points for this VM
        let recovery_points = client.list_recovery_points(&container_name, &protected_item_name, None).await?;
        
        // Get recent backup jobs for this VM
        let filter = format!("backupManagementType eq 'AzureIaasVM' and entityFriendlyName eq '{}'", vm_name);
        let recent_jobs = client.list_backup_jobs(Some(&filter)).await?;
        
        // Calculate summary statistics
        let successful_jobs = recent_jobs.iter().filter(|job| job.properties.status == "Completed").count();
        let failed_jobs = recent_jobs.iter().filter(|job| job.properties.status == "Failed").count();
        let in_progress_jobs = recent_jobs.iter().filter(|job| job.properties.status == "InProgress").count();
        
        // Get the latest recovery point
        let latest_recovery_point = recovery_points.first().map(|rp| {
            json!({
                "recovery_point_id": rp.name,
                "recovery_point_type": rp.properties.recovery_point_type
            })
        });
        
        Ok(json!({
            "vm_name": vm_name,
            "vm_resource_group": vm_resource_group,
            "vault_name": vault_name,
            "protection_status": "Protected",
            "summary": {
                "total_recovery_points": recovery_points.len(),
                "latest_recovery_point": latest_recovery_point,
                "recent_jobs": {
                    "total": recent_jobs.len(),
                    "successful": successful_jobs,
                    "failed": failed_jobs,
                    "in_progress": in_progress_jobs
                }
            },
            "recovery_points": recovery_points,
            "recent_jobs": recent_jobs
        }))
    }
}
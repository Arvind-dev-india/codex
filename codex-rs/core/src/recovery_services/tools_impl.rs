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
    pub async fn list_vaults(&self, _args: Value) -> Result<Value> {
        let client = self.get_client(None)?;
        
        tracing::info!("Listing Recovery Services vaults");
        
        let vaults = client.list_vaults().await?;
        
        Ok(json!({
            "vaults": vaults,
            "total_count": vaults.len()
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
        
        // Test 1: Basic connectivity
        tracing::info!("Testing basic connectivity...");
        let connectivity_test = match client.test_connectivity().await {
            Ok(_) => json!({
                "test": "basic_connectivity",
                "status": "success",
                "message": "Successfully connected to Recovery Services vault"
            }),
            Err(e) => json!({
                "test": "basic_connectivity", 
                "status": "failed",
                "error": format!("{}", e)
            })
        };
        tests.push(connectivity_test);
        
        // Test 2: List containers (check permissions)
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
                "error": format!("{}", e)
            })
        };
        tests.push(containers_test);
        
        // Test 3: List policies (check read permissions)
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
                "error": format!("{}", e)
            })
        };
        tests.push(policies_test);
        
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
        
        Ok(results)
    }

    /// Register VM for backup
    pub async fn register_vm(&self, args: Value) -> Result<Value> {
        let vm_resource_id = args["vm_resource_id"].as_str().ok_or_else(|| {
            CodexErr::Other("vm_resource_id parameter is required".to_string())
        })?;
        
        let workload_type_str = args["workload_type"].as_str().ok_or_else(|| {
            CodexErr::Other("workload_type parameter is required".to_string())
        })?;
        
        let workload_type = match workload_type_str {
            "SAPHANA" => WorkloadType::SapHana,
            "SQLDataBase" => WorkloadType::SqlServer,
            _ => return Err(CodexErr::Other("workload_type must be 'SAPHANA' or 'SQLDataBase'".to_string())),
        };
        
        let vault_name = args["vault_name"].as_str();
        let client = self.get_client(vault_name)?;
        
        tracing::info!("Registering VM {} for {} backup", vm_resource_id, workload_type_str);
        
        let result = client.register_vm(vm_resource_id, workload_type).await?;
        
        Ok(json!({
            "success": true,
            "message": format!("Successfully registered VM {} for {} backup", vm_resource_id, workload_type_str),
            "vm_resource_id": vm_resource_id,
            "workload_type": workload_type_str,
            "result": result
        }))
    }

    /// List protectable items
    pub async fn list_protectable_items(&self, args: Value) -> Result<Value> {
        let workload_type = if let Some(wl_str) = args["workload_type"].as_str() {
            match wl_str {
                "SAPHANA" => Some(WorkloadType::SapHana),
                "SQLDataBase" => Some(WorkloadType::SqlServer),
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
        let workload_type = if let Some(wl_str) = args["workload_type"].as_str() {
            match wl_str {
                "SAPHANA" => Some(WorkloadType::SapHana),
                "SQLDataBase" => Some(WorkloadType::SqlServer),
                _ => return Err(CodexErr::Other("workload_type must be 'SAPHANA' or 'SQLDataBase'".to_string())),
            }
        } else {
            None
        };
        
        let server_name = args["server_name"].as_str();
        let vault_name = args["vault_name"].as_str();
        let client = self.get_client(vault_name)?;
        
        tracing::info!("Listing protected items for workload: {:?}, server: {:?}", workload_type, server_name);
        
        let items = client.list_protected_items(workload_type.clone()).await?;
        
        // Filter by server name if provided
        let filtered_items: Vec<_> = if let Some(server) = server_name {
            items.into_iter()
                .filter(|item| item.properties.server_name == server)
                .collect()
        } else {
            items
        };
        
        Ok(json!({
            "protected_items": filtered_items,
            "total_count": filtered_items.len(),
            "filter": {
                "workload_type": workload_type,
                "server_name": server_name
            }
        }))
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
                "SAPHANA" => Some(WorkloadType::SapHana),
                "SQLDataBase" => Some(WorkloadType::SqlServer),
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

    // TODO: Implement remaining methods:
    // - reregister_vm
    // - unregister_vm
    // - check_registration_status
    // - create_policy
    // - get_policy_details
    // - enable_protection
    // - disable_protection
    // - trigger_backup
    // - get_job_status
    // - get_backup_summary
    // - list_recovery_points
    // - restore_original_location
    // - restore_alternate_location
    // - restore_as_files
}
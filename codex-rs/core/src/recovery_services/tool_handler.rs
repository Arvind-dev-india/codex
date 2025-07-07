//! Handler for Recovery Services (Azure Backup) tool calls.

use serde_json::Value;
use std::sync::Arc;

use crate::recovery_services::tools_impl::RecoveryServicesTools;
use crate::config_types::RecoveryServicesConfig;
use crate::error::{CodexErr, Result};
use crate::mcp_tool_call::ToolCall;

/// Handle Recovery Services tool calls
pub async fn handle_recovery_services_tool_call(
    tool_call: &ToolCall,
    config: &RecoveryServicesConfig,
) -> Result<Value> {
    // Create tools instance
    let tools = Arc::new(RecoveryServicesTools::new(config).await?);
    
    // Extract tool name and arguments
    let name = &tool_call.name;
    let args = tool_call.arguments.clone();
    
    // Dispatch to appropriate tool function
    match name.as_str() {
        // Vault management
        "recovery_services_list_vaults" => {
            tools.list_vaults(args).await
        },
        "recovery_services_test_connection" => {
            tools.test_connection(args).await
        },
        
        // Discovery tools
        "recovery_services_refresh_containers" => {
            tools.refresh_containers(args).await
        },
        "recovery_services_list_protectable_containers" => {
            tools.list_protectable_containers(args).await
        },
        "recovery_services_list_protectable_items" => {
            tools.list_protectable_items(args).await
        },
        "recovery_services_list_workload_items" => {
            tools.list_workload_items(args).await
        },
        
        // VM registration
        "recovery_services_register_vm" => {
            tools.register_vm(args).await
        },
        "recovery_services_reregister_vm" => {
            let vault_name = args["vault_name"].as_str().unwrap_or("default");
            let vm_name = args["vm_name"].as_str().ok_or_else(|| {
                CodexErr::Other("vm_name parameter is required".to_string())
            })?;
            let vm_resource_group = args["vm_resource_group"].as_str().ok_or_else(|| {
                CodexErr::Other("vm_resource_group parameter is required".to_string())
            })?;
            tools.reregister_vm(vault_name, vm_name, vm_resource_group).await
        },
        "recovery_services_unregister_vm" => {
            let vault_name = args["vault_name"].as_str().unwrap_or("default");
            let vm_name = args["vm_name"].as_str().ok_or_else(|| {
                CodexErr::Other("vm_name parameter is required".to_string())
            })?;
            let vm_resource_group = args["vm_resource_group"].as_str().ok_or_else(|| {
                CodexErr::Other("vm_resource_group parameter is required".to_string())
            })?;
            tools.unregister_vm(vault_name, vm_name, vm_resource_group).await
        },
        "recovery_services_check_registration_status" => {
            tools.check_registration_status(args).await
        },
        
        // Policy management
        "recovery_services_list_policies" => {
            tools.list_policies(args).await
        },
        "recovery_services_get_policy_details" => {
            let vault_name = args["vault_name"].as_str().unwrap_or("default");
            let policy_name = args["policy_name"].as_str().ok_or_else(|| {
                CodexErr::Other("policy_name parameter is required".to_string())
            })?;
            tools.get_policy_details(vault_name, policy_name).await
        },
        "recovery_services_create_policy" => {
            let vault_name = args["vault_name"].as_str().unwrap_or("default");
            let policy_name = args["policy_name"].as_str().ok_or_else(|| {
                CodexErr::Other("policy_name parameter is required".to_string())
            })?;
            let schedule_type = args["schedule_type"].as_str().unwrap_or("Daily");
            let retention_days = args["retention_days"].as_u64().unwrap_or(30) as u32;
            tools.create_policy(vault_name, policy_name, schedule_type, retention_days).await
        },
        
        // Protection management
        "recovery_services_list_protected_items" => {
            tools.list_protected_items(args).await
        },
        "recovery_services_enable_protection" => {
            let vault_name = args["vault_name"].as_str().unwrap_or("default");
            let vm_name = args["vm_name"].as_str().ok_or_else(|| {
                CodexErr::Other("vm_name parameter is required".to_string())
            })?;
            let vm_resource_group = args["vm_resource_group"].as_str().ok_or_else(|| {
                CodexErr::Other("vm_resource_group parameter is required".to_string())
            })?;
            let policy_name = args["policy_name"].as_str().ok_or_else(|| {
                CodexErr::Other("policy_name parameter is required".to_string())
            })?;
            tools.enable_protection(vault_name, vm_name, vm_resource_group, policy_name).await
        },
        "recovery_services_disable_protection" => {
            let vault_name = args["vault_name"].as_str().unwrap_or("default");
            let vm_name = args["vm_name"].as_str().ok_or_else(|| {
                CodexErr::Other("vm_name parameter is required".to_string())
            })?;
            let vm_resource_group = args["vm_resource_group"].as_str().ok_or_else(|| {
                CodexErr::Other("vm_resource_group parameter is required".to_string())
            })?;
            let delete_backup_data = args["delete_backup_data"].as_bool();
            tools.disable_protection(vault_name, vm_name, vm_resource_group, delete_backup_data).await
        },
        
        // Backup operations
        "recovery_services_list_backup_jobs" => {
            tools.list_backup_jobs(args).await
        },
        "recovery_services_trigger_backup" => {
            let vault_name = args["vault_name"].as_str().unwrap_or("default");
            let vm_name = args["vm_name"].as_str().ok_or_else(|| {
                CodexErr::Other("vm_name parameter is required".to_string())
            })?;
            let vm_resource_group = args["vm_resource_group"].as_str().ok_or_else(|| {
                CodexErr::Other("vm_resource_group parameter is required".to_string())
            })?;
            let retention_days = args["retention_days"].as_u64().map(|d| d as u32);
            tools.trigger_backup(vault_name, vm_name, vm_resource_group, retention_days).await
        },
        "recovery_services_get_job_details" => {
            let vault_name = args["vault_name"].as_str().unwrap_or("default");
            let job_id = args["job_id"].as_str().ok_or_else(|| {
                CodexErr::Other("job_id parameter is required".to_string())
            })?;
            tools.get_job_details(vault_name, job_id).await
        },
        "recovery_services_cancel_job" => {
            let vault_name = args["vault_name"].as_str().unwrap_or("default");
            let job_id = args["job_id"].as_str().ok_or_else(|| {
                CodexErr::Other("job_id parameter is required".to_string())
            })?;
            // Note: cancel_job method needs to be implemented in tools_impl.rs
            Err(CodexErr::Other("cancel_job method not yet implemented".to_string()))
        },
        
        // Recovery operations
        "recovery_services_list_recovery_points" => {
            let vault_name = args["vault_name"].as_str().unwrap_or("default");
            let vm_name = args["vm_name"].as_str().ok_or_else(|| {
                CodexErr::Other("vm_name parameter is required".to_string())
            })?;
            let vm_resource_group = args["vm_resource_group"].as_str().ok_or_else(|| {
                CodexErr::Other("vm_resource_group parameter is required".to_string())
            })?;
            let filter = args["filter"].as_str();
            tools.list_recovery_points(vault_name, vm_name, vm_resource_group, filter).await
        },
        "recovery_services_restore_vm" => {
            let vault_name = args["vault_name"].as_str().unwrap_or("default");
            let vm_name = args["vm_name"].as_str().ok_or_else(|| {
                CodexErr::Other("vm_name parameter is required".to_string())
            })?;
            let vm_resource_group = args["vm_resource_group"].as_str().ok_or_else(|| {
                CodexErr::Other("vm_resource_group parameter is required".to_string())
            })?;
            let recovery_point_id = args["recovery_point_id"].as_str().ok_or_else(|| {
                CodexErr::Other("recovery_point_id parameter is required".to_string())
            })?;
            let restore_type = args["restore_type"].as_str().unwrap_or("OriginalLocation");
            let target_vm_name = args["target_vm_name"].as_str();
            let target_resource_group = args["target_resource_group"].as_str();
            tools.restore_vm(vault_name, vm_name, vm_resource_group, recovery_point_id, restore_type, target_vm_name, target_resource_group).await
        },
        "recovery_services_restore_files" => {
            let vault_name = args["vault_name"].as_str().unwrap_or("default");
            let vm_name = args["vm_name"].as_str().ok_or_else(|| {
                CodexErr::Other("vm_name parameter is required".to_string())
            })?;
            let vm_resource_group = args["vm_resource_group"].as_str().ok_or_else(|| {
                CodexErr::Other("vm_resource_group parameter is required".to_string())
            })?;
            let recovery_point_id = args["recovery_point_id"].as_str().ok_or_else(|| {
                CodexErr::Other("recovery_point_id parameter is required".to_string())
            })?;
            let target_storage_account = args["target_storage_account"].as_str().ok_or_else(|| {
                CodexErr::Other("target_storage_account parameter is required".to_string())
            })?;
            let file_paths = args["file_paths"].as_array()
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_else(Vec::new);
            tools.restore_files(vault_name, vm_name, vm_resource_group, recovery_point_id, file_paths, target_storage_account).await
        },
        
        // Database workload tools
        "recovery_services_discover_databases" => {
            // This is a placeholder - the actual implementation may need to be added to tools_impl.rs
            Err(CodexErr::Other("discover_databases method not yet implemented in tools_impl.rs".to_string()))
        },
        "recovery_services_inquire_workload_databases" => {
            tools.inquire_workload_databases(args).await
        },
        
        // Utility tools
        "recovery_services_clear_auth_cache" => {
            tools.clear_auth_cache(args).await
        },
        "recovery_services_track_async_operation" => {
            tools.track_async_operation(args).await
        },
        
        _ => {
            Err(CodexErr::Other(format!("Unknown Recovery Services tool: {}", name)))
        }
    }
}
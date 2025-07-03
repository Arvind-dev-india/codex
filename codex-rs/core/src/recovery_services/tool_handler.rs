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
        
        // VM registration
        "recovery_services_register_vm" => {
            tools.register_vm(args).await
        },
        "recovery_services_check_registration_status" => {
            tools.check_registration_status(args).await
        },
        // TODO: Add other VM registration methods
        
        // Policy management
        "recovery_services_list_policies" => {
            tools.list_policies(args).await
        },
        // TODO: Add other policy methods
        
        // Protection management
        "recovery_services_list_protectable_items" => {
            tools.list_protectable_items(args).await
        },
        "recovery_services_list_protected_items" => {
            tools.list_protected_items(args).await
        },
        // TODO: Add enable/disable protection methods
        
        // Backup operations
        "recovery_services_list_backup_jobs" => {
            tools.list_backup_jobs(args).await
        },
        // TODO: Add other backup methods
        
        // Recovery operations
        // TODO: Add recovery methods
        
        // Utility tools
        "recovery_services_clear_auth_cache" => {
            tools.clear_auth_cache(args).await
        },
        
        _ => {
            Err(CodexErr::Other(format!("Unknown Recovery Services tool: {}", name)))
        }
    }
}
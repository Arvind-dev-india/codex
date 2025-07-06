//! Tool definitions for Recovery Services (Azure Backup) operations.

use std::collections::BTreeMap;
use crate::openai_tools::{OpenAiTool, JsonSchema, create_function_tool};

/// Create all Recovery Services tools
pub fn create_recovery_services_tools() -> Vec<OpenAiTool> {
    vec![
        // Vault management
        create_list_vaults_tool(),
        create_test_connection_tool(),
        
        // Discovery tools
        create_refresh_containers_tool(),
        create_list_protectable_containers_tool(),
        create_list_protectable_items_new_tool(),
        create_list_workload_items_tool(),
        
        // VM registration
        create_register_vm_tool(),
        create_reregister_vm_tool(),
        create_unregister_vm_tool(),
        create_check_registration_status_tool(),
        
        // Policy management
        create_create_policy_tool(),
        create_list_policies_tool(),
        create_get_policy_details_tool(),
        
        // Protection management
        create_enable_protection_tool(),
        create_disable_protection_tool(),
        create_list_protected_items_tool(),
        
        // Backup operations
        create_trigger_backup_tool(),
        create_list_backup_jobs_tool(),
        create_get_job_status_tool(),
        create_get_backup_summary_tool(),
        
        // Recovery operations
        create_list_recovery_points_tool(),
        create_restore_original_location_tool(),
        create_restore_alternate_location_tool(),
        create_restore_as_files_tool(),
        
        // Database workload tools
        create_discover_databases_tool(),
        create_register_vm_for_workload_tool(),
        create_reregister_container_tool(),
        create_unregister_container_tool(),
        create_create_workload_policy_tool(),
        create_enable_database_protection_tool(),
        create_disable_database_protection_tool(),
        create_trigger_database_backup_tool(),
        create_restore_database_original_tool(),
        
        // Async operation tracking
        create_track_async_operation_tool(),
        create_restore_database_alternate_tool(),
        create_restore_database_as_files_tool(),
        create_generate_recovery_config_tool(),
        
        // Utility tools
        create_clear_auth_cache_tool(),
    ]
}

/// Create a tool for listing Recovery Services vaults
fn create_list_vaults_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("subscription_id".to_string(), JsonSchema::String);
    parameters.insert("resource_group".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_list_vaults",
        "List all accessible Recovery Services vaults. If subscription_id is provided, lists vaults in that subscription. If resource_group is also provided, filters to vaults in that resource group only.",
        parameters,
        &[],
    )
}

/// Create a tool for testing connection
fn create_test_connection_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_test_connection",
        "Test connectivity to Recovery Services vault and validate permissions",
        parameters,
        &[],
    )
}

/// Create a tool for refreshing containers (discovery operation)
fn create_refresh_containers_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    parameters.insert("fabric_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_refresh_containers",
        "Trigger a discovery operation to refresh the list of containers that can be registered to the Recovery Services vault. This is required before listing protectable containers to ensure the vault has the latest list of eligible resources.",
        parameters,
        &["fabric_name"],
    )
}

/// Create a tool for listing protectable containers
fn create_list_protectable_containers_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    parameters.insert("fabric_name".to_string(), JsonSchema::String);
    parameters.insert("backup_management_type".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_list_protectable_containers",
        "List containers that can be registered to the Recovery Services vault but are not yet registered. Use this after running refresh_containers to discover VMs, storage accounts, and other resources eligible for backup. Backup management types: AzureIaasVM (VMs), AzureWorkload (SQL/SAP), AzureStorage (File Shares), MAB (MARS Agent), AzureSqlDb, Exchange, Sharepoint, VMwareVM, SystemState, Client, GenericDataSource, AzureFileShare.",
        parameters,
        &["fabric_name"],
    )
}


/// Create a tool for listing protectable items (workloads/databases) - new version
fn create_list_protectable_items_new_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    parameters.insert("workload_type".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_list_protectable_items",
        "List protectable items (databases, workloads) that can be protected but are not yet registered for backup. This discovers SQL Server, SAP HANA, SAP ASE databases and other workloads inside Azure VMs.",
        parameters,
        &["workload_type"],
    )
}

/// Create a tool for listing workload items (registered/protected)
fn create_list_workload_items_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    parameters.insert("workload_type".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_list_workload_items",
        "List workload items (databases, applications) that are already registered or protected. Shows SQL Server, SAP HANA, SAP ASE databases and other workloads currently managed by the vault.",
        parameters,
        &["workload_type"],
    )
}

/// Create a tool for registering VM for backup
fn create_register_vm_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("vm_name".to_string(), JsonSchema::String);
    parameters.insert("vm_resource_group".to_string(), JsonSchema::String);
    parameters.insert("workload_type".to_string(), JsonSchema::String);
    parameters.insert("backup_management_type".to_string(), JsonSchema::String);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_register_vm",
        "Register a VM for backup protection. Automatically constructs VM resource ID from vm_name and vm_resource_group. Parameters: vm_name (required): Name of the virtual machine, vm_resource_group (required): Resource group containing the VM, workload_type (required): 'VM' (standard Azure VM backup), 'AnyDatabase' (generic database workload), 'SAPHanaDatabase' (SAP HANA), 'SQLDataBase' (SQL Server), 'SAPAseDatabase' (SAP ASE). backup_management_type (optional): 'AzureIaasVM' (standard VM), 'AzureWorkload' (databases), auto-detected if not specified. vault_name (optional): Target vault. API creates container with format 'VMAppContainer;Compute;{resourceGroup};{vmName}' for workloads or 'iaasvmcontainer;iaasvmcontainerv2;{resourceGroup};{vmName}' for VMs.",
        parameters,
        &["vm_name", "vm_resource_group", "workload_type"],
    )
}

/// Create a tool for re-registering VM
fn create_reregister_vm_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("vm_name".to_string(), JsonSchema::String);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_reregister_vm",
        "Re-register a VM for backup (useful for troubleshooting registration issues)",
        parameters,
        &["vm_name"],
    )
}

/// Create a tool for unregistering VM
fn create_unregister_vm_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("vm_name".to_string(), JsonSchema::String);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_unregister_vm",
        "Unregister a VM from backup protection",
        parameters,
        &["vm_name"],
    )
}

/// Create a tool for checking registration status
fn create_check_registration_status_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("vm_name".to_string(), JsonSchema::String);
    parameters.insert("vm_resource_group".to_string(), JsonSchema::String);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_check_registration_status",
        "Check if a VM is registered for backup in the vault. Parameters: vm_name (required): Name of the virtual machine. vm_resource_group (recommended): Resource group containing the VM - improves accuracy. vault_name (optional): Specific vault to check. This tool detects VMs registered for: Standard VM backup (AzureIaasVM management type), Database workload backup (AzureWorkload management type): SAP HANA, SQL Server, etc., Any other backup workload types. Returns detailed registration status, health status, container type, and backup management type.",
        parameters,
        &["vm_name"],
    )
}

/// Create a tool for creating backup policy
fn create_create_policy_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("policy_name".to_string(), JsonSchema::String);
    parameters.insert("workload_type".to_string(), JsonSchema::String);
    parameters.insert("policy_config".to_string(), JsonSchema::String);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_create_policy",
        "Create a backup policy for SAP HANA or SQL Server workloads",
        parameters,
        &["policy_name", "workload_type"],
    )
}

/// Create a tool for listing backup policies
fn create_list_policies_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("workload_type".to_string(), JsonSchema::String);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_list_policies",
        "List backup policies in the vault, optionally filtered by workload type",
        parameters,
        &[],
    )
}

/// Create a tool for getting policy details
fn create_get_policy_details_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("policy_name".to_string(), JsonSchema::String);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_get_policy_details",
        "Get detailed configuration of a specific backup policy",
        parameters,
        &["policy_name"],
    )
}

/// Create a tool for listing protectable items
fn create_list_protectable_items_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("workload_type".to_string(), JsonSchema::String);
    parameters.insert("server_name".to_string(), JsonSchema::String);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_list_protectable_items",
        "List databases available for backup protection, optionally filtered by workload type and server",
        parameters,
        &[],
    )
}

/// Create a tool for enabling protection
fn create_enable_protection_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("item_name".to_string(), JsonSchema::String);
    parameters.insert("policy_name".to_string(), JsonSchema::String);
    parameters.insert("server_name".to_string(), JsonSchema::String);
    parameters.insert("workload_type".to_string(), JsonSchema::String);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_enable_protection",
        "Enable backup protection for a database using specified policy",
        parameters,
        &["item_name", "policy_name", "server_name", "workload_type"],
    )
}

/// Create a tool for disabling protection
fn create_disable_protection_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("item_name".to_string(), JsonSchema::String);
    parameters.insert("server_name".to_string(), JsonSchema::String);
    parameters.insert("delete_backup_data".to_string(), JsonSchema::Boolean);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_disable_protection",
        "Disable backup protection for a database, optionally deleting existing backup data",
        parameters,
        &["item_name", "server_name"],
    )
}

/// Create a tool for listing protected items
fn create_list_protected_items_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("backup_management_type".to_string(), JsonSchema::String);
    parameters.insert("workload_type".to_string(), JsonSchema::String);
    parameters.insert("server_name".to_string(), JsonSchema::String);
    parameters.insert("container_name".to_string(), JsonSchema::String);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_list_protected_items",
        "List items currently protected by backup. Works with just vault_name and automatically searches across all backup management types and item types. Optional filters: backup_management_type ('AzureIaasVM' for standard VM backups, 'AzureWorkload' for database workloads like SAP HANA/SQL Server, 'AzureStorage' for file shares), workload_type ('VM', 'SAPHanaDatabase', 'SAPHanaDBInstance', 'SQLDataBase', 'SAPAseDatabase', 'AnyDatabase', 'AzureFileShare', etc. - Azure API uses 'itemType'), server_name (partial match), container_name (exact match). If no filters specified, searches all types to find protected items.",
        parameters,
        &[],
    )
}

/// Create a tool for triggering backup
fn create_trigger_backup_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("item_name".to_string(), JsonSchema::String);
    parameters.insert("server_name".to_string(), JsonSchema::String);
    parameters.insert("backup_type".to_string(), JsonSchema::String);
    parameters.insert("retain_until".to_string(), JsonSchema::String);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_trigger_backup",
        "Trigger an ad-hoc backup for a protected database. backup_type can be 'Full', 'Incremental', or 'Log'",
        parameters,
        &["item_name", "server_name", "backup_type"],
    )
}

/// Create a tool for listing backup jobs
fn create_list_backup_jobs_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("status_filter".to_string(), JsonSchema::String);
    parameters.insert("operation_filter".to_string(), JsonSchema::String);
    parameters.insert("time_range_hours".to_string(), JsonSchema::Number);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_list_backup_jobs",
        "List backup jobs with optional filtering by status, operation type, and time range",
        parameters,
        &[],
    )
}

/// Create a tool for getting job status
fn create_get_job_status_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("job_id".to_string(), JsonSchema::String);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_get_job_status",
        "Get detailed status and information for a specific backup job",
        parameters,
        &["job_id"],
    )
}

/// Create a tool for getting backup summary
fn create_get_backup_summary_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("workload_type".to_string(), JsonSchema::String);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_get_backup_summary",
        "Get summary of backup status and recent activity for the vault",
        parameters,
        &[],
    )
}

/// Create a tool for listing recovery points
fn create_list_recovery_points_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("item_name".to_string(), JsonSchema::String);
    parameters.insert("server_name".to_string(), JsonSchema::String);
    parameters.insert("time_range_days".to_string(), JsonSchema::Number);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_list_recovery_points",
        "List available recovery points for a protected database",
        parameters,
        &["item_name", "server_name"],
    )
}

/// Create a tool for restoring to original location
fn create_restore_original_location_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("item_name".to_string(), JsonSchema::String);
    parameters.insert("server_name".to_string(), JsonSchema::String);
    parameters.insert("recovery_point_id".to_string(), JsonSchema::String);
    parameters.insert("log_point_in_time".to_string(), JsonSchema::String);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_restore_original_location",
        "Restore a database to its original location from a recovery point",
        parameters,
        &["item_name", "server_name", "recovery_point_id"],
    )
}

/// Create a tool for restoring to alternate location
fn create_restore_alternate_location_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("item_name".to_string(), JsonSchema::String);
    parameters.insert("server_name".to_string(), JsonSchema::String);
    parameters.insert("recovery_point_id".to_string(), JsonSchema::String);
    parameters.insert("target_server".to_string(), JsonSchema::String);
    parameters.insert("target_database".to_string(), JsonSchema::String);
    parameters.insert("log_point_in_time".to_string(), JsonSchema::String);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_restore_alternate_location",
        "Restore a database to an alternate location/server from a recovery point",
        parameters,
        &["item_name", "server_name", "recovery_point_id", "target_server", "target_database"],
    )
}

/// Create a tool for restoring as files
fn create_restore_as_files_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("item_name".to_string(), JsonSchema::String);
    parameters.insert("server_name".to_string(), JsonSchema::String);
    parameters.insert("recovery_point_id".to_string(), JsonSchema::String);
    parameters.insert("target_container".to_string(), JsonSchema::String);
    parameters.insert("file_path".to_string(), JsonSchema::String);
    parameters.insert("log_point_in_time".to_string(), JsonSchema::String);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_restore_as_files",
        "Restore a database as files to a specified location",
        parameters,
        &["item_name", "server_name", "recovery_point_id", "target_container", "file_path"],
    )
}

/// Create a tool for discovering databases
fn create_discover_databases_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("server_name".to_string(), JsonSchema::String);
    parameters.insert("workload_type".to_string(), JsonSchema::String);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_discover_databases",
        "Discover databases on a registered VM for backup protection",
        parameters,
        &["server_name", "workload_type"],
    )
}

/// Create a tool for registering VM for workload
fn create_register_vm_for_workload_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("vm_resource_id".to_string(), JsonSchema::String);
    parameters.insert("workload_type".to_string(), JsonSchema::String);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_register_vm_for_workload",
        "Register a VM for database workload backup (SAP HANA, SQL Server)",
        parameters,
        &["vm_resource_id", "workload_type"],
    )
}

/// Create a tool for re-registering container
fn create_reregister_container_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("container_name".to_string(), JsonSchema::String);
    parameters.insert("workload_type".to_string(), JsonSchema::String);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_reregister_container",
        "Re-register a container for workload backup (useful for troubleshooting)",
        parameters,
        &["container_name", "workload_type"],
    )
}

/// Create a tool for unregistering container
fn create_unregister_container_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("container_name".to_string(), JsonSchema::String);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_unregister_container",
        "Unregister a container from backup protection",
        parameters,
        &["container_name"],
    )
}

/// Create a tool for creating workload policy
fn create_create_workload_policy_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("policy_name".to_string(), JsonSchema::String);
    parameters.insert("workload_type".to_string(), JsonSchema::String);
    parameters.insert("policy_config".to_string(), JsonSchema::String);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_create_workload_policy",
        "Create a backup policy for database workloads with specific configuration",
        parameters,
        &["policy_name", "workload_type", "policy_config"],
    )
}

/// Create a tool for enabling database protection
fn create_enable_database_protection_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("container_name".to_string(), JsonSchema::String);
    parameters.insert("protectable_item_name".to_string(), JsonSchema::String);
    parameters.insert("policy_name".to_string(), JsonSchema::String);
    parameters.insert("workload_type".to_string(), JsonSchema::String);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_enable_database_protection",
        "Enable backup protection for a specific database using a policy",
        parameters,
        &["container_name", "protectable_item_name", "policy_name", "workload_type"],
    )
}

/// Create a tool for disabling database protection
fn create_disable_database_protection_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("container_name".to_string(), JsonSchema::String);
    parameters.insert("protected_item_name".to_string(), JsonSchema::String);
    parameters.insert("delete_backup_data".to_string(), JsonSchema::Boolean);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_disable_database_protection",
        "Disable backup protection for a database, optionally deleting backup data",
        parameters,
        &["container_name", "protected_item_name"],
    )
}

/// Create a tool for triggering database backup
fn create_trigger_database_backup_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("container_name".to_string(), JsonSchema::String);
    parameters.insert("protected_item_name".to_string(), JsonSchema::String);
    parameters.insert("backup_type".to_string(), JsonSchema::String);
    parameters.insert("retention_date".to_string(), JsonSchema::String);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_trigger_database_backup",
        "Trigger an ad-hoc backup for a protected database",
        parameters,
        &["container_name", "protected_item_name", "backup_type"],
    )
}

/// Create a tool for restoring database to original location
fn create_restore_database_original_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("container_name".to_string(), JsonSchema::String);
    parameters.insert("protected_item_name".to_string(), JsonSchema::String);
    parameters.insert("recovery_point_id".to_string(), JsonSchema::String);
    parameters.insert("log_point_in_time".to_string(), JsonSchema::String);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_restore_database_original",
        "Restore a database to its original location from a recovery point",
        parameters,
        &["container_name", "protected_item_name", "recovery_point_id"],
    )
}

/// Create a tool for restoring database to alternate location
fn create_restore_database_alternate_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("container_name".to_string(), JsonSchema::String);
    parameters.insert("protected_item_name".to_string(), JsonSchema::String);
    parameters.insert("recovery_point_id".to_string(), JsonSchema::String);
    parameters.insert("target_server".to_string(), JsonSchema::String);
    parameters.insert("target_database".to_string(), JsonSchema::String);
    parameters.insert("log_point_in_time".to_string(), JsonSchema::String);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_restore_database_alternate",
        "Restore a database to an alternate location from a recovery point",
        parameters,
        &["container_name", "protected_item_name", "recovery_point_id", "target_server", "target_database"],
    )
}

/// Create a tool for restoring database as files
fn create_restore_database_as_files_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("container_name".to_string(), JsonSchema::String);
    parameters.insert("protected_item_name".to_string(), JsonSchema::String);
    parameters.insert("recovery_point_id".to_string(), JsonSchema::String);
    parameters.insert("target_container".to_string(), JsonSchema::String);
    parameters.insert("file_path".to_string(), JsonSchema::String);
    parameters.insert("log_point_in_time".to_string(), JsonSchema::String);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_restore_database_as_files",
        "Restore a database as files to a specified location",
        parameters,
        &["container_name", "protected_item_name", "recovery_point_id", "target_container", "file_path"],
    )
}

/// Create a tool for generating recovery configuration
fn create_generate_recovery_config_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("container_name".to_string(), JsonSchema::String);
    parameters.insert("protected_item_name".to_string(), JsonSchema::String);
    parameters.insert("recovery_point_name".to_string(), JsonSchema::String);
    parameters.insert("restore_mode".to_string(), JsonSchema::String);
    parameters.insert("target_server".to_string(), JsonSchema::String);
    parameters.insert("target_database".to_string(), JsonSchema::String);
    parameters.insert("log_point_in_time".to_string(), JsonSchema::String);
    parameters.insert("file_path".to_string(), JsonSchema::String);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_generate_recovery_config",
        "Generate recovery configuration for database restore operations",
        parameters,
        &["container_name", "protected_item_name", "recovery_point_name", "restore_mode"],
    )
}

/// Create a tool for clearing authentication cache
fn create_clear_auth_cache_tool() -> OpenAiTool {
    let parameters = BTreeMap::new();
    
    create_function_tool(
        "recovery_services_clear_auth_cache",
        "Clear Recovery Services authentication cache to force re-authentication",
        parameters,
        &[],
    )
}

/// Create a tool for tracking async operations
fn create_track_async_operation_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("location_url".to_string(), JsonSchema::String);
    parameters.insert("wait_for_completion".to_string(), JsonSchema::Boolean);
    parameters.insert("max_wait_seconds".to_string(), JsonSchema::Number);
    
    create_function_tool(
        "recovery_services_track_async_operation",
        "Track the status of an asynchronous Recovery Services operation using the location URL",
        parameters,
        &["location_url"],
    )
}
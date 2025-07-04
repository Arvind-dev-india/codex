//! Tool definitions for Recovery Services (Azure Backup) operations.

use std::collections::BTreeMap;
use crate::openai_tools::{OpenAiTool, JsonSchema, create_function_tool};

/// Create all Recovery Services tools
pub fn create_recovery_services_tools() -> Vec<OpenAiTool> {
    vec![
        // Vault management
        create_list_vaults_tool(),
        create_test_connection_tool(),
        
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
        create_list_protectable_items_tool(),
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

/// Create a tool for registering VM for backup
fn create_register_vm_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("vm_resource_id".to_string(), JsonSchema::String);
    parameters.insert("workload_type".to_string(), JsonSchema::String);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_register_vm",
        "Register a VM for backup protection. workload_type should be 'SAPHANA' or 'SQLDataBase'",
        parameters,
        &["vm_resource_id", "workload_type"],
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
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_check_registration_status",
        "Check the registration status of a VM in the backup vault",
        parameters,
        &[],
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
    parameters.insert("workload_type".to_string(), JsonSchema::String);
    parameters.insert("server_name".to_string(), JsonSchema::String);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_list_protected_items",
        "List databases currently protected by backup, optionally filtered by workload type and server",
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
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_get_backup_summary",
        "Get a summary of backup status for all protected items in the vault",
        parameters,
        &[],
    )
}

/// Create a tool for listing recovery points
fn create_list_recovery_points_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("item_name".to_string(), JsonSchema::String);
    parameters.insert("server_name".to_string(), JsonSchema::String);
    parameters.insert("recovery_point_type".to_string(), JsonSchema::String);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_list_recovery_points",
        "List available recovery points for a protected database, optionally filtered by type (Full, Incremental, Log)",
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
    parameters.insert("point_in_time".to_string(), JsonSchema::String);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_restore_original_location",
        "Restore a database to its original location using specified recovery point or point-in-time",
        parameters,
        &["item_name", "server_name"],
    )
}

/// Create a tool for restoring to alternate location
fn create_restore_alternate_location_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("source_item_name".to_string(), JsonSchema::String);
    parameters.insert("source_server_name".to_string(), JsonSchema::String);
    parameters.insert("target_server_name".to_string(), JsonSchema::String);
    parameters.insert("target_database_name".to_string(), JsonSchema::String);
    parameters.insert("recovery_point_id".to_string(), JsonSchema::String);
    parameters.insert("point_in_time".to_string(), JsonSchema::String);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_restore_alternate_location",
        "Restore a database to an alternate server/location using specified recovery point or point-in-time",
        parameters,
        &["source_item_name", "source_server_name", "target_server_name"],
    )
}

/// Create a tool for restoring as files
fn create_restore_as_files_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("item_name".to_string(), JsonSchema::String);
    parameters.insert("server_name".to_string(), JsonSchema::String);
    parameters.insert("target_file_path".to_string(), JsonSchema::String);
    parameters.insert("recovery_point_id".to_string(), JsonSchema::String);
    parameters.insert("point_in_time".to_string(), JsonSchema::String);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_restore_as_files",
        "Restore a database as files to a specified directory path",
        parameters,
        &["item_name", "server_name", "target_file_path"],
    )
}

/// Create a tool for discovering databases
fn create_discover_databases_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    parameters.insert("workload_type".to_string(), JsonSchema::String);
    parameters.insert("server_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_discover_databases",
        "Discover databases on a server for backup. workload_type can be 'SAPHANA' or 'SQLDataBase'",
        parameters,
        &["workload_type"],
    )
}

/// Create a tool for registering VM for workload
fn create_register_vm_for_workload_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    parameters.insert("vm_name".to_string(), JsonSchema::String);
    parameters.insert("vm_resource_group".to_string(), JsonSchema::String);
    parameters.insert("workload_type".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_register_vm_for_workload",
        "Register a VM for workload backup (SAP HANA, SQL Server). workload_type can be 'SAPHANA' or 'SQLDataBase'",
        parameters,
        &["vm_name", "vm_resource_group", "workload_type"],
    )
}

/// Create a tool for re-registering container
fn create_reregister_container_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    parameters.insert("container_name".to_string(), JsonSchema::String);
    parameters.insert("workload_type".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_reregister_container",
        "Re-register a container for workload backup discovery",
        parameters,
        &["container_name", "workload_type"],
    )
}

/// Create a tool for unregistering container
fn create_unregister_container_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    parameters.insert("container_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_unregister_container",
        "Unregister a container from backup",
        parameters,
        &["container_name"],
    )
}

/// Create a tool for creating workload policy
fn create_create_workload_policy_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    parameters.insert("policy_name".to_string(), JsonSchema::String);
    parameters.insert("workload_type".to_string(), JsonSchema::String);
    parameters.insert("full_backup_schedule".to_string(), JsonSchema::String);
    parameters.insert("log_backup_frequency_minutes".to_string(), JsonSchema::Number);
    parameters.insert("retention_days".to_string(), JsonSchema::Number);
    
    create_function_tool(
        "recovery_services_create_workload_policy",
        "Create a workload-specific backup policy for databases. workload_type can be 'SAPHanaDatabase' or 'SQLDataBase'",
        parameters,
        &["policy_name", "workload_type"],
    )
}

/// Create a tool for enabling database protection
fn create_enable_database_protection_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    parameters.insert("server_name".to_string(), JsonSchema::String);
    parameters.insert("database_name".to_string(), JsonSchema::String);
    parameters.insert("policy_name".to_string(), JsonSchema::String);
    parameters.insert("workload_type".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_enable_database_protection",
        "Enable backup protection for a specific database. workload_type can be 'SAPHANA' or 'SQLDataBase'",
        parameters,
        &["server_name", "database_name", "policy_name", "workload_type"],
    )
}

/// Create a tool for disabling database protection
fn create_disable_database_protection_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    parameters.insert("server_name".to_string(), JsonSchema::String);
    parameters.insert("database_name".to_string(), JsonSchema::String);
    parameters.insert("workload_type".to_string(), JsonSchema::String);
    parameters.insert("delete_backup_data".to_string(), JsonSchema::Boolean);
    
    create_function_tool(
        "recovery_services_disable_database_protection",
        "Disable backup protection for a specific database",
        parameters,
        &["server_name", "database_name", "workload_type"],
    )
}

/// Create a tool for triggering database backup
fn create_trigger_database_backup_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    parameters.insert("server_name".to_string(), JsonSchema::String);
    parameters.insert("database_name".to_string(), JsonSchema::String);
    parameters.insert("backup_type".to_string(), JsonSchema::String);
    parameters.insert("retention_date".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_trigger_database_backup",
        "Trigger an on-demand backup for a database. backup_type can be 'Full', 'Differential', or 'Log'",
        parameters,
        &["server_name", "database_name", "backup_type"],
    )
}

/// Create a tool for restoring database to original location
fn create_restore_database_original_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    parameters.insert("server_name".to_string(), JsonSchema::String);
    parameters.insert("database_name".to_string(), JsonSchema::String);
    parameters.insert("recovery_point_id".to_string(), JsonSchema::String);
    parameters.insert("log_point_in_time".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_restore_database_original",
        "Restore a database to its original location. log_point_in_time format: 'dd-MM-yyyy-HH:mm:ss'",
        parameters,
        &["server_name", "database_name", "recovery_point_id"],
    )
}

/// Create a tool for restoring database to alternate location
fn create_restore_database_alternate_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    parameters.insert("source_server_name".to_string(), JsonSchema::String);
    parameters.insert("source_database_name".to_string(), JsonSchema::String);
    parameters.insert("target_server_name".to_string(), JsonSchema::String);
    parameters.insert("target_database_name".to_string(), JsonSchema::String);
    parameters.insert("recovery_point_id".to_string(), JsonSchema::String);
    parameters.insert("log_point_in_time".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_restore_database_alternate",
        "Restore a database to an alternate location/server. log_point_in_time format: 'dd-MM-yyyy-HH:mm:ss'",
        parameters,
        &["source_server_name", "source_database_name", "target_server_name", "target_database_name", "recovery_point_id"],
    )
}

/// Create a tool for restoring database as files
fn create_restore_database_as_files_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    parameters.insert("server_name".to_string(), JsonSchema::String);
    parameters.insert("database_name".to_string(), JsonSchema::String);
    parameters.insert("recovery_point_id".to_string(), JsonSchema::String);
    parameters.insert("target_server_name".to_string(), JsonSchema::String);
    parameters.insert("file_path".to_string(), JsonSchema::String);
    parameters.insert("log_point_in_time".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_restore_database_as_files",
        "Restore a database as files to a specified path. log_point_in_time format: 'dd-MM-yyyy-HH:mm:ss'",
        parameters,
        &["server_name", "database_name", "recovery_point_id", "target_server_name", "file_path"],
    )
}

/// Create a tool for generating recovery configuration
fn create_generate_recovery_config_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    parameters.insert("server_name".to_string(), JsonSchema::String);
    parameters.insert("database_name".to_string(), JsonSchema::String);
    parameters.insert("recovery_point_name".to_string(), JsonSchema::String);
    parameters.insert("restore_mode".to_string(), JsonSchema::String);
    parameters.insert("target_server_name".to_string(), JsonSchema::String);
    parameters.insert("target_database_name".to_string(), JsonSchema::String);
    parameters.insert("log_point_in_time".to_string(), JsonSchema::String);
    parameters.insert("file_path".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_generate_recovery_config",
        "Generate recovery configuration for database restore. restore_mode can be 'OriginalLocation', 'AlternateLocation', or 'RestoreAsFiles'",
        parameters,
        &["server_name", "database_name", "recovery_point_name", "restore_mode"],
    )
}

/// Create a tool for clearing authentication cache
fn create_clear_auth_cache_tool() -> OpenAiTool {
    let parameters = BTreeMap::new();
    
    create_function_tool(
        "recovery_services_clear_auth_cache",
        "Clear Recovery Services authentication cache and force re-authentication",
        parameters,
        &[],
    )
}
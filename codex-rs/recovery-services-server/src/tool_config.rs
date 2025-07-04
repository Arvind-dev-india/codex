//! Configuration for Recovery Services tools

use mcp_types::{Tool, ToolInputSchema};
use serde_json::json;

/// Create the list of available Recovery Services tools
pub fn create_recovery_services_tools() -> Vec<Tool> {
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
        create_get_job_details_tool(),
        create_cancel_job_tool(),
        
        // Recovery operations
        create_list_recovery_points_tool(),
        create_restore_vm_tool(),
        create_restore_files_tool(),
        
        // Utility tools
        create_clear_auth_cache_tool(),
    ]
}

/// Create the list_vaults tool definition
fn create_list_vaults_tool() -> Tool {
    Tool {
        name: "recovery_services_list_vaults".to_string(),
        description: Some("List Recovery Services vaults in the subscription with optional filtering by resource group".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "subscription_id": {
                    "type": "string",
                    "description": "Azure subscription ID (optional, uses default from config if not specified)"
                },
                "resource_group": {
                    "type": "string",
                    "description": "Resource group name (optional, lists all vaults in subscription if not specified)"
                }
            })),
            required: Some(vec![]),
        },
    }
}

/// Create the test_connection tool definition
fn create_test_connection_tool() -> Tool {
    Tool {
        name: "recovery_services_test_connection".to_string(),
        description: Some("Test connection to Recovery Services API and validate authentication".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({})),
            required: Some(vec![]),
        },
    }
}

/// Create the register_vm tool definition
fn create_register_vm_tool() -> Tool {
    Tool {
        name: "recovery_services_register_vm".to_string(),
        description: Some("Register a virtual machine for backup with a Recovery Services vault for different workload types".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "vault_name": {
                    "type": "string",
                    "description": "Name of the Recovery Services vault (optional, uses default from config if not specified)"
                },
                "vm_resource_id": {
                    "type": "string",
                    "description": "Full Azure resource ID of the virtual machine to register"
                },
                "workload_type": {
                    "type": "string",
                    "description": "Type of workload to backup",
                    "enum": ["VM", "FileFolder", "AzureSqlDb", "SqlDb", "Exchange", "Sharepoint", "VMwareVM", "SystemState", "Client", "GenericDataSource", "SqlDatabase", "AzureFileShare", "SapHanaDatabase", "SapAseDatabase", "SapHanaDbInstance"],
                    "default": "VM"
                },
                "backup_management_type": {
                    "type": "string",
                    "description": "Backup management type (optional, auto-detected based on workload type)",
                    "enum": ["AzureIaasVM", "AzureWorkload", "AzureStorage", "AzureSql"]
                }
            })),
            required: Some(vec!["vm_resource_id".to_string()]),
        },
    }
}

/// Create the reregister_vm tool definition
fn create_reregister_vm_tool() -> Tool {
    Tool {
        name: "recovery_services_reregister_vm".to_string(),
        description: Some("Re-register a virtual machine for backup (useful after VM changes)".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "vault_name": {
                    "type": "string",
                    "description": "Name of the Recovery Services vault"
                },
                "vm_name": {
                    "type": "string",
                    "description": "Name of the virtual machine to re-register"
                },
                "vm_resource_group": {
                    "type": "string",
                    "description": "Resource group containing the VM"
                }
            })),
            required: Some(vec!["vault_name".to_string(), "vm_name".to_string(), "vm_resource_group".to_string()]),
        },
    }
}

/// Create the unregister_vm tool definition
fn create_unregister_vm_tool() -> Tool {
    Tool {
        name: "recovery_services_unregister_vm".to_string(),
        description: Some("Unregister a virtual machine from backup".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "vault_name": {
                    "type": "string",
                    "description": "Name of the Recovery Services vault"
                },
                "vm_name": {
                    "type": "string",
                    "description": "Name of the virtual machine to unregister"
                },
                "vm_resource_group": {
                    "type": "string",
                    "description": "Resource group containing the VM"
                }
            })),
            required: Some(vec!["vault_name".to_string(), "vm_name".to_string(), "vm_resource_group".to_string()]),
        },
    }
}

/// Create the check_registration_status tool definition
fn create_check_registration_status_tool() -> Tool {
    Tool {
        name: "recovery_services_check_registration_status".to_string(),
        description: Some("Check if a virtual machine is registered for backup in a Recovery Services vault".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "vault_name": {
                    "type": "string",
                    "description": "Name of the Recovery Services vault (optional, uses default from config if not specified)"
                },
                "vm_name": {
                    "type": "string",
                    "description": "Name of the virtual machine to check"
                },
                "vm_resource_group": {
                    "type": "string",
                    "description": "Resource group containing the VM (optional, improves search accuracy if provided)"
                }
            })),
            required: Some(vec!["vm_name".to_string()]),
        },
    }
}

/// Create the create_policy tool definition
fn create_create_policy_tool() -> Tool {
    Tool {
        name: "recovery_services_create_policy".to_string(),
        description: Some("Create a new backup policy".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "vault_name": {
                    "type": "string",
                    "description": "Name of the Recovery Services vault"
                },
                "policy_name": {
                    "type": "string",
                    "description": "Name for the new backup policy"
                },
                "schedule_type": {
                    "type": "string",
                    "description": "Schedule type (Daily, Weekly)",
                    "enum": ["Daily", "Weekly"]
                },
                "retention_days": {
                    "type": "integer",
                    "description": "Number of days to retain backups",
                    "minimum": 1,
                    "maximum": 9999
                }
            })),
            required: Some(vec!["vault_name".to_string(), "policy_name".to_string(), "schedule_type".to_string(), "retention_days".to_string()]),
        },
    }
}

/// Create the list_policies tool definition
fn create_list_policies_tool() -> Tool {
    Tool {
        name: "recovery_services_list_policies".to_string(),
        description: Some("List backup policies in a Recovery Services vault".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "vault_name": {
                    "type": "string",
                    "description": "Name of the Recovery Services vault"
                }
            })),
            required: Some(vec!["vault_name".to_string()]),
        },
    }
}

/// Create the get_policy_details tool definition
fn create_get_policy_details_tool() -> Tool {
    Tool {
        name: "recovery_services_get_policy_details".to_string(),
        description: Some("Get detailed information about a backup policy".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "vault_name": {
                    "type": "string",
                    "description": "Name of the Recovery Services vault"
                },
                "policy_name": {
                    "type": "string",
                    "description": "Name of the backup policy"
                }
            })),
            required: Some(vec!["vault_name".to_string(), "policy_name".to_string()]),
        },
    }
}

/// Create the list_protectable_items tool definition
fn create_list_protectable_items_tool() -> Tool {
    Tool {
        name: "recovery_services_list_protectable_items".to_string(),
        description: Some("List items that can be protected (backed up) in a vault".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "vault_name": {
                    "type": "string",
                    "description": "Name of the Recovery Services vault"
                }
            })),
            required: Some(vec!["vault_name".to_string()]),
        },
    }
}

/// Create the enable_protection tool definition
fn create_enable_protection_tool() -> Tool {
    Tool {
        name: "recovery_services_enable_protection".to_string(),
        description: Some("Enable backup protection for a virtual machine".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "vault_name": {
                    "type": "string",
                    "description": "Name of the Recovery Services vault"
                },
                "vm_name": {
                    "type": "string",
                    "description": "Name of the virtual machine"
                },
                "vm_resource_group": {
                    "type": "string",
                    "description": "Resource group containing the VM"
                },
                "policy_name": {
                    "type": "string",
                    "description": "Name of the backup policy to use"
                }
            })),
            required: Some(vec!["vault_name".to_string(), "vm_name".to_string(), "vm_resource_group".to_string(), "policy_name".to_string()]),
        },
    }
}

/// Create the disable_protection tool definition
fn create_disable_protection_tool() -> Tool {
    Tool {
        name: "recovery_services_disable_protection".to_string(),
        description: Some("Disable backup protection for a virtual machine".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "vault_name": {
                    "type": "string",
                    "description": "Name of the Recovery Services vault"
                },
                "vm_name": {
                    "type": "string",
                    "description": "Name of the virtual machine"
                },
                "vm_resource_group": {
                    "type": "string",
                    "description": "Resource group containing the VM"
                },
                "delete_backup_data": {
                    "type": "boolean",
                    "description": "Whether to delete existing backup data",
                    "default": false
                }
            })),
            required: Some(vec!["vault_name".to_string(), "vm_name".to_string(), "vm_resource_group".to_string()]),
        },
    }
}

/// Create the list_protected_items tool definition
fn create_list_protected_items_tool() -> Tool {
    Tool {
        name: "recovery_services_list_protected_items".to_string(),
        description: Some("List items that are currently protected (backed up) in a vault. Works with just vault_name and automatically searches across all backup management types and item types. Optional filters: backup_management_type ('AzureIaasVM' for standard VM backups, 'AzureWorkload' for database workloads like SAP HANA/SQL Server, 'AzureStorage' for file shares), workload_type ('VM', 'SAPHanaDatabase', 'SAPHanaDBInstance', 'SQLDataBase', 'SAPAseDatabase', 'AnyDatabase', 'AzureFileShare', etc. - Azure API uses 'itemType'), server_name (partial match), container_name (exact match). If no filters specified, searches all types to find protected items.".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "vault_name": {
                    "type": "string",
                    "description": "Name of the Recovery Services vault (optional, uses default from config if not specified)"
                },
                "backup_management_type": {
                    "type": "string",
                    "description": "Optional filter by backup management type. 'AzureIaasVM' for standard VM backups, 'AzureWorkload' for database workloads (SAP HANA, SQL Server), 'AzureStorage' for file shares, 'AzureSql' for Azure SQL databases. If not specified, searches all types.",
                    "enum": ["AzureIaasVM", "AzureWorkload", "AzureStorage", "AzureSql"]
                },
                "workload_type": {
                    "type": "string",
                    "description": "Optional filter by workload/item type (Azure API uses 'itemType'). Common values: 'VM' (virtual machines), 'SAPHanaDatabase' (SAP HANA databases), 'SAPHanaDBInstance' (SAP HANA instances), 'SQLDataBase' (SQL Server), 'SAPAseDatabase' (SAP ASE), 'AnyDatabase' (any database), 'AzureFileShare' (file shares). If not specified, searches all types.",
                    "enum": ["VM", "SAPHanaDatabase", "SAPHanaDBInstance", "SQLDataBase", "SAPAseDatabase", "AnyDatabase", "AzureFileShare", "FileFolder", "AzureSqlDb", "Exchange", "Sharepoint", "VMwareVM", "SystemState", "Client", "GenericDataSource"]
                },
                "server_name": {
                    "type": "string",
                    "description": "Optional filter by server name (supports partial matching)"
                },
                "container_name": {
                    "type": "string",
                    "description": "Optional filter by specific container name (exact match)"
                }
            })),
            required: Some(vec![]),
        },
    }
}

/// Create the trigger_backup tool definition
fn create_trigger_backup_tool() -> Tool {
    Tool {
        name: "recovery_services_trigger_backup".to_string(),
        description: Some("Trigger an on-demand backup for a protected virtual machine".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "vault_name": {
                    "type": "string",
                    "description": "Name of the Recovery Services vault"
                },
                "vm_name": {
                    "type": "string",
                    "description": "Name of the virtual machine"
                },
                "vm_resource_group": {
                    "type": "string",
                    "description": "Resource group containing the VM"
                },
                "retention_days": {
                    "type": "integer",
                    "description": "Number of days to retain this backup",
                    "minimum": 1,
                    "maximum": 99,
                    "default": 30
                }
            })),
            required: Some(vec!["vault_name".to_string(), "vm_name".to_string(), "vm_resource_group".to_string()]),
        },
    }
}

/// Create the list_backup_jobs tool definition
fn create_list_backup_jobs_tool() -> Tool {
    Tool {
        name: "recovery_services_list_backup_jobs".to_string(),
        description: Some("List backup jobs in a Recovery Services vault".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "vault_name": {
                    "type": "string",
                    "description": "Name of the Recovery Services vault"
                },
                "status": {
                    "type": "string",
                    "description": "Filter by job status",
                    "enum": ["InProgress", "Completed", "Failed", "Cancelled"]
                },
                "operation": {
                    "type": "string",
                    "description": "Filter by operation type",
                    "enum": ["Backup", "Restore"]
                }
            })),
            required: Some(vec!["vault_name".to_string()]),
        },
    }
}

/// Create the get_job_details tool definition
fn create_get_job_details_tool() -> Tool {
    Tool {
        name: "recovery_services_get_job_details".to_string(),
        description: Some("Get detailed information about a specific backup job".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "vault_name": {
                    "type": "string",
                    "description": "Name of the Recovery Services vault"
                },
                "job_id": {
                    "type": "string",
                    "description": "ID of the backup job"
                }
            })),
            required: Some(vec!["vault_name".to_string(), "job_id".to_string()]),
        },
    }
}

/// Create the cancel_job tool definition
fn create_cancel_job_tool() -> Tool {
    Tool {
        name: "recovery_services_cancel_job".to_string(),
        description: Some("Cancel a running backup or restore job".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "vault_name": {
                    "type": "string",
                    "description": "Name of the Recovery Services vault"
                },
                "job_id": {
                    "type": "string",
                    "description": "ID of the job to cancel"
                }
            })),
            required: Some(vec!["vault_name".to_string(), "job_id".to_string()]),
        },
    }
}

/// Create the list_recovery_points tool definition
fn create_list_recovery_points_tool() -> Tool {
    Tool {
        name: "recovery_services_list_recovery_points".to_string(),
        description: Some("List available recovery points for a protected virtual machine".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "vault_name": {
                    "type": "string",
                    "description": "Name of the Recovery Services vault"
                },
                "vm_name": {
                    "type": "string",
                    "description": "Name of the virtual machine"
                },
                "vm_resource_group": {
                    "type": "string",
                    "description": "Resource group containing the VM"
                }
            })),
            required: Some(vec!["vault_name".to_string(), "vm_name".to_string(), "vm_resource_group".to_string()]),
        },
    }
}

/// Create the restore_vm tool definition
fn create_restore_vm_tool() -> Tool {
    Tool {
        name: "recovery_services_restore_vm".to_string(),
        description: Some("Restore a virtual machine from a recovery point".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "vault_name": {
                    "type": "string",
                    "description": "Name of the Recovery Services vault"
                },
                "vm_name": {
                    "type": "string",
                    "description": "Name of the virtual machine to restore"
                },
                "vm_resource_group": {
                    "type": "string",
                    "description": "Resource group containing the VM"
                },
                "recovery_point_id": {
                    "type": "string",
                    "description": "ID of the recovery point to restore from"
                },
                "restore_type": {
                    "type": "string",
                    "description": "Type of restore operation",
                    "enum": ["AlternateLocation", "OriginalLocation", "RestoreDisks"]
                },
                "target_vm_name": {
                    "type": "string",
                    "description": "Name for the restored VM (required for AlternateLocation)"
                },
                "target_resource_group": {
                    "type": "string",
                    "description": "Target resource group for restore (required for AlternateLocation)"
                }
            })),
            required: Some(vec!["vault_name".to_string(), "vm_name".to_string(), "vm_resource_group".to_string(), "recovery_point_id".to_string(), "restore_type".to_string()]),
        },
    }
}

/// Create the restore_files tool definition
fn create_restore_files_tool() -> Tool {
    Tool {
        name: "recovery_services_restore_files".to_string(),
        description: Some("Restore specific files from a recovery point".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "vault_name": {
                    "type": "string",
                    "description": "Name of the Recovery Services vault"
                },
                "vm_name": {
                    "type": "string",
                    "description": "Name of the virtual machine"
                },
                "vm_resource_group": {
                    "type": "string",
                    "description": "Resource group containing the VM"
                },
                "recovery_point_id": {
                    "type": "string",
                    "description": "ID of the recovery point to restore from"
                },
                "file_paths": {
                    "type": "array",
                    "items": {
                        "type": "string"
                    },
                    "description": "List of file paths to restore"
                },
                "target_storage_account": {
                    "type": "string",
                    "description": "Target storage account for restored files"
                }
            })),
            required: Some(vec!["vault_name".to_string(), "vm_name".to_string(), "vm_resource_group".to_string(), "recovery_point_id".to_string(), "file_paths".to_string(), "target_storage_account".to_string()]),
        },
    }
}

/// Create the clear_auth_cache tool definition
fn create_clear_auth_cache_tool() -> Tool {
    Tool {
        name: "recovery_services_clear_auth_cache".to_string(),
        description: Some("Clear Recovery Services authentication cache and force re-authentication".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({})),
            required: Some(vec![]),
        },
    }
}
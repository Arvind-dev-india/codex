//! Configuration for Recovery Services tools

use mcp_types::{Tool, ToolInputSchema};
use serde_json::json;

/// Create the list of available Recovery Services tools
pub fn create_recovery_services_tools() -> Vec<Tool> {
    vec![
        // Vault management
        create_list_vaults_tool(),
        create_test_connection_tool(),
        
        // Discovery tools
        create_refresh_containers_tool(),
        create_list_protectable_containers_tool(),
        create_list_protectable_items_tool(),
        create_list_workload_items_tool(),
        create_inquire_workload_databases_tool(),
        
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
        create_get_job_details_tool(),
        create_cancel_job_tool(),
        
        // Recovery operations
        create_list_recovery_points_tool(),
        create_restore_vm_tool(),
        create_restore_files_tool(),
        
        // Async operation tracking
        create_track_async_operation_tool(),
        
        // Utility tools
        create_clear_auth_cache_tool(),
    ]
}

/// Create the inquire_workload_databases tool definition
fn create_inquire_workload_databases_tool() -> Tool {
    Tool {
        name: "recovery_services_inquire_workload_databases".to_string(),
        description: Some("Discover databases for a specific workload type using the Azure Recovery Services inquire endpoint. You can either provide vm_name and vm_resource_group (recommended) to auto-generate the container name, OR provide the container_name directly. The tool will automatically format the container name as 'VMAppContainer;compute;RESOURCE_GROUP;VM_NAME'. Supported workload types: 'SAPAseDatabase' (SAP ASE databases), 'SAPHanaDatabase' (SAP HANA databases), 'SQLDataBase' (SQL Server databases), 'AnyDatabase' (generic databases), 'VM' (virtual machines), 'AzureFileShare' (file shares), 'Exchange' (Exchange servers), 'Sharepoint' (SharePoint servers). This implements the POST /inquire endpoint with workload type filtering to discover databases.".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "vm_name": {
                    "type": "string",
                    "description": "VM name (e.g., 'aseecyvm1') - used to auto-generate container name. Either provide this with vm_resource_group, OR provide container_name directly."
                },
                "vm_resource_group": {
                    "type": "string", 
                    "description": "VM resource group (e.g., 'ASERG') - used to auto-generate container name. Required when using vm_name."
                },
                "workload_type": {
                    "type": "string",
                    "description": "The workload type to discover databases for",
                    "enum": [
                        "SAPAseDatabase",
                        "SAPHanaDatabase", 
                        "SQLDataBase",
                        "AnyDatabase",
                        "VM",
                        "AzureFileShare",
                        "Exchange",
                        "Sharepoint"
                    ]
                },
                "vault_name": {
                    "type": "string",
                    "description": "Name of the Recovery Services vault (optional, uses default from config if not specified)"
                },
                "container_name": {
                    "type": "string",
                    "description": "Protection container name in format 'VMAppContainer;compute;RESOURCE_GROUP;VM_NAME' (optional, auto-generated from vm_name and vm_resource_group if not provided)"
                }
            })),
            required: Some(vec!["workload_type".to_string()]),
        },
    }
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

/// Create the refresh_containers tool definition
fn create_refresh_containers_tool() -> Tool {
    Tool {
        name: "recovery_services_refresh_containers".to_string(),
        description: Some("Trigger a discovery operation to refresh the list of containers that can be registered to the Recovery Services vault. This is required before listing protectable containers to ensure the vault has the latest list of eligible resources.".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "vault_name": {
                    "type": "string",
                    "description": "Name of the vault (optional, defaults to 'default')"
                },
                "fabric_name": {
                    "type": "string", 
                    "description": "Name of the backup fabric (optional, defaults to 'Azure')"
                }
            })),
            required: Some(vec![]),
        },
    }
}

/// Create the list_protectable_containers tool definition
fn create_list_protectable_containers_tool() -> Tool {
    Tool {
        name: "recovery_services_list_protectable_containers".to_string(),
        description: Some("List containers that can be registered to the Recovery Services vault but are not yet registered. Use this after running refresh_containers to discover VMs, storage accounts, and other resources eligible for backup. Supports filtering by backup management type (AzureIaasVM, AzureWorkload, AzureStorage, MAB).".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "vault_name": {
                    "type": "string",
                    "description": "Name of the vault (optional, defaults to 'default')"
                },
                "fabric_name": {
                    "type": "string",
                    "description": "Name of the backup fabric (optional, defaults to 'Azure')"
                },
                "backup_management_type": {
                    "type": "string",
                    "description": "Filter by backup management type - select the type of resources to discover",
                    "enum": [
                        "AzureIaasVM",
                        "AzureWorkload",
                        "AzureStorage", 
                        "MAB",
                        "AzureSqlDb",
                        "Exchange",
                        "Sharepoint",
                        "VMwareVM",
                        "SystemState",
                        "Client",
                        "GenericDataSource",
                        "AzureFileShare"
                    ]
                }
            })),
            required: Some(vec![]),
        },
    }
}


/// Create the list_workload_items tool definition  
fn create_list_workload_items_tool() -> Tool {
    Tool {
        name: "recovery_services_list_workload_items".to_string(),
        description: Some("List workload items (databases, applications) that are already registered or protected. Shows SQL Server, SAP HANA, SAP ASE databases and other workloads currently managed by the vault.".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "vault_name": {
                    "type": "string",
                    "description": "Name of the vault (optional, defaults to 'default')"
                },
                "workload_type": {
                    "type": "string", 
                    "description": "Filter by workload type",
                    "enum": [
                        "SQL",
                        "SAPHana",
                        "SAPAse", 
                        "Exchange",
                        "Sharepoint"
                    ]
                }
            })),
            required: Some(vec![]),
        },
    }
}

/// Create the register_vm tool definition
fn create_register_vm_tool() -> Tool {
    Tool {
        name: "recovery_services_register_vm".to_string(),
        description: Some("Register a virtual machine for backup with a Recovery Services vault. Automatically constructs VM resource ID from name and resource group. Supports different workload types including standard VM backup and database workloads (SAP HANA, SQL Server, etc.). API Reference: PUT /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.RecoveryServices/vaults/{vaultName}/backupFabrics/Azure/protectionContainers/{containerName}".to_string()),
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
                    "description": "Name of the virtual machine to register for backup"
                },
                "vm_resource_group": {
                    "type": "string",
                    "description": "Resource group containing the virtual machine"
                },
                "workload_type": {
                    "type": "string",
                    "description": "Type of workload to backup. 'VM' for standard Azure VM backup, 'AnyDatabase' for generic database workload, 'SAPHanaDatabase' for SAP HANA, 'SQLDataBase' for SQL Server, 'SAPAseDatabase' for SAP ASE, 'AzureFileShare' for file shares, 'FileFolder' for file/folder backup, 'AzureSqlDb' for Azure SQL databases, 'Exchange' for Exchange Server, 'Sharepoint' for SharePoint, 'VMwareVM' for VMware VMs, 'SystemState' for system state backup, 'Client' for client backup, 'GenericDataSource' for generic data sources. This determines the containerType and backup capabilities.",
                    "enum": ["VM", "AnyDatabase", "SAPHanaDatabase", "SQLDataBase", "SAPAseDatabase", "AzureFileShare", "FileFolder", "AzureSqlDb", "Exchange", "Sharepoint", "VMwareVM", "SystemState", "Client", "GenericDataSource"],
                    "default": "AnyDatabase"
                },
                "backup_management_type": {
                    "type": "string",
                    "description": "Backup management type. 'AzureIaasVM' for standard VM backups, 'AzureWorkload' for database workloads (SAP HANA, SQL Server, etc.), 'AzureStorage' for file shares. If not specified, auto-detected based on workload_type.",
                    "enum": ["AzureIaasVM", "AzureWorkload", "AzureStorage", "AzureSql"]
                }
            })),
            required: Some(vec!["vm_name".to_string(), "vm_resource_group".to_string(), "workload_type".to_string()]),
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
        description: Some("Unregister a VM from backup protection. Automatically detects if the VM is registered for standard backup (AzureIaasVM) or workload backup (AzureWorkload) and uses the appropriate unregistration method. For workload VMs, provide workload_type ('SAPAseDatabase', 'SAPHanaDatabase', 'SQLDataBase', 'AnyDatabase') to ensure correct unregistration. For workload VMs, uses DELETE request with body containing container details. For standard VMs, uses simple DELETE request without body. TOOL RESULT CHAINING: Use vm_name and vm_resource_group from VM registration operations. If you don't know the exact workload_type or want to see what containers are registered, first use recovery_services_list_protectable_containers with backup_management_type='AzureWorkload' to list workload containers (equivalent to: GET /backupProtectionContainers?$filter=providertype eq 'AzureWorkload'), then use the container details and workload types in this tool.".to_string()),
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
                    "description": "Name of the virtual machine to unregister"
                },
                "vm_resource_group": {
                    "type": "string",
                    "description": "Resource group containing the VM"
                },
                "workload_type": {
                    "type": "string",
                    "description": "Type of workload for workload containers (optional, auto-detected if not provided). Use 'SAPAseDatabase' for SAP ASE, 'SAPHanaDatabase' for SAP HANA, 'SQLDataBase' for SQL Server, 'AnyDatabase' for generic databases. If not specified, the tool will attempt to detect the workload type automatically.",
                    "enum": ["SAPAseDatabase", "SAPHanaDatabase", "SQLDataBase", "AnyDatabase", "VM"]
                }
            })),
            required: Some(vec!["vm_name".to_string(), "vm_resource_group".to_string()]),
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
        description: Some("List protectable items (databases, VMs, etc.) that can be backed up. Required parameters: workload_type supports 'SAPAseDatabase', 'SAPHanaDatabase', 'SQLDataBase', 'VM', 'AzureFileShare', etc. backup_management_type should be 'AzureWorkload' for databases, 'AzureIaasVM' for VMs, 'AzureStorage' for file shares. Example: workload_type='SAPAseDatabase' and backup_management_type='AzureWorkload'. You can also use simplified names like 'SAPASE', 'SAPHANA', 'SQL' for workload_type.".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "workload_type": {
                    "type": "string",
                    "description": "Type of workload to list protectable items for. Supports: 'SAPAseDatabase', 'SAPHanaDatabase', 'SQLDataBase', 'VM', 'AzureFileShare', etc. You can also use simplified names like 'SAPASE', 'SAPHANA', 'SQL'."
                },
                "backup_management_type": {
                    "type": "string", 
                    "description": "Backup management type. Use 'AzureWorkload' for databases (SAP, SQL), 'AzureIaasVM' for virtual machines, 'AzureStorage' for file shares."
                },
                "server_name": {
                    "type": "string",
                    "description": "Optional server name to filter results"
                },
                "vault_name": {
                    "type": "string",
                    "description": "Name of the Recovery Services vault"
                }
            })),
            required: Some(vec!["workload_type".to_string(), "backup_management_type".to_string()]),
        },
    }
}

/// Create the enable_protection tool definition
fn create_enable_protection_tool() -> Tool {
    Tool {
        name: "recovery_services_enable_protection".to_string(),
        description: Some("Enable backup protection for workloads (databases) or VMs. For workload backups like SAP ASE: provide item_name (e.g., 'SAPAseDatabase;azu;azu'), policy_name, workload_type ('SAPAse'), backup_management_type ('AzureWorkload'), protected_item_type ('SAPAseDatabase'), friendly_name ('azu'), and either container_name OR vm_name+vm_resource_group to auto-generate container. For VMs: provide vm_name, vm_resource_group, policy_name.".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "item_name": {
                    "type": "string",
                    "description": "Protected item name (e.g., 'SAPAseDatabase;azu;azu' for workloads, or auto-generated for VMs)"
                },
                "policy_name": {
                    "type": "string",
                    "description": "Name of the backup policy to use (e.g., 'DailyFullHourlyLog')"
                },
                "container_name": {
                    "type": "string",
                    "description": "Container name (e.g., 'VMAppContainer;compute;ASERG;aseecyvm1'). Either provide this OR vm_name+vm_resource_group to auto-generate."
                },
                "vm_name": {
                    "type": "string",
                    "description": "VM name (e.g., 'aseecyvm1') - used to auto-generate container name if container_name not provided"
                },
                "vm_resource_group": {
                    "type": "string",
                    "description": "Resource group containing the VM (e.g., 'ASERG') - used with vm_name to auto-generate container"
                },
                "workload_type": {
                    "type": "string",
                    "description": "Workload type for workload backups: 'SAPAse', 'SAPHana', 'SQL', etc. For VMs, use 'VM'."
                },
                "backup_management_type": {
                    "type": "string",
                    "description": "Backup management type: 'AzureWorkload' for databases, 'AzureIaasVM' for VMs"
                },
                "protected_item_type": {
                    "type": "string",
                    "description": "Protected item type: 'SAPAseDatabase', 'SAPHanaDatabase', 'SQLDataBase', 'Microsoft.Compute/virtualMachines'"
                },
                "friendly_name": {
                    "type": "string",
                    "description": "Friendly name for the protected item (e.g., database name like 'azu')"
                },
                "vault_name": {
                    "type": "string",
                    "description": "Name of the Recovery Services vault"
                }
            })),
            required: Some(vec!["item_name".to_string(), "policy_name".to_string()]),
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
        description: Some("Trigger an ad-hoc backup for protected workloads (databases) or VMs. For workload backups: provide item_name (e.g., 'SAPAseDatabase;azu;azu'), backup_type ('Full', 'Incremental', 'Log'), object_type ('AzureWorkloadBackupRequest'), and either container_name OR vm_name+vm_resource_group. For VMs: provide vm_name, vm_resource_group, object_type ('IaasVMBackupRequest').".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "item_name": {
                    "type": "string",
                    "description": "Protected item name (e.g., 'SAPAseDatabase;azu;azu' for workloads, or auto-generated for VMs)"
                },
                "container_name": {
                    "type": "string",
                    "description": "Container name (e.g., 'VMAppContainer;compute;ASERG;aseecyvm1'). Either provide this OR vm_name+vm_resource_group to auto-generate."
                },
                "vm_name": {
                    "type": "string",
                    "description": "VM name (e.g., 'aseecyvm1') - used to auto-generate container name if container_name not provided"
                },
                "vm_resource_group": {
                    "type": "string",
                    "description": "Resource group containing the VM (e.g., 'ASERG') - used with vm_name to auto-generate container"
                },
                "backup_type": {
                    "type": "string",
                    "description": "Type of backup: 'Full', 'Incremental', 'Log' for workloads; 'Full' for VMs",
                    "enum": ["Full", "Incremental", "Log"]
                },
                "object_type": {
                    "type": "string",
                    "description": "Request object type: 'AzureWorkloadBackupRequest' for databases, 'IaasVMBackupRequest' for VMs",
                    "enum": ["AzureWorkloadBackupRequest", "IaasVMBackupRequest"]
                },
                "enable_compression": {
                    "type": "boolean",
                    "description": "Enable compression for workload backups (optional, default: true)",
                    "default": true
                },
                "recovery_point_expiry_time": {
                    "type": "string",
                    "description": "Expiry time in UTC format (e.g., '2019-02-28T18:29:59.000Z'). If not provided, calculated from retention_days."
                },
                "retention_days": {
                    "type": "integer",
                    "description": "Number of days to retain this backup (default: 30)",
                    "minimum": 1,
                    "maximum": 99,
                    "default": 30
                },
                "vault_name": {
                    "type": "string",
                    "description": "Name of the Recovery Services vault"
                }
            })),
            required: Some(vec!["item_name".to_string(), "backup_type".to_string()]),
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
        description: Some("List available recovery points for protected VMs or workloads (databases). For workload backups: provide container_name and item_name (e.g., container_name='VMAppContainer;compute;ASERG;aseecyvm1', item_name='SAPAseDatabase;azu;azu'). For VM backups: provide vm_name and vm_resource_group. Optional: backup_management_type ('AzureWorkload' for databases, 'AzureIaasVM' for VMs), start_date/end_date in format '2019-01-01 05:23:52 AM', or time_range_days for recent points.".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "vault_name": {
                    "type": "string",
                    "description": "Name of the Recovery Services vault"
                },
                "container_name": {
                    "type": "string",
                    "description": "Container name for workload backups (e.g., 'VMAppContainer;compute;ASERG;aseecyvm1'). Either provide this with item_name OR vm_name+vm_resource_group."
                },
                "item_name": {
                    "type": "string",
                    "description": "Protected item name for workload backups (e.g., 'SAPAseDatabase;azu;azu'). Required when using container_name."
                },
                "vm_name": {
                    "type": "string",
                    "description": "VM name for VM backups (e.g., 'aseecyvm1'). Either provide this with vm_resource_group OR container_name+item_name."
                },
                "vm_resource_group": {
                    "type": "string",
                    "description": "Resource group containing the VM for VM backups (e.g., 'ASERG'). Required when using vm_name."
                },
                "backup_management_type": {
                    "type": "string",
                    "description": "Backup management type: 'AzureWorkload' for databases, 'AzureIaasVM' for VMs. Auto-detected if not provided.",
                    "enum": ["AzureWorkload", "AzureIaasVM", "AzureStorage"]
                },
                "start_date": {
                    "type": "string",
                    "description": "Start date for recovery points in format '2019-01-01 05:23:52 AM'. If not provided, calculated from time_range_days."
                },
                "end_date": {
                    "type": "string",
                    "description": "End date for recovery points in format '2019-02-07 05:23:52 AM'. If not provided, uses current time."
                },
                "time_range_days": {
                    "type": "integer",
                    "description": "Number of days to look back for recovery points (default: 30)",
                    "minimum": 1,
                    "maximum": 365,
                    "default": 30
                }
            })),
            required: Some(vec![]),
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

/// Create the track_async_operation tool definition
fn create_track_async_operation_tool() -> Tool {
    Tool {
        name: "recovery_services_track_async_operation".to_string(),
        description: Some("Track the status of an asynchronous Recovery Services operation using the location URL returned from async operations".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "location_url": {
                    "type": "string",
                    "description": "The location URL from an async operation response header"
                },
                "wait_for_completion": {
                    "type": "boolean",
                    "description": "Whether to wait for the operation to complete (default: false)"
                },
                "max_wait_seconds": {
                    "type": "number",
                    "description": "Maximum time to wait for completion in seconds (default: 300)"
                }
            })),
            required: Some(vec!["location_url".to_string()]),
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
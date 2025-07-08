# Recovery Services Workload Schema - Complete Implementation

## Overview

This document provides comprehensive documentation for Azure Recovery Services workload backup tools, ensuring they follow the proper schema as defined in the SAP ASE API documentation and support all workload types (SAP ASE, SAP HANA, SQL Server, AnyDatabase).

## Workload Schema Compliance

### 1. Container Naming Convention
All workload containers follow the Azure standard:
```
VMAppContainer;compute;{resourceGroup};{vmName}
```

### 2. Protected Item Naming Convention
Protected items follow workload-specific patterns:
```
{workloadType};{server};{database}
```

Examples:
- SAP ASE: `SAPAseDatabase;azu;azu`
- SAP HANA: `SAPHanaDatabase;hanaserver;HXE`
- SQL Server: `SQLDataBase;sqlserver;MyDatabase`

### 3. Supported Workload Types
- `SAPAseDatabase` - SAP ASE databases
- `SAPHanaDatabase` - SAP HANA databases  
- `SAPHanaDBInstance` - SAP HANA DB instances
- `SQLDataBase` - SQL Server databases
- `AnyDatabase` - Generic database workloads
- `VM` - Virtual machines
- `AzureFileShare` - Azure file shares
- `Exchange` - Exchange servers
- `Sharepoint` - SharePoint servers
- `VMwareVM` - VMware virtual machines
- `SystemState` - System state backups
- `Client` - Client backups
- `GenericDataSource` - Generic data sources

## Complete Workflow Documentation

### Phase 1: VM Registration and Discovery

#### Step 1: Register VM for Workload Backup
```bash
recovery_services_register_vm(
  vm_name: "aseecyvm1",
  vm_resource_group: "ASERG", 
  workload_type: "SAPAseDatabase",
  backup_management_type: "AzureWorkload"
)
```

**Tool Result Usage:**
- Returns `container_name` for use in subsequent operations
- Returns `registration_status` to confirm success

#### Step 2: Trigger Database Discovery
```bash
recovery_services_inquire_workload_databases(
  vm_name: "aseecyvm1",
  vm_resource_group: "ASERG",
  workload_type: "SAPAseDatabase"
)
```

**Tool Result Usage:**
- Returns `container_name`: Use in protection operations
- Returns `async_operation.location_header`: Use for tracking
- After completion, proceed to list protectable items

#### Step 3: List Discovered Databases
```bash
recovery_services_list_protectable_items(
  workload_type: "SAPAseDatabase",
  backup_management_type: "AzureWorkload"
)
```

**Tool Result Usage:**
- Returns array of `protectable_items` with database names
- Each item contains `name` field for protection configuration
- Use `parentName` for container reference

### Phase 2: Policy Creation and Protection

#### Step 4: Create Backup Policy
```bash
recovery_services_create_workload_policy(
  policy_name: "DailyFullHourlyLog",
  workload_type: "SAPAseDatabase",
  policy_config: {
    "full_backup": {
      "schedule": "daily",
      "time": "16:00:00Z",
      "retention_days": 30
    },
    "log_backup": {
      "frequency_minutes": 60,
      "retention_days": 30
    }
  }
)
```

**Tool Result Usage:**
- Returns `policy_id` for protection configuration
- Returns `policy_name` for reference in protection

#### Step 5: Enable Database Protection
```bash
recovery_services_enable_protection(
  item_name: "SAPAseDatabase;azu;azu",
  policy_name: "DailyFullHourlyLog",
  workload_type: "SAPAse",
  backup_management_type: "AzureWorkload",
  protected_item_type: "SAPAseDatabase",
  friendly_name: "azu",
  container_name: "VMAppContainer;compute;ASERG;aseecyvm1"
)
```

**Tool Result Usage:**
- Returns `protected_item_id` for backup operations
- Returns `protection_status` to confirm enablement

### Phase 3: Backup Operations

#### Step 6: Trigger Backup
```bash
recovery_services_trigger_backup(
  item_name: "SAPAseDatabase;azu;azu",
  container_name: "VMAppContainer;compute;ASERG;aseecyvm1",
  backup_type: "Full",
  object_type: "AzureWorkloadBackupRequest",
  enable_compression: true,
  retention_days: 30
)
```

**Tool Result Usage:**
- Returns `job_id` for monitoring
- Returns `async_operation.location_header` for tracking

#### Step 7: Monitor Backup Job
```bash
recovery_services_get_job_status(
  job_id: "{job_id_from_trigger_backup}"
)
```

### Phase 4: Restore Operations

#### Step 8: List Recovery Points
```bash
recovery_services_list_recovery_points(
  container_name: "VMAppContainer;compute;ASERG;aseecyvm1",
  item_name: "SAPAseDatabase;azu;azu",
  time_range_days: 30
)
```

**Tool Result Usage:**
- Returns array of `recovery_points` with IDs
- Use `recovery_point_id` for restore operations

#### Step 9a: Restore to Original Location
```bash
recovery_services_restore_database_original(
  item_name: "SAPAseDatabase;azu;azu",
  server_name: "aseecyvm1",
  recovery_point_id: "82592972300173",
  container_name: "VMAppContainer;compute;ASERG;aseecyvm1"
)
```

#### Step 9b: Restore to Alternate Location
```bash
recovery_services_restore_database_alternate(
  item_name: "SAPAseDatabase;azu;azu",
  server_name: "aseecyvm1",
  recovery_point_id: "82592972300173",
  target_server: "aseecyvm2",
  target_database: "AZU_RestoreTest",
  container_name: "VMAppContainer;compute;ASERG;aseecyvm1"
)
```

#### Step 9c: Restore as Files
```bash
recovery_services_restore_as_files(
  item_name: "SAPAseDatabase;azu;azu",
  server_name: "aseecyvm1", 
  recovery_point_id: "82592972300173",
  target_container: "VMAppContainer;compute;ASERG;aseecyvm2",
  file_path: "/backup/restore/"
)
```

## Tool Result Chaining Patterns

### Pattern 1: Container Name Propagation
```
register_vm → container_name → inquire_workload → container_name → enable_protection
```

### Pattern 2: Async Operation Tracking
```
inquire_workload → async_operation.location_header → track_async_operation → status
```

### Pattern 3: Discovery to Protection
```
inquire_workload → list_protectable_items → protectable_items[].name → enable_protection
```

### Pattern 4: Backup to Restore
```
trigger_backup → job_id → list_recovery_points → recovery_points[].id → restore_*
```

## API Schema Compliance

### Request Body Schemas

#### Trigger Inquiry (SAP ASE)
```json
{
  "method": "POST",
  "endpoint": "/backupFabrics/Azure/protectionContainers/{container}/inquire",
  "query": "?api-version=2018-01-10&$filter=workloadType eq 'SAPAseDatabase'"
}
```

#### Configure Protection (SAP ASE)
```json
{
  "properties": {
    "backupManagementType": "AzureWorkload",
    "workloadType": "SAPAse",
    "protectedItemType": "SAPAseDatabase",
    "friendlyName": "azu",
    "policyId": "/subscriptions/{sub}/resourceGroups/{rg}/providers/Microsoft.RecoveryServices/vaults/{vault}/backupPolicies/{policy}"
  }
}
```

#### Trigger Backup (SAP ASE)
```json
{
  "properties": {
    "objectType": "AzureWorkloadBackupRequest",
    "backupType": "Full",
    "enableCompression": true,
    "recoveryPointExpiryTimeInUTC": "2019-02-28T18:29:59.000Z"
  }
}
```

#### Restore Original Location (SAP ASE)
```json
{
  "properties": {
    "objectType": "AzureWorkloadRestoreRequest",
    "targetInfo": {
      "containerId": "/subscriptions/{sub}/resourceGroups/{rg}/providers/Microsoft.RecoveryServices/vaults/{vault}/backupFabrics/Azure/protectionContainers/{container}",
      "databaseName": "azu/azu",
      "overwriteOption": "Overwrite"
    },
    "RecoveryType": "OriginalLocation",
    "SourceResourceId": "/subscriptions/{sub}/resourcegroups/{rg}/providers/VMAppContainer/{vm}"
  }
}
```

## Error Handling and Validation

### Common Validation Rules
1. **Container Name Format**: Must match `VMAppContainer;compute;{rg};{vm}`
2. **Item Name Format**: Must match `{workloadType};{server};{database}`
3. **Workload Type**: Must be from supported list
4. **API Version**: Use `2018-01-10` for workload operations

### Error Response Handling
```json
{
  "error": {
    "code": "InvalidParameter",
    "message": "Workload type not supported",
    "details": "Supported types: SAPAseDatabase, SAPHanaDatabase, SQLDataBase"
  }
}
```

## Best Practices

### 1. Tool Chaining
- Always use exact values returned from previous tools
- Check async operation status before proceeding
- Validate container and item names before operations

### 2. Naming Conventions
- Use consistent casing for workload types
- Follow Azure resource naming standards
- Include resource group and subscription context

### 3. Error Recovery
- Implement retry logic for async operations
- Provide clear error messages with next steps
- Include API reference information in responses

## Conclusion

This implementation ensures all Recovery Services workload tools follow the proper Azure API schema as demonstrated in the SAP ASE documentation. The tools support seamless chaining of operations and provide comprehensive documentation for users to understand how to use tool results in subsequent operations.
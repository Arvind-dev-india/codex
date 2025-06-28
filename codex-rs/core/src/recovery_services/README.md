# Recovery Services Agent Tool

The Recovery Services Agent Tool provides comprehensive backup and recovery management for Azure Recovery Services vaults, supporting SAP HANA and SQL Server workloads on Azure VMs.

## Overview

This tool enables you to manage Azure Backup operations for database workloads through a unified interface, supporting the complete backup lifecycle from VM registration to database restoration.

## Supported Workloads

- **SAP HANA** - Complete backup and recovery for SAP HANA databases
- **SQL Server** - Full backup and recovery for SQL Server databases
- Both workloads use the `AzureWorkload` backup management type

## Features

### Vault Management
- List Recovery Services vaults
- Get vault properties and configuration
- Manage vault settings

### VM Registration
- Register VMs for backup protection
- Re-register VMs (useful for troubleshooting)
- Unregister VMs from backup protection
- Check registration status

### Policy Management
- Create backup policies for SAP HANA and SQL Server
- List existing backup policies
- Manage policy configurations

### Protection Management
- List protectable items (databases)
- Enable protection for databases
- Disable protection (with option to delete backup data)
- List currently protected items

### Backup Operations
- Trigger ad-hoc backups (Full/Incremental/Log)
- Monitor backup job status
- List backup jobs with filtering
- Get detailed job information

### Recovery Operations
- List available recovery points
- Restore to original location
- Restore to alternate location
- Restore as files to specified path
- Point-in-time recovery support

## Configuration

Add the following to your `config.toml`:

```toml
[recovery_services]
# Enable/disable the Recovery Services tool (default: true)
enabled = true

# Primary vault configuration
subscription_id = "your-subscription-id"
resource_group = "your-resource-group"
vault_name = "your-vault-name"

# Optional: Multiple vaults support
[recovery_services.vaults.production]
subscription_id = "prod-subscription-id"
resource_group = "prod-resource-group"
vault_name = "prod-vault-name"
description = "Production backup vault"

[recovery_services.vaults.staging]
subscription_id = "staging-subscription-id"
resource_group = "staging-resource-group"
vault_name = "staging-vault-name"
description = "Staging backup vault"
```

## Authentication

The tool uses OAuth 2.0 device code flow with Azure Management API scope:
- **Scope**: `https://management.azure.com/.default`
- **Token storage**: `~/.codex/recovery_services_auth.json`
- **Auto-refresh**: Tokens are automatically refreshed when needed

## Required Permissions

Your Azure account needs the following RBAC roles:
- **Backup Contributor** - For backup operations
- **Backup Operator** - For restore operations
- **Virtual Machine Contributor** - For VM registration

## Available Tools

### Vault Management
- `recovery_services_list_vaults` - List accessible Recovery Services vaults
- `recovery_services_test_connection` - Test connectivity and permissions

### VM Registration
- `recovery_services_register_vm` - Register VM for backup
- `recovery_services_reregister_vm` - Re-register VM (troubleshooting)
- `recovery_services_unregister_vm` - Unregister VM from backup
- `recovery_services_check_registration_status` - Check VM registration status

### Policy Management
- `recovery_services_create_policy` - Create backup policy
- `recovery_services_list_policies` - List backup policies
- `recovery_services_get_policy_details` - Get policy configuration

### Protection Management
- `recovery_services_list_protectable_items` - List databases available for protection
- `recovery_services_enable_protection` - Enable backup protection for database
- `recovery_services_disable_protection` - Disable protection (optionally delete data)
- `recovery_services_list_protected_items` - List currently protected databases

### Backup Operations
- `recovery_services_trigger_backup` - Trigger ad-hoc backup
- `recovery_services_list_backup_jobs` - List backup jobs
- `recovery_services_get_job_status` - Get specific job status
- `recovery_services_get_backup_summary` - Get backup status summary

### Recovery Operations
- `recovery_services_list_recovery_points` - List available recovery points
- `recovery_services_restore_original_location` - Restore to original location
- `recovery_services_restore_alternate_location` - Restore to different server
- `recovery_services_restore_as_files` - Restore as files to specified path

### Utility Tools
- `recovery_services_clear_auth_cache` - Clear authentication cache

## Usage Examples

### Basic Operations
```bash
# List available vaults
codex "list recovery services vaults"

# Register VM for backup
codex "register VM saphana-vm1 for backup in vault my-vault"

# Create backup policy
codex "create SAP HANA backup policy with daily full backups"

# Enable protection
codex "enable protection for SAP HANA database HXE on server saphana-vm1"

# Trigger backup
codex "trigger full backup for database HXE"

# Check backup status
codex "show backup job status for last 24 hours"
```

### Advanced Operations
```bash
# Restore operations
codex "list recovery points for database HXE on server saphana-vm1"
codex "restore database HXE to original location using latest recovery point"
codex "restore database HXE to alternate server saphana-vm2"
codex "restore database HXE as files to /hana/restore/"

# Bulk operations
codex "list all protected databases in vault my-vault"
codex "disable protection for all databases on server old-vm and delete backup data"
```

### Troubleshooting
```bash
# Test connectivity
codex "test recovery services connection"

# Re-register problematic VM
codex "reregister VM saphana-vm1 in vault my-vault"

# Clear authentication cache
codex "clear recovery services auth cache"
```

## Error Handling

The tool provides detailed error messages for common issues:

- **Authentication failures** - Clear guidance on token refresh
- **Permission errors** - Specific RBAC role requirements
- **Vault access issues** - Subscription and resource group validation
- **VM registration problems** - Step-by-step troubleshooting
- **Backup job failures** - Detailed error analysis

## API Reference

The tool uses Azure Management REST APIs:
- **Base URL**: `https://management.azure.com`
- **API Version**: `2021-12-01` (Recovery Services)
- **Authentication**: Bearer token (OAuth 2.0)

### Key Endpoints
- Vaults: `/subscriptions/{sub}/resourceGroups/{rg}/providers/Microsoft.RecoveryServices/vaults`
- VM Registration: `/subscriptions/{sub}/resourceGroups/{rg}/providers/Microsoft.RecoveryServices/vaults/{vault}/backupFabrics/Azure/protectionContainers`
- Protected Items: `/subscriptions/{sub}/resourceGroups/{rg}/providers/Microsoft.RecoveryServices/vaults/{vault}/backupProtectedItems`
- Backup Jobs: `/subscriptions/{sub}/resourceGroups/{rg}/providers/Microsoft.RecoveryServices/vaults/{vault}/backupJobs`
- Recovery Points: `/subscriptions/{sub}/resourceGroups/{rg}/providers/Microsoft.RecoveryServices/vaults/{vault}/backupProtectedItems/{item}/recoveryPoints`

## Limitations

- Supports only SAP HANA and SQL Server workloads currently
- Requires Azure VMs (not Azure Database services)
- Limited to AzureWorkload backup management type
- Cross-region restore may have additional requirements

## Troubleshooting

### Common Issues

1. **Authentication Errors**
   ```bash
   codex "clear recovery services auth cache"
   ```

2. **Permission Denied**
   - Verify RBAC roles (Backup Contributor, Backup Operator)
   - Check subscription access

3. **VM Registration Failures**
   ```bash
   codex "reregister VM problematic-vm in vault my-vault"
   ```

4. **Backup Job Failures**
   ```bash
   codex "get job status for job-id xyz123"
   ```

### Debug Mode

Enable detailed logging by setting environment variable:
```bash
export RUST_LOG=debug
```

## Contributing

When adding new features:
1. Update this README
2. Add appropriate error handling
3. Include usage examples
4. Update configuration schema
5. Add integration tests

## Related Documentation

- [Azure Backup REST API](https://docs.microsoft.com/en-us/rest/api/backup/)
- [SAP HANA Backup Guide](https://docs.microsoft.com/en-us/azure/backup/sap-hana-backup-guide)
- [SQL Server Backup Guide](https://docs.microsoft.com/en-us/azure/backup/sql-support-matrix)
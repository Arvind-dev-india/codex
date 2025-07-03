# Recovery Services MCP Server Quick Start Guide

This guide will help you get started with the Recovery Services MCP Server, which provides access to Azure Recovery Services (Azure Backup) functionality through the Model Context Protocol (MCP).

## Prerequisites

- Rust toolchain (rustc, cargo)
- Azure subscription with Recovery Services vault
- Appropriate Azure permissions for backup operations
- Basic understanding of Azure Backup concepts

## Installation

### Building from Source

1. Clone the repository:
   ```bash
   git clone https://github.com/your-org/codex.git
   cd codex
   ```

2. Build the Recovery Services MCP Server:
   ```bash
   cd codex-rs/recovery-services-server
   ./build.sh
   ```

3. The binary will be available at `bin/recovery-services-server`

## Configuration

The server uses the same configuration as the main Codex application. Create a configuration file with your Recovery Services settings:

```toml
[recovery_services]
enabled = true
subscription_id = "your-subscription-id"
resource_group = "your-resource-group"
vault_name = "your-vault-name"
```

## Running the Server

### Basic Usage

```bash
./bin/recovery-services-server
```

### With Custom Configuration

```bash
./bin/recovery-services-server --config path/to/your/config.toml
```

### With Verbose Logging

```bash
./bin/recovery-services-server --verbose
```

## Testing the Server

Run the included test script to verify the server is working correctly:

```bash
./test-server.sh
```

## Using with MCP Clients

### Claude Desktop

Add the following to your Claude Desktop configuration:

```json
{
  "mcpServers": {
    "recovery-services": {
      "command": "/path/to/recovery-services-server",
      "args": ["--config", "/path/to/config.toml"]
    }
  }
}
```

### Other MCP Clients

Configure your MCP client to use the Recovery Services MCP Server as an external tool provider. The server communicates via stdin/stdout using the MCP protocol.

## Available Tools

The server provides comprehensive tools for Azure Backup operations:

### Vault Management
- `recovery_services_list_vaults` - List Recovery Services vaults
- `recovery_services_test_connection` - Test connection and authentication

### VM Registration
- `recovery_services_register_vm` - Register a VM for backup
- `recovery_services_check_registration_status` - Check VM registration status

### Policy Management
- `recovery_services_create_policy` - Create backup policies
- `recovery_services_list_policies` - List backup policies

### Protection Management
- `recovery_services_enable_protection` - Enable VM backup protection
- `recovery_services_list_protected_items` - List protected items

### Backup Operations
- `recovery_services_trigger_backup` - Trigger on-demand backup
- `recovery_services_list_backup_jobs` - Monitor backup jobs

### Recovery Operations
- `recovery_services_list_recovery_points` - List recovery points
- `recovery_services_restore_vm` - Restore VMs from backup

## Example Usage

Once connected to an MCP client, you can use the Recovery Services tools like this:

1. **List vaults**:
   ```
   recovery_services_list_vaults()
   ```

2. **Register a VM for backup**:
   ```
   recovery_services_register_vm(
     vault_name: "my-vault",
     vm_name: "my-vm",
     vm_resource_group: "my-rg"
   )
   ```

3. **Enable protection**:
   ```
   recovery_services_enable_protection(
     vault_name: "my-vault",
     vm_name: "my-vm",
     vm_resource_group: "my-rg",
     policy_name: "daily-backup"
   )
   ```

4. **Trigger backup**:
   ```
   recovery_services_trigger_backup(
     vault_name: "my-vault",
     vm_name: "my-vm",
     vm_resource_group: "my-rg"
   )
   ```

5. **List backup jobs**:
   ```
   recovery_services_list_backup_jobs(vault_name: "my-vault")
   ```

## Troubleshooting

If you encounter issues:

1. **Authentication errors**: Use `recovery_services_clear_auth_cache` to force re-authentication
2. **Permission issues**: Verify you have appropriate Azure permissions for backup operations
3. **Configuration problems**: Check your config file format and Azure resource names
4. **Server crashes**: Run with `--verbose` flag to see detailed logs

## Next Steps

- Explore the [Azure Backup documentation](https://docs.microsoft.com/en-us/azure/backup/)
- Learn about [Recovery Services vaults](https://docs.microsoft.com/en-us/azure/backup/backup-azure-recovery-services-vault-overview)
- Check the [full documentation](../README.md) for advanced features
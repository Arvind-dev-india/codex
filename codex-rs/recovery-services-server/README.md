# Recovery Services MCP Server

A standalone MCP (Model Context Protocol) server for Azure Recovery Services (Azure Backup) operations.

## Overview

This server provides MCP-compatible access to Azure Recovery Services functionality, allowing AI assistants and other MCP clients to:

- Manage Recovery Services vaults
- Register and unregister virtual machines for backup
- Create and manage backup policies
- Enable and disable protection for VMs
- Trigger on-demand backups
- Monitor backup jobs and their status
- Restore VMs and files from recovery points
- Manage authentication and troubleshoot issues

## Features

- **Vault Management**: List and manage Recovery Services vaults
- **VM Registration**: Register VMs for backup protection
- **Policy Management**: Create and manage backup policies
- **Protection Control**: Enable/disable backup protection for VMs
- **Backup Operations**: Trigger backups and monitor job status
- **Recovery Operations**: Restore VMs and files from backup points
- **Authentication**: Uses OAuth authentication compatible with Azure
- **Error Handling**: Comprehensive error reporting and diagnostics

## Installation

Build the server from the codex-rs workspace:

```bash
cd codex-rs
cargo build --release --bin recovery-services-server
```

The binary will be available at `target/release/recovery-services-server`.

## Configuration

The server uses the same configuration as the main Codex application. It will look for configuration in the following order:

1. File specified with `--config` flag
2. Main Codex config file (`~/.codex/config.toml`)
3. Standalone config files in common locations:
   - `recovery_services_config.toml`
   - `config/recovery_services.toml`
   - `~/.config/codex/recovery_services.toml`
4. Environment variables (`AZURE_SUBSCRIPTION_ID`, `AZURE_RESOURCE_GROUP`)

### Example Configuration

Add to your `~/.codex/config.toml`:

```toml
[recovery_services]
subscription_id = "your-subscription-id"
resource_group = "your-resource-group"
vault_name = "your-vault-name"  # Optional, can be specified per operation
location = "East US"
```

## Usage

### As MCP Server

Run the server in MCP mode (communicates via stdin/stdout):

```bash
recovery-services-server
```

With custom config:

```bash
recovery-services-server --config /path/to/config.toml
```

With verbose logging:

```bash
recovery-services-server --verbose
```

### Available Tools

The server provides the following MCP tools:

#### Vault Management
- `recovery_services_list_vaults` - List Recovery Services vaults
- `recovery_services_test_connection` - Test connection and authentication

#### VM Registration
- `recovery_services_register_vm` - Register a VM for backup
- `recovery_services_reregister_vm` - Re-register a VM after changes
- `recovery_services_unregister_vm` - Unregister a VM from backup
- `recovery_services_check_registration_status` - Check VM registration status

#### Policy Management
- `recovery_services_create_policy` - Create a new backup policy
- `recovery_services_list_policies` - List backup policies
- `recovery_services_get_policy_details` - Get policy details

#### Protection Management
- `recovery_services_list_protectable_items` - List items that can be protected
- `recovery_services_enable_protection` - Enable backup protection for a VM
- `recovery_services_disable_protection` - Disable backup protection
- `recovery_services_list_protected_items` - List currently protected items

#### Backup Operations
- `recovery_services_trigger_backup` - Trigger an on-demand backup
- `recovery_services_list_backup_jobs` - List backup jobs
- `recovery_services_get_job_details` - Get job details
- `recovery_services_cancel_job` - Cancel a running job

#### Recovery Operations
- `recovery_services_list_recovery_points` - List available recovery points
- `recovery_services_restore_vm` - Restore a VM from backup
- `recovery_services_restore_files` - Restore specific files

#### Utility Tools
- `recovery_services_clear_auth_cache` - Clear authentication cache

### Integration with MCP Clients

Configure your MCP client to use this server. For example, with Claude Desktop:

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

## Authentication

The server uses OAuth authentication compatible with Azure Management APIs. Authentication tokens are stored in `~/.codex/recovery_services_auth.json` and are automatically refreshed as needed.

If you encounter authentication issues:

1. Use the `recovery_services_clear_auth_cache` tool to force re-authentication
2. Check that your configuration includes the correct subscription ID and resource group
3. Ensure you have appropriate permissions for Azure Backup operations

## Development

### Building

```bash
cargo build --bin recovery-services-server
```

### Testing

```bash
cargo test --bin recovery-services-server
```

### Adding New Tools

1. Add the tool definition to `src/tool_config.rs`
2. Update the tool handler in the core Recovery Services module
3. Update this README with the new tool information

## Troubleshooting

### Common Issues

1. **Configuration not found**: Ensure config file exists and contains `[recovery_services]` section
2. **Authentication errors**: Use `recovery_services_clear_auth_cache` tool or check Azure permissions
3. **Subscription access**: Verify you have access to the specified subscription and resource group
4. **Vault permissions**: Ensure you have appropriate permissions for the Recovery Services vault

### Logging

Enable verbose logging for debugging:

```bash
recovery-services-server --verbose
```

Or set the environment variable:

```bash
RUST_LOG=recovery_services_server=debug recovery-services-server
```

## License

This project is licensed under the same terms as the main Codex project.
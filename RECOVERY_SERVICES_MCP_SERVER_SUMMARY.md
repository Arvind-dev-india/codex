# Recovery Services MCP Server Implementation

## Overview

The Recovery Services MCP Server is a standalone server that implements the Model Context Protocol (MCP) for Azure Recovery Services (Azure Backup) operations. It follows the same pattern as the existing Azure DevOps Server and Kusto Server, providing a consistent interface for AI assistants to interact with Azure Backup services.

## Features

- **MCP Protocol Support**: Implements the standard MCP protocol for tool discovery and execution
- **Vault Management**: List and manage Recovery Services vaults
- **VM Registration**: Register and unregister virtual machines for backup
- **Policy Management**: Create and manage backup policies
- **Protection Control**: Enable/disable backup protection for VMs
- **Backup Operations**: Trigger backups and monitor job status
- **Recovery Operations**: Restore VMs and files from backup points
- **Authentication**: Uses OAuth authentication compatible with Azure Management APIs
- **Configuration Sharing**: Uses the same configuration as the Codex CLI

## Implementation Details

The Recovery Services MCP Server consists of the following components:

1. **Main Server**: Handles MCP protocol communication via stdin/stdout
2. **Message Processor**: Processes MCP requests and dispatches to appropriate handlers
3. **Tool Configuration**: Defines the available Recovery Services tools and their schemas
4. **Recovery Services Bridge**: Connects to the existing Recovery Services functionality in codex-core

### Directory Structure

```
codex-rs/recovery-services-server/
├── Cargo.toml              # Package configuration
├── build.sh                # Build script
├── test-server.sh          # Test script
├── README.md               # Documentation
└── src/
    ├── main.rs             # Entry point and CLI argument handling
    ├── server.rs           # MCP server implementation
    ├── recovery_services_bridge.rs # Bridge to codex-core Recovery Services functionality
    ├── tool_config.rs      # Tool definitions
    └── server/
        └── message_processor.rs  # MCP message processing
```

### Available Tools

The server provides 23 tools for interacting with Azure Recovery Services:

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

## Configuration

The server uses the same configuration as the main Codex application. It will look for configuration in the following order:

1. File specified with `--config` flag
2. Main Codex config file (`~/.codex/config.toml`)
3. Standalone config files in common locations:
   - `recovery_services_config.toml`
   - `config/recovery_services.toml`
   - `~/.config/codex/recovery_services.toml`
4. Environment variables (`AZURE_SUBSCRIPTION_ID`, `AZURE_RESOURCE_GROUP`, `AZURE_VAULT_NAME`)

### Example Configuration

```toml
[recovery_services]
enabled = true
subscription_id = "your-subscription-id"
resource_group = "your-resource-group"
vault_name = "your-vault-name"

# Multiple vaults (optional)
[recovery_services.vaults.production]
name = "production-vault"
subscription_id = "prod-subscription-id"
resource_group = "prod-resource-group"
description = "Production backup vault"
is_default = true

[recovery_services.vaults.staging]
name = "staging-vault"
subscription_id = "staging-subscription-id"
resource_group = "staging-resource-group"
description = "Staging backup vault"
is_default = false
```

## Usage

### Building

```bash
cd codex-rs/recovery-services-server
./build.sh
```

### Running

```bash
# Use default config
./bin/recovery-services-server

# Use specific config
./bin/recovery-services-server --config path/to/config.toml

# Enable verbose logging
./bin/recovery-services-server --verbose
```

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

## Testing

A test script is provided to verify the server's functionality:

```bash
./test-server.sh
```

## Benefits

1. **Consistent Interface**: Follows the same pattern as other MCP servers
2. **Reuse of Existing Code**: Leverages the existing Recovery Services functionality in codex-core
3. **Configuration Sharing**: Uses the same configuration as the main application
4. **Authentication Integration**: Shares authentication patterns with other Azure services
5. **Comprehensive Tools**: Provides a complete set of tools for Azure Backup operations

## Next Steps

1. **Testing**: Create comprehensive tests for the Recovery Services MCP Server
2. **Documentation**: Add detailed usage examples and API documentation
3. **Integration**: Integrate with more MCP clients and AI assistants
4. **Features**: Add support for more advanced Recovery Services features like cross-region restore

## Conclusion

The Recovery Services MCP Server provides a robust and consistent interface for interacting with Azure Backup services through the MCP protocol. By following the same patterns as the existing servers, it integrates seamlessly with the Codex ecosystem while providing powerful capabilities for backup and recovery operations.
# Kusto MCP Server Implementation

## Overview

The Kusto MCP Server is a standalone server that implements the Model Context Protocol (MCP) for Azure Data Explorer (Kusto) operations. It follows the same pattern as the existing Azure DevOps Server and Code Analysis Server, providing a consistent interface for AI assistants to interact with Kusto databases.

## Features

- **MCP Protocol Support**: Implements the standard MCP protocol for tool discovery and execution
- **Kusto Query Execution**: Run KQL queries against Azure Data Explorer clusters
- **Schema Discovery**: Explore database structures, table schemas, and available functions
- **Knowledge Base Management**: Maintain and search a knowledge base of table descriptions and query patterns
- **Multi-Database Support**: Work with multiple databases across different clusters
- **Authentication**: Uses the same OAuth authentication as the main Codex application
- **Configuration Sharing**: Uses the same configuration as the Codex CLI

## Implementation Details

The Kusto MCP Server consists of the following components:

1. **Main Server**: Handles MCP protocol communication via stdin/stdout
2. **Message Processor**: Processes MCP requests and dispatches to appropriate handlers
3. **Tool Configuration**: Defines the available Kusto tools and their schemas
4. **Kusto Bridge**: Connects to the existing Kusto functionality in codex-core

### Directory Structure

```
codex-rs/kusto-server/
├── Cargo.toml              # Package configuration
├── build.sh                # Build script
├── README.md               # Documentation
└── src/
    ├── main.rs             # Entry point and CLI argument handling
    ├── server.rs           # MCP server implementation
    ├── kusto_bridge.rs     # Bridge to codex-core Kusto functionality
    ├── tool_config.rs      # Tool definitions
    └── server/
        └── message_processor.rs  # MCP message processing
```

### Available Tools

The server provides the following MCP tools:

- `kusto_execute_query` - Execute KQL queries
- `kusto_get_table_schema` - Get table schema information
- `kusto_list_tables` - List available tables
- `kusto_list_databases` - List configured databases
- `kusto_get_knowledge_base_summary` - Get knowledge base summary
- `kusto_update_table_description` - Update table descriptions
- `kusto_search_knowledge_base` - Search the knowledge base
- `kusto_list_functions` - List available functions
- `kusto_describe_function` - Get function details
- `kusto_test_connection` - Test connection and run diagnostics
- `kusto_clear_auth_cache` - Clear authentication cache

## Configuration

The server uses the same configuration as the main Codex application. It will look for configuration in the following order:

1. File specified with `--config` flag
2. Main Codex config file (`~/.codex/config.toml`)
3. Standalone config files in common locations:
   - `kusto_config.toml`
   - `config/kusto.toml`
   - `~/.config/codex/kusto.toml`
4. Environment variables (`KUSTO_CLUSTER_URL`, `KUSTO_DATABASE`)

### Example Configuration

```toml
[kusto]
cluster_url = "https://help.kusto.windows.net"
database = "Samples"
knowledge_base_path = "kusto_knowledge_base.json"
auto_update_knowledge_base = true
max_knowledge_base_rows = 100

# Multiple databases (optional)
[kusto.databases.samples]
name = "Samples"
description = "Sample database with demo data"
is_default = true

[kusto.databases.analytics]
name = "AnalyticsDB"
description = "Analytics and reporting database"
cluster_url = "https://analytics-cluster.kusto.windows.net"
```

## Usage

### Building

```bash
cd codex-rs/kusto-server
./build.sh
```

### Running

```bash
# Use default config
./bin/kusto-server

# Use specific config
./bin/kusto-server --config path/to/config.toml

# Enable verbose logging
./bin/kusto-server --verbose
```

### Integration with MCP Clients

Configure your MCP client to use this server. For example, with Claude Desktop:

```json
{
  "mcpServers": {
    "kusto": {
      "command": "/path/to/kusto-server",
      "args": ["--config", "/path/to/config.toml"]
    }
  }
}
```

## Authentication

The server uses OAuth authentication compatible with Azure Data Explorer. Authentication tokens are shared with the main Codex application and other Azure services.

## Next Steps

1. **Testing**: Create comprehensive tests for the Kusto MCP Server
2. **Documentation**: Add detailed usage examples and API documentation
3. **Integration**: Integrate with more MCP clients and AI assistants
4. **Features**: Add support for more advanced Kusto features like data ingestion and management commands
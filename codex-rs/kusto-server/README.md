# Kusto MCP Server

A standalone MCP (Model Context Protocol) server for Kusto (Azure Data Explorer) operations.

## Overview

This server provides MCP-compatible access to Kusto functionality, allowing AI assistants and other MCP clients to:

- Execute KQL (Kusto Query Language) queries
- Explore database schemas and table structures
- List available databases, tables, and functions
- Manage knowledge base information
- Test connections and troubleshoot authentication

## Features

- **Query Execution**: Run KQL queries against Azure Data Explorer clusters
- **Schema Discovery**: Explore database structures, table schemas, and available functions
- **Knowledge Base**: Maintain and search a knowledge base of table descriptions and query patterns
- **Multi-Database Support**: Work with multiple databases across different clusters
- **Authentication**: Uses the same OAuth authentication as the main Codex application
- **Error Handling**: Comprehensive error reporting and connection diagnostics

## Installation

Build the server from the codex-rs workspace:

```bash
cd codex-rs
cargo build --release --bin kusto-server
```

The binary will be available at `target/release/kusto-server`.

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

Add to your `~/.codex/config.toml`:

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

### As MCP Server

Run the server in MCP mode (communicates via stdin/stdout):

```bash
kusto-server
```

With custom config:

```bash
kusto-server --config /path/to/config.toml
```

With verbose logging:

```bash
kusto-server --verbose
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

If you encounter authentication issues:

1. Use the `kusto_clear_auth_cache` tool to force re-authentication
2. Check that your configuration includes the correct cluster URL
3. Ensure you have appropriate permissions for the target databases

## Development

### Building

```bash
cargo build --bin kusto-server
```

### Testing

```bash
cargo test --bin kusto-server
```

### Adding New Tools

1. Add the tool definition to `src/tool_config.rs`
2. Update the tool handler in the core Kusto module
3. Update this README with the new tool information

## Troubleshooting

### Common Issues

1. **Configuration not found**: Ensure config file exists and contains `[kusto]` section
2. **Authentication errors**: Use `kusto_clear_auth_cache` tool or check Azure permissions
3. **Connection timeouts**: Verify cluster URL and network connectivity
4. **Query errors**: Use `kusto_test_connection` to validate queries

### Logging

Enable verbose logging for debugging:

```bash
kusto-server --verbose
```

Or set the environment variable:

```bash
RUST_LOG=kusto_server=debug kusto-server
```

## License

This project is licensed under the same terms as the main Codex project.
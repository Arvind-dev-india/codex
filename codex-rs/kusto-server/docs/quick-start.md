# Kusto MCP Server Quick Start Guide

This guide will help you get started with the Kusto MCP Server, which provides access to Azure Data Explorer (Kusto) functionality through the Model Context Protocol (MCP).

## Prerequisites

- Rust toolchain (rustc, cargo)
- Access to an Azure Data Explorer cluster
- Basic understanding of KQL (Kusto Query Language)

## Installation

### Building from Source

1. Clone the repository:
   ```bash
   git clone https://github.com/your-org/codex.git
   cd codex
   ```

2. Build the Kusto MCP Server:
   ```bash
   cd codex-rs/kusto-server
   ./build.sh
   ```

3. The binary will be available at `bin/kusto-server`

## Configuration

The server uses the same configuration as the main Codex application. Create a configuration file with your Kusto settings:

```toml
[kusto]
cluster_url = "https://your-cluster.kusto.windows.net"
database = "YourDatabase"
knowledge_base_path = "kusto_knowledge_base.json"
```

## Running the Server

### Basic Usage

```bash
./bin/kusto-server
```

### With Custom Configuration

```bash
./bin/kusto-server --config path/to/your/config.toml
```

### With Verbose Logging

```bash
./bin/kusto-server --verbose
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
    "kusto": {
      "command": "/path/to/kusto-server",
      "args": ["--config", "/path/to/config.toml"]
    }
  }
}
```

### Other MCP Clients

Configure your MCP client to use the Kusto MCP Server as an external tool provider. The server communicates via stdin/stdout using the MCP protocol.

## Available Tools

The server provides the following tools:

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

## Example Usage

Once connected to an MCP client, you can use the Kusto tools like this:

1. **Execute a query**:
   ```
   kusto_execute_query(query: "StormEvents | take 10")
   ```

2. **Get table schema**:
   ```
   kusto_get_table_schema(table_name: "StormEvents")
   ```

3. **List tables**:
   ```
   kusto_list_tables()
   ```

4. **Test connection**:
   ```
   kusto_test_connection()
   ```

## Troubleshooting

If you encounter issues:

1. **Authentication errors**: Use `kusto_clear_auth_cache` to force re-authentication
2. **Connection issues**: Verify your cluster URL and network connectivity
3. **Configuration problems**: Check your config file format and paths
4. **Server crashes**: Run with `--verbose` flag to see detailed logs

## Next Steps

- Explore the [KQL documentation](https://docs.microsoft.com/en-us/azure/data-explorer/kusto/query/)
- Learn about [Azure Data Explorer](https://docs.microsoft.com/en-us/azure/data-explorer/)
- Check the [full documentation](../README.md) for advanced features
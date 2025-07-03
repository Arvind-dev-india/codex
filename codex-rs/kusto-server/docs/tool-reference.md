# Kusto MCP Server Tool Reference

This document provides detailed information about the tools available in the Kusto MCP Server.

## Tool Overview

The Kusto MCP Server provides the following tools for interacting with Azure Data Explorer (Kusto):

| Tool Name | Description | Required Parameters |
|-----------|-------------|---------------------|
| `kusto_execute_query` | Execute a Kusto query | `query` |
| `kusto_get_table_schema` | Get schema information for a table | `table_name` |
| `kusto_list_tables` | List available tables | None |
| `kusto_list_databases` | List configured databases | None |
| `kusto_get_knowledge_base_summary` | Get knowledge base summary | None |
| `kusto_update_table_description` | Update table description | `database`, `table_name`, `description` |
| `kusto_search_knowledge_base` | Search the knowledge base | `search_term` |
| `kusto_list_functions` | List available functions | None |
| `kusto_describe_function` | Get function details | `function_name` |
| `kusto_test_connection` | Test connection and run diagnostics | None |
| `kusto_clear_auth_cache` | Clear authentication cache | None |

## Detailed Tool Reference

### kusto_execute_query

Execute a Kusto query against Azure Data Explorer.

**Parameters:**
- `query` (string, required): Kusto query to execute (KQL)
- `database` (string, optional): Database name (uses default if not specified)

**Example:**
```json
{
  "name": "kusto_execute_query",
  "arguments": {
    "query": "StormEvents | take 10",
    "database": "Samples"
  }
}
```

**Response:**
Returns the query results as a JSON object with columns and rows.

### kusto_get_table_schema

Get schema information for a Kusto table.

**Parameters:**
- `table_name` (string, required): Name of the table to get schema for
- `database` (string, optional): Database name (uses default if not specified)

**Example:**
```json
{
  "name": "kusto_get_table_schema",
  "arguments": {
    "table_name": "StormEvents",
    "database": "Samples"
  }
}
```

**Response:**
Returns the table schema with column names, types, and descriptions.

### kusto_list_tables

List available tables in the Kusto database.

**Parameters:**
- `database` (string, optional): Database name (uses default if not specified)

**Example:**
```json
{
  "name": "kusto_list_tables",
  "arguments": {
    "database": "Samples"
  }
}
```

**Response:**
Returns a list of tables with their names and descriptions.

### kusto_list_databases

List all configured Kusto databases and their information.

**Parameters:** None

**Example:**
```json
{
  "name": "kusto_list_databases",
  "arguments": {}
}
```

**Response:**
Returns a list of databases with their names, cluster URLs, and descriptions.

### kusto_get_knowledge_base_summary

Get a summary of the Kusto knowledge base including databases, tables, and common query patterns.

**Parameters:** None

**Example:**
```json
{
  "name": "kusto_get_knowledge_base_summary",
  "arguments": {}
}
```

**Response:**
Returns a summary of the knowledge base contents.

### kusto_update_table_description

Update the description of a table in the knowledge base.

**Parameters:**
- `database` (string, required): Database name
- `table_name` (string, required): Name of the table
- `description` (string, required): New description for the table

**Example:**
```json
{
  "name": "kusto_update_table_description",
  "arguments": {
    "database": "Samples",
    "table_name": "StormEvents",
    "description": "Contains storm event data from the National Weather Service"
  }
}
```

**Response:**
Returns a confirmation message.

### kusto_search_knowledge_base

Search the knowledge base for tables, columns, or query patterns.

**Parameters:**
- `search_term` (string, required): Term to search for
- `search_type` (string, optional): Type of search to perform ('tables', 'columns', 'patterns', or 'all')

**Example:**
```json
{
  "name": "kusto_search_knowledge_base",
  "arguments": {
    "search_term": "storm",
    "search_type": "tables"
  }
}
```

**Response:**
Returns search results matching the search term.

### kusto_list_functions

List available functions in the Kusto database.

**Parameters:** None

**Example:**
```json
{
  "name": "kusto_list_functions",
  "arguments": {}
}
```

**Response:**
Returns a list of available functions with their names and descriptions.

### kusto_describe_function

Get detailed information about a specific Kusto function.

**Parameters:**
- `function_name` (string, required): Name of the function to describe

**Example:**
```json
{
  "name": "kusto_describe_function",
  "arguments": {
    "function_name": "series_stats"
  }
}
```

**Response:**
Returns detailed information about the function including parameters and usage.

### kusto_test_connection

Test Kusto connection and run a diagnostic query.

**Parameters:**
- `test_query` (string, optional): Optional test query to run (defaults to 'print "Hello, Kusto!"')
- `database` (string, optional): Database name (uses default if not specified)

**Example:**
```json
{
  "name": "kusto_test_connection",
  "arguments": {
    "test_query": "print datetime(now())",
    "database": "Samples"
  }
}
```

**Response:**
Returns connection status and query results.

### kusto_clear_auth_cache

Clear Kusto authentication cache and force re-authentication.

**Parameters:** None

**Example:**
```json
{
  "name": "kusto_clear_auth_cache",
  "arguments": {}
}
```

**Response:**
Returns a confirmation message.

## Error Handling

All tools return standard error responses when they fail, with the `is_error` field set to `true` and an error message in the `text` field.

Example error response:
```json
{
  "content": [
    {
      "type": "text",
      "text": "Error calling Kusto tool: Failed to connect to cluster: Connection refused"
    }
  ],
  "is_error": true
}
```

## Authentication

The Kusto MCP Server uses the same authentication mechanism as the main Codex application. Authentication tokens are cached to minimize login prompts.

If you encounter authentication issues, use the `kusto_clear_auth_cache` tool to force re-authentication.

## Best Practices

1. **Start with test_connection**: Verify connectivity before running complex queries
2. **Use knowledge base**: Leverage the knowledge base for schema exploration
3. **Limit query results**: Add `| take N` to your queries to limit result size
4. **Use appropriate database**: Specify the database parameter when working with multiple databases
5. **Handle errors gracefully**: Check for error responses and provide appropriate feedback
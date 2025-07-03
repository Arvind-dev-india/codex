# Kusto Tool Fix: get_table_schema

## Issue

The `kusto_get_table_schema` tool was failing with a 400 Bad Request error when attempting to retrieve schema information for tables in Azure Data Explorer (Kusto). The error occurred because the tool was using an incorrect query syntax for schema retrieval.

## Root Cause

The original implementation was using the following query syntax:
```kql
{table_name} | getschema
```

This approach has several issues:
1. The `| getschema` operator doesn't work on all table types
2. It requires the table to have data to work properly
3. It may fail with certain table configurations or permissions

## Solution

The fix replaces the query with the more reliable management command:
```kql
.show table {table_name} schema
```

This command:
1. Works consistently across all table types
2. Doesn't require the table to have data
3. Provides more complete schema information
4. Is the recommended approach in the Kusto documentation

## Implementation

The change was made in `codex-rs/core/src/kusto/tools_impl.rs` in the `get_table_schema` method:

```rust
// BEFORE:
let query = format!("{} | getschema", table_name);

// AFTER:
let query = format!(".show table {} schema", table_name);
```

## Testing

The fix was tested with various table types and confirmed to work correctly. The `.show table schema` command is a management command that returns detailed schema information including:

- Column names
- Column types
- Column descriptions
- Table properties
- Table statistics

## Additional Notes

This fix is part of the Kusto MCP Server implementation, which provides a standalone server for interacting with Azure Data Explorer through the Model Context Protocol (MCP).

The fix ensures that the `kusto_get_table_schema` tool works reliably across different table types and configurations, improving the overall robustness of the Kusto integration.
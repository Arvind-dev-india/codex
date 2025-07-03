# Kusto Tool Fix: list_tables

## Issue

The `kusto_list_tables` tool was only showing tables from the knowledge base cache instead of querying Kusto directly. This meant that users couldn't see newly created tables or tables that weren't in the knowledge base.

## Root Cause

The original implementation was prioritizing the knowledge base over direct Kusto queries:

```rust
// First check knowledge base for cached tables
let kb = self.knowledge_base.read().await;
if let Some(tables) = kb.get_database_tables(db_name) {
    let table_list: Vec<_> = tables.iter().map(|(name, info)| {
        // ... create table info ...
    }).collect();
    
    if !table_list.is_empty() {
        return Ok(json!({
            "database": db_name,
            "tables": table_list,
            "source": "knowledge_base"
        }));
    }
}
```

This approach had several issues:
1. It only queried Kusto if the knowledge base had no tables for the database
2. It didn't show newly created tables that weren't in the knowledge base
3. It didn't provide the most up-to-date view of the database

## Solution

The fix modifies the tool to:
1. Always query Kusto directly for the most up-to-date list of tables
2. Include more table metadata (TableName, DatabaseName, Folder)
3. Optionally enrich the results with descriptions from the knowledge base
4. Return the count of tables in the response

## Implementation

The change was made in `codex-rs/core/src/kusto/tools_impl.rs` in the `list_tables` method:

```rust
// BEFORE:
// First check knowledge base for cached tables
let kb = self.knowledge_base.read().await;
if let Some(tables) = kb.get_database_tables(db_name) {
    // ... return tables from knowledge base if available ...
}

// AFTER:
// Always query Kusto directly for the most up-to-date list of tables
let query = ".show tables | project TableName, DatabaseName, Folder";
// ... execute query and enrich with descriptions from knowledge base ...
```

## Benefits

The improved implementation:
1. Always shows the complete and up-to-date list of tables from Kusto
2. Includes more metadata about each table
3. Still leverages the knowledge base for descriptions when available
4. Provides a count of tables in the response

## Testing

The fix was tested with various databases and confirmed to work correctly. The `.show tables` command is a management command that returns all tables in the database, regardless of whether they're in the knowledge base.

## Additional Notes

This fix is part of the Kusto MCP Server implementation, which provides a standalone server for interacting with Azure Data Explorer through the Model Context Protocol (MCP).

The fix ensures that the `kusto_list_tables` tool always provides a complete and up-to-date view of the tables in a database, improving the overall usability of the Kusto integration.
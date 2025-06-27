# Kusto Debugging Guide - Fixing Query and Response Issues

## Problem Summary

You're experiencing:
1. **Failed to parse response** errors
2. **Queries not working** as expected
3. Need to see **actual queries being executed**

## Solution Implemented

### 1. **Enhanced Logging**
Added comprehensive logging to see exactly what's happening:

- **Request details**: URL, database, query, request body
- **Response details**: Status code, raw response text
- **Error details**: Parsing errors with full response content

### 2. **Fixed Kusto Query Syntax**
Updated queries to use correct Kusto syntax:

#### **List Tables** (Fixed)
```rust
// OLD (incorrect)
let query = ".show tables | project TableName, DatabaseName";

// NEW (correct)
let query = ".show tables | project TableName";
```

#### **Get Table Schema** (Fixed)
```rust
// OLD (incorrect)
let query = format!(".show table {} schema as json", table_name);

// NEW (correct - your suggestion)
let query = format!("{} | getschema", table_name);
```

#### **List Functions** (New)
```rust
let query = ".show functions | project Name";
```

#### **Describe Function** (New)
```rust
let query = format!(".show function {}", function_name);
```

### 3. **New Tools Added**
- `kusto_list_functions` - List available functions
- `kusto_describe_function` - Get function details

## How to Debug

### 1. **Enable Logging**
Set the `RUST_LOG` environment variable to see detailed logs:

```bash
export RUST_LOG=info
# or for more detailed logging
export RUST_LOG=debug
```

### 2. **Run a Simple Query**
```bash
codex "list tables in kusto"
```

### 3. **Check the Logs**
You'll now see detailed output like:

```
INFO codex_core::kusto::tools_impl: Executing Kusto list tables query: .show tables | project TableName on database: Samples
INFO codex_core::kusto::client: Kusto Query Request:
INFO codex_core::kusto::client:   URL: https://help.kusto.windows.net/v2/rest/query
INFO codex_core::kusto::client:   Database: Samples
INFO codex_core::kusto::client:   Query: .show tables | project TableName
INFO codex_core::kusto::client:   Request Body: {
  "csl": ".show tables | project TableName",
  "db": "Samples"
}
INFO codex_core::kusto::client: Kusto Response Status: 200
INFO codex_core::kusto::client: Kusto Raw Response: {"Tables":[{"TableName":"PrimaryResult","Columns":[{"ColumnName":"TableName","DataType":"String","ColumnType":"string"}],"Rows":[["StormEvents"],["PopulationData"]]}]}
```

### 4. **Analyze the Response**
The logs will show you:
- **Exact query sent** to Kusto
- **Raw response** from Kusto API
- **Any parsing errors** with full details

## Common Issues and Solutions

### Issue 1: "Failed to parse response"

**Cause**: Kusto response format doesn't match expected structure

**Debug Steps**:
1. Check the "Kusto Raw Response" in logs
2. Compare with expected `KustoQueryResult` structure
3. Verify the response has the expected fields

**Example Fix**:
If the response structure is different, we may need to update the `KustoQueryResult` model.

### Issue 2: "401 Unauthorized"

**Cause**: Authentication issues

**Debug Steps**:
1. Check if you see "Kusto authentication successful!" message
2. Verify cluster URL is correct
3. Ensure you have access to the Kusto cluster

**Fix**: Re-authenticate by removing tokens:
```bash
rm ~/.codex/kusto_auth.json
codex "list tables"  # Will prompt for re-authentication
```

### Issue 3: "Query syntax errors"

**Cause**: Incorrect Kusto query syntax

**Debug Steps**:
1. Look for the "Executing Kusto query" log line
2. Copy the exact query and test it in Kusto Web Explorer
3. Verify the query syntax is correct

**Available Query Patterns**:
```kusto
# List tables
.show tables | project TableName

# Get table schema
TableName | getschema

# List functions
.show functions | project Name

# Describe function
.show function FunctionName

# Sample data
TableName | take 10
```

## Testing Different Queries

### Test Basic Connectivity
```bash
codex "show me available tables"
```

### Test Schema Query
```bash
codex "what columns are in the StormEvents table?"
```

### Test Function Queries
```bash
codex "list available functions"
codex "describe the count function"
```

### Test Data Queries
```bash
codex "show me 5 rows from StormEvents"
```

## Log File Locations

Logs are typically written to:
- **Console output** (when running codex directly)
- **Log files** in `~/.codex/log/` (if configured)

## Advanced Debugging

### 1. **Test Queries Manually**
Copy the exact query from logs and test in Kusto Web Explorer:
1. Go to https://dataexplorer.azure.com/
2. Connect to your cluster
3. Paste the query and run it
4. Compare results

### 2. **Check Response Structure**
If parsing fails, the raw response will be in the logs. You can:
1. Copy the raw JSON response
2. Use a JSON formatter to make it readable
3. Compare with the expected `KustoQueryResult` structure

### 3. **Verify API Endpoint**
The logs show the exact URL being called:
```
URL: https://help.kusto.windows.net/v2/rest/query
```

Verify this matches your cluster's REST API endpoint.

## Expected Response Format

Kusto should return responses in this format:
```json
{
  "Tables": [
    {
      "TableName": "PrimaryResult",
      "Columns": [
        {
          "ColumnName": "TableName",
          "DataType": "String",
          "ColumnType": "string"
        }
      ],
      "Rows": [
        ["StormEvents"],
        ["PopulationData"]
      ]
    }
  ]
}
```

## Next Steps

1. **Run with logging enabled** and share the logs
2. **Test the corrected queries** with your Kusto cluster
3. **Check if the response format** matches expectations
4. **Report any remaining issues** with the full log output

The enhanced logging will show us exactly what's happening and help identify the root cause of the parsing errors.

## Quick Test Commands

```bash
# Enable detailed logging
export RUST_LOG=info

# Test authentication and basic connectivity
codex "list kusto databases"

# Test table listing with corrected query
codex "show me all tables"

# Test schema query with corrected syntax
codex "get schema for StormEvents table"

# Test new function queries
codex "list available functions"
```

The logs will now show you exactly what queries are being executed and what responses are received, making it much easier to debug any issues.
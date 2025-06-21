# Azure DevOps Tool Integration Fix

## Issue Description

The Azure DevOps integration was properly implemented in the codebase, but there was a mismatch between how the tools were registered and how they were expected to be called. This resulted in the error message:

```
unsupported call: azure_devops_query_work_items
```

## Root Cause

The issue was in the function call handling:

1. The Azure DevOps tools were registered with names like `azure_devops_query_work_items` in `tools.rs`.
2. When the model called these functions, the function call was being processed in `handle_function_call` in `codex.rs`.
3. The function handler didn't recognize the Azure DevOps tool names directly, and when it tried to parse them as fully qualified tool names, it failed.
4. The MCP connection manager was expecting the server name to be `azure_devops` and the tool name to be the full name including the prefix.

## Solution

We made two changes to fix this issue:

1. Modified the `handle_function_call` function in `codex.rs` to specifically recognize function names that start with "azure_devops_" and route them correctly.

2. Updated the `call_tool` method in `mcp_connection_manager.rs` to handle both formats:
   - The original format: server=`azure_devops`, tool=`query_work_items`
   - The direct format: server=`azure_devops`, tool=`azure_devops_query_work_items`

These changes allow the system to properly handle Azure DevOps tool calls regardless of how they're formatted.

## Changes Made

1. Updated `codex.rs` to add a specific case for Azure DevOps tools:
   ```rust
   // Check for Azure DevOps tools directly
   name if name.starts_with("azure_devops_") => {
       // For Azure DevOps tools, use empty server name and the full name as the tool name
       let timeout = None;
       handle_mcp_tool_call(
           sess, &sub_id, call_id, "azure_devops".to_string(), name.to_string(), arguments, timeout,
       )
       .await
   }
   ```

2. Updated `mcp_connection_manager.rs` to handle Azure DevOps tool calls with the full name:
   ```rust
   // Extract the actual tool name if it's a full name (azure_devops_query_work_items)
   let actual_tool_name = if tool.starts_with("azure_devops_") {
       tool.to_string()
   } else {
       format!("azure_devops_{}", tool)
   };
   ```

## Testing

To test this fix:

1. Configure Azure DevOps in your `~/.codex/config.toml` file as described in `AZURE_DEVOPS_USAGE.md`
2. Set your Azure DevOps PAT in the environment variable
3. Try querying work items with:
   ```
   codex "Find all work items assigned to me in the project"
   ```

The tool should now correctly process the Azure DevOps function calls.

## Additional Notes

This fix maintains backward compatibility with any existing code that might be using the original format, while also supporting the direct format that the model seems to prefer.
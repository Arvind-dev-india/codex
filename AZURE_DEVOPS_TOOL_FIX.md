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

We made four changes to fix this issue:

1. Modified the `handle_function_call` function in `codex.rs` to specifically recognize function names that start with "azure_devops_" and route them correctly.

2. Updated the `call_tool` method in `mcp_connection_manager.rs` to handle both formats:
   - The original format: server=`azure_devops`, tool=`query_work_items`
   - The direct format: server=`azure_devops`, tool=`azure_devops_query_work_items`

3. Modified the `handle_azure_devops_tool_call` function in `tool_handler.rs` to use the default project from the configuration when it's not provided in the arguments.

4. Fixed the `build_url` function in `client.rs` to properly format the API version parameter as `api-version=7.0` instead of just `7.0`.

These changes allow the system to properly handle Azure DevOps tool calls regardless of how they're formatted, use the default project when needed, and correctly format the API version parameter in requests.

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

3. Modified `tool_handler.rs` to use the default project from the configuration:
   ```rust
   // If the arguments don't include a project and we have a default project in the config,
   // add the default project to the arguments
   if !args.is_object() {
       args = serde_json::Value::Object(serde_json::Map::new());
   }
   
   if args.get("project").is_none() {
       if let Some(default_project) = &config.default_project {
           if let serde_json::Value::Object(ref mut map) = args {
               map.insert("project".to_string(), serde_json::Value::String(default_project.clone()));
           }
       }
   }
   ```

4. Fixed the `build_url` function in `client.rs` to properly format the API version parameter:
   ```rust
   fn build_url(&self, project: Option<&str>, endpoint: &str) -> String {
       let base_url = &self.auth.organization_url;
       
       // Determine the correct separator (? or &) for the API version parameter
       let separator = if endpoint.contains('?') { "&" } else { "?" };
       
       if let Some(project) = project {
           format!("{}/{}/_apis/{}{}{}", base_url, project, endpoint, separator, format!("api-version={}", self.api_version))
       } else {
           format!("{}/_apis/{}{}{}", base_url, endpoint, separator, format!("api-version={}", self.api_version))
       }
   }
   ```

## Testing

To test this fix:

1. Configure Azure DevOps in your `~/.codex/config.toml` file as described in `AZURE_DEVOPS_USAGE.md`
2. Set your Azure DevOps PAT in the environment variable
3. Make sure you have a valid project name in your configuration or query
4. Try querying work items with:
   ```
   codex "Find all work items assigned to me in the project"
   ```

The tool should now correctly process the Azure DevOps function calls.

### Important Note About Project Names

When using Azure DevOps tools, make sure to use project names that actually exist in your Azure DevOps organization. If you see an error like:

```
TF200016: The following project does not exist: project-name
```

You need to either:
1. Use an existing project name in your query
2. Set a valid `default_project` in your Azure DevOps configuration
3. Create the project in your Azure DevOps organization

See the "Common Errors and Solutions" section in `AZURE_DEVOPS_USAGE.md` for more details.

## Additional Notes

This fix maintains backward compatibility with any existing code that might be using the original format, while also supporting the direct format that the model seems to prefer.
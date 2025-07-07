# Recovery Services List Protectable Items Tool Fix

## Problem
The `recovery_services_list_protectable_items` tool was failing with:
- **Error**: "400 Bad Request - Input provided for the call is invalid"
- **Root Cause**: Missing required `backupManagementType` parameter in Azure API call
- **MCP Server Issue**: Tool configuration didn't expose required parameters to users

## Solution Applied

### 1. Core Tool Definition Fix (`codex-rs/core/src/recovery_services/tools.rs`)
```rust
// BEFORE: Only had workload_type, server_name, vault_name
// AFTER: Added backup_management_type as required parameter
fn create_list_protectable_items_tool() -> OpenAiTool {
    let mut parameters = BTreeMap::new();
    parameters.insert("workload_type".to_string(), JsonSchema::String);
    parameters.insert("backup_management_type".to_string(), JsonSchema::String);  // NEW
    parameters.insert("server_name".to_string(), JsonSchema::String);
    parameters.insert("vault_name".to_string(), JsonSchema::String);
    
    create_function_tool(
        "recovery_services_list_protectable_items",
        "List protectable items... Required parameters: workload_type and backup_management_type...",
        parameters,
        &["workload_type", "backup_management_type"],  // Both required
    )
}
```

### 2. Client Implementation Fix (`codex-rs/core/src/recovery_services/client.rs`)
```rust
// BEFORE: Only workload_type filter
pub async fn list_protectable_items_new(&self, workload_type: Option<&str>) -> Result<Vec<Value>>

// AFTER: Both parameters with proper filter construction
pub async fn list_protectable_items_new(&self, workload_type: Option<&str>, backup_management_type: Option<&str>) -> Result<Vec<Value>> {
    // Build filter - Azure API requires backupManagementType filter
    let mut filters = Vec::new();
    
    // Add backup management type filter (required)
    if let Some(backup_type) = backup_management_type {
        filters.push(format!("backupManagementType eq '{}'", backup_type));
    } else {
        filters.push("backupManagementType eq 'AzureWorkload'".to_string());
    }
    
    // Add workload type filter if specified
    if let Some(workload) = workload_type {
        filters.push(format!("workloadType eq '{}'", workload));
    }
    
    if !filters.is_empty() {
        endpoint.push_str(&format!("?$filter={}", filters.join(" and ")));
    }
    // ...
}
```

### 3. Tool Implementation Fix (`codex-rs/core/src/recovery_services/tools_impl.rs`)
```rust
// BEFORE: Optional workload_type
// AFTER: Both parameters required with validation
pub async fn list_protectable_items(&self, args: Value) -> Result<Value> {
    let workload_type_str = args["workload_type"].as_str();
    let backup_management_type_str = args["backup_management_type"].as_str();
    
    // Both parameters are now required
    let api_workload_type = if let Some(workload) = workload_type_str {
        // Map common names to API format
        match workload.to_uppercase().as_str() {
            "SAPASE" | "SAPASEDATABASE" => "SAPAseDatabase",
            "SAPHANA" | "SAPHANADATABASE" => "SAPHanaDatabase", 
            "SQL" | "SQLDATABASE" => "SQLDataBase",
            // ...
        }
    } else {
        return Err(CodexErr::Other("workload_type parameter is required".to_string()));
    };
    
    let api_backup_management_type = if let Some(backup_type) = backup_management_type_str {
        backup_type
    } else {
        return Err(CodexErr::Other("backup_management_type parameter is required".to_string()));
    };
    
    // Use both parameters
    let result = client.list_protectable_items_new(Some(api_workload_type), Some(api_backup_management_type)).await?;
}
```

### 4. MCP Server Configuration Fix (`codex-rs/recovery-services-server/src/tool_config.rs`)
```rust
// BEFORE: Only vault_name parameter exposed
// AFTER: All required parameters exposed to MCP clients
fn create_list_protectable_items_tool() -> Tool {
    Tool {
        name: "recovery_services_list_protectable_items".to_string(),
        description: Some("List protectable items... Required parameters: workload_type and backup_management_type..."),
        input_schema: ToolInputSchema {
            properties: Some(json!({
                "workload_type": {
                    "type": "string",
                    "description": "Type of workload... Supports: 'SAPAseDatabase', 'SAPHanaDatabase', 'SQLDataBase', 'VM', 'AzureFileShare'..."
                },
                "backup_management_type": {
                    "type": "string", 
                    "description": "Backup management type. Use 'AzureWorkload' for databases, 'AzureIaasVM' for VMs, 'AzureStorage' for file shares."
                },
                "server_name": {
                    "type": "string",
                    "description": "Optional server name to filter results"
                },
                "vault_name": {
                    "type": "string",
                    "description": "Name of the Recovery Services vault"
                }
            })),
            required: Some(vec!["workload_type".to_string(), "backup_management_type".to_string()]),
        },
    }
}
```

## API Call Format
The tool now generates correct Azure API calls:
```
GET /backupProtectableItems?$filter=backupManagementType eq 'AzureWorkload' and workloadType eq 'SAPAseDatabase'
API Version: 2018-01-10
```

This matches the example URL:
```
https://management.azure.com/subscriptions/{sub}/resourceGroups/{rg}/providers/Microsoft.RecoveryServices/vaults/{vault}/backupProtectableItems?api-version=2018-01-10&$filter=backupManagementType eq 'AzureWorkload' and workloadType eq 'SAPAseDatabase'
```

## Usage Examples

### For SAP ASE Databases:
```json
{
  "workload_type": "SAPAseDatabase",
  "backup_management_type": "AzureWorkload"
}
```

### For Virtual Machines:
```json
{
  "workload_type": "VM",
  "backup_management_type": "AzureIaasVM"
}
```

### For SQL Server Databases:
```json
{
  "workload_type": "SQLDataBase",
  "backup_management_type": "AzureWorkload"
}
```

## Status
✅ **FIXED**: All components updated and compiled successfully
✅ **MCP Server**: Now exposes required parameters to clients
✅ **API Compliance**: Generates correct Azure API calls
✅ **Error Handling**: Proper validation for required parameters

The tool should now work correctly and provide the required input fields in MCP clients.
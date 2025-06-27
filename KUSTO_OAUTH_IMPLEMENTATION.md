# Kusto OAuth Implementation with Token Sharing

## Overview

I've implemented Kusto (Azure Data Explorer) integration for Codex CLI that leverages the same OAuth authentication mechanism as Azure DevOps. This allows users to query Kusto clusters without needing to create separate credentials or authenticate multiple times.

## Key Features

### 1. Token Sharing with Azure DevOps

The implementation reuses the OAuth tokens from Azure DevOps, providing:
- **Single Sign-On**: Authenticate once for both services
- **Unified Experience**: Same authentication flow and token storage
- **Reduced Friction**: No need to create and manage separate tokens

### 2. Complete Kusto Integration

- **Query Execution**: Run KQL queries against Kusto clusters
- **Schema Exploration**: Get table schemas and list available tables
- **OAuth Authentication**: Device code flow with automatic token refresh

## Implementation Details

### Files Created

1. **Kusto Module Structure**:
   - `codex-rs/core/src/kusto/mod.rs` - Main module definition
   - `codex-rs/core/src/kusto/auth.rs` - Authentication handler
   - `codex-rs/core/src/kusto/client.rs` - Kusto API client
   - `codex-rs/core/src/kusto/models.rs` - Data models
   - `codex-rs/core/src/kusto/tools.rs` - Tool definitions
   - `codex-rs/core/src/kusto/tools_impl.rs` - Tool implementations
   - `codex-rs/core/src/kusto/tool_handler.rs` - Tool call handler

2. **Configuration**:
   - `codex-rs/core/src/config_types/config_types_kusto.rs` - Kusto configuration types
   - Updated `codex-rs/core/src/config.rs` to include Kusto configuration

3. **Integration with OpenAI Tools**:
   - Updated `codex-rs/core/src/openai_tools.rs` to register Kusto tools

4. **Documentation and Examples**:
   - `KUSTO_INTEGRATION.md` - User documentation
   - `kusto_config_example.toml` - Example configuration

5. **Tests**:
   - `codex-rs/core/tests/kusto_config_parsing.rs` - Configuration parsing tests

### Authentication Flow

1. **Token Reuse**:
   ```rust
   // Reuse the Azure DevOps OAuth handler
   let oauth_handler = AzureDevOpsOAuthHandler::new(codex_home);
   let access_token = oauth_handler.get_access_token().await?;
   ```

2. **Unified Storage**:
   - Both Azure DevOps and Kusto tokens are stored in `~/.codex/azure_devops_auth.json`
   - Same token refresh mechanism is used

3. **Authorization Headers**:
   ```rust
   // OAuth uses Bearer token
   Some(format!("Bearer {}", access_token))
   ```

## Configuration

Users can configure Kusto in their `~/.codex/config.toml`:

```toml
[kusto]
cluster_url = "https://help.kusto.windows.net"
database = "Samples"
```

## Available Tools

1. **kusto_execute_query**:
   - Execute KQL queries against Kusto clusters
   - Parameters: `query` (string)

2. **kusto_get_table_schema**:
   - Get schema information for a specific table
   - Parameters: `table_name` (string)

3. **kusto_list_tables**:
   - List all available tables in the configured database
   - No parameters required

## User Experience

### First Time Usage

1. User runs a Kusto command: `codex "query StormEvents from Kusto"`
2. If not already authenticated with Azure DevOps:
   - Device code flow initiates
   - User authenticates in browser
   - Tokens are saved
3. Kusto query executes using the obtained token

### Subsequent Usage

1. User runs another Kusto command
2. Existing tokens are used automatically
3. Tokens are refreshed as needed
4. Re-authentication only required when refresh token expires (~90 days)

## Benefits

1. **Simplified Authentication**: No need to create PATs or app registrations
2. **Reduced Friction**: Single authentication for multiple Azure services
3. **Better Security**: Short-lived access tokens with automatic refresh
4. **Improved User Experience**: Seamless integration between services

## Next Steps

1. **Testing**: Comprehensive testing with real Kusto clusters
2. **Additional Features**: Support for more Kusto operations
3. **Error Handling**: Improved error messages for Kusto-specific issues
4. **Documentation**: More examples and usage scenarios
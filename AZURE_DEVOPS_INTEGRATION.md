# Azure DevOps Integration for Codex CLI

This document outlines the implementation plan for adding Azure DevOps integration to the Codex CLI Rust implementation.

## Overview

The Azure DevOps integration will allow Codex CLI to interact with Azure DevOps services, enabling users to:

1. Query work items, pull requests, and repositories
2. Create and update work items
3. Comment on pull requests
4. Access and modify Azure DevOps wikis
5. Run and monitor pipelines

## Implementation Plan

### 1. Add Azure DevOps API Client

Create a new module in `codex-rs/core/src/azure_devops/` that will handle API communication with Azure DevOps:

- `client.rs`: Core API client functionality
- `auth.rs`: Authentication handling
- `models.rs`: Data models for Azure DevOps entities
- `tools.rs`: Tool definitions for Azure DevOps operations

### 2. Define Azure DevOps Tools

Add the following tools to `codex-rs/core/src/openai_tools.rs`:

- `azure_devops_query_work_items`: Search and retrieve work items
- `azure_devops_create_work_item`: Create new work items
- `azure_devops_update_work_item`: Update existing work items
- `azure_devops_query_pull_requests`: Search and retrieve pull requests
- `azure_devops_comment_on_pull_request`: Add comments to pull requests
- `azure_devops_get_wiki_page`: Retrieve wiki pages
- `azure_devops_update_wiki_page`: Update wiki pages
- `azure_devops_run_pipeline`: Trigger pipeline runs
- `azure_devops_get_pipeline_status`: Check pipeline status

### 3. Update Configuration

Modify `codex-rs/core/src/config_types.rs` and `codex-rs/core/src/config.rs` to add Azure DevOps configuration options:

```rust
pub struct AzureDevOpsConfig {
    /// Azure DevOps organization URL
    pub organization_url: String,
    /// Personal Access Token for authentication
    pub pat: Option<String>,
    /// Environment variable name that contains the PAT
    pub pat_env_var: Option<String>,
    /// Default project to use when not specified
    pub default_project: Option<String>,
}
```

### 4. Add Authentication Handling

Update `codex-rs/core/src/model_provider_info.rs` to include Azure DevOps authentication:

- Add PAT (Personal Access Token) authentication support
- Support environment variable configuration for credentials
- Add secure credential storage

### 5. Update MCP Server Configuration

Modify `codex-rs/mcp-server/src/codex_tool_config.rs` to handle Azure DevOps specific configuration:

- Add Azure DevOps tool definitions
- Add configuration parameters for Azure DevOps connections

### 6. Add Tests

Create tests in `codex-rs/core/tests/` to verify Azure DevOps integration:

- Unit tests for API client functionality
- Integration tests for tool operations
- Mock server for testing without real Azure DevOps instance

### 7. Update Documentation

- Add Azure DevOps configuration section to `codex-rs/config.md`
- Add examples of using Azure DevOps tools
- Update main README.md to mention Azure DevOps integration

## Configuration Example

Here's how users will configure Azure DevOps integration in their `~/.codex/config.toml`:

```toml
[azure_devops]
organization_url = "https://dev.azure.com/your-organization"
pat_env_var = "AZURE_DEVOPS_PAT"
default_project = "YourProject"
```

## Usage Examples

### Query Work Items

```
codex "Find all high priority bugs assigned to me in the current sprint"
```

### Create Work Item

```
codex "Create a new task titled 'Update documentation for API v2' with description 'We need to update the API docs to reflect recent changes'"
```

### Comment on Pull Request

```
codex "Add a comment to PR #123 asking for more test coverage"
```

## Implementation Timeline

1. **Week 1**: Set up basic structure and Azure DevOps API client
2. **Week 2**: Implement core tools and authentication
3. **Week 3**: Add remaining tools and update configuration
4. **Week 4**: Write tests and documentation

## Future Enhancements

- Add support for Azure DevOps Server (on-premises)
- Implement caching for better performance
- Add visualization of work item relationships
- Support for Azure Boards custom fields
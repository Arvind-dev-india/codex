# Azure DevOps MCP Server

A standalone Azure DevOps server that implements the Model Context Protocol (MCP) to provide Azure DevOps functionality to AI assistants and other tools.

## Features

This server provides the following Azure DevOps tools:

### Work Items
- **azure_devops_query_work_items**: Search for work items using WIQL queries
- **azure_devops_get_work_item**: Get details of a specific work item
- **azure_devops_create_work_item**: Create new work items
- **azure_devops_update_work_item**: Update existing work items
- **azure_devops_add_work_item_comment**: Add comments to work items

### Pull Requests
- **azure_devops_query_pull_requests**: Query pull requests in repositories
- **azure_devops_get_pull_request**: Get details of specific pull requests
- **azure_devops_comment_on_pull_request**: Add comments to pull requests

### Wiki
- **azure_devops_get_wiki_page**: Get content of wiki pages
- **azure_devops_update_wiki_page**: Update wiki page content

### Pipelines
- **azure_devops_run_pipeline**: Trigger pipeline runs
- **azure_devops_get_pipeline_status**: Get pipeline run status and details

## Installation

### Prerequisites

- Rust 1.70 or later
- Azure DevOps organization access
- Personal Access Token (PAT) or OAuth configuration

### Building

```bash
cd codex-rs/azure-devops-server
cargo build --release
```

The binary will be available at `target/release/azure-devops-server`.

## Configuration

The Azure DevOps MCP Server can be configured in multiple ways, with automatic fallback:

### Option 1: Main Codex Configuration (Recommended)

Add an `[azure_devops]` section to your main codex configuration file (`~/.codex/config.toml`):

```toml
# ~/.codex/config.toml
[azure_devops]
organization_url = "https://dev.azure.com/your-org-name"
auth_method = "oauth"  # or "pat" or "auto"
default_project = "your-default-project"  # Optional
```

This approach shares the same OAuth tokens with the main codex CLI and provides a seamless experience.

### Option 2: Standalone Configuration File

Create a separate TOML configuration file:

```toml
# azure_devops_config.toml
organization_url = "https://dev.azure.com/your-org-name"
auth_method = "pat"  # or "oauth" or "auto"
pat = "your-pat-token"
default_project = "your-default-project"  # Optional
```

### Option 3: Environment Variables

Use environment variables (fallback option):

```bash
export AZURE_DEVOPS_ORG="your-org-name"
export AZURE_DEVOPS_PAT="your-pat-token"
export AZURE_DEVOPS_PROJECT="your-default-project"  # Optional
```

## Usage

### Running the Server

```bash
# Using main codex config (recommended)
./azure-devops-server

# Using a standalone configuration file
./azure-devops-server --config azure_devops_config.toml

# With verbose logging
./azure-devops-server --verbose
```

### MCP Client Integration

The server implements the MCP protocol and can be used with any MCP-compatible client. Here's an example configuration for Claude Desktop:

```json
{
  "mcpServers": {
    "azure-devops": {
      "command": "/path/to/azure-devops-server"
    }
  }
}
```

Or with a standalone config file:

```json
{
  "mcpServers": {
    "azure-devops": {
      "command": "/path/to/azure-devops-server",
      "args": ["--config", "/path/to/azure_devops_config.toml"]
    }
  }
}
```

### Example Tool Calls

#### Query Work Items
```json
{
  "name": "azure_devops_query_work_items",
  "arguments": {
    "project": "MyProject",
    "query": "SELECT [System.Id], [System.Title] FROM WorkItems WHERE [System.WorkItemType] = 'Bug' AND [System.State] = 'Active'",
    "top": 50
  }
}
```

#### Create Work Item
```json
{
  "name": "azure_devops_create_work_item",
  "arguments": {
    "project": "MyProject",
    "type": "Bug",
    "title": "Fix login issue",
    "description": "Users cannot log in with their credentials",
    "assigned_to": "user@example.com",
    "priority": 2
  }
}
```

#### Get Pull Request
```json
{
  "name": "azure_devops_get_pull_request",
  "arguments": {
    "project": "MyProject",
    "repository": "MyRepo",
    "pull_request_id": 123,
    "include_commits": true
  }
}
```

## Configuration Priority

The server uses the following priority order for configuration:

1. **Main codex config**: `~/.codex/config.toml` with `[azure_devops]` section (recommended)
2. **Standalone config file** specified with `--config` flag
3. **Standalone config files** in these locations:
   - `azure_devops_config.toml` (current directory)
   - `config/azure_devops.toml`
   - `.config/azure_devops.toml`
   - `~/.config/codex/azure_devops.toml`
4. **Environment variables**: `AZURE_DEVOPS_ORG`, `AZURE_DEVOPS_PAT`, etc.

## Permissions

Your Personal Access Token or OAuth application needs the following scopes:

- **Work Items**: `vso.work` (read and write)
- **Code/Pull Requests**: `vso.code` (read and write)
- **Build/Pipelines**: `vso.build` (read and execute)
- **Wiki**: `vso.wiki` (read and write)

## Troubleshooting

### Common Issues

1. **Authentication Errors**: Verify your PAT token has the required permissions
2. **Project Not Found**: Ensure the project name is correct and you have access
3. **Configuration Not Found**: Check file paths and environment variables

### Verbose Logging

Use the `--verbose` flag to enable detailed logging:

```bash
./azure-devops-server --verbose --config azure_devops_config.toml
```

### Testing Configuration

You can test your configuration by running a simple query:

```bash
# The server will validate configuration on startup
./azure-devops-server --config azure_devops_config.toml
```

## Development

### Building from Source

```bash
git clone <repository-url>
cd codex-rs/azure-devops-server
cargo build
```

### Running Tests

```bash
cargo test
```

### Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the same license as the parent Codex project.
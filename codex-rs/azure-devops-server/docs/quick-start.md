# Azure DevOps MCP Server - Quick Start Guide

This guide will help you get the Azure DevOps MCP Server up and running quickly.

## Prerequisites

1. **Rust Environment**: Ensure you have Rust 1.70 or later installed
2. **Azure DevOps Access**: You need access to an Azure DevOps organization
3. **Personal Access Token**: Generate a PAT with appropriate permissions

## Step 1: Generate Personal Access Token

1. Go to your Azure DevOps organization: `https://dev.azure.com/{your-organization}`
2. Click on your profile picture → Personal access tokens
3. Click "New Token"
4. Configure the token with these scopes:
   - **Work Items**: Read & write
   - **Code**: Read & write  
   - **Build**: Read & execute
   - **Wiki**: Read & write
5. Copy the generated token (you won't see it again!)

## Step 2: Build the Server

```bash
cd codex-rs/azure-devops-server
cargo build --release
```

The binary will be available at `target/release/azure-devops-server`.

## Step 3: Configure the Server

### Option A: Configuration File (Recommended)

Create a file named `azure_devops_config.toml`:

```toml
organization_url = "https://dev.azure.com/your-organization-name"
auth_method = "pat"
pat = "your-personal-access-token"
default_project = "your-default-project"  # Optional
```

### Option B: Environment Variables

```bash
export AZURE_DEVOPS_ORG="your-organization-name"
export AZURE_DEVOPS_PAT="your-personal-access-token"
export AZURE_DEVOPS_PROJECT="your-default-project"  # Optional
```

## Step 4: Run the Server

### With Configuration File
```bash
./target/release/azure-devops-server --config azure_devops_config.toml
```

### With Environment Variables
```bash
./target/release/azure-devops-server
```

### With Verbose Logging
```bash
./target/release/azure-devops-server --verbose --config azure_devops_config.toml
```

## Step 5: Test the Server

The server implements the MCP protocol and communicates via stdin/stdout. You can test it with any MCP-compatible client.

### Example with Claude Desktop

Add this to your Claude Desktop configuration:

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

## Available Tools

Once running, the server provides these tools:

### Work Items
- `azure_devops_query_work_items` - Search work items with WIQL
- `azure_devops_get_work_item` - Get work item details
- `azure_devops_create_work_item` - Create new work items
- `azure_devops_update_work_item` - Update existing work items
- `azure_devops_add_work_item_comment` - Add comments to work items

### Pull Requests
- `azure_devops_query_pull_requests` - Query pull requests
- `azure_devops_get_pull_request` - Get pull request details
- `azure_devops_comment_on_pull_request` - Comment on pull requests

### Wiki
- `azure_devops_get_wiki_page` - Get wiki page content
- `azure_devops_update_wiki_page` - Update wiki pages

### Pipelines
- `azure_devops_run_pipeline` - Trigger pipeline runs
- `azure_devops_get_pipeline_status` - Get pipeline status

## Example Usage

### Query Active Bugs
```json
{
  "name": "azure_devops_query_work_items",
  "arguments": {
    "project": "MyProject",
    "query": "SELECT [System.Id], [System.Title] FROM WorkItems WHERE [System.WorkItemType] = 'Bug' AND [System.State] = 'Active'"
  }
}
```

### Create a New Task
```json
{
  "name": "azure_devops_create_work_item",
  "arguments": {
    "project": "MyProject",
    "type": "Task",
    "title": "Implement new feature",
    "description": "Add the requested functionality",
    "assigned_to": "user@example.com"
  }
}
```

## Troubleshooting

### Common Issues

1. **Authentication Failed**: Check your PAT token and permissions
2. **Project Not Found**: Verify the project name and your access
3. **Configuration Not Found**: Ensure the config file path is correct

### Getting Help

- Check the logs with `--verbose` flag
- Verify your Azure DevOps permissions
- Ensure the organization URL is correct

## Next Steps

- Read the full [README](../README.md) for detailed configuration options
- Explore the available tools and their parameters
- Set up OAuth authentication for enhanced security
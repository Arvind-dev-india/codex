# Using Azure DevOps Integration with Codex CLI

This guide explains how to set up and use the Azure DevOps integration with Codex CLI.

## Setup

### 1. Configure Azure DevOps in your Codex config

Add the following to your `~/.codex/config.toml` file:

```toml
[azure_devops]
organization_url = "https://dev.azure.com/your-organization"
pat_env_var = "AZURE_DEVOPS_PAT"
default_project = "YourProject"
```

### 2. Set up your Personal Access Token (PAT)

1. Create a Personal Access Token in Azure DevOps:
   - Go to your Azure DevOps organization
   - Click on your profile picture in the top right
   - Select "Personal access tokens"
   - Click "New Token"
   - Give it a name and select the following scopes:
     - Work Items: Read & Write
     - Code: Read & Write
     - Pull Request Threads: Read & Write
     - Wiki: Read & Write
     - Build: Read & Execute
   - Click "Create" and copy the token

2. Set the environment variable:

```bash
export AZURE_DEVOPS_PAT="your-pat-here"
```

For persistent configuration, add this to your shell profile (`~/.bashrc`, `~/.zshrc`, etc.).

## Using the Integration

Once configured, you can use Codex CLI to interact with Azure DevOps using natural language commands.

### Examples

#### Work Items

```bash
# Query work items
codex "Find all high priority bugs assigned to me in the current sprint"

# Create a work item
codex "Create a new task titled 'Update API documentation' with description 'We need to update the API docs to reflect recent changes'"

# Update a work item
codex "Change the status of bug #123 to 'Resolved'"
```

#### Pull Requests

```bash
# Query pull requests
codex "Show me all active pull requests that need my review"

# Get details of a specific PR
codex "Show me the details of pull request #45 in the 'backend' repository"

# Comment on a PR
codex "Add a comment to PR #45 saying 'Please add more test coverage for the new feature'"
```

#### Wiki

```bash
# Get wiki content
codex "Show me the content of the 'Getting Started' page in our wiki"

# Update wiki content
codex "Update the 'Deployment Process' wiki page to include information about the new CI/CD pipeline"
```

#### Pipelines

```bash
# Run a pipeline
codex "Run the 'Build and Deploy' pipeline on the 'main' branch"

# Check pipeline status
codex "What's the status of the latest build in the 'Release' pipeline?"
```

## Advanced Usage

### Working with Multiple Projects

If you work with multiple projects, you can specify the project in your commands:

```bash
codex "Find all bugs in the 'Mobile App' project"
```

Or set a different default project temporarily:

```bash
codex "Switch to the 'Backend API' project and show me all open bugs"
```

### Custom Queries

You can use complex queries to find specific work items:

```bash
codex "Find all user stories that are blocked by bugs and assigned to our team"
```

### Bulk Operations

Perform operations on multiple items:

```bash
codex "Assign all unassigned high priority bugs to John"
```

## Troubleshooting

### Authentication Issues

If you encounter authentication errors:

1. Verify your PAT is correct and not expired
2. Check that the environment variable is properly set
3. Ensure your PAT has the necessary permissions

### API Limitations

Azure DevOps API has rate limits. If you encounter rate limiting:

1. Reduce the frequency of requests
2. Use more specific queries to reduce the number of items returned

### Common Error Messages

- "TF401019: The Git repository with name or identifier X does not exist" - Check that you're using the correct repository name
- "VS403463: The user does not have permission to access this feature" - Your PAT needs additional permissions

## Building Your Own Custom CLI

If you've built a custom version of Codex CLI with Azure DevOps integration, you can use it by:

1. Building the binary:
   ```bash
   cd codex-rs
   cargo build --release
   ```

2. Running your custom CLI:
   ```bash
   ./target/release/codex
   ```

3. Creating an alias for easy access:
   ```bash
   alias mycodex="path/to/your/custom/codex"
   ```
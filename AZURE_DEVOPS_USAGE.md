# Using Azure DevOps Integration with Codex CLI

This guide explains how to set up and use the Azure DevOps integration with Codex CLI.

## Setup

### 1. Configure Azure DevOps in your Codex config

Add the following to your `~/.codex/config.toml` file:

```toml
[azure_devops]
organization_url = "https://dev.azure.com/your-organization"
pat_env_var = "AZURE_DEVOPS_PAT"  # Environment variable containing your PAT
# OR directly specify the PAT (less secure)
# pat = "your-personal-access-token"
default_project = "YourProject"  # Optional: Default project to use
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

## Available Azure DevOps Tools

Codex CLI provides the following Azure DevOps tools that can be used by the AI:

### Work Items

- **azure_devops_query_work_items**: Search for work items using WIQL
  - Parameters: `project`, `query`, `top` (optional)
  
- **azure_devops_get_work_item**: Get details of a specific work item
  - Parameters: `project`, `id`
  
- **azure_devops_create_work_item**: Create a new work item
  - Parameters: `project`, `type`, `title`, `description` (optional), `area_path` (optional), `iteration_path` (optional), `assigned_to` (optional)
  
- **azure_devops_update_work_item**: Update an existing work item
  - Parameters: `project`, `id`, `title` (optional), `description` (optional), `state` (optional), `assigned_to` (optional)

### Pull Requests

- **azure_devops_query_pull_requests**: Search for pull requests
  - Parameters: `project`, `repository`, `status` (optional), `creator` (optional), `reviewer` (optional), `top` (optional)
  
- **azure_devops_get_pull_request**: Get details of a specific pull request
  - Parameters: `project`, `repository`, `pull_request_id`
  
- **azure_devops_comment_on_pull_request**: Add a comment to a pull request
  - Parameters: `project`, `repository`, `pull_request_id`, `content`

### Wiki

- **azure_devops_get_wiki_page**: Get content of a wiki page
  - Parameters: `project`, `wiki`, `path`
  
- **azure_devops_update_wiki_page**: Update content of a wiki page
  - Parameters: `project`, `wiki`, `path`, `content`, `comment` (optional)

### Pipelines

- **azure_devops_run_pipeline**: Run a pipeline
  - Parameters: `project`, `pipeline_id`, `branch` (optional), `variables` (optional)
  
- **azure_devops_get_pipeline_status**: Get status of a pipeline run
  - Parameters: `project`, `build_id`

## Using the Integration

Once configured, you can use Codex CLI to interact with Azure DevOps using natural language commands.

### Examples

#### Work Items

```bash
# Query work items
codex "Find all high priority bugs assigned to me in the current sprint"

# Get a specific work item
codex "Show me the details of work item #123"

# Create a work item
codex "Create a new task titled 'Update API documentation' with description 'We need to update the API docs to reflect recent changes'"

# Update a work item
codex "Change the status of bug #123 to 'Resolved'"
```

#### Pull Requests

```bash
# Query pull requests
codex "Show me all active pull requests in the 'backend' repository"

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
codex "Run the 'Build and Deploy' pipeline (ID: 42) on the 'main' branch"

# Check pipeline status
codex "What's the status of build #789 in our project?"
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

### Using WIQL Queries

You can use WIQL (Work Item Query Language) for more complex queries:

```bash
codex "Run this WIQL query in the 'MyProject' project: SELECT [System.Id], [System.Title] FROM WorkItems WHERE [System.WorkItemType] = 'Bug' AND [System.State] = 'Active' ORDER BY [System.CreatedDate] DESC"
```

### Bulk Operations

Perform operations on multiple items:

```bash
codex "Find all unassigned high priority bugs and assign them to John"
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

## Technical Details

### How It Works

When you configure Azure DevOps integration, Codex CLI:

1. Registers the Azure DevOps tools with the OpenAI API
2. Creates a client that can communicate with the Azure DevOps REST API
3. Handles authentication using your PAT
4. Processes tool calls from the AI model and executes them against the Azure DevOps API
5. Returns the results back to the AI model

### Security Considerations

- Your PAT is sensitive information. We recommend using the environment variable approach rather than storing it directly in the config file.
- The PAT is only used for communication between Codex CLI and Azure DevOps API.
- All communication is done over HTTPS.

### Customizing the Integration

Advanced users can modify the Azure DevOps integration by:

1. Editing the Azure DevOps client implementation in `codex-rs/core/src/azure_devops/`
2. Adding new tools or enhancing existing ones
3. Building a custom version of Codex CLI with their changes
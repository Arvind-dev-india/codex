# Azure DevOps MCP Server - Tool Reference

This document provides detailed information about all available tools in the Azure DevOps MCP Server.

## Work Item Tools

### azure_devops_query_work_items

Search for work items using WIQL (Work Item Query Language).

**Parameters:**
- `project` (required): Project name or ID
- `query` (required): WIQL query string
- `top` (optional): Maximum number of results (default: 100)

**Example:**
```json
{
  "name": "azure_devops_query_work_items",
  "arguments": {
    "project": "MyProject",
    "query": "SELECT [System.Id], [System.Title], [System.State] FROM WorkItems WHERE [System.WorkItemType] = 'Bug'",
    "top": 50
  }
}
```

### azure_devops_get_work_item

Get detailed information about a specific work item.

**Parameters:**
- `project` (required): Project name or ID
- `id` (required): Work item ID
- `expand` (optional): Fields to expand (default: "fields")

**Example:**
```json
{
  "name": "azure_devops_get_work_item",
  "arguments": {
    "project": "MyProject",
    "id": 123,
    "expand": "fields,relations"
  }
}
```

### azure_devops_create_work_item

Create a new work item.

**Parameters:**
- `project` (required): Project name or ID
- `type` (required): Work item type (Bug, Task, User Story, etc.)
- `title` (required): Work item title
- `description` (optional): Work item description
- `assigned_to` (optional): Assignee email or display name
- `area_path` (optional): Area path
- `iteration_path` (optional): Iteration path
- `priority` (optional): Priority level (1-4)
- `tags` (optional): Semicolon-separated tags

**Example:**
```json
{
  "name": "azure_devops_create_work_item",
  "arguments": {
    "project": "MyProject",
    "type": "Bug",
    "title": "Login page not loading",
    "description": "Users report that the login page fails to load on mobile devices",
    "assigned_to": "developer@company.com",
    "priority": 2,
    "tags": "mobile;login;critical"
  }
}
```

### azure_devops_update_work_item

Update an existing work item.

**Parameters:**
- `project` (required): Project name or ID
- `id` (required): Work item ID
- `title` (optional): Updated title
- `description` (optional): Updated description
- `assigned_to` (optional): New assignee
- `state` (optional): New state (New, Active, Resolved, Closed, etc.)
- `area_path` (optional): Updated area path
- `iteration_path` (optional): Updated iteration path
- `priority` (optional): Updated priority
- `tags` (optional): Updated tags

**Example:**
```json
{
  "name": "azure_devops_update_work_item",
  "arguments": {
    "project": "MyProject",
    "id": 123,
    "state": "Resolved",
    "assigned_to": "tester@company.com"
  }
}
```

### azure_devops_add_work_item_comment

Add a comment to a work item.

**Parameters:**
- `project` (required): Project name or ID
- `id` (required): Work item ID
- `comment` (required): Comment text

**Example:**
```json
{
  "name": "azure_devops_add_work_item_comment",
  "arguments": {
    "project": "MyProject",
    "id": 123,
    "comment": "Fixed the issue by updating the mobile CSS styles."
  }
}
```

## Pull Request Tools

### azure_devops_query_pull_requests

Query pull requests in a repository.

**Parameters:**
- `project` (required): Project name or ID
- `repository` (required): Repository name or ID
- `status` (optional): PR status - "active", "completed", "abandoned", "all" (default: "active")
- `creator` (optional): Filter by creator
- `reviewer` (optional): Filter by reviewer
- `source_branch` (optional): Filter by source branch
- `target_branch` (optional): Filter by target branch
- `top` (optional): Maximum results (default: 100)

**Example:**
```json
{
  "name": "azure_devops_query_pull_requests",
  "arguments": {
    "project": "MyProject",
    "repository": "MyRepo",
    "status": "active",
    "target_branch": "main"
  }
}
```

### azure_devops_get_pull_request

Get details of a specific pull request.

**Parameters:**
- `project` (required): Project name or ID
- `repository` (required): Repository name or ID
- `pull_request_id` (required): Pull request ID
- `include_commits` (optional): Include commit details (default: false)
- `include_work_items` (optional): Include linked work items (default: false)

**Example:**
```json
{
  "name": "azure_devops_get_pull_request",
  "arguments": {
    "project": "MyProject",
    "repository": "MyRepo",
    "pull_request_id": 456,
    "include_commits": true
  }
}
```

### azure_devops_comment_on_pull_request

Add a comment to a pull request.

**Parameters:**
- `project` (required): Project name or ID
- `repository` (required): Repository name or ID
- `pull_request_id` (required): Pull request ID
- `comment` (required): Comment text
- `parent_comment_id` (optional): Parent comment ID for replies

**Example:**
```json
{
  "name": "azure_devops_comment_on_pull_request",
  "arguments": {
    "project": "MyProject",
    "repository": "MyRepo",
    "pull_request_id": 456,
    "comment": "LGTM! Great work on the error handling."
  }
}
```

## Wiki Tools

### azure_devops_get_wiki_page

Get content of a wiki page.

**Parameters:**
- `project` (required): Project name or ID
- `wiki_identifier` (required): Wiki name or ID
- `path` (required): Page path (e.g., "/Home" or "/Folder/Page")
- `version` (optional): Version descriptor (branch, tag, or commit)
- `include_content` (optional): Include page content (default: true)

**Example:**
```json
{
  "name": "azure_devops_get_wiki_page",
  "arguments": {
    "project": "MyProject",
    "wiki_identifier": "MyWiki",
    "path": "/Development/API-Guidelines"
  }
}
```

### azure_devops_update_wiki_page

Update content of a wiki page.

**Parameters:**
- `project` (required): Project name or ID
- `wiki_identifier` (required): Wiki name or ID
- `path` (required): Page path
- `content` (required): New page content in markdown
- `comment` (optional): Commit comment
- `version` (optional): Version descriptor

**Example:**
```json
{
  "name": "azure_devops_update_wiki_page",
  "arguments": {
    "project": "MyProject",
    "wiki_identifier": "MyWiki",
    "path": "/Development/API-Guidelines",
    "content": "# API Guidelines\n\nUpdated guidelines for REST API development...",
    "comment": "Updated API guidelines with new security requirements"
  }
}
```

## Pipeline Tools

### azure_devops_run_pipeline

Trigger a pipeline run.

**Parameters:**
- `project` (required): Project name or ID
- `pipeline_id` (required): Pipeline ID
- `branch` (optional): Source branch (default: "main")
- `parameters` (optional): Pipeline parameters as key-value pairs
- `variables` (optional): Pipeline variables as key-value pairs

**Example:**
```json
{
  "name": "azure_devops_run_pipeline",
  "arguments": {
    "project": "MyProject",
    "pipeline_id": 789,
    "branch": "feature/new-api",
    "parameters": {
      "environment": "staging",
      "runTests": "true"
    }
  }
}
```

### azure_devops_get_pipeline_status

Get status and details of a pipeline run.

**Parameters:**
- `project` (required): Project name or ID
- `pipeline_id` (required): Pipeline ID
- `run_id` (optional): Specific run ID (gets latest if not provided)
- `include_logs` (optional): Include build logs (default: false)

**Example:**
```json
{
  "name": "azure_devops_get_pipeline_status",
  "arguments": {
    "project": "MyProject",
    "pipeline_id": 789,
    "include_logs": true
  }
}
```

## Common WIQL Query Examples

### Active Work Items by Type
```sql
SELECT [System.Id], [System.Title], [System.State] 
FROM WorkItems 
WHERE [System.WorkItemType] = 'Bug' 
AND [System.State] = 'Active'
```

### Work Items Assigned to User
```sql
SELECT [System.Id], [System.Title], [System.AssignedTo] 
FROM WorkItems 
WHERE [System.AssignedTo] = 'user@company.com'
```

### Recently Created Items
```sql
SELECT [System.Id], [System.Title], [System.CreatedDate] 
FROM WorkItems 
WHERE [System.CreatedDate] >= @Today - 7
```

### Items in Current Sprint
```sql
SELECT [System.Id], [System.Title], [System.IterationPath] 
FROM WorkItems 
WHERE [System.IterationPath] UNDER 'MyProject\Sprint 1'
```

## Error Handling

All tools return structured error information when operations fail:

- **Authentication errors**: Invalid or expired PAT token
- **Permission errors**: Insufficient permissions for the operation
- **Not found errors**: Project, work item, or resource doesn't exist
- **Validation errors**: Invalid parameters or data

Check the `is_error` field in responses to determine if an operation succeeded.
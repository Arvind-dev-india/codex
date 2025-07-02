//! Configuration for Azure DevOps tools

use mcp_types::{Tool, ToolInputSchema};
use serde_json::json;

/// Create the list of available Azure DevOps tools
pub fn create_azure_devops_tools() -> Vec<Tool> {
    vec![
        create_query_work_items_tool(),
        create_get_work_item_tool(),
        create_create_work_item_tool(),
        create_update_work_item_tool(),
        create_add_work_item_comment_tool(),
        create_query_pull_requests_tool(),
        create_get_pull_request_tool(),
        create_comment_on_pull_request_tool(),
        create_get_wiki_page_tool(),
        create_update_wiki_page_tool(),
        create_run_pipeline_tool(),
        create_get_pipeline_status_tool(),
    ]
}

/// Create the query_work_items tool definition
fn create_query_work_items_tool() -> Tool {
    Tool {
        name: "azure_devops_query_work_items".to_string(),
        description: Some("Search for work items in Azure DevOps using a WIQL query".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "project": {
                    "type": "string",
                    "description": "Project name or ID"
                },
                "query": {
                    "type": "string",
                    "description": "WIQL query string"
                },
                "top": {
                    "type": "integer",
                    "description": "Maximum number of results to return",
                    "default": 100
                }
            })),
            required: Some(vec!["project".to_string(), "query".to_string()]),
        },
    }
}

/// Create the get_work_item tool definition
fn create_get_work_item_tool() -> Tool {
    Tool {
        name: "azure_devops_get_work_item".to_string(),
        description: Some("Get details of a specific work item by ID".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "project": {
                    "type": "string",
                    "description": "Project name or ID"
                },
                "id": {
                    "type": "integer",
                    "description": "Work item ID"
                },
                "expand": {
                    "type": "string",
                    "description": "Comma-separated list of fields to expand (relations, fields, links, etc.)",
                    "default": "fields"
                }
            })),
            required: Some(vec!["project".to_string(), "id".to_string()]),
        },
    }
}

/// Create the create_work_item tool definition
fn create_create_work_item_tool() -> Tool {
    Tool {
        name: "azure_devops_create_work_item".to_string(),
        description: Some("Create a new work item in Azure DevOps".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "project": {
                    "type": "string",
                    "description": "Project name or ID"
                },
                "type": {
                    "type": "string",
                    "description": "Work item type (Bug, Task, User Story, etc.)"
                },
                "title": {
                    "type": "string",
                    "description": "Work item title"
                },
                "description": {
                    "type": "string",
                    "description": "Work item description"
                },
                "assigned_to": {
                    "type": "string",
                    "description": "Email or display name of the assignee"
                },
                "area_path": {
                    "type": "string",
                    "description": "Area path for the work item"
                },
                "iteration_path": {
                    "type": "string",
                    "description": "Iteration path for the work item"
                },
                "priority": {
                    "type": "integer",
                    "description": "Priority level (1-4)"
                },
                "tags": {
                    "type": "string",
                    "description": "Semicolon-separated list of tags"
                }
            })),
            required: Some(vec!["project".to_string(), "type".to_string(), "title".to_string()]),
        },
    }
}

/// Create the update_work_item tool definition
fn create_update_work_item_tool() -> Tool {
    Tool {
        name: "azure_devops_update_work_item".to_string(),
        description: Some("Update an existing work item in Azure DevOps".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "project": {
                    "type": "string",
                    "description": "Project name or ID"
                },
                "id": {
                    "type": "integer",
                    "description": "Work item ID"
                },
                "title": {
                    "type": "string",
                    "description": "Updated work item title"
                },
                "description": {
                    "type": "string",
                    "description": "Updated work item description"
                },
                "assigned_to": {
                    "type": "string",
                    "description": "Email or display name of the assignee"
                },
                "state": {
                    "type": "string",
                    "description": "Work item state (New, Active, Resolved, Closed, etc.)"
                },
                "area_path": {
                    "type": "string",
                    "description": "Area path for the work item"
                },
                "iteration_path": {
                    "type": "string",
                    "description": "Iteration path for the work item"
                },
                "priority": {
                    "type": "integer",
                    "description": "Priority level (1-4)"
                },
                "tags": {
                    "type": "string",
                    "description": "Semicolon-separated list of tags"
                }
            })),
            required: Some(vec!["project".to_string(), "id".to_string()]),
        },
    }
}

/// Create the add_work_item_comment tool definition
fn create_add_work_item_comment_tool() -> Tool {
    Tool {
        name: "azure_devops_add_work_item_comment".to_string(),
        description: Some("Add a comment to a work item".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "project": {
                    "type": "string",
                    "description": "Project name or ID"
                },
                "id": {
                    "type": "integer",
                    "description": "Work item ID"
                },
                "comment": {
                    "type": "string",
                    "description": "Comment text"
                }
            })),
            required: Some(vec!["project".to_string(), "id".to_string(), "comment".to_string()]),
        },
    }
}

/// Create the query_pull_requests tool definition
fn create_query_pull_requests_tool() -> Tool {
    Tool {
        name: "azure_devops_query_pull_requests".to_string(),
        description: Some("Query pull requests in a repository".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "project": {
                    "type": "string",
                    "description": "Project name or ID"
                },
                "repository": {
                    "type": "string",
                    "description": "Repository name or ID"
                },
                "status": {
                    "type": "string",
                    "description": "Pull request status (active, completed, abandoned, all)",
                    "enum": ["active", "completed", "abandoned", "all"],
                    "default": "active"
                },
                "creator": {
                    "type": "string",
                    "description": "Filter by creator email or display name"
                },
                "reviewer": {
                    "type": "string",
                    "description": "Filter by reviewer email or display name"
                },
                "source_branch": {
                    "type": "string",
                    "description": "Filter by source branch name"
                },
                "target_branch": {
                    "type": "string",
                    "description": "Filter by target branch name"
                },
                "top": {
                    "type": "integer",
                    "description": "Maximum number of results to return",
                    "default": 100
                }
            })),
            required: Some(vec!["project".to_string(), "repository".to_string()]),
        },
    }
}

/// Create the get_pull_request tool definition
fn create_get_pull_request_tool() -> Tool {
    Tool {
        name: "azure_devops_get_pull_request".to_string(),
        description: Some("Get details of a specific pull request".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "project": {
                    "type": "string",
                    "description": "Project name or ID"
                },
                "repository": {
                    "type": "string",
                    "description": "Repository name or ID"
                },
                "pull_request_id": {
                    "type": "integer",
                    "description": "Pull request ID"
                },
                "include_commits": {
                    "type": "boolean",
                    "description": "Include commit details",
                    "default": false
                },
                "include_work_items": {
                    "type": "boolean",
                    "description": "Include linked work items",
                    "default": false
                }
            })),
            required: Some(vec!["project".to_string(), "repository".to_string(), "pull_request_id".to_string()]),
        },
    }
}

/// Create the comment_on_pull_request tool definition
fn create_comment_on_pull_request_tool() -> Tool {
    Tool {
        name: "azure_devops_comment_on_pull_request".to_string(),
        description: Some("Add a comment to a pull request".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "project": {
                    "type": "string",
                    "description": "Project name or ID"
                },
                "repository": {
                    "type": "string",
                    "description": "Repository name or ID"
                },
                "pull_request_id": {
                    "type": "integer",
                    "description": "Pull request ID"
                },
                "comment": {
                    "type": "string",
                    "description": "Comment text"
                },
                "parent_comment_id": {
                    "type": "integer",
                    "description": "ID of parent comment for replies"
                }
            })),
            required: Some(vec!["project".to_string(), "repository".to_string(), "pull_request_id".to_string(), "comment".to_string()]),
        },
    }
}

/// Create the get_wiki_page tool definition
fn create_get_wiki_page_tool() -> Tool {
    Tool {
        name: "azure_devops_get_wiki_page".to_string(),
        description: Some("Get content of a wiki page".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "project": {
                    "type": "string",
                    "description": "Project name or ID"
                },
                "wiki_identifier": {
                    "type": "string",
                    "description": "Wiki name or ID"
                },
                "path": {
                    "type": "string",
                    "description": "Page path (e.g., '/Home' or '/Folder/Page')"
                },
                "version": {
                    "type": "string",
                    "description": "Version descriptor (branch, tag, or commit)"
                },
                "include_content": {
                    "type": "boolean",
                    "description": "Include page content",
                    "default": true
                }
            })),
            required: Some(vec!["project".to_string(), "wiki_identifier".to_string(), "path".to_string()]),
        },
    }
}

/// Create the update_wiki_page tool definition
fn create_update_wiki_page_tool() -> Tool {
    Tool {
        name: "azure_devops_update_wiki_page".to_string(),
        description: Some("Update content of a wiki page".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "project": {
                    "type": "string",
                    "description": "Project name or ID"
                },
                "wiki_identifier": {
                    "type": "string",
                    "description": "Wiki name or ID"
                },
                "path": {
                    "type": "string",
                    "description": "Page path (e.g., '/Home' or '/Folder/Page')"
                },
                "content": {
                    "type": "string",
                    "description": "New page content in markdown format"
                },
                "comment": {
                    "type": "string",
                    "description": "Commit comment for the update"
                },
                "version": {
                    "type": "string",
                    "description": "Version descriptor (branch, tag, or commit)"
                }
            })),
            required: Some(vec!["project".to_string(), "wiki_identifier".to_string(), "path".to_string(), "content".to_string()]),
        },
    }
}

/// Create the run_pipeline tool definition
fn create_run_pipeline_tool() -> Tool {
    Tool {
        name: "azure_devops_run_pipeline".to_string(),
        description: Some("Trigger a pipeline run".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "project": {
                    "type": "string",
                    "description": "Project name or ID"
                },
                "pipeline_id": {
                    "type": "integer",
                    "description": "Pipeline ID"
                },
                "branch": {
                    "type": "string",
                    "description": "Source branch for the run",
                    "default": "main"
                },
                "parameters": {
                    "type": "object",
                    "description": "Pipeline parameters as key-value pairs"
                },
                "variables": {
                    "type": "object",
                    "description": "Pipeline variables as key-value pairs"
                }
            })),
            required: Some(vec!["project".to_string(), "pipeline_id".to_string()]),
        },
    }
}

/// Create the get_pipeline_status tool definition
fn create_get_pipeline_status_tool() -> Tool {
    Tool {
        name: "azure_devops_get_pipeline_status".to_string(),
        description: Some("Get status and details of a pipeline run".to_string()),
        annotations: None,
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "project": {
                    "type": "string",
                    "description": "Project name or ID"
                },
                "pipeline_id": {
                    "type": "integer",
                    "description": "Pipeline ID"
                },
                "run_id": {
                    "type": "integer",
                    "description": "Specific run ID (optional, gets latest if not provided)"
                },
                "include_logs": {
                    "type": "boolean",
                    "description": "Include build logs",
                    "default": false
                }
            })),
            required: Some(vec!["project".to_string(), "pipeline_id".to_string()]),
        },
    }
}
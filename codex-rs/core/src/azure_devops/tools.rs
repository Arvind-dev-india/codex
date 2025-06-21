//! Tool definitions for Azure DevOps operations.

use serde_json::{json, Value};
use std::collections::BTreeMap;

use crate::openai_tools::{JsonSchema, OpenAiTool, ResponsesApiTool};

/// Register all Azure DevOps tools
pub fn register_azure_devops_tools() -> Vec<OpenAiTool> {
    vec![
        create_query_work_items_tool(),
        create_get_work_item_tool(),
        create_create_work_item_tool(),
        create_update_work_item_tool(),
        create_query_pull_requests_tool(),
        create_get_pull_request_tool(),
        create_comment_on_pull_request_tool(),
        create_get_wiki_page_tool(),
        create_update_wiki_page_tool(),
        create_run_pipeline_tool(),
        create_get_pipeline_status_tool(),
    ]
}

/// Create a tool for querying work items
fn create_query_work_items_tool() -> OpenAiTool {
    let mut properties = BTreeMap::new();
    
    properties.insert(
        "project".to_string(),
        JsonSchema::String,
    );
    
    properties.insert(
        "query".to_string(),
        JsonSchema::String,
    );
    
    properties.insert(
        "top".to_string(),
        JsonSchema::Number,
    );

    OpenAiTool::Function(ResponsesApiTool {
        name: "azure_devops_query_work_items",
        description: "Search for work items in Azure DevOps using a WIQL query",
        strict: false,
        parameters: JsonSchema::Object {
            properties,
            required: &["project", "query"],
            additional_properties: false,
        },
    })
}

/// Create a tool for getting a specific work item
fn create_get_work_item_tool() -> OpenAiTool {
    let mut properties = BTreeMap::new();
    
    properties.insert(
        "project".to_string(),
        JsonSchema::String,
    );
    
    properties.insert(
        "id".to_string(),
        JsonSchema::Number,
    );

    OpenAiTool::Function(ResponsesApiTool {
        name: "azure_devops_get_work_item",
        description: "Get details of a specific work item in Azure DevOps",
        strict: false,
        parameters: JsonSchema::Object {
            properties,
            required: &["project", "id"],
            additional_properties: false,
        },
    })
}

/// Create a tool for creating a work item
fn create_create_work_item_tool() -> OpenAiTool {
    let mut properties = BTreeMap::new();
    
    properties.insert(
        "project".to_string(),
        JsonSchema::String,
    );
    
    properties.insert(
        "type".to_string(),
        JsonSchema::String,
    );
    
    properties.insert(
        "title".to_string(),
        JsonSchema::String,
    );
    
    properties.insert(
        "description".to_string(),
        JsonSchema::String,
    );
    
    properties.insert(
        "area_path".to_string(),
        JsonSchema::String,
    );
    
    properties.insert(
        "iteration_path".to_string(),
        JsonSchema::String,
    );
    
    properties.insert(
        "assigned_to".to_string(),
        JsonSchema::String,
    );

    OpenAiTool::Function(ResponsesApiTool {
        name: "azure_devops_create_work_item",
        description: "Create a new work item in Azure DevOps",
        strict: false,
        parameters: JsonSchema::Object {
            properties,
            required: &["project", "type", "title"],
            additional_properties: false,
        },
    })
}

/// Create a tool for updating a work item
fn create_update_work_item_tool() -> OpenAiTool {
    let mut properties = BTreeMap::new();
    
    properties.insert(
        "project".to_string(),
        JsonSchema::String,
    );
    
    properties.insert(
        "id".to_string(),
        JsonSchema::Number,
    );
    
    properties.insert(
        "title".to_string(),
        JsonSchema::String,
    );
    
    properties.insert(
        "description".to_string(),
        JsonSchema::String,
    );
    
    properties.insert(
        "state".to_string(),
        JsonSchema::String,
    );
    
    properties.insert(
        "assigned_to".to_string(),
        JsonSchema::String,
    );

    OpenAiTool::Function(ResponsesApiTool {
        name: "azure_devops_update_work_item",
        description: "Update an existing work item in Azure DevOps",
        strict: false,
        parameters: JsonSchema::Object {
            properties,
            required: &["project", "id"],
            additional_properties: false,
        },
    })
}

/// Create a tool for querying pull requests
fn create_query_pull_requests_tool() -> OpenAiTool {
    let mut properties = BTreeMap::new();
    
    properties.insert(
        "project".to_string(),
        JsonSchema::String,
    );
    
    properties.insert(
        "repository".to_string(),
        JsonSchema::String,
    );
    
    properties.insert(
        "status".to_string(),
        JsonSchema::String,
    );
    
    properties.insert(
        "creator".to_string(),
        JsonSchema::String,
    );
    
    properties.insert(
        "reviewer".to_string(),
        JsonSchema::String,
    );
    
    properties.insert(
        "top".to_string(),
        JsonSchema::Number,
    );

    OpenAiTool::Function(ResponsesApiTool {
        name: "azure_devops_query_pull_requests",
        description: "Search for pull requests in Azure DevOps",
        strict: false,
        parameters: JsonSchema::Object {
            properties,
            required: &["project", "repository"],
            additional_properties: false,
        },
    })
}

/// Create a tool for getting a specific pull request
fn create_get_pull_request_tool() -> OpenAiTool {
    let mut properties = BTreeMap::new();
    
    properties.insert(
        "project".to_string(),
        JsonSchema::String,
    );
    
    properties.insert(
        "repository".to_string(),
        JsonSchema::String,
    );
    
    properties.insert(
        "pull_request_id".to_string(),
        JsonSchema::Number,
    );

    OpenAiTool::Function(ResponsesApiTool {
        name: "azure_devops_get_pull_request",
        description: "Get details of a specific pull request in Azure DevOps",
        strict: false,
        parameters: JsonSchema::Object {
            properties,
            required: &["project", "repository", "pull_request_id"],
            additional_properties: false,
        },
    })
}

/// Create a tool for commenting on a pull request
fn create_comment_on_pull_request_tool() -> OpenAiTool {
    let mut properties = BTreeMap::new();
    
    properties.insert(
        "project".to_string(),
        JsonSchema::String,
    );
    
    properties.insert(
        "repository".to_string(),
        JsonSchema::String,
    );
    
    properties.insert(
        "pull_request_id".to_string(),
        JsonSchema::Number,
    );
    
    properties.insert(
        "content".to_string(),
        JsonSchema::String,
    );

    OpenAiTool::Function(ResponsesApiTool {
        name: "azure_devops_comment_on_pull_request",
        description: "Add a comment to a pull request in Azure DevOps",
        strict: false,
        parameters: JsonSchema::Object {
            properties,
            required: &["project", "repository", "pull_request_id", "content"],
            additional_properties: false,
        },
    })
}

/// Create a tool for getting a wiki page
fn create_get_wiki_page_tool() -> OpenAiTool {
    let mut properties = BTreeMap::new();
    
    properties.insert(
        "project".to_string(),
        JsonSchema::String,
    );
    
    properties.insert(
        "wiki".to_string(),
        JsonSchema::String,
    );
    
    properties.insert(
        "path".to_string(),
        JsonSchema::String,
    );

    OpenAiTool::Function(ResponsesApiTool {
        name: "azure_devops_get_wiki_page",
        description: "Get content of a wiki page in Azure DevOps",
        strict: false,
        parameters: JsonSchema::Object {
            properties,
            required: &["project", "wiki", "path"],
            additional_properties: false,
        },
    })
}

/// Create a tool for updating a wiki page
fn create_update_wiki_page_tool() -> OpenAiTool {
    let mut properties = BTreeMap::new();
    
    properties.insert(
        "project".to_string(),
        JsonSchema::String,
    );
    
    properties.insert(
        "wiki".to_string(),
        JsonSchema::String,
    );
    
    properties.insert(
        "path".to_string(),
        JsonSchema::String,
    );
    
    properties.insert(
        "content".to_string(),
        JsonSchema::String,
    );
    
    properties.insert(
        "comment".to_string(),
        JsonSchema::String,
    );

    OpenAiTool::Function(ResponsesApiTool {
        name: "azure_devops_update_wiki_page",
        description: "Update content of a wiki page in Azure DevOps",
        strict: false,
        parameters: JsonSchema::Object {
            properties,
            required: &["project", "wiki", "path", "content"],
            additional_properties: false,
        },
    })
}

/// Create a tool for running a pipeline
fn create_run_pipeline_tool() -> OpenAiTool {
    let mut properties = BTreeMap::new();
    
    properties.insert(
        "project".to_string(),
        JsonSchema::String,
    );
    
    properties.insert(
        "pipeline_id".to_string(),
        JsonSchema::Number,
    );
    
    properties.insert(
        "branch".to_string(),
        JsonSchema::String,
    );
    
    properties.insert(
        "variables".to_string(),
        JsonSchema::Object {
            properties: BTreeMap::new(),
            required: &[],
            additional_properties: true,
        },
    );

    OpenAiTool::Function(ResponsesApiTool {
        name: "azure_devops_run_pipeline",
        description: "Run a pipeline in Azure DevOps",
        strict: false,
        parameters: JsonSchema::Object {
            properties,
            required: &["project", "pipeline_id"],
            additional_properties: false,
        },
    })
}

/// Create a tool for getting pipeline status
fn create_get_pipeline_status_tool() -> OpenAiTool {
    let mut properties = BTreeMap::new();
    
    properties.insert(
        "project".to_string(),
        JsonSchema::String,
    );
    
    properties.insert(
        "build_id".to_string(),
        JsonSchema::Number,
    );

    OpenAiTool::Function(ResponsesApiTool {
        name: "azure_devops_get_pipeline_status",
        description: "Get status of a pipeline run in Azure DevOps",
        strict: false,
        parameters: JsonSchema::Object {
            properties,
            required: &["project", "build_id"],
            additional_properties: false,
        },
    })
}
//! Implementation of Azure DevOps tool functions.

use serde_json::{json, Value};
use std::sync::Arc;

use crate::azure_devops::client::AzureDevOpsClient;
use crate::azure_devops::models::*;
use crate::config_types::AzureDevOpsConfig;
use crate::error::{CodexErr, Result};

/// Implementation of Azure DevOps tools
pub struct AzureDevOpsTools {
    pub client: Arc<AzureDevOpsClient>,
}

impl AzureDevOpsTools {
    /// Create a new instance of Azure DevOps tools
    pub async fn new(config: &AzureDevOpsConfig) -> Result<Self> {
        use crate::azure_devops::auth::AzureDevOpsAuthHandler;
        use crate::config_types::AzureDevOpsAuthMethod;
        
        // Get codex home directory for OAuth token storage
        let codex_home = dirs::home_dir()
            .ok_or_else(|| CodexErr::Other("Could not determine home directory".to_string()))?
            .join(".codex");
        
        // Create auth handler based on configured method
        let auth = match &config.auth_method {
            AzureDevOpsAuthMethod::OAuth => {
                // Use OAuth only
                AzureDevOpsAuthHandler::from_oauth(&config.organization_url, &codex_home).await?
            }
            AzureDevOpsAuthMethod::Pat => {
                // Use PAT only
                if let Some(pat) = &config.pat {
                    AzureDevOpsAuthHandler::with_pat(&config.organization_url, pat)
                } else if let Some(env_var) = &config.pat_env_var {
                    AzureDevOpsAuthHandler::from_env(env_var, &config.organization_url)?
                } else {
                    return Err(CodexErr::Other(
                        "PAT authentication method selected but no pat or pat_env_var configured".to_string(),
                    ));
                }
            }
            AzureDevOpsAuthMethod::Auto => {
                // Try OAuth first, fall back to PAT
                AzureDevOpsAuthHandler::from_config_with_oauth(
                    &config.organization_url,
                    config.pat_env_var.as_deref(),
                    &codex_home,
                ).await?
            }
        };
        
        // Create client with auth
        let client = AzureDevOpsClient::new(auth)
            .with_api_version(&config.api_version);
            
        Ok(Self {
            client: Arc::new(client),
        })
    }

    /// Query work items using WIQL
    pub async fn query_work_items(&self, args: Value) -> Result<Value> {
        let project = args["project"].as_str().ok_or_else(|| {
            CodexErr::Other("project parameter is required".to_string())
        })?;
        
        let query = args["query"].as_str().ok_or_else(|| {
            CodexErr::Other("query parameter is required".to_string())
        })?;
        
        let top = args["top"].as_u64().unwrap_or(10) as i32;
        
        // Create the WIQL query request
        let wiql_query = json!({
            "query": query
        });
        
        // Execute the WIQL query
        let endpoint = format!("wit/wiql?$top={}", top);
        let query_result: WorkItemQueryResult = self.client
            .post(Some(project), &endpoint, &wiql_query)
            .await?;
            
        // If there are no work items, return empty array
        if query_result.work_items.is_empty() {
            return Ok(json!({ "workItems": [] }));
        }
        
        // Get the work item IDs
        let ids: Vec<i32> = query_result.work_items.iter()
            .map(|wi| wi.id)
            .collect();
            
        // Get the work items details
        let ids_str = ids.iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");
            
        let endpoint = format!("wit/workitems?ids={}&$expand=all", ids_str);
        let work_items_response: Value = self.client
            .get(Some(project), &endpoint)
            .await?;
            
        Ok(work_items_response)
    }
    
    /// Get a specific work item by ID
    pub async fn get_work_item(&self, args: Value) -> Result<Value> {
        let project = args["project"].as_str().ok_or_else(|| {
            CodexErr::Other("project parameter is required".to_string())
        })?;
        
        let id = args["id"].as_u64().ok_or_else(|| {
            CodexErr::Other("id parameter is required".to_string())
        })? as i32;
        
        // Get the work item details
        let endpoint = format!("wit/workitems/{}?$expand=all", id);
        let work_item: Value = self.client
            .get(Some(project), &endpoint)
            .await?;
            
        Ok(work_item)
    }
    
    /// Create a new work item
    pub async fn create_work_item(&self, args: Value) -> Result<Value> {
        let project = args["project"].as_str().ok_or_else(|| {
            CodexErr::Other("project parameter is required".to_string())
        })?;
        
        let work_item_type = args["type"].as_str().ok_or_else(|| {
            CodexErr::Other("type parameter is required".to_string())
        })?;
        
        let title = args["title"].as_str().ok_or_else(|| {
            CodexErr::Other("title parameter is required".to_string())
        })?;
        
        // Prepare the work item field updates
        let mut field_updates = Vec::new();
        
        // Add title field
        field_updates.push(json!({
            "op": "add",
            "path": "/fields/System.Title",
            "value": title
        }));
        
        // Add description field if provided
        if let Some(description) = args["description"].as_str() {
            field_updates.push(json!({
                "op": "add",
                "path": "/fields/System.Description",
                "value": description
            }));
        }
        
        // Add area path if provided
        if let Some(area_path) = args["area_path"].as_str() {
            field_updates.push(json!({
                "op": "add",
                "path": "/fields/System.AreaPath",
                "value": area_path
            }));
        }
        
        // Add iteration path if provided
        if let Some(iteration_path) = args["iteration_path"].as_str() {
            field_updates.push(json!({
                "op": "add",
                "path": "/fields/System.IterationPath",
                "value": iteration_path
            }));
        }
        
        // Add assigned to if provided
        if let Some(assigned_to) = args["assigned_to"].as_str() {
            field_updates.push(json!({
                "op": "add",
                "path": "/fields/System.AssignedTo",
                "value": assigned_to
            }));
        }
        
        // Create the work item
        let endpoint = format!("wit/workitems/${}", work_item_type);
        let work_item: Value = self.client
            .patch_work_item(Some(project), &endpoint, &field_updates)
            .await?;
            
        Ok(work_item)
    }

    /// Update an existing work item
    pub async fn update_work_item(&self, args: Value) -> Result<Value> {
        let project = args["project"].as_str().ok_or_else(|| {
            CodexErr::Other("project parameter is required".to_string())
        })?;
        
        let id = args["id"].as_u64().ok_or_else(|| {
            CodexErr::Other("id parameter is required".to_string())
        })? as i32;
        
        // Prepare the work item field updates
        let mut field_updates = Vec::new();
        
        // Update title field if provided
        if let Some(title) = args["title"].as_str() {
            field_updates.push(json!({
                "op": "add",
                "path": "/fields/System.Title",
                "value": title
            }));
        }
        
        // Update description field if provided
        if let Some(description) = args["description"].as_str() {
            field_updates.push(json!({
                "op": "add",
                "path": "/fields/System.Description",
                "value": description
            }));
        }
        
        // Update state if provided
        if let Some(state) = args["state"].as_str() {
            field_updates.push(json!({
                "op": "add",
                "path": "/fields/System.State",
                "value": state
            }));
        }
        
        // Update assigned to if provided
        if let Some(assigned_to) = args["assigned_to"].as_str() {
            field_updates.push(json!({
                "op": "add",
                "path": "/fields/System.AssignedTo",
                "value": assigned_to
            }));
        }
        
        // If no fields to update, return error
        if field_updates.is_empty() {
            return Err(CodexErr::Other(
                "No fields to update. Provide at least one of: title, description, state, assigned_to".to_string(),
            ));
        }
        
        // Update the work item
        let endpoint = format!("wit/workitems/{}", id);
        let work_item: Value = self.client
            .patch_work_item(Some(project), &endpoint, &field_updates)
            .await?;
            
        Ok(work_item)
    }

    /// Add a comment to a work item
    pub async fn add_work_item_comment(&self, args: Value) -> Result<Value> {
        let project = args["project"].as_str().ok_or_else(|| {
            CodexErr::Other("project parameter is required".to_string())
        })?;
        
        let id = args["id"].as_u64().ok_or_else(|| {
            CodexErr::Other("id parameter is required".to_string())
        })? as i32;
        
        let comment_text = args["comment"].as_str().ok_or_else(|| {
            CodexErr::Other("comment parameter is required".to_string())
        })?;
        
        // Create the comment request body
        let comment_request = json!({
            "text": comment_text
        });
        
        // Add the comment to the work item
        let endpoint = format!("wit/workitems/{}/comments", id);
        let comment: Value = self.client
            .post(Some(project), &endpoint, &comment_request)
            .await?;
            
        Ok(comment)
    }
    
    /// Query pull requests
    pub async fn query_pull_requests(&self, args: Value) -> Result<Value> {
        let project = args["project"].as_str().ok_or_else(|| {
            CodexErr::Other("project parameter is required".to_string())
        })?;
        
        let repository = args["repository"].as_str().ok_or_else(|| {
            CodexErr::Other("repository parameter is required".to_string())
        })?;
        
        // Build query parameters
        let mut query_params = vec![
            format!("searchCriteria.repositoryId={}", repository),
        ];
        
        // Add status filter if provided
        if let Some(status) = args["status"].as_str() {
            query_params.push(format!("searchCriteria.status={}", status));
        }
        
        // Add creator filter if provided
        if let Some(creator) = args["creator"].as_str() {
            query_params.push(format!("searchCriteria.creatorId={}", creator));
        }
        
        // Add reviewer filter if provided
        if let Some(reviewer) = args["reviewer"].as_str() {
            query_params.push(format!("searchCriteria.reviewerId={}", reviewer));
        }
        
        // Add top parameter if provided
        let top = args["top"].as_u64().unwrap_or(10);
        query_params.push(format!("$top={}", top));
        
        // Build the endpoint
        let query_string = query_params.join("&");
        let endpoint = format!("git/repositories/{}/pullrequests?{}", repository, query_string);
        
        // Get the pull requests
        let pull_requests: Value = self.client
            .get(Some(project), &endpoint)
            .await?;
            
        Ok(pull_requests)
    }

    /// Get a specific pull request by ID
    pub async fn get_pull_request(&self, args: Value) -> Result<Value> {
        let project = args["project"].as_str().ok_or_else(|| {
            CodexErr::Other("project parameter is required".to_string())
        })?;
        
        let repository = args["repository"].as_str().ok_or_else(|| {
            CodexErr::Other("repository parameter is required".to_string())
        })?;
        
        let pull_request_id = args["pull_request_id"].as_u64().ok_or_else(|| {
            CodexErr::Other("pull_request_id parameter is required".to_string())
        })? as i32;
        
        // Get the pull request details
        let endpoint = format!("git/repositories/{}/pullrequests/{}", repository, pull_request_id);
        let pull_request: Value = self.client
            .get(Some(project), &endpoint)
            .await?;
            
        Ok(pull_request)
    }
    
    /// Add a comment to a pull request
    pub async fn comment_on_pull_request(&self, args: Value) -> Result<Value> {
        let project = args["project"].as_str().ok_or_else(|| {
            CodexErr::Other("project parameter is required".to_string())
        })?;
        
        let repository = args["repository"].as_str().ok_or_else(|| {
            CodexErr::Other("repository parameter is required".to_string())
        })?;
        
        let pull_request_id = args["pull_request_id"].as_u64().ok_or_else(|| {
            CodexErr::Other("pull_request_id parameter is required".to_string())
        })? as i32;
        
        let comment = args["content"].as_str().ok_or_else(|| {
            CodexErr::Other("content parameter is required".to_string())
        })?;
        
        // Create the comment request
        let comment_request = json!({
            "content": comment,
            "commentType": "text"
        });
        
        // Add the comment
        let endpoint = format!("git/repositories/{}/pullrequests/{}/threads", repository, pull_request_id);
        let thread: Value = self.client
            .post(Some(project), &endpoint, &comment_request)
            .await?;
            
        Ok(thread)
    }

    /// Get a wiki page
    pub async fn get_wiki_page(&self, args: Value) -> Result<Value> {
        let project = args["project"].as_str().ok_or_else(|| {
            CodexErr::Other("project parameter is required".to_string())
        })?;
        
        let wiki_identifier = args["wiki"].as_str().ok_or_else(|| {
            CodexErr::Other("wiki parameter is required".to_string())
        })?;
        
        let path = args["path"].as_str().ok_or_else(|| {
            CodexErr::Other("path parameter is required".to_string())
        })?;
        
        // Get the wiki page
        let endpoint = format!("wiki/wikis/{}/pages?path={}", wiki_identifier, path);
        let page: Value = self.client
            .get(Some(project), &endpoint)
            .await?;
            
        Ok(page)
    }

    /// Update a wiki page
    pub async fn update_wiki_page(&self, args: Value) -> Result<Value> {
        let project = args["project"].as_str().ok_or_else(|| {
            CodexErr::Other("project parameter is required".to_string())
        })?;
        
        let wiki_identifier = args["wiki"].as_str().ok_or_else(|| {
            CodexErr::Other("wiki parameter is required".to_string())
        })?;
        
        let path = args["path"].as_str().ok_or_else(|| {
            CodexErr::Other("path parameter is required".to_string())
        })?;
        
        let content = args["content"].as_str().ok_or_else(|| {
            CodexErr::Other("content parameter is required".to_string())
        })?;
        
        // Create the update request
        let update_request = json!({
            "content": content
        });
        
        // Update the wiki page
        let endpoint = format!("wiki/wikis/{}/pages?path={}", wiki_identifier, path);
        let page: Value = self.client
            .put(Some(project), &endpoint, &update_request)
            .await?;
            
        Ok(page)
    }

    /// Run a pipeline
    pub async fn run_pipeline(&self, args: Value) -> Result<Value> {
        let project = args["project"].as_str().ok_or_else(|| {
            CodexErr::Other("project parameter is required".to_string())
        })?;
        
        let pipeline_id = args["pipeline_id"].as_u64().ok_or_else(|| {
            CodexErr::Other("pipeline_id parameter is required".to_string())
        })? as i32;
        
        // Create the run request
        let mut run_request = json!({});
        
        // Add branch if provided
        if let Some(branch) = args["branch"].as_str() {
            run_request["resources"] = json!({
                "repositories": {
                    "self": {
                        "refName": format!("refs/heads/{}", branch)
                    }
                }
            });
        }
        
        // Add variables if provided
        if let Some(variables) = args["variables"].as_object() {
            run_request["variables"] = json!(variables);
        }
        
        // Run the pipeline
        let endpoint = format!("pipelines/{}/runs", pipeline_id);
        let run: Value = self.client
            .post(Some(project), &endpoint, &run_request)
            .await?;
            
        Ok(run)
    }

    /// Get pipeline run status
    pub async fn get_pipeline_status(&self, args: Value) -> Result<Value> {
        let project = args["project"].as_str().ok_or_else(|| {
            CodexErr::Other("project parameter is required".to_string())
        })?;
        
        let build_id = args["build_id"].as_u64().ok_or_else(|| {
            CodexErr::Other("build_id parameter is required".to_string())
        })? as i32;
        
        // Get build details - this is actually for getting build status, not pipeline runs
        let endpoint = format!("build/builds/{}", build_id);
        let build: Value = self.client
            .get(Some(project), &endpoint)
            .await?;
        Ok(build)
    }
}
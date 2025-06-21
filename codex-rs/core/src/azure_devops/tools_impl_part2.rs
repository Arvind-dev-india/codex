//! Implementation of Azure DevOps tool functions (part 2).

use serde_json::{json, Value};
use crate::error::{CodexErr, Result};

use super::tools_impl::AzureDevOpsTools;

impl AzureDevOpsTools {
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
            .patch(Some(project), &endpoint, &field_updates)
            .await?;
            
        Ok(work_item)
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
}
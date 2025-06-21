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
    pub fn new(config: &AzureDevOpsConfig) -> Result<Self> {
        use crate::azure_devops::auth::AzureDevOpsAuthHandler;
        
        // Create auth handler from config
        let auth = if let Some(pat) = &config.pat {
            AzureDevOpsAuthHandler::with_pat(&config.organization_url, pat)
        } else if let Some(env_var) = &config.pat_env_var {
            AzureDevOpsAuthHandler::from_env(env_var, &config.organization_url)?
        } else {
            return Err(CodexErr::Other(
                "Azure DevOps configuration must include either pat or pat_env_var".to_string(),
            ));
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
            .patch(Some(project), &endpoint, &field_updates)
            .await?;
            
        Ok(work_item)
    }
}
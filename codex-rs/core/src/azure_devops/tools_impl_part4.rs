//! Implementation of Azure DevOps tool functions (part 4).

use serde_json::{json, Value};
use crate::error::{CodexErr, Result};

use super::tools_impl::AzureDevOpsTools;

impl AzureDevOpsTools {
    /// Update a wiki page
    pub async fn update_wiki_page(&self, args: Value) -> Result<Value> {
        let project = args["project"].as_str().ok_or_else(|| {
            CodexErr::Other("project parameter is required".to_string())
        })?;
        
        let wiki = args["wiki"].as_str().ok_or_else(|| {
            CodexErr::Other("wiki parameter is required".to_string())
        })?;
        
        let path = args["path"].as_str().ok_or_else(|| {
            CodexErr::Other("path parameter is required".to_string())
        })?;
        
        let content = args["content"].as_str().ok_or_else(|| {
            CodexErr::Other("content parameter is required".to_string())
        })?;
        
        // Get optional comment
        let comment = args["comment"].as_str().unwrap_or("Updated by Codex");
        
        // URL encode the path
        let encoded_path = urlencoding::encode(path);
        
        // Create the update request
        let update_request = json!({
            "content": content,
            "message": comment
        });
        
        // Update the wiki page
        let endpoint = format!("wiki/wikis/{}/pages?path={}", wiki, encoded_path);
        let wiki_page: Value = self.client
            .put(Some(project), &endpoint, &update_request)
            .await?;
            
        Ok(wiki_page)
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
        let mut run_request = json!({
            "resources": {
                "repositories": {
                    "self": {
                        "refName": "refs/heads/main" // Default to main branch
                    }
                }
            }
        });
        
        // Override branch if provided
        if let Some(branch) = args["branch"].as_str() {
            run_request["resources"]["repositories"]["self"]["refName"] = json!(format!("refs/heads/{}", branch));
        }
        
        // Add variables if provided
        if let Some(variables) = args["variables"].as_object() {
            let mut vars_map = serde_json::Map::new();
            
            for (key, value) in variables {
                vars_map.insert(key.clone(), json!({
                    "value": value,
                    "isSecret": false
                }));
            }
            
            run_request["variables"] = json!(vars_map);
        }
        
        // Run the pipeline
        let endpoint = format!("pipelines/{}/runs", pipeline_id);
        let pipeline_run: Value = self.client
            .post(Some(project), &endpoint, &run_request)
            .await?;
            
        Ok(pipeline_run)
    }
    
    /// Get pipeline status
    pub async fn get_pipeline_status(&self, args: Value) -> Result<Value> {
        let project = args["project"].as_str().ok_or_else(|| {
            CodexErr::Other("project parameter is required".to_string())
        })?;
        
        let build_id = args["build_id"].as_u64().ok_or_else(|| {
            CodexErr::Other("build_id parameter is required".to_string())
        })? as i32;
        
        // Get the build details
        let endpoint = format!("build/builds/{}", build_id);
        let build: Value = self.client
            .get(Some(project), &endpoint)
            .await?;
            
        Ok(build)
    }
}
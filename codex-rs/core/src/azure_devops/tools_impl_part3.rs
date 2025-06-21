//! Implementation of Azure DevOps tool functions (part 3).

use serde_json::{json, Value};
use crate::error::{CodexErr, Result};

use super::tools_impl::AzureDevOpsTools;

impl AzureDevOpsTools {
    /// Get a specific pull request
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
    
    /// Comment on a pull request
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
        
        let content = args["content"].as_str().ok_or_else(|| {
            CodexErr::Other("content parameter is required".to_string())
        })?;
        
        // Create the comment
        let comment_request = json!({
            "content": content,
            "commentType": 1  // 1 = text, 2 = code, 3 = system
        });
        
        // Post the comment
        let endpoint = format!(
            "git/repositories/{}/pullRequests/{}/threads", 
            repository, 
            pull_request_id
        );
        
        let comment_response: Value = self.client
            .post(Some(project), &endpoint, &comment_request)
            .await?;
            
        Ok(comment_response)
    }
    
    /// Get a wiki page
    pub async fn get_wiki_page(&self, args: Value) -> Result<Value> {
        let project = args["project"].as_str().ok_or_else(|| {
            CodexErr::Other("project parameter is required".to_string())
        })?;
        
        let wiki = args["wiki"].as_str().ok_or_else(|| {
            CodexErr::Other("wiki parameter is required".to_string())
        })?;
        
        let path = args["path"].as_str().ok_or_else(|| {
            CodexErr::Other("path parameter is required".to_string())
        })?;
        
        // URL encode the path
        let encoded_path = urlencoding::encode(path);
        
        // Get the wiki page
        let endpoint = format!("wiki/wikis/{}/pages?path={}&includeContent=true", wiki, encoded_path);
        let wiki_page: Value = self.client
            .get(Some(project), &endpoint)
            .await?;
            
        Ok(wiki_page)
    }
}
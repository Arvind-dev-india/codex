//! Data models for Azure DevOps entities.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Azure DevOps work item types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum WorkItemType {
    Bug,
    Epic,
    Feature,
    Issue,
    Task,
    UserStory,
    #[serde(other)]
    Other,
}

/// Azure DevOps work item state
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum WorkItemState {
    New,
    Active,
    Resolved,
    Closed,
    Removed,
    #[serde(other)]
    Other,
}

/// Azure DevOps work item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkItem {
    /// Work item ID
    pub id: i32,
    /// Work item revision
    pub rev: Option<i32>,
    /// Work item fields
    pub fields: HashMap<String, serde_json::Value>,
    /// Work item URL
    pub url: String,
}

/// Azure DevOps work item field operation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WorkItemFieldOperation {
    Add,
    Remove,
    Replace,
    Test,
    Copy,
    Move,
}

/// Azure DevOps work item field update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkItemFieldUpdate {
    /// Operation to perform
    pub op: WorkItemFieldOperation,
    /// Field path
    pub path: String,
    /// Field value
    pub value: serde_json::Value,
}

/// Azure DevOps work item query result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkItemQueryResult {
    /// Query result type
    #[serde(rename = "queryType")]
    pub query_type: String,
    /// Query result items
    #[serde(rename = "workItems")]
    pub work_items: Vec<WorkItemReference>,
}

/// Azure DevOps work item reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkItemReference {
    /// Work item ID
    pub id: i32,
    /// Work item URL
    pub url: String,
}

/// Azure DevOps pull request status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PullRequestStatus {
    Active,
    Abandoned,
    Completed,
    NotSet,
}

/// Azure DevOps pull request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequest {
    /// Pull request ID
    #[serde(rename = "pullRequestId")]
    pub pull_request_id: i32,
    /// Pull request title
    #[serde(rename = "title")]
    pub title: String,
    /// Pull request description
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Pull request status
    #[serde(rename = "status")]
    pub status: PullRequestStatus,
    /// Pull request created by
    #[serde(rename = "createdBy")]
    pub created_by: IdentityRef,
    /// Pull request creation date
    #[serde(rename = "creationDate")]
    pub creation_date: String,
    /// Pull request source branch
    #[serde(rename = "sourceRefName")]
    pub source_ref_name: String,
    /// Pull request target branch
    #[serde(rename = "targetRefName")]
    pub target_ref_name: String,
    /// Pull request URL
    #[serde(rename = "url")]
    pub url: String,
}

/// Azure DevOps identity reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityRef {
    /// Identity ID
    #[serde(rename = "id")]
    pub id: String,
    /// Identity display name
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// Identity unique name
    #[serde(rename = "uniqueName")]
    pub unique_name: Option<String>,
    /// Identity URL
    #[serde(rename = "url")]
    pub url: Option<String>,
    /// Identity image URL
    #[serde(rename = "imageUrl")]
    pub image_url: Option<String>,
}

/// Azure DevOps pull request comment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequestComment {
    /// Comment ID
    pub id: i32,
    /// Comment content
    pub content: String,
    /// Comment author
    pub author: IdentityRef,
    /// Comment creation date
    #[serde(rename = "publishedDate")]
    pub published_date: String,
    /// Comment last updated date
    #[serde(rename = "lastUpdatedDate")]
    pub last_updated_date: String,
}

/// Azure DevOps wiki page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiPage {
    /// Page ID
    pub id: i32,
    /// Page path
    pub path: String,
    /// Page content
    pub content: Option<String>,
    /// Page URL
    pub url: String,
    /// Page ETag
    #[serde(rename = "eTag")]
    pub etag: Option<String>,
    /// Page Git item path
    #[serde(rename = "gitItemPath")]
    pub git_item_path: Option<String>,
}

/// Azure DevOps build definition reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildDefinitionReference {
    /// Build definition ID
    pub id: i32,
    /// Build definition name
    pub name: String,
    /// Build definition URL
    pub url: String,
}

/// Azure DevOps build status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BuildStatus {
    None,
    InProgress,
    Completed,
    Cancelling,
    Postponed,
    NotStarted,
    All,
}

/// Azure DevOps build result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BuildResult {
    None,
    Succeeded,
    PartiallySucceeded,
    Failed,
    Canceled,
}

/// Azure DevOps build
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Build {
    /// Build ID
    pub id: i32,
    /// Build number
    #[serde(rename = "buildNumber")]
    pub build_number: String,
    /// Build status
    pub status: BuildStatus,
    /// Build result
    pub result: Option<BuildResult>,
    /// Build queue time
    #[serde(rename = "queueTime")]
    pub queue_time: String,
    /// Build start time
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,
    /// Build finish time
    #[serde(rename = "finishTime")]
    pub finish_time: Option<String>,
    /// Build URL
    pub url: String,
    /// Build definition reference
    #[serde(rename = "definition")]
    pub definition: BuildDefinitionReference,
}
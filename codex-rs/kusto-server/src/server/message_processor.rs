//! Message processor for handling MCP requests

use mcp_types::*;
use serde_json::json;
use tokio::sync::mpsc;
use tracing::{info, warn, error};

use crate::tool_config::create_kusto_tools;
use crate::kusto_bridge;

pub struct MessageProcessor {
    outgoing: mpsc::Sender<JSONRPCMessage>,
    initialized: bool,
}

impl MessageProcessor {
    pub fn new(outgoing: mpsc::Sender<JSONRPCMessage>) -> Self {
        Self {
            outgoing,
            initialized: false,
        }
    }

    pub async fn process_message(&mut self, message: JSONRPCMessage) {
        match message {
            JSONRPCMessage::Request(request) => {
                self.process_request(request).await;
            }
            JSONRPCMessage::Response(response) => {
                info!("Received response: {:?}", response);
            }
            JSONRPCMessage::Notification(notification) => {
                self.process_notification(notification).await;
            }
            JSONRPCMessage::BatchRequest(batch) => {
                for item in batch {
                    match item {
                        JSONRPCBatchRequestItem::JSONRPCRequest(req) => {
                            self.process_request(req).await;
                        }
                        JSONRPCBatchRequestItem::JSONRPCNotification(notif) => {
                            self.process_notification(notif).await;
                        }
                    }
                }
            }
            JSONRPCMessage::Error(error) => {
                error!("Received error: {:?}", error);
            }
            JSONRPCMessage::BatchResponse(batch) => {
                info!("Received batch response with {} items", batch.len());
            }
        }
    }

    async fn process_request(&mut self, request: JSONRPCRequest) {
        let request_id = request.id.clone();
        
        let client_request = match ClientRequest::try_from(request) {
            Ok(req) => req,
            Err(e) => {
                warn!("Failed to convert request: {e}");
                return;
            }
        };

        match client_request {
            ClientRequest::InitializeRequest(params) => {
                self.handle_initialize(request_id, params).await;
            }
            ClientRequest::ListToolsRequest(params) => {
                self.handle_list_tools(request_id, params).await;
            }
            ClientRequest::CallToolRequest(params) => {
                self.handle_call_tool(request_id, params).await;
            }
            ClientRequest::PingRequest(params) => {
                self.handle_ping(request_id, params).await;
            }
            _ => {
                // For now, we only support the essential requests
                self.send_error(request_id, -32601, "Method not found").await;
            }
        }
    }

    async fn process_notification(&mut self, _notification: JSONRPCNotification) {
        // Handle notifications if needed
    }

    async fn handle_initialize(&mut self, id: RequestId, params: InitializeRequestParams) {
        info!("Initialize request: {:?}", params);

        if self.initialized {
            self.send_error(id, -32600, "Already initialized").await;
            return;
        }

        self.initialized = true;

        let result = InitializeResult {
            capabilities: ServerCapabilities {
                tools: Some(ServerCapabilitiesTools {
                    list_changed: Some(true),
                }),
                completions: None,
                experimental: None,
                logging: None,
                prompts: None,
                resources: None,
            },
            instructions: Some("Kusto Server - Execute queries, explore schemas, and manage Azure Data Explorer databases".to_string()),
            protocol_version: params.protocol_version,
            server_info: Implementation {
                name: "kusto-server".to_string(),
                version: "0.1.0".to_string(),
            },
        };

        self.send_response(id, result).await;
    }

    async fn handle_list_tools(&self, id: RequestId, _params: Option<ListToolsRequestParams>) {
        let tools = create_kusto_tools();
        let result = ListToolsResult {
            tools,
            next_cursor: None,
        };

        self.send_response(id, result).await;
    }

    async fn handle_call_tool(&self, id: RequestId, params: CallToolRequestParams) {
        info!("Tool call: {} with args: {:?}", params.name, params.arguments);
        
        let arguments = params.arguments.unwrap_or(json!({}));
        let result = match kusto_bridge::call_kusto_tool(&params.name, arguments).await {
            Ok(result) => CallToolResult {
                content: vec![CallToolResultContent::TextContent(TextContent {
                    r#type: "text".to_string(),
                    text: serde_json::to_string_pretty(&result).unwrap_or_else(|_| result.to_string()),
                    annotations: None,
                })],
                is_error: Some(false),
            },
            Err(e) => CallToolResult {
                content: vec![CallToolResultContent::TextContent(TextContent {
                    r#type: "text".to_string(),
                    text: format!("Error calling Kusto tool: {}", e),
                    annotations: None,
                })],
                is_error: Some(true),
            },
        };

        self.send_response(id, result).await;
    }

    async fn handle_ping(&self, id: RequestId, _params: Option<serde_json::Value>) {
        self.send_response(id, json!({})).await;
    }

    async fn send_response<T: serde::Serialize>(&self, id: RequestId, result: T) {
        let response = JSONRPCMessage::Response(JSONRPCResponse {
            jsonrpc: JSONRPC_VERSION.to_string(),
            id,
            result: serde_json::to_value(result).unwrap_or(json!({})),
        });

        if let Err(e) = self.outgoing.send(response).await {
            error!("Failed to send response: {e}");
        }
    }

    async fn send_error(&self, id: RequestId, code: i64, message: &str) {
        let error = JSONRPCMessage::Error(JSONRPCError {
            jsonrpc: JSONRPC_VERSION.to_string(),
            id,
            error: JSONRPCErrorError {
                code,
                message: message.to_string(),
                data: None,
            },
        });

        if let Err(e) = self.outgoing.send(error).await {
            error!("Failed to send error: {e}");
        }
    }
}
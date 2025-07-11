//! MCP server implementation for code analysis tools

use anyhow::Result;
use mcp_types::JSONRPCMessage;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::mpsc;
use tracing::{debug, error, info};

// HTTP/SSE imports
use axum::{
    extract::State,
    http::header,
    response::{IntoResponse, Response, Sse},
    routing::{get, post},
    Json, Router,
};
use futures::stream::Stream;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use tower_http::cors::CorsLayer;

mod message_processor;

use message_processor::MessageProcessor;

/// Size of the bounded channels used to communicate between tasks
const CHANNEL_CAPACITY: usize = 128;

/// HTTP request structure for MCP calls
#[derive(Debug, Deserialize)]
struct McpRequest {
    jsonrpc: String,
    id: serde_json::Value,
    method: String,
    params: Option<serde_json::Value>,
}

/// HTTP notification structure for MCP calls (no id)
#[derive(Debug, Deserialize)]
struct McpNotification {
    jsonrpc: String,
    method: String,
    params: Option<serde_json::Value>,
}

/// HTTP response structure for MCP calls
#[derive(Debug, Serialize)]
struct McpResponse {
    jsonrpc: String,
    id: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<serde_json::Value>,
}

/// Server state for HTTP mode
#[derive(Clone)]
struct ServerState {
    processor: Arc<RwLock<message_processor::MessageProcessor>>,
    event_sender: broadcast::Sender<String>,
}

/// SSE event structure
#[derive(Debug, Serialize)]
struct SseEvent {
    event: String,
    data: serde_json::Value,
    timestamp: u64,
}

/// Run the MCP server using stdin/stdout
pub async fn run_server() -> Result<()> {
    info!("Starting Code Analysis MCP Server");
    
    // Set up channels for message passing
    let (incoming_tx, mut incoming_rx) = mpsc::channel::<JSONRPCMessage>(CHANNEL_CAPACITY);
    let (outgoing_tx, mut outgoing_rx) = mpsc::channel::<JSONRPCMessage>(CHANNEL_CAPACITY);

    // Task: read from stdin, push to incoming_tx
    let stdin_reader_handle = tokio::spawn({
        let incoming_tx = incoming_tx.clone();
        async move {
            let stdin = tokio::io::stdin();
            let reader = BufReader::new(stdin);
            let mut lines = reader.lines();

            while let Some(line) = lines.next_line().await.unwrap_or_default() {
                match serde_json::from_str::<JSONRPCMessage>(&line) {
                    Ok(msg) => {
                        if incoming_tx.send(msg).await.is_err() {
                            break;
                        }
                    }
                    Err(e) => error!("Failed to deserialize JSONRPCMessage: {e}"),
                }
            }
            debug!("stdin reader finished (EOF)");
        }
    });

    // Task: process incoming messages
    let processor_handle = tokio::spawn({
        let mut processor = MessageProcessor::new(outgoing_tx.clone());
        async move {
            while let Some(msg) = incoming_rx.recv().await {
                processor.process_message(msg).await;
            }
            info!("processor task exited (channel closed)");
        }
    });

    // Task: write outgoing messages to stdout
    let stdout_writer_handle = tokio::spawn(async move {
        let mut stdout = tokio::io::stdout();
        while let Some(msg) = outgoing_rx.recv().await {
            match serde_json::to_string(&msg) {
                Ok(json) => {
                    if let Err(e) = stdout.write_all(json.as_bytes()).await {
                        error!("Failed to write to stdout: {e}");
                        break;
                    }
                    if let Err(e) = stdout.write_all(b"\n").await {
                        error!("Failed to write newline to stdout: {e}");
                        break;
                    }
                    if let Err(e) = stdout.flush().await {
                        error!("Failed to flush stdout: {e}");
                        break;
                    }
                }
                Err(e) => error!("Failed to serialize JSONRPCMessage: {e}"),
            }
        }
        info!("stdout writer exited (channel closed)");
    });

    // Wait for all tasks to finish
    let _ = tokio::join!(stdin_reader_handle, processor_handle, stdout_writer_handle);

    Ok(())
}

/// Run the HTTP/SSE server for easier testing
pub async fn run_http_server(port: u16) -> Result<()> {
    info!("Starting Code Analysis HTTP/SSE Server on port {}", port);
    
    // Create broadcast channel for SSE events
    let (event_sender, _) = broadcast::channel::<String>(1000);
    
    // Create message processor with a dummy outgoing channel for HTTP mode
    let (outgoing_tx, mut outgoing_rx) = mpsc::channel::<JSONRPCMessage>(CHANNEL_CAPACITY);
    let processor = Arc::new(RwLock::new(message_processor::MessageProcessor::new(outgoing_tx)));
    
    // Task to handle outgoing messages (convert to SSE events)
    let event_sender_clone = event_sender.clone();
    tokio::spawn(async move {
        while let Some(msg) = outgoing_rx.recv().await {
            if let Ok(json) = serde_json::to_string(&msg) {
                let _ = event_sender_clone.send(json);
            }
        }
    });
    
    let state = ServerState {
        processor,
        event_sender,
    };
    
    // Build the router
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/mcp", post(mcp_handler))
        .route("/events", get(sse_handler))
        .route("/tools", get(tools_handler))
        .route("/health", get(health_handler))
        .route("/test", get(test_page_handler))
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(tower_http::trace::TraceLayer::new_for_http());
    
    // Start the server
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    info!("HTTP server listening on http://0.0.0.0:{}", port);
    info!("Test page available at: http://localhost:{}/test", port);
    info!("SSE events at: http://localhost:{}/events", port);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

/// Index handler - shows available endpoints
async fn index_handler() -> impl IntoResponse {
    let html = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Code Analysis MCP Server</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; }
        .endpoint { background: #f5f5f5; padding: 10px; margin: 10px 0; border-radius: 5px; }
        .method { color: #007acc; font-weight: bold; }
    </style>
</head>
<body>
    <h1>Code Analysis MCP Server</h1>
    <p>HTTP/SSE interface for the MCP Code Analysis Server</p>
    
    <h2>Available Endpoints:</h2>
    
    <div class="endpoint">
        <span class="method">GET</span> <code>/</code> - This page
    </div>
    
    <div class="endpoint">
        <span class="method">POST</span> <code>/mcp</code> - Send MCP JSON-RPC requests
    </div>
    
    <div class="endpoint">
        <span class="method">GET</span> <code>/events</code> - SSE stream of server events
    </div>
    
    <div class="endpoint">
        <span class="method">GET</span> <code>/tools</code> - List available tools
    </div>
    
    <div class="endpoint">
        <span class="method">GET</span> <code>/health</code> - Health check
    </div>
    
    <div class="endpoint">
        <span class="method">GET</span> <code>/test</code> - Interactive test page
    </div>
    
    <h2>Usage Examples:</h2>
    <pre>
# Initialize the server
curl -X POST http://localhost:3000/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":"init","method":"initialize","params":{"protocol_version":"2025-03-26","capabilities":{},"client_info":{"name":"test","version":"1.0"}}}'

# List available tools
curl -X POST http://localhost:3000/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":"tools","method":"tools/list","params":{}}'

# Get skeleton for a file
curl -X POST http://localhost:3000/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":"skeleton","method":"tools/call","params":{"name":"get_multiple_files_skeleton","arguments":{"file_paths":["path/to/file.cs"],"max_tokens":4000}}}'
    </pre>
</body>
</html>
    "#;
    
    Response::builder()
        .header(header::CONTENT_TYPE, "text/html")
        .body(html.to_string())
        .unwrap()
}

/// MCP JSON-RPC handler
use axum::body::Body;
use axum::body::Bytes;
use axum::http::StatusCode;

async fn mcp_handler(
    State(state): State<ServerState>,
    body: Body,
) -> impl IntoResponse {
    // Read the raw body for debugging
    let bytes = axum::body::to_bytes(body, 2 * 1024 * 1024).await.unwrap_or_else(|_| Bytes::new());
    let body_str = String::from_utf8_lossy(&bytes);
    info!("Raw /mcp request body: {}", body_str);

    // Try to parse as JSON
    let request_json: serde_json::Value = match serde_json::from_str(&body_str) {
        Ok(val) => val,
        Err(e) => {
            error!("Failed to parse JSON: {}", e);
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(serde_json::json!({
                    "error": format!("Failed to parse JSON: {}", e)
                }))
            );
        }
    };

    // Try to parse as McpRequest (with id)
    if let Ok(request) = serde_json::from_value::<McpRequest>(request_json.clone()) {
        info!("Received MCP request: {} - {}", request.method, request.id);

        // Convert HTTP request to JSONRPCMessage
        let request_id = match &request.id {
            serde_json::Value::String(s) => mcp_types::RequestId::String(s.clone()),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    mcp_types::RequestId::Integer(i)
                } else {
                    mcp_types::RequestId::String(n.to_string())
                }
            }
            _ => mcp_types::RequestId::String(request.id.to_string()),
        };
        
        let jsonrpc_request = mcp_types::JSONRPCRequest {
            id: request_id.clone(),
            jsonrpc: request.jsonrpc.clone(),
            method: request.method.clone(),
            params: request.params,
        };
        
        // Create a channel to capture the response
        let (response_tx, mut response_rx) = mpsc::channel::<JSONRPCMessage>(1);
        
        // Create a temporary processor that sends responses to our channel
        let mut temp_processor = message_processor::MessageProcessor::new(response_tx);
        
        // Process the message and wait for response
        let mcp_message = JSONRPCMessage::Request(jsonrpc_request);
        temp_processor.process_message(mcp_message).await;
        
        // Wait for the response with timeout (increased for large files)
        let response = match tokio::time::timeout(
            std::time::Duration::from_secs(120),  // Increased from 30 to 120 seconds
            response_rx.recv()
        ).await {
            Ok(Some(JSONRPCMessage::Response(resp))) => {
                // Return the actual tool result
                McpResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: Some(resp.result),
                    error: None,
                }
            }
            Ok(Some(JSONRPCMessage::Error(err))) => {
                // Return the error
                McpResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: None,
                    error: Some(serde_json::json!({
                        "code": err.error.code,
                        "message": err.error.message,
                        "data": err.error.data
                    })),
                }
            }
            Ok(_) => {
                // Unexpected message type
                McpResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: None,
                    error: Some(serde_json::json!({
                        "code": -32603,
                        "message": "Internal error: unexpected response type"
                    })),
                }
            }
            Err(_) => {
                // Timeout
                McpResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: None,
                    error: Some(serde_json::json!({
                        "code": -32603,
                        "message": "Internal error: request timeout"
                    })),
                }
            }
        };

        return (StatusCode::OK, Json(serde_json::to_value(response).unwrap()));
    }

    // Try to parse as McpNotification (without id)
    if let Ok(notification) = serde_json::from_value::<McpNotification>(request_json.clone()) {
        info!("Received MCP notification: {} (no id)", notification.method);
        // Optionally, handle known notifications here, or just return 200 OK
        return (
            StatusCode::OK,
            Json(serde_json::json!({
                "jsonrpc": "2.0",
                "result": "Notification received",
                "method": notification.method
            }))
        );
    }

    // If neither, return error
    error!("Failed to deserialize as McpRequest or McpNotification");
    (
        StatusCode::UNPROCESSABLE_ENTITY,
        Json(serde_json::json!({
            "error": "Failed to deserialize as JSON-RPC 2.0 request or notification",
            "hint": "Make sure your request includes 'jsonrpc' and 'method' fields.",
            "example_request": {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "initialize",
                "params": {}
            },
            "example_notification": {
                "jsonrpc": "2.0",
                "method": "notifications/initialized",
                "params": {}
            }
        }))
    )
}

/// SSE event stream handler
async fn sse_handler(
    State(state): State<ServerState>,
) -> Sse<impl Stream<Item = Result<axum::response::sse::Event, Infallible>>> {
    info!("New SSE connection established");
    
    let mut receiver = state.event_sender.subscribe();
    
    let stream = async_stream::stream! {
        // Send initial connection event
        let event = axum::response::sse::Event::default()
            .event("connected")
            .data("SSE connection established");
        yield Ok(event);
        
        // Stream events from the broadcast channel
        while let Ok(data) = receiver.recv().await {
            let event = axum::response::sse::Event::default()
                .event("mcp_response")
                .data(data);
            yield Ok(event);
        }
    };
    
    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(std::time::Duration::from_secs(30))
            .text("keep-alive"),
    )
}

/// Tools list handler
async fn tools_handler(State(_state): State<ServerState>) -> impl IntoResponse {
    // Return the available tools as JSON
    let tools = serde_json::json!({
        "tools": [
            {
                "name": "analyze_code",
                "description": "Analyze code structure and extract symbols from a file"
            },
            {
                "name": "find_symbol_references", 
                "description": "Find all references to a symbol in the codebase"
            },
            {
                "name": "find_symbol_definitions",
                "description": "Find the definition of a symbol in the codebase"
            },
            {
                "name": "get_symbol_subgraph",
                "description": "Get a subgraph of symbols related to a specific symbol"
            },
            {
                "name": "get_related_files_skeleton",
                "description": "Get skeleton views of files related to provided active files"
            },
            {
                "name": "get_multiple_files_skeleton",
                "description": "Get skeleton views of specified files"
            }
        ]
    });
    
    Json(tools)
}

/// Health check handler
async fn health_handler() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "server": "code-analysis-mcp-server",
        "version": "0.1.0",
        "timestamp": std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }))
}

/// Test page handler - interactive testing interface
async fn test_page_handler() -> impl IntoResponse {
    let html = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Code Analysis MCP Server - Test Page</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        .container { max-width: 1200px; margin: 0 auto; }
        .section { margin: 20px 0; padding: 20px; border: 1px solid #ddd; border-radius: 5px; }
        .button { background: #007acc; color: white; padding: 10px 20px; border: none; border-radius: 3px; cursor: pointer; margin: 5px; }
        .button:hover { background: #005a9e; }
        textarea { width: 100%; height: 150px; font-family: monospace; }
        .response { background: #f5f5f5; padding: 15px; border-radius: 3px; white-space: pre-wrap; font-family: monospace; }
        .events { height: 200px; overflow-y: auto; background: #f9f9f9; padding: 10px; border: 1px solid #ccc; }
        .event { margin: 5px 0; padding: 5px; background: white; border-radius: 3px; }
    </style>
</head>
<body>
    <div class="container">
        <h1>Code Analysis MCP Server - Test Interface</h1>
        
        <div class="section">
            <h2>Quick Tests</h2>
            <button class="button" onclick="testInitialize()">Initialize Server</button>
            <button class="button" onclick="testListTools()">List Tools</button>
            <button class="button" onclick="testSkeleton()">Test Skeleton Generation</button>
            <button class="button" onclick="testAnalyzeCode()">Analyze Code</button>
        </div>
        
        <div class="section">
            <h2>Custom Request</h2>
            <textarea id="requestInput" placeholder="Enter JSON-RPC request here...">
{
  "jsonrpc": "2.0",
  "id": "test",
  "method": "initialize",
  "params": {
    "protocol_version": "2025-03-26",
    "capabilities": {},
    "client_info": {"name": "test", "version": "1.0"}
  }
}
            </textarea>
            <br>
            <button class="button" onclick="sendCustomRequest()">Send Request</button>
        </div>
        
        <div class="section">
            <h2>Response</h2>
            <div id="response" class="response">Response will appear here...</div>
        </div>
        
        <div class="section">
            <h2>SSE Events</h2>
            <div id="events" class="events">Connecting to event stream...</div>
        </div>
    </div>

    <script>
        // SSE connection
        const eventSource = new EventSource('/events');
        const eventsDiv = document.getElementById('events');
        
        eventSource.onmessage = function(event) {
            const eventDiv = document.createElement('div');
            eventDiv.className = 'event';
            eventDiv.textContent = new Date().toLocaleTimeString() + ': ' + event.data;
            eventsDiv.appendChild(eventDiv);
            eventsDiv.scrollTop = eventsDiv.scrollHeight;
        };
        
        eventSource.onerror = function(event) {
            const eventDiv = document.createElement('div');
            eventDiv.className = 'event';
            eventDiv.textContent = 'SSE Error: ' + JSON.stringify(event);
            eventsDiv.appendChild(eventDiv);
        };
        
        // Helper function to send requests
        async function sendRequest(data) {
            try {
                const response = await fetch('/mcp', {
                    method: 'POST',
                    headers: {'Content-Type': 'application/json'},
                    body: JSON.stringify(data)
                });
                const result = await response.json();
                document.getElementById('response').textContent = JSON.stringify(result, null, 2);
            } catch (error) {
                document.getElementById('response').textContent = 'Error: ' + error.message;
            }
        }
        
        // Test functions
        function testInitialize() {
            sendRequest({
                jsonrpc: "2.0",
                id: "init",
                method: "initialize",
                params: {
                    protocol_version: "2025-03-26",
                    capabilities: {},
                    client_info: {name: "test", version: "1.0"}
                }
            });
        }
        
        function testListTools() {
            sendRequest({
                jsonrpc: "2.0",
                id: "tools",
                method: "tools/list",
                params: {}
            });
        }
        
        function testSkeleton() {
            sendRequest({
                jsonrpc: "2.0",
                id: "skeleton",
                method: "tools/call",
                params: {
                    name: "get_multiple_files_skeleton",
                    arguments: {
                        file_paths: ["codex-rs/test_files/csharp_test_suite/BasicClass.cs"],
                        max_tokens: 4000
                    }
                }
            });
        }
        
        function testAnalyzeCode() {
            sendRequest({
                jsonrpc: "2.0",
                id: "analyze",
                method: "tools/call",
                params: {
                    name: "analyze_code",
                    arguments: {
                        file_path: "codex-rs/test_files/csharp_test_suite/BasicClass.cs"
                    }
                }
            });
        }
        
        function sendCustomRequest() {
            try {
                const requestText = document.getElementById('requestInput').value;
                const requestData = JSON.parse(requestText);
                sendRequest(requestData);
            } catch (error) {
                document.getElementById('response').textContent = 'Invalid JSON: ' + error.message;
            }
        }
    </script>
</body>
</html>
    "#;
    
    Response::builder()
        .header(header::CONTENT_TYPE, "text/html")
        .body(html.to_string())
        .unwrap()
}

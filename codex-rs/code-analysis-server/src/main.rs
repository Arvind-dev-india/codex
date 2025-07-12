use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

mod server;
mod tool_config;
mod code_analysis_bridge;

/// Standalone Code Analysis Server using the MCP protocol
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Project directory to analyze
    #[arg(short, long, value_name = "DIR")]
    project_dir: Option<PathBuf>,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
    
    /// Port to listen on for HTTP/SSE server (0 = stdio mode)
    #[arg(short, long, default_value = "0")]
    port: u16,
    
    /// Enable SSE (Server-Sent Events) mode for easier testing
    #[arg(long)]
    sse: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Configure logging based on verbosity
    let log_level = if args.verbose { "debug" } else { "info" };
    std::env::set_var("RUST_LOG", format!("code_analysis_server={},codex_core={}", log_level, log_level));
    
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();
    
    // Set the working directory if provided
    if let Some(project_dir) = args.project_dir.as_ref() {
        std::env::set_current_dir(project_dir)?;
        tracing::info!("Set working directory to: {}", project_dir.display());
    }
    
    // Log current directory
    let current_dir = std::env::current_dir()?;
    tracing::info!("Current working directory: {}", current_dir.display());
    
    // Initialize code graph and wait for it to complete before starting server
    tracing::info!("Starting server with async graph initialization...");
    
    // Create a shared state to track graph initialization
    let graph_ready = std::sync::Arc::new(tokio::sync::Notify::new());
    let graph_ready_clone = graph_ready.clone();
    
    // Spawn graph initialization in background
    tokio::spawn(async move {
        tracing::info!("Initializing code graph in background...");
        if let Err(e) = code_analysis_bridge::init_code_graph_and_wait(None).await {
            tracing::error!("Failed to initialize code graph: {}", e);
        } else {
            tracing::info!("Code graph is ready for use");
            graph_ready_clone.notify_waiters(); // Signal that graph is ready
        }
    });
    
    // Run the server
    if args.sse || args.port > 0 {
        // HTTP/SSE mode for easier testing
        let port = if args.port > 0 { args.port } else { 3000 };
        server::run_http_server(port).await?;
    } else {
        // Standard MCP mode (stdin/stdout)
        server::run_server_with_graph_ready(graph_ready).await?;
    }
    
    Ok(())
}

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
    std::env::set_var("RUST_LOG", format!("code_analysis_server={}", log_level));
    
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();
    
    // Set the working directory if provided
    if let Some(project_dir) = args.project_dir.as_ref() {
        std::env::set_current_dir(project_dir)?;
        tracing::info!("Set working directory to: {}", project_dir.display());
    }
    
    // Run the server
    if args.sse || args.port > 0 {
        // HTTP/SSE mode for easier testing
        let port = if args.port > 0 { args.port } else { 3000 };
        server::run_http_server(port).await?;
    } else {
        // Standard MCP mode (stdin/stdout)
        server::run_server().await?;
    }
    
    Ok(())
}
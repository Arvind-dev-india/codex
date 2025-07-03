use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

mod server;
mod tool_config;
mod recovery_services_bridge;

/// Standalone Recovery Services (Azure Backup) Server using the MCP protocol
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Configuration file path (optional - will use main codex config by default)
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
    
    /// Port to listen on (if running as a network service)
    #[arg(short, long, default_value = "0")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Configure logging based on verbosity
    let log_level = if args.verbose { "debug" } else { "info" };
    std::env::set_var("RUST_LOG", format!("recovery_services_server={}", log_level));
    
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();
    
    // Initialize Recovery Services configuration
    if let Some(config_path) = args.config.as_ref() {
        recovery_services_bridge::init_config(config_path)?;
        tracing::info!("Loaded configuration from: {}", config_path.display());
    } else {
        // Try to load from main codex config first, then fallback to other locations
        recovery_services_bridge::init_default_config()?;
    }
    
    // Run the server
    if args.port > 0 {
        // Network mode (for future implementation)
        tracing::info!("Network mode not yet implemented");
        return Ok(());
    } else {
        // Standard MCP mode (stdin/stdout)
        server::run_server().await?;
    }
    
    Ok(())
}
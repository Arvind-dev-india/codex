use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod server;
mod tool_config;
mod azure_devops_bridge;

/// Standalone Azure DevOps Server using the MCP protocol
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
    
    /// Authentication and server commands
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Start the MCP server (default behavior)
    Serve,
    /// Authenticate with Azure DevOps using OAuth device code flow
    Login,
    /// Clear stored authentication tokens and log out
    Logout,
    /// Check current authentication status and token validity
    Status,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Configure logging based on verbosity
    let log_level = if args.verbose { "debug" } else { "info" };
    std::env::set_var("RUST_LOG", format!("azure_devops_server={}", log_level));
    
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();
    
    // Handle authentication commands first (don't need config for some)
    match args.command {
        Some(Commands::Login) => {
            return handle_login().await;
        }
        Some(Commands::Logout) => {
            return handle_logout().await;
        }
        Some(Commands::Status) => {
            return handle_status().await;
        }
        Some(Commands::Serve) | None => {
            // Continue to server startup
        }
    }
    
    // Initialize Azure DevOps configuration for server mode
    if let Some(config_path) = args.config.as_ref() {
        azure_devops_bridge::init_config(config_path)?;
        tracing::info!("Loaded configuration from: {}", config_path.display());
    } else {
        // Try to load from main codex config first, then fallback to other locations
        azure_devops_bridge::init_default_config()?;
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

/// Handle login command
async fn handle_login() -> Result<()> {
    use codex_core::azure_devops::auth::oauth_auth::AzureDevOpsOAuthHandler;
    
    let codex_home = get_codex_home()?;
    let oauth_handler = AzureDevOpsOAuthHandler::new(&codex_home);
    
    println!("Starting Azure DevOps authentication...");
    
    match oauth_handler.get_access_token().await {
        Ok(_) => {
            println!("Successfully authenticated with Azure DevOps!");
            println!("Tokens stored in: {}/.codex/azure_devops_auth.json", codex_home.display());
            println!("You can now use the Azure DevOps MCP server tools.");
        }
        Err(e) => {
            eprintln!("Authentication failed: {}", e);
            std::process::exit(1);
        }
    }
    
    Ok(())
}

/// Handle logout command
async fn handle_logout() -> Result<()> {
    use codex_core::azure_devops::auth::oauth_auth::AzureDevOpsOAuthHandler;
    
    let codex_home = get_codex_home()?;
    let oauth_handler = AzureDevOpsOAuthHandler::new(&codex_home);
    
    match oauth_handler.logout().await {
        Ok(_) => {
            println!("Successfully logged out from Azure DevOps.");
            println!("Authentication tokens have been cleared.");
        }
        Err(e) => {
            eprintln!("Logout failed: {}", e);
            std::process::exit(1);
        }
    }
    
    Ok(())
}

/// Handle status command
async fn handle_status() -> Result<()> {
    use codex_core::azure_devops::auth::oauth_auth::AzureDevOpsOAuthHandler;
    
    let codex_home = get_codex_home()?;
    let oauth_handler = AzureDevOpsOAuthHandler::new(&codex_home);
    
    println!("Checking Azure DevOps authentication status...");
    
    match oauth_handler.get_token_status().await {
        Ok(status) => {
            if status.authenticated {
                println!("Status: Authenticated with Azure DevOps");
                println!("Token location: {}/.codex/azure_devops_auth.json", codex_home.display());
                
                if let Some(created_at) = status.created_at {
                    println!("Token created: {} ({} days ago)", 
                        created_at.format("%Y-%m-%d %H:%M:%S UTC"),
                        status.token_age_days);
                }
                
                if let Some(access_expires_at) = status.access_expires_at {
                    println!("Access token: {} (expires: {})", 
                        if status.access_token_valid { "Valid" } else { "Expired" },
                        access_expires_at.format("%Y-%m-%d %H:%M:%S UTC"));
                    
                    if status.access_expires_in_minutes > 0 {
                        println!("  - Expires in: {} minutes", status.access_expires_in_minutes);
                    } else {
                        println!("  - Expired {} minutes ago", -status.access_expires_in_minutes);
                    }
                }
                
                if let Some(refresh_expires_at) = status.refresh_expires_at {
                    println!("Refresh token: {} (expires: {})", 
                        if status.refresh_token_valid { "Valid" } else { "Expired" },
                        refresh_expires_at.format("%Y-%m-%d %H:%M:%S UTC"));
                    
                    if status.refresh_expires_in_days > 0 {
                        println!("  - Expires in: {} days", status.refresh_expires_in_days);
                    } else {
                        println!("  - Expired {} days ago", -status.refresh_expires_in_days);
                    }
                }
                
                if !status.access_token_valid {
                    println!("Recommendation: Run 'azure-devops-server login' to refresh authentication");
                }
            } else {
                println!("Status: Not authenticated with Azure DevOps");
                println!("Token location: {}/.codex/azure_devops_auth.json", codex_home.display());
                println!("Recommendation: Run 'azure-devops-server login' to authenticate");
            }
        }
        Err(e) => {
            println!("Failed to check authentication status: {}", e);
        }
    }
    
    Ok(())
}

/// Get the codex home directory
fn get_codex_home() -> Result<std::path::PathBuf> {
    let home_dir = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| anyhow::anyhow!("Could not determine home directory"))?;
    
    Ok(std::path::PathBuf::from(home_dir).join(".codex"))
}
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
    
    match oauth_handler.get_access_token().await {
        Ok(token) => {
            // Try to make a test API call
            let client = reqwest::Client::new();
            let test_url = "https://app.vssps.visualstudio.com/_apis/profile/profiles/me?api-version=6.0";
            
            match client.get(test_url).bearer_auth(&token).send().await {
                Ok(resp) if resp.status().is_success() => {
                    println!("Authentication is valid and working");
                    println!("Token location: {}/.codex/azure_devops_auth.json", codex_home.display());
                    println!("API test: Successful");
                }
                Ok(resp) => {
                    println!("Authentication token exists but API test failed");
                    println!("Token location: {}/.codex/azure_devops_auth.json", codex_home.display());
                    println!("API status: {}", resp.status());
                    println!("Recommendation: Try running 'azure-devops-server logout' and 'azure-devops-server login'");
                }
                Err(e) => {
                    println!("Authentication token exists but network test failed");
                    println!("Token location: {}/.codex/azure_devops_auth.json", codex_home.display());
                    println!("Network error: {}", e);
                    println!("Recommendation: Check network connectivity");
                }
            }
        }
        Err(_) => {
            println!("Not authenticated with Azure DevOps");
            println!("Token location: {}/.codex/azure_devops_auth.json", codex_home.display());
            println!("Recommendation: Run 'azure-devops-server login' to authenticate");
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
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod server;
mod tool_config;
mod kusto_bridge;

/// Standalone Kusto (Azure Data Explorer) Server using the MCP protocol
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
    /// Authenticate with Kusto (Azure Data Explorer) using OAuth device code flow
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
    std::env::set_var("RUST_LOG", format!("kusto_server={}", log_level));
    
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
    
    // Initialize Kusto configuration for server mode
    if let Some(config_path) = args.config.as_ref() {
        kusto_bridge::init_config(config_path)?;
        tracing::info!("Loaded configuration from: {}", config_path.display());
    } else {
        // Try to load from main codex config first, then fallback to other locations
        kusto_bridge::init_default_config()?;
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
    use codex_core::kusto::auth::KustoOAuthHandler;
    
    let codex_home = get_codex_home()?;
    let oauth_handler = KustoOAuthHandler::new(&codex_home);
    
    println!("Starting Kusto (Azure Data Explorer) authentication...");
    
    match oauth_handler.get_access_token().await {
        Ok(_) => {
            println!("Successfully authenticated with Kusto!");
            println!("Tokens stored in: {}/.codex/kusto_auth.json", codex_home.display());
            println!("You can now use the Kusto MCP server tools.");
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
    use codex_core::kusto::auth::KustoOAuthHandler;
    
    let codex_home = get_codex_home()?;
    let oauth_handler = KustoOAuthHandler::new(&codex_home);
    
    match oauth_handler.logout().await {
        Ok(_) => {
            println!("Successfully logged out from Kusto.");
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
    let codex_home = get_codex_home()?;
    let auth_file = codex_home.join("kusto_auth.json");
    
    println!("Checking Kusto authentication status...");
    
    if !auth_file.exists() {
        println!("Not authenticated with Kusto");
        println!("Token location: {}", auth_file.display());
        println!("Recommendation: Run 'kusto-server login' to authenticate");
        return Ok(());
    }
    
    // Try to read and parse the token file
    match std::fs::read_to_string(&auth_file) {
        Ok(content) => {
            match serde_json::from_str::<serde_json::Value>(&content) {
                Ok(token_data) => {
                    if let Some(expires_at_str) = token_data.get("expires_at").and_then(|v| v.as_str()) {
                        match chrono::DateTime::parse_from_rfc3339(expires_at_str) {
                            Ok(expires_at) => {
                                let now = chrono::Utc::now();
                                if expires_at.with_timezone(&chrono::Utc) > now {
                                    println!("Authentication token exists and appears valid");
                                    println!("Token location: {}", auth_file.display());
                                    println!("Expires at: {}", expires_at_str);
                                    println!("Status: Valid (expires in {} minutes)", 
                                        (expires_at.with_timezone(&chrono::Utc) - now).num_minutes());
                                } else {
                                    println!("Authentication token exists but has expired");
                                    println!("Token location: {}", auth_file.display());
                                    println!("Expired at: {}", expires_at_str);
                                    println!("Recommendation: Run 'kusto-server login' to re-authenticate");
                                }
                            }
                            Err(_) => {
                                println!("Authentication token exists but expiration date is invalid");
                                println!("Token location: {}", auth_file.display());
                                println!("Recommendation: Run 'kusto-server logout' and 'kusto-server login'");
                            }
                        }
                    } else {
                        println!("Authentication token exists but is missing expiration information");
                        println!("Token location: {}", auth_file.display());
                        println!("Recommendation: Run 'kusto-server logout' and 'kusto-server login'");
                    }
                }
                Err(_) => {
                    println!("Authentication token file exists but is corrupted");
                    println!("Token location: {}", auth_file.display());
                    println!("Recommendation: Run 'kusto-server logout' and 'kusto-server login'");
                }
            }
        }
        Err(_) => {
            println!("Authentication token file exists but cannot be read");
            println!("Token location: {}", auth_file.display());
            println!("Recommendation: Check file permissions or run 'kusto-server logout' and 'kusto-server login'");
        }
    }
    
    Ok(())
}

/// Handle status command (old implementation - kept for reference)
async fn _handle_status_with_api_test() -> Result<()> {
    use codex_core::kusto::auth::KustoOAuthHandler;
    
    let codex_home = get_codex_home()?;
    let oauth_handler = KustoOAuthHandler::new(&codex_home);
    
    println!("Checking Kusto authentication status...");
    
    match oauth_handler.get_access_token().await {
        Ok(token) => {
            // Try to make a test API call
            let client = reqwest::Client::new();
            let test_url = "https://help.kusto.windows.net/v1/rest/mgmt";
            let test_query = serde_json::json!({
                "csl": ".show version",
                "db": "Samples"
            });
            
            match client.post(test_url).bearer_auth(&token).json(&test_query).send().await {
                Ok(resp) if resp.status().is_success() => {
                    println!("Authentication is valid and working");
                    println!("Token location: {}/.codex/kusto_auth.json", codex_home.display());
                    println!("API test: Successful");
                }
                Ok(resp) => {
                    println!("Authentication token exists but API test failed");
                    println!("Token location: {}/.codex/kusto_auth.json", codex_home.display());
                    println!("API status: {}", resp.status());
                    println!("Recommendation: Try running 'kusto-server logout' and 'kusto-server login'");
                }
                Err(e) => {
                    println!("Authentication token exists but network test failed");
                    println!("Token location: {}/.codex/kusto_auth.json", codex_home.display());
                    println!("Network error: {}", e);
                    println!("Recommendation: Check network connectivity");
                }
            }
        }
        Err(_) => {
            println!("Not authenticated with Kusto");
            println!("Token location: {}/.codex/kusto_auth.json", codex_home.display());
            println!("Recommendation: Run 'kusto-server login' to authenticate");
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
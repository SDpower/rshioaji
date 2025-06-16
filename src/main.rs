use clap::Parser;
use log::info;
use rshioaji::{Shioaji, Exchange, Action, OrderType, StockPriceType, Config};
use std::collections::HashMap;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Run in simulation mode
    #[arg(short, long)]
    simulation: bool,
    
    /// API key for authentication (can also use SHIOAJI_API_KEY env var)
    #[arg(long)]
    api_key: Option<String>,
    
    /// Secret key for authentication (can also use SHIOAJI_SECRET_KEY env var)
    #[arg(long)]
    secret_key: Option<String>,
    
    /// Path to .env file (default: .env)
    #[arg(long)]
    env_file: Option<String>,
    
    /// Stock code to query
    #[arg(long)]
    stock: Option<String>,
    
    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    // Initialize logger
    if cli.debug {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else {
        env_logger::init();
    }
    
    info!("Starting rshioaji CLI");
    
    // Load configuration from environment variables or .env file
    let config = if let Some(env_file) = &cli.env_file {
        info!("Loading configuration from .env file: {}", env_file);
        Config::from_env_file(env_file)
    } else {
        Config::from_env()
    };
    
    // Use command line arguments if provided, otherwise use environment variables
    let (api_key, secret_key, simulation) = match (&cli.api_key, &cli.secret_key) {
        (Some(api), Some(secret)) => {
            info!("Using API credentials from command line arguments");
            (api.clone(), secret.clone(), cli.simulation)
        }
        _ => {
            match config {
                Ok(cfg) => {
                    info!("Using API credentials from environment variables");
                    info!("Configuration: {}", cfg.summary());
                    if let Err(e) = cfg.validate() {
                        eprintln!("Configuration validation failed: {}", e);
                        std::process::exit(1);
                    }
                    (cfg.api_key, cfg.secret_key, cfg.simulation)
                }
                Err(e) => {
                    eprintln!("Failed to load configuration: {}", e);
                    eprintln!("Please provide API credentials via:");
                    eprintln!("  1. Command line: --api-key YOUR_KEY --secret-key YOUR_SECRET");
                    eprintln!("  2. Environment variables: SHIOAJI_API_KEY and SHIOAJI_SECRET_KEY");
                    eprintln!("  3. .env file with the above variables");
                    std::process::exit(1);
                }
            }
        }
    };
    
    // Create Shioaji client
    let proxies = HashMap::new();
    let client = Shioaji::new(simulation, proxies)?;
    
    // Initialize the client
    client.init().await?;
    info!("Shioaji client initialized");
    
    // Login with credentials
    if !api_key.is_empty() && !secret_key.is_empty() {
        info!("Logging in...");
        let accounts = client.login(&api_key, &secret_key, true).await?;
        info!("Login successful! Found {} accounts", accounts.len());
        
        for account in &accounts {
            info!(
                "Account: {} ({}), Type: {:?}, Signed: {}",
                account.account_id, account.username, account.account_type, account.signed
            );
        }
        
        // If stock code provided, demonstrate some functionality
        if let Some(stock_code) = cli.stock {
            info!("Fetching data for stock: {}", stock_code);
            
            // Create stock contract
            let stock = client.create_stock(&stock_code, Exchange::TSE);
            
            // Subscribe to market data
            if let Err(e) = client.subscribe(stock.contract.clone(), rshioaji::QuoteType::Tick).await {
                log::warn!("Failed to subscribe to market data: {}", e);
            }
            
            // Get historical K-bar data
            let end_date = chrono::Utc::now().format("%Y-%m-%d").to_string();
            let start_date = (chrono::Utc::now() - chrono::Duration::days(30))
                .format("%Y-%m-%d")
                .to_string();
            
            match client.kbars(stock.contract.clone(), &start_date, &end_date).await {
                Ok(kbars) => {
                    info!("Fetched {} K-bars for {}", kbars.data.len(), stock_code);
                    if let Some(latest) = kbars.data.last() {
                        info!(
                            "Latest: Open={}, High={}, Low={}, Close={}, Volume={}",
                            latest.open, latest.high, latest.low, latest.close, latest.volume
                        );
                    }
                }
                Err(e) => log::warn!("Failed to fetch K-bars: {}", e),
            }
            
            // Demonstrate order creation (not placing it)
            let order = rshioaji::Order::new(
                Action::Buy,
                100.0,  // price
                1000,   // quantity
                OrderType::ROD,
                StockPriceType::LMT,
            );
            
            info!("Created sample order: {:?}", order);
            info!("Order would buy {} shares at {} per share", order.quantity, order.price);
        }
        
        // List accounts
        let accounts = client.list_accounts().await?;
        info!("Total accounts: {}", accounts.len());
        
        // Logout
        info!("Logging out...");
        let logout_success = client.logout().await?;
        if logout_success {
            info!("Logout successful");
        } else {
            log::warn!("Logout may have failed");
        }
    } else {
        info!("No valid credentials found.");
        info!("Please provide API credentials via:");
        info!("  1. Command line: --api-key YOUR_KEY --secret-key YOUR_SECRET");
        info!("  2. Environment variables: SHIOAJI_API_KEY and SHIOAJI_SECRET_KEY");
        info!("  3. .env file with the above variables");
        info!("Example usage:");
        info!("  rshioaji-cli --simulation --api-key YOUR_KEY --secret-key YOUR_SECRET --stock 2330");
    }
    
    Ok(())
}


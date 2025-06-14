use rshioaji::{Config, Shioaji, Exchange};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::init();

    println!("🔧 rshioaji Environment Configuration Example");
    println!();

    // Check if .env file exists before loading
    if std::path::Path::new(".env").exists() {
        println!("📁 Found .env file in current directory");
    } else {
        println!("⚠️  No .env file found in current directory");
    }
    
    println!("📋 Attempting to load configuration from .env file and environment variables...");
    println!();

    // Try to load configuration from environment variables
    match Config::from_env() {
        Ok(config) => {
            println!("✅ Successfully loaded configuration from environment");
            println!("📋 {}", config.summary());
            
            // Validate the configuration
            if let Err(e) = config.validate() {
                eprintln!("❌ Configuration validation failed: {}", e);
                return Ok(());
            }
            
            println!("✅ Configuration validated successfully");
            println!();

            // Create Shioaji client using the configuration
            let proxies = HashMap::new();
            let client = Shioaji::new(config.simulation, proxies)?;
            
            println!("🚀 Initializing Shioaji client...");
            client.init().await?;
            println!("✅ Client initialized successfully");
            
            // Login using the configuration
            println!("🔐 Logging in with environment credentials...");
            match client.login(&config.api_key, &config.secret_key, true).await {
                Ok(accounts) => {
                    println!("✅ Login successful! Found {} accounts", accounts.len());
                    
                    for account in &accounts {
                        println!(
                            "👤 Account: {} ({}), Type: {:?}, Signed: {}",
                            account.account_id, account.username, account.account_type, account.signed
                        );
                    }
                    
                    // Demonstrate some basic functionality
                    println!();
                    println!("📈 Testing basic functionality...");
                    
                    // Create a stock contract
                    let tsmc = client.create_stock("2330", Exchange::TSE);
                    println!("📊 Created contract for TSMC (2330): {:?}", tsmc.contract);
                    
                    // Try to subscribe to market data
                    if let Err(e) = client.subscribe(tsmc.contract.clone(), rshioaji::QuoteType::Tick).await {
                        println!("⚠️  Market data subscription failed (expected in demo): {}", e);
                    }
                    
                    // List all accounts
                    let all_accounts = client.list_accounts().await?;
                    println!("📋 Total accounts available: {}", all_accounts.len());
                    
                    // Logout
                    println!();
                    println!("🔒 Logging out...");
                    let logout_success = client.logout().await?;
                    if logout_success {
                        println!("✅ Logout successful");
                    } else {
                        println!("⚠️  Logout may have failed");
                    }
                }
                Err(e) => {
                    println!("❌ Login failed: {}", e);
                    println!("💡 This might be expected if using example credentials");
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to load configuration: {}", e);
            println!();
            println!("💡 To use this example:");
            println!("   1. Copy .env.example to .env");
            println!("   2. Fill in your actual API credentials in .env");
            println!("   3. Run this example again");
            println!();
            println!("📋 Expected .env format:");
            println!("   SHIOAJI_API_KEY=your_actual_api_key");
            println!("   SHIOAJI_SECRET_KEY=your_actual_secret_key");
            println!("   SHIOAJI_SIMULATION=true");
            println!();
            println!("🔄 Alternative: Set environment variables directly:");
            println!("   export SHIOAJI_API_KEY=your_key");
            println!("   export SHIOAJI_SECRET_KEY=your_secret");
            println!("   cargo run --example env_config_example");
        }
    }

    Ok(())
}
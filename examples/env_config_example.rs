use rshioaji::{Config, Exchange, Shioaji};
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
            match client
                .login_simple(&config.api_key, &config.secret_key, true)
                .await
            {
                Ok(accounts) => {
                    println!("✅ Login successful! Found {} accounts", accounts.len());

                    for account in &accounts {
                        println!(
                            "👤 Account: {} ({}), Type: {:?}, Signed: {}",
                            account.account_id,
                            account.username,
                            account.account_type,
                            account.signed
                        );
                    }

                    // Demonstrate some basic functionality
                    println!();
                    println!("📈 Testing basic functionality...");

                    // Create a TXFG5 futures contract
                    let txfg5_future = client.create_future("TXFG5", Exchange::TAIFEX);
                    println!(
                        "📊 Created contract for TXFG5 future: {:?}",
                        txfg5_future.contract
                    );

                    // Try to subscribe to market data
                    if let Err(e) = client
                        .subscribe(txfg5_future.contract.clone(), "tick")
                        .await
                    {
                        println!(
                            "⚠️  Market data subscription failed (expected in demo): {}",
                            e
                        );
                    }

                    // Check login status
                    let logged_in = client.is_logged_in().await;
                    println!(
                        "📋 Login status: {}",
                        if logged_in {
                            "Logged in"
                        } else {
                            "Not logged in"
                        }
                    );

                    // Show current functionality
                    println!();
                    println!("🎉 Environment configuration example completed successfully!");
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

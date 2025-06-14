use rshioaji::{Shioaji, Exchange, Action, OrderType, StockPriceType, QuoteType};
use std::collections::HashMap;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::init();
    
    // Show platform information
    let platform = rshioaji::platform::Platform::detect();
    println!("🖥️  Detected platform: {:?}", platform);
    
    if let Some(platform_dir) = platform.directory_name() {
        println!("📁 Using platform directory: {}", platform_dir);
        
        // Validate installation
        let base_path = std::env::current_dir()?;
        match platform.validate_installation(&base_path) {
            Ok(()) => println!("✅ Platform installation validated successfully"),
            Err(e) => {
                println!("❌ Platform validation failed: {}", e);
                println!("💡 Please ensure you have the correct shioaji libraries for your platform");
                return Ok(());
            }
        }
    } else {
        println!("❌ Unsupported platform");
        return Ok(());
    }
    
    // Create Shioaji client in simulation mode
    let proxies = HashMap::new();
    let client = Shioaji::new(true, proxies)?;
    
    // Initialize the client
    client.init().await?;
    println!("✅ Shioaji client initialized successfully");
    
    // Note: Replace with your actual API credentials
    let api_key = "YOUR_API_KEY";
    let secret_key = "YOUR_SECRET_KEY";
    
    // Login (comment out if you don't have credentials)
    /*
    println!("🔑 Logging in...");
    let accounts = client.login(api_key, secret_key, true).await?;
    println!("✅ Login successful! Found {} accounts", accounts.len());
    
    for account in &accounts {
        println!(
            "📊 Account: {} ({}), Type: {:?}, Signed: {}",
            account.account_id, account.username, account.account_type, account.signed
        );
    }
    */
    
    // Create some sample contracts
    println!("\n📈 Creating sample contracts...");
    
    // Taiwan Semiconductor (2330)
    let tsmc = client.create_stock("2330", Exchange::TSE);
    println!("Created TSMC stock contract: {}", tsmc.contract.base.code);
    
    // TAIEX Future
    let taiex_future = client.create_future("TXFA4");
    println!("Created TAIEX future contract: {}", taiex_future.contract.base.code);
    
    // Create sample orders (not actually placing them)
    println!("\n📝 Creating sample orders...");
    
    let stock_order = rshioaji::Order::new(
        Action::Buy,
        500.0,      // price: NT$500
        1000,       // quantity: 1 lot (1000 shares)
        OrderType::ROD,
        StockPriceType::LMT,
    );
    println!("📦 Stock order: {:?}", stock_order);
    
    let futures_order = rshioaji::FuturesOrder::new(
        Action::Buy,
        17000.0,    // price
        1,          // quantity: 1 contract
        OrderType::ROD,
        rshioaji::FuturesPriceType::LMT,
        rshioaji::FuturesOCType::Auto,
    );
    println!("🔮 Futures order: {:?}", futures_order);
    
    // Demonstrate market data subscription (will only work if logged in)
    /*
    println!("\n📡 Subscribing to market data...");
    if let Err(e) = client.subscribe(tsmc.contract.clone(), QuoteType::Tick).await {
        println!("⚠️  Market data subscription failed: {}", e);
    } else {
        println!("✅ Subscribed to TSMC tick data");
    }
    
    // Get historical data
    println!("\n📊 Fetching historical data...");
    let end_date = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let start_date = (chrono::Utc::now() - chrono::Duration::days(7))
        .format("%Y-%m-%d")
        .to_string();
    
    match client.kbars(tsmc.contract.clone(), &start_date, &end_date).await {
        Ok(kbars) => {
            println!("✅ Fetched {} K-bars for TSMC", kbars.data.len());
            if let Some(latest) = kbars.data.last() {
                println!(
                    "📈 Latest TSMC data: Open={}, High={}, Low={}, Close={}, Volume={}",
                    latest.open, latest.high, latest.low, latest.close, latest.volume
                );
            }
        }
        Err(e) => println!("⚠️  Failed to fetch K-bars: {}", e),
    }
    
    // Logout
    println!("\n🚪 Logging out...");
    let logout_success = client.logout().await?;
    if logout_success {
        println!("✅ Logout successful");
    } else {
        println!("⚠️  Logout may have failed");
    }
    */
    
    println!("\n🎉 Demo completed!");
    println!("💡 To use with real data, uncomment the login/logout sections and provide your API credentials.");
    
    Ok(())
}
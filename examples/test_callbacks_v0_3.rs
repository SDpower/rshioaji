//! # rshioaji v0.3.0 Callback System Example
//! 
//! This example demonstrates the full Python-Rust event bridging capabilities
//! of rshioaji v0.3.0, showing how to register callbacks and receive real-time
//! market data events from Python shioaji through the Rust event bridge.

use rshioaji::{
    Shioaji, TickCallback, BidAskCallback, OrderCallback, SystemCallback,
    Exchange, TickSTKv1, TickFOPv1, BidAskSTKv1, Stock, QuoteType,
};
use std::sync::Arc;
use std::collections::HashMap;
use serde_json::Value;

/// Example tick callback handler
struct MyTickHandler {
    name: String,
}

impl MyTickHandler {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl TickCallback for MyTickHandler {
    fn on_tick_stk_v1(&self, exchange: Exchange, tick: TickSTKv1) {
        println!("[{}] 📈 Stock Tick - {}:{} @ {} (Vol: {}, Amt: {})", 
                 self.name, exchange, tick.code, tick.close, tick.volume, tick.amount);
    }
    
    fn on_tick_fop_v1(&self, exchange: Exchange, tick: TickFOPv1) {
        println!("[{}] 📊 FOP Tick - {}:{} @ {} (Vol: {}, Underlying: {})", 
                 self.name, exchange, tick.code, tick.close, tick.volume, tick.underlying_price);
    }
}

/// Example bid/ask callback handler
struct MyBidAskHandler;

impl BidAskCallback for MyBidAskHandler {
    fn on_bidask_stk_v1(&self, exchange: Exchange, bidask: BidAskSTKv1) {
        println!("💰 BidAsk - {}:{} Bid:{}/{} Ask:{}/{}", 
                 exchange, bidask.code, 
                 bidask.bid_price.first().unwrap_or(&0.0),
                 bidask.bid_volume.first().unwrap_or(&0),
                 bidask.ask_price.first().unwrap_or(&0.0),
                 bidask.ask_volume.first().unwrap_or(&0));
    }
    
    fn on_bidask_fop_v1(&self, exchange: Exchange, bidask: rshioaji::BidAskFOPv1) {
        println!("💹 FOP BidAsk - {}:{} Bid:{} Ask:{}", 
                 exchange, bidask.code, 
                 bidask.bid_price.first().unwrap_or(&0.0),
                 bidask.ask_price.first().unwrap_or(&0.0));
    }
}

/// Example order callback handler
struct MyOrderHandler;

impl OrderCallback for MyOrderHandler {
    fn on_order(&self, order_state: rshioaji::OrderState, order_data: Value) {
        println!("📋 Order Update - State: {:?}, Data: {}", order_state, 
                 serde_json::to_string_pretty(&order_data).unwrap_or_default());
    }
}

/// Example system callback handler
struct MySystemHandler;

impl SystemCallback for MySystemHandler {
    fn on_system_event(&self, event_type: String, event_data: Value) {
        println!("🔧 System Event - Type: {}, Data: {}", event_type,
                 serde_json::to_string_pretty(&event_data).unwrap_or_default());
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    rshioaji::init_logging();
    
    println!("🚀 rshioaji v0.3.0 - Full Python-Rust Event Bridging Example");
    println!("================================================================");
    
    // Create Shioaji client
    let proxies = HashMap::new();
    let client = Shioaji::new(true, proxies)?; // simulation mode
    
    // Initialize client
    println!("🔧 Initializing client...");
    client.init().await?;
    
    // Register multiple callback handlers
    println!("📋 Registering callback handlers...");
    
    // Register tick callback
    let tick_handler = Arc::new(MyTickHandler::new("TickHandler"));
    client.register_tick_callback(tick_handler).await;
    
    // Register bid/ask callback
    let bidask_handler = Arc::new(MyBidAskHandler);
    client.register_bidask_callback(bidask_handler).await;
    
    // Register order callback
    let order_handler = Arc::new(MyOrderHandler);
    client.register_order_callback(order_handler).await;
    
    // Register system callback
    let system_handler = Arc::new(MySystemHandler);
    client.register_system_callback(system_handler).await;
    
    // Setup the full Python-Rust event bridge (v0.3.0)
    println!("🌉 Setting up Python-Rust event bridge...");
    match client.setup_callbacks().await {
        Ok(()) => {
            println!("✅ v0.3.0 Event bridge initialized successfully!");
            println!("   - Python callbacks created and registered");
            println!("   - Rust handlers connected to event bridge");
            println!("   - Ready to receive real-time market data");
        },
        Err(e) => {
            println!("⚠️  Event bridge setup incomplete: {}", e);
            println!("   Note: This is expected in proof-of-concept mode");
            println!("   Full integration requires specific Python shioaji methods");
        }
    }
    
    // NOTE: In a real application, you would:
    // 1. Login with your credentials
    // 2. Subscribe to specific contracts
    // 3. Start receiving real-time events
    //
    // Example (commented out - requires real credentials):
    /*
    println!("🔑 Logging in...");
    let accounts = client.login("your_api_key", "your_secret_key", false).await?;
    println!("✅ Login successful! Found {} accounts", accounts.len());
    
    // Create and subscribe to a contract
    let stock = client.create_stock("2330", Exchange::TSE); // TSMC
    let contract = stock.into_contract();
    
    println!("📊 Subscribing to tick data for 2330...");
    client.subscribe(contract, QuoteType::Tick).await?;
    
    // Keep the program running to receive callbacks
    println!("🎯 Listening for events... (Press Ctrl+C to exit)");
    tokio::signal::ctrl_c().await?;
    */
    
    println!("🎯 Example completed!");
    println!("   To see real callbacks, uncomment the login/subscribe section");
    println!("   and provide your Shioaji API credentials.");
    
    Ok(())
}
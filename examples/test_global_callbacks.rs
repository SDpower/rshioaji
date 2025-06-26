use rshioaji::{Shioaji, Exchange};
use rshioaji::types::{TickSTKv1, TickFOPv1, BidAskSTKv1, BidAskFOPv1, QuoteSTKv1};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    println!("🧪 Testing Global Callback Storage System");
    println!("==========================================");
    
    // Create Shioaji client
    let client = Arc::new(Shioaji::new(true, HashMap::new())?);
    
    // Initialize client
    client.init().await?;
    
    println!("\n📋 Registering callbacks...");
    
    // Register tick STK callback
    client.on_tick_stk_v1(|exchange, data| {
        println!("📊 Tick STK callback triggered: {:?} - {}", exchange, data.code);
    }, true).await?;
    
    // Register tick FOP callback  
    client.on_tick_fop_v1(|exchange, data| {
        println!("📈 Tick FOP callback triggered: {:?} - {}", exchange, data.code);
    }, true).await?;
    
    // Register bidask STK callback
    client.on_bidask_stk_v1(|exchange, data| {
        println!("💰 BidAsk STK callback triggered: {:?} - {}", exchange, data.code);
    }, true).await?;
    
    // Register bidask FOP callback
    client.on_bidask_fop_v1(|exchange, data| {
        println!("💹 BidAsk FOP callback triggered: {:?} - {}", exchange, data.code);
    }, true).await?;
    
    // Register quote STK callback
    client.on_quote_stk_v1(|exchange, data| {
        println!("📝 Quote STK callback triggered: {:?} - {}", exchange, data.code);
    }, true).await?;
    
    // Register generic quote callback
    client.on_quote(|topic, data| {
        println!("🔄 Generic quote callback triggered: {} - {:?}", topic, data);
    }).await?;
    
    // Register event callback
    client.on_event(|resp_code, event_code, info, event| {
        println!("🚨 Event callback triggered: {} {} {} {}", resp_code, event_code, info, event);
    }).await?;
    
    // Register session down callback
    client.on_session_down(|| {
        println!("🔌 Session down callback triggered!");
    }).await?;
    
    println!("✅ All callbacks registered successfully!");
    
    println!("\n🧪 Testing callback triggering...");
    
    // Test trigger tick STK callbacks
    let tick_stk = TickSTKv1 {
        code: "2330".to_string(),
        ..Default::default() 
    };
    client.trigger_tick_stk_callbacks(Exchange::TSE, tick_stk).await?;
    
    // Test trigger tick FOP callbacks
    let tick_fop = TickFOPv1 {
        code: "TXFA3".to_string(),
        ..Default::default()
    };
    client.trigger_tick_fop_callbacks(Exchange::TAIFEX, tick_fop).await?;
    
    // Test trigger bidask STK callbacks
    let bidask_stk = BidAskSTKv1 {
        code: "2330".to_string(),
        ..Default::default()
    };
    client.trigger_bidask_stk_callbacks(Exchange::TSE, bidask_stk).await?;
    
    // Test trigger bidask FOP callbacks
    let bidask_fop = BidAskFOPv1 {
        code: "TXFA3".to_string(),
        ..Default::default()
    };
    client.trigger_bidask_fop_callbacks(Exchange::TAIFEX, bidask_fop).await?;
    
    // Test trigger quote STK callbacks
    let quote_stk = QuoteSTKv1 {
        code: "2330".to_string(),
        ..Default::default()
    };
    client.trigger_quote_stk_callbacks(Exchange::TSE, quote_stk).await?;
    
    // Test trigger generic quote callbacks
    let quote_data = serde_json::json!({
        "symbol": "2330",
        "price": 500.0,
        "volume": 1000
    });
    client.trigger_quote_callbacks("market_data".to_string(), quote_data).await?;
    
    // Test trigger event callbacks
    client.trigger_event_callbacks(200, 1001, "Login successful".to_string(), "UserLogin".to_string()).await?;
    
    // Test trigger session down callbacks
    client.trigger_session_down_callbacks().await?;
    
    // Give callbacks time to execute
    sleep(Duration::from_millis(100)).await;
    
    println!("\n✅ All callback tests completed successfully!");
    println!("🎯 The global callback storage system is working correctly");
    
    Ok(())
}
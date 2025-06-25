use rshioaji::{Shioaji, Exchange, TickSTKv1, BidAskSTKv1, QuoteSTKv1, TickFOPv1, BidAskFOPv1};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize environment
    dotenvy::dotenv().ok();
    env_logger::init();
    
    println!("🔧 初始化 rshioaji 並註冊回調函數...");
    
    // Create client
    let client = Shioaji::new(true, HashMap::new())?;
    client.init().await?;
    
    println!("✅ 客戶端初始化成功");
    
    // Register tick callback for stocks
    println!("📊 註冊股票 tick 回調...");
    client.on_tick_stk_v1(|exchange: Exchange, tick: TickSTKv1| {
        println!("📈 股票 Tick 資料: {} - {} @ {}", exchange, tick.code, tick.close);
    }, false).await?;
    
    // Register tick callback for futures/options
    println!("📊 註冊期貨/選擇權 tick 回調...");
    client.on_tick_fop_v1(|exchange: Exchange, tick: TickFOPv1| {
        println!("📊 期貨 Tick 資料: {} - {} @ {}", exchange, tick.code, tick.close);
    }, false).await?;
    
    // Register bidask callback for stocks
    println!("💰 註冊股票 bid/ask 回調...");
    client.on_bidask_stk_v1(|exchange: Exchange, bidask: BidAskSTKv1| {
        println!("💹 股票 BidAsk: {} - {} Bid: {:?} Ask: {:?}", 
                 exchange, bidask.code, bidask.bid_price, bidask.ask_price);
    }, false).await?;
    
    // Register bidask callback for futures/options
    println!("💰 註冊期貨/選擇權 bid/ask 回調...");
    client.on_bidask_fop_v1(|exchange: Exchange, bidask: BidAskFOPv1| {
        println!("💰 期貨 BidAsk: {} - {} Bid: {:?} Ask: {:?}", 
                 exchange, bidask.code, bidask.bid_price, bidask.ask_price);
    }, false).await?;
    
    // Register quote callback for stocks
    println!("📋 註冊股票 quote 回調...");
    client.on_quote_stk_v1(|exchange: Exchange, quote: QuoteSTKv1| {
        println!("📊 股票 Quote: {} - {} @ {} Vol: {}", 
                 exchange, quote.code, quote.close, quote.volume);
    }, false).await?;
    
    // Register generic quote callback
    println!("📋 註冊通用 quote 回調...");
    client.on_quote(|topic: String, data: HashMap<String, String>| {
        println!("📡 通用 Quote: {} - {:?}", topic, data);
    }).await?;
    
    // Register event callback
    println!("🔔 註冊系統事件回調...");
    client.on_event(|resp_code: i32, event_code: i32, info: String, event: String| {
        println!("⚡ 系統事件: {} {} - {} {}", resp_code, event_code, info, event);
    }).await?;
    
    // Register session down callback
    println!("⚠️ 註冊連線中斷回調...");
    client.on_session_down(|| {
        println!("🚨 連線中斷警告！");
    }).await?;
    
    println!("✅ 所有回調函數註冊完成！");
    println!("📋 已註冊的回調類型:");
    println!("   📈 on_tick_stk_v1    - 股票即時 tick 資料");
    println!("   📊 on_tick_fop_v1    - 期貨/選擇權即時 tick 資料"); 
    println!("   💹 on_bidask_stk_v1  - 股票買賣五檔資料");
    println!("   💰 on_bidask_fop_v1  - 期貨/選擇權買賣五檔資料");
    println!("   📋 on_quote_stk_v1   - 股票報價資料");
    println!("   📡 on_quote          - 通用報價資料");
    println!("   ⚡ on_event          - 系統事件通知");
    println!("   🚨 on_session_down   - 連線中斷通知");
    
    println!("\n💡 注意:");
    println!("   - 這些是佔位符回調，會顯示 Python 回調記錄");
    println!("   - 要接收真實市場資料，需要:");
    println!("     1️⃣  完成登入程序 (client.login)");
    println!("     2️⃣  訂閱特定商品 (client.subscribe)");
    println!("     3️⃣  實現真實的 Python-Rust 回調橋接");
    
    println!("\n🎯 當前狀態: 回調架構已就緒，等待真實市場資料整合");
    
    Ok(())
}
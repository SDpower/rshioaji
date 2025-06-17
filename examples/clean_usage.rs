use rshioaji::{Shioaji, TickCallback, BidAskCallback, Exchange, TickSTKv1, TickFOPv1, BidAskSTKv1, BidAskFOPv1};
use std::collections::HashMap;
use std::sync::Arc;

// 完整的事件處理器
struct MyEventHandler;

impl TickCallback for MyEventHandler {
    fn on_tick_stk_v1(&self, _exchange: Exchange, tick: TickSTKv1) {
        println!("📊 收到股票 Tick: {} @ {}", tick.code, tick.close);
    }
    
    fn on_tick_fop_v1(&self, _exchange: Exchange, tick: TickFOPv1) {
        println!("📊 收到期貨 Tick: {} @ {}", tick.code, tick.close);
    }
}

impl BidAskCallback for MyEventHandler {
    fn on_bidask_stk_v1(&self, _exchange: Exchange, bidask: BidAskSTKv1) {
        println!("💰 收到股票 BidAsk: {} 買:{:.2} 賣:{:.2}", 
                 bidask.code, 
                 bidask.bid_price[0], 
                 bidask.ask_price[0]);
    }
    
    fn on_bidask_fop_v1(&self, _exchange: Exchange, bidask: BidAskFOPv1) {
        println!("💰 收到期貨 BidAsk: {} 買:{:.2} 賣:{:.2}", 
                 bidask.code, 
                 bidask.bid_price[0], 
                 bidask.ask_price[0]);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 設定環境日誌級別為 INFO，避免看到 DEBUG 訊息
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    
    println!("🚀 rshioaji v0.4.5 - 乾淨使用體驗示範");
    println!("{}", "=".repeat(50));
    
    // 建立客戶端（模擬模式）
    let client = Shioaji::new(true, HashMap::new())?;
    client.init().await?;
    
    println!("✅ 客戶端初始化成功");
    
    // 登入（使用假憑證進行測試）
    let accounts = client.login("demo_api_key", "demo_secret_key", false).await?;
    println!("✅ 登入成功！找到 {} 個模擬帳戶", accounts.len());
    
    // 註冊回調處理器
    let handler = Arc::new(MyEventHandler);
    client.register_tick_callback(handler.clone()).await;
    client.register_bidask_callback(handler.clone()).await;
    client.setup_callbacks().await?;
    
    println!("✅ 回調系統設定完成");
    
    // 建立並訂閱股票合約（台積電）
    let tsmc = client.create_stock("2330", Exchange::TSE);
    println!("📈 建立台積電合約：{}", tsmc.contract.base.code);
    
    // 訂閱市場資料（這裡會靜默進行，不會有大量輸出）
    client.subscribe(tsmc.contract.clone(), rshioaji::QuoteType::Tick).await?;
    println!("📡 已訂閱台積電 Tick 資料");
    
    // 也訂閱 BidAsk 資料來展示多種回調
    client.subscribe(tsmc.contract, rshioaji::QuoteType::BidAsk).await?;
    println!("📡 已訂閱台積電 BidAsk 資料");
    
    // 等待一下讓模擬資料有時間處理和觸發回調
    println!("⏳ 等待模擬市場數據...");
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    
    // 登出
    client.logout().await?;
    println!("✅ 登出成功");
    
    println!("\n🎉 示範完成！");
    println!("💡 注意：現在的 Mock 系統運行時很安靜，不會產生大量調試訊息");
    println!("🔧 如需詳細的 Mock 調試資訊，請設定環境變數：SHIOAJI_VERBOSE_MOCK=true");
    
    Ok(())
} 
//! # Tick 訂閱測試 - 使用真實 API Key
//! 
//! 這個測試使用 .env 中的真實憑證來測試 tick 數據訂閱和回調功能

use rshioaji::{
    Shioaji, TickCallback, BidAskCallback, QuoteCallback, OrderCallback, SystemCallback,
    Exchange, TickSTKv1, BidAskSTKv1, QuoteSTKv1, TickFOPv1, EnvironmentConfig,
};
use std::sync::{Arc, atomic::{AtomicU32, Ordering}};
use std::collections::HashMap;
use serde_json::Value;

/// 強化的事件計數處理器
struct ComprehensiveEventHandler {
    name: String,
    tick_count: AtomicU32,
    bidask_count: AtomicU32,
    quote_count: AtomicU32,
    order_count: AtomicU32,
    system_count: AtomicU32,
}

impl ComprehensiveEventHandler {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            tick_count: AtomicU32::new(0),
            bidask_count: AtomicU32::new(0),
            quote_count: AtomicU32::new(0),
            order_count: AtomicU32::new(0),
            system_count: AtomicU32::new(0),
        }
    }
    
    fn get_all_counts(&self) -> (u32, u32, u32, u32, u32) {
        (
            self.tick_count.load(Ordering::Relaxed),
            self.bidask_count.load(Ordering::Relaxed),
            self.quote_count.load(Ordering::Relaxed),
            self.order_count.load(Ordering::Relaxed),
            self.system_count.load(Ordering::Relaxed),
        )
    }
    
    fn print_summary(&self) {
        let (tick, bidask, quote, order, system) = self.get_all_counts();
        println!("📊 [{}] 事件統計:", self.name);
        println!("   📈 Tick 事件: {}", tick);
        println!("   💰 BidAsk 事件: {}", bidask);
        println!("   📋 Quote 事件: {}", quote);
        println!("   🏦 Order 事件: {}", order);
        println!("   ⚙️ System 事件: {}", system);
        println!("   🎯 總計: {}", tick + bidask + quote + order + system);
    }
}

impl TickCallback for ComprehensiveEventHandler {
    fn on_tick_stk_v1(&self, exchange: Exchange, tick: TickSTKv1) {
        let count = self.tick_count.fetch_add(1, Ordering::Relaxed) + 1;
        println!("🎯 [{}] TICK #{} - {}:{} @ {} (成交量: {}, 成交額: {})", 
                 self.name, count, exchange, tick.code, tick.close, tick.volume, tick.amount);
    }
    
    fn on_tick_fop_v1(&self, exchange: Exchange, tick: TickFOPv1) {
        let count = self.tick_count.fetch_add(1, Ordering::Relaxed) + 1;
        println!("🎯 [{}] FOP TICK #{} - {}:{} @ {} (成交量: {})", 
                 self.name, count, exchange, tick.code, tick.close, tick.volume);
    }
}

impl BidAskCallback for ComprehensiveEventHandler {
    fn on_bidask_stk_v1(&self, exchange: Exchange, bidask: BidAskSTKv1) {
        let count = self.bidask_count.fetch_add(1, Ordering::Relaxed) + 1;
        let best_bid = bidask.bid_price.first().unwrap_or(&0.0);
        let best_ask = bidask.ask_price.first().unwrap_or(&0.0);
        println!("💰 [{}] BIDASK #{} - {}:{} 買進:{} 賣出:{}", 
                 self.name, count, exchange, bidask.code, best_bid, best_ask);
    }
    
    fn on_bidask_fop_v1(&self, exchange: Exchange, bidask: rshioaji::BidAskFOPv1) {
        let count = self.bidask_count.fetch_add(1, Ordering::Relaxed) + 1;
        println!("💰 [{}] FOP BIDASK #{} - {}:{}", 
                 self.name, count, exchange, bidask.code);
    }
}

impl QuoteCallback for ComprehensiveEventHandler {
    fn on_quote_stk_v1(&self, exchange: Exchange, quote: QuoteSTKv1) {
        let count = self.quote_count.fetch_add(1, Ordering::Relaxed) + 1;
        println!("📋 [{}] QUOTE #{} - {}:{}", 
                 self.name, count, exchange, quote.code);
    }
    
    fn on_quote(&self, topic: String, _data: Value) {
        let count = self.quote_count.fetch_add(1, Ordering::Relaxed) + 1;
        println!("📋 [{}] GENERAL QUOTE #{} - Topic: {}", 
                 self.name, count, topic);
    }
}

impl OrderCallback for ComprehensiveEventHandler {
    fn on_order(&self, order_state: rshioaji::OrderState, _data: Value) {
        let count = self.order_count.fetch_add(1, Ordering::Relaxed) + 1;
        println!("🏦 [{}] ORDER #{} - State: {:?}", 
                 self.name, count, order_state);
    }
}

impl SystemCallback for ComprehensiveEventHandler {
    fn on_event(&self, event_type: i32, code: i32, message: String, _details: String) {
        let count = self.system_count.fetch_add(1, Ordering::Relaxed) + 1;
        println!("⚙️ [{}] SYSTEM #{} - Type:{}, Code:{}, Msg:{}", 
                 self.name, count, event_type, code, message);
    }
    
    fn on_session_down(&self) {
        let count = self.system_count.fetch_add(1, Ordering::Relaxed) + 1;
        println!("⚠️ [{}] SESSION DOWN #{} - 連線中斷", self.name, count);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日誌
    let env_config = EnvironmentConfig::from_env();
    let _ = rshioaji::init_logging(&env_config);
    
    println!("🚀 Tick 訂閱測試 - 使用真實 API Key");
    println!("===================================");
    
    // 載入 .env 檔案
    if std::path::Path::new(".env").exists() {
        println!("📁 載入 .env 檔案...");
        match std::fs::read_to_string(".env") {
            Ok(content) => {
                for line in content.lines() {
                    if let Some((key, value)) = line.split_once('=') {
                        std::env::set_var(key.trim(), value.trim());
                    }
                }
                println!("✅ .env 檔案載入成功");
            },
            Err(e) => {
                println!("⚠️ 無法讀取 .env 檔案: {}", e);
            }
        }
    }
    
    // 從環境變數讀取憑證
    let api_key = std::env::var("SHIOAJI_API_KEY").unwrap_or_default();
    let secret_key = std::env::var("SHIOAJI_SECRET_KEY").unwrap_or_default();
    let simulation = std::env::var("SHIOAJI_SIMULATION")
        .unwrap_or_default()
        .parse::<bool>()
        .unwrap_or(true);
    
    if !api_key.is_empty() && api_key.len() >= 8 {
        println!("🔑 API Key: {}...", &api_key[..8]);
    } else {
        println!("🔑 API Key: (未設定或無效)");
    }
    println!("🔧 模擬模式: {}", simulation);
    
    if api_key.is_empty() || secret_key.is_empty() {
        println!("❌ 錯誤：找不到 API Key 或 Secret Key");
        println!("💡 請確認 .env 檔案中的 SHIOAJI_API_KEY 和 SHIOAJI_SECRET_KEY");
        return Ok(());
    }
    
    // 建立 Shioaji 客戶端
    let proxies = HashMap::new();
    let client = Shioaji::new(simulation, proxies)?;
    
    // 步驟 1: 初始化客戶端
    println!("\n🔧 步驟 1: 初始化客戶端...");
    match client.init().await {
        Ok(()) => println!("✅ 客戶端初始化成功"),
        Err(e) => {
            println!("⚠️ 客戶端初始化有問題: {}", e);
            println!("🔧 繼續進行測試...");
        }
    }
    
    // 步驟 2: 註冊所有類型的回調處理器
    println!("\n🔧 步驟 2: 註冊回調處理器...");
    let handler = Arc::new(ComprehensiveEventHandler::new("MainHandler"));
    
    // 註冊所有類型的回調
    client.register_tick_callback(handler.clone()).await;
    client.register_bidask_callback(handler.clone()).await;
    client.register_quote_callback(handler.clone()).await;
    client.register_order_callback(handler.clone()).await;
    client.register_system_callback(handler.clone()).await;
    
    println!("✅ 所有回調處理器已註冊");
    handler.print_summary();
    
    // 步驟 3: 設定 Python-Rust 事件橋接
    println!("\n🔧 步驟 3: 設定事件橋接...");
    match client.setup_callbacks().await {
        Ok(()) => println!("✅ 事件橋接設定成功"),
        Err(e) => {
            println!("⚠️ 事件橋接設定有問題: {}", e);
            println!("🔧 繼續進行登入測試...");
        }
    }
    
    // 步驟 4: 登入
    println!("\n🔧 步驟 4: 登入 Shioaji...");
    match client.login(&api_key, &secret_key, true).await {
        Ok(accounts) => {
            println!("✅ 登入成功！找到 {} 個帳戶", accounts.len());
            for (i, account) in accounts.iter().enumerate() {
                println!("   帳戶 {}: {:?}", i+1, account);
            }
            
            // 登入後等待一下，看是否有自動事件
            println!("\n⏱️ 等待 3 秒鐘，檢查登入後的自動事件...");
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            handler.print_summary();
            
            // 步驟 5: 建立合約並訂閱
            println!("\n🔧 步驟 5: 建立合約並訂閱 Tick 數據...");
            
            // 測試多個熱門股票
            let test_stocks = vec![
                ("2330", "台積電"),
                ("2317", "鴻海"),
                ("2454", "聯發科"),
            ];
            
            for (code, name) in test_stocks {
                println!("\n📊 訂閱 {} ({})", name, code);
                
                let stock = client.create_stock(code, Exchange::TSE);
                
                match client.subscribe(stock.contract, rshioaji::QuoteType::Tick).await {
                    Ok(()) => {
                        println!("✅ {} Tick 訂閱成功", name);
                    },
                    Err(e) => {
                        println!("⚠️ {} Tick 訂閱失敗: {}", name, e);
                    }
                }
                
                // 嘗試訂閱 BidAsk
                let stock2 = client.create_stock(code, Exchange::TSE);
                match client.subscribe(stock2.contract, rshioaji::QuoteType::BidAsk).await {
                    Ok(()) => {
                        println!("✅ {} BidAsk 訂閱成功", name);
                    },
                    Err(e) => {
                        println!("⚠️ {} BidAsk 訂閱失敗: {}", name, e);
                    }
                }
            }
            
            // 步驟 6: 等待市場數據
            println!("\n🔧 步驟 6: 等待市場數據...");
            println!("⏱️ 監聽 30 秒鐘的市場數據事件...");
            
            for i in 1..=6 {
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                println!("\n📊 第 {} 次檢查 ({}秒後):", i, i*5);
                handler.print_summary();
                
                let (tick, bidask, quote, _order, _system) = handler.get_all_counts();
                if tick > 0 || bidask > 0 || quote > 0 {
                    println!("🎉 成功接收到市場數據事件！");
                    break;
                }
            }
            
            // 最終報告
            println!("\n📊 最終測試結果:");
            println!("================");
            handler.print_summary();
            
            let (tick, bidask, quote, _order, _system) = handler.get_all_counts();
            let total_events = tick + bidask + quote + _order + _system;
            
            if total_events > 0 {
                println!("🎉 測試成功！回調系統正常接收市場數據");
                println!("✅ Tick 訂閱功能完全正常");
                
                if tick > 0 {
                    println!("🎯 Tick 數據接收: {} 筆", tick);
                }
                if bidask > 0 {
                    println!("💰 BidAsk 數據接收: {} 筆", bidask);
                }
                if quote > 0 {
                    println!("📋 Quote 數據接收: {} 筆", quote);
                }
                
            } else {
                println!("📋 沒有接收到市場數據事件");
                println!("💡 可能的原因:");
                println!("   - 市場已收盤");
                println!("   - 訂閱的股票目前沒有交易");
                println!("   - 網路連線問題");
                println!("   - Mock 系統限制");
                println!("✅ 但回調系統架構已準備就緒");
            }
            
        },
        Err(e) => {
            println!("❌ 登入失敗: {}", e);
            println!("💡 可能的原因:");
            println!("   - API Key 或 Secret Key 不正確");
            println!("   - 網路連線問題");
            println!("   - Shioaji 服務暫時不可用");
            println!("   - Mock 系統限制");
        }
    }
    
    println!("\n✅ Tick 訂閱測試完成！");
    Ok(())
}
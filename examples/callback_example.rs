use std::sync::Arc;
use rshioaji::{
    Shioaji, 
    TickCallback, BidAskCallback, QuoteCallback, OrderCallback, SystemCallback,
    TickSTKv1, TickFOPv1, BidAskSTKv1, BidAskFOPv1, QuoteSTKv1, OrderState,
    Exchange
};
use std::collections::HashMap;

/// Example implementation of all callback traits
#[derive(Debug)]
struct MyEventHandler {
    name: String,
}

impl MyEventHandler {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl TickCallback for MyEventHandler {
    fn on_tick_stk_v1(&self, exchange: Exchange, tick: TickSTKv1) {
        println!(
            "[{}] 股票 Tick: {} @ {:?} - 價格: {}, 成交量: {}, 時間: {}", 
            self.name, tick.code, exchange, tick.close, tick.volume, tick.datetime
        );
    }
    
    fn on_tick_fop_v1(&self, exchange: Exchange, tick: TickFOPv1) {
        println!(
            "[{}] 期權 Tick: {} @ {:?} - 價格: {}, 成交量: {}, 標的價格: {}, 時間: {}", 
            self.name, tick.code, exchange, tick.close, tick.volume, tick.underlying_price, tick.datetime
        );
    }
}

impl BidAskCallback for MyEventHandler {
    fn on_bidask_stk_v1(&self, exchange: Exchange, bidask: BidAskSTKv1) {
        println!(
            "[{}] 股票委買委賣: {} @ {:?} - 買價: {:?}, 賣價: {:?}, 時間: {}", 
            self.name, bidask.code, exchange, bidask.bid_price, bidask.ask_price, bidask.datetime
        );
    }
    
    fn on_bidask_fop_v1(&self, exchange: Exchange, bidask: BidAskFOPv1) {
        println!(
            "[{}] 期權委買委賣: {} @ {:?} - 買價: {:?}, 賣價: {:?}, 標的價格: {}, 時間: {}", 
            self.name, bidask.code, exchange, bidask.bid_price, bidask.ask_price, 
            bidask.underlying_price, bidask.datetime
        );
    }
}

impl QuoteCallback for MyEventHandler {
    fn on_quote_stk_v1(&self, exchange: Exchange, quote: QuoteSTKv1) {
        println!(
            "[{}] 股票報價: {} @ {:?} - 買價: {}, 賣價: {}, 成交價: {}, 時間: {}", 
            self.name, quote.code, exchange, quote.bid_price, quote.ask_price, quote.close, quote.datetime
        );
    }
    
    fn on_quote(&self, topic: String, data: serde_json::Value) {
        println!("[{}] 一般報價: {} - 資料: {}", self.name, topic, data);
    }
}

impl OrderCallback for MyEventHandler {
    fn on_order(&self, order_state: OrderState, data: serde_json::Value) {
        println!("[{}] 訂單狀態變更: {:?} - 資料: {}", self.name, order_state, data);
    }
}

impl SystemCallback for MyEventHandler {
    fn on_event(&self, event_type: i32, code: i32, message: String, details: String) {
        println!("[{}] 系統事件: 類型={}, 代碼={}, 訊息={}, 詳情={}", 
                self.name, event_type, code, message, details);
    }
    
    fn on_session_down(&self) {
        println!("[{}] 連線中斷！", self.name);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日誌
    env_logger::init();
    
    println!("🚀 rshioaji Callback 範例程式");
    
    // 建立 Shioaji 客戶端
    let proxies = HashMap::new();
    let client = Shioaji::new(true, proxies)?; // 使用模擬模式
    
    // 初始化客戶端
    client.init().await?;
    
    // 建立事件處理器
    let handler = Arc::new(MyEventHandler::new("主要處理器"));
    
    // 註冊各種回調
    println!("📝 註冊回調處理器...");
    client.register_tick_callback(handler.clone()).await;
    client.register_bidask_callback(handler.clone()).await;
    client.register_quote_callback(handler.clone()).await;
    client.register_order_callback(handler.clone()).await;
    client.register_system_callback(handler.clone()).await;
    
    // 設定回調
    client.setup_callbacks().await?;
    
    println!("✅ 回調系統設定完成");
    println!("📊 開始監聽市場資料事件...");
    
    // 模擬一些回調觸發（實際使用中這些會由市場資料觸發）
    // 這裡只是展示回調系統的架構
    
    // 在實際應用中，你需要：
    // 1. 登入到 Shioaji 系統
    // 2. 訂閱市場資料
    // 3. 讓程式持續運行以接收回調
    
    println!("💡 提示：這是回調系統的架構展示");
    println!("💡 實際使用時需要登入並訂閱市場資料來觸發回調");
    
    Ok(())
}

// 展示如何實作特定的回調處理器
#[derive(Debug)]
struct PriceAlertHandler {
    alert_price: f64,
    symbol: String,
}

impl PriceAlertHandler {
    fn new(symbol: &str, alert_price: f64) -> Self {
        Self {
            symbol: symbol.to_string(),
            alert_price,
        }
    }
}

impl TickCallback for PriceAlertHandler {
    fn on_tick_stk_v1(&self, _exchange: Exchange, tick: TickSTKv1) {
        if tick.code == self.symbol {
            if tick.close >= self.alert_price {
                println!("🚨 價格警示！{} 已達到目標價格 {} (當前價格: {})", 
                        tick.code, self.alert_price, tick.close);
            }
        }
    }
    
    fn on_tick_fop_v1(&self, _exchange: Exchange, tick: TickFOPv1) {
        if tick.code == self.symbol {
            if tick.close >= self.alert_price {
                println!("🚨 期權價格警示！{} 已達到目標價格 {} (當前價格: {})", 
                        tick.code, self.alert_price, tick.close);
            }
        }
    }
}

#[allow(dead_code)]
async fn price_alert_example() -> Result<(), Box<dyn std::error::Error>> {
    let proxies = HashMap::new();
    let client = Shioaji::new(true, proxies)?;
    client.init().await?;
    
    // 建立價格警示處理器
    let alert_handler = Arc::new(PriceAlertHandler::new("2330", 600.0));
    
    // 只註冊 tick 回調用於價格警示
    client.register_tick_callback(alert_handler).await;
    
    client.setup_callbacks().await?;
    
    println!("價格警示系統已啟動，監控 2330 達到 600 元");
    
    Ok(())
}
# rshioaji 回調系統使用說明

## 📋 概述

rshioaji v0.4.9 提供完整的 Python → Rust 回調轉發系統，支援所有主要市場資料和系統事件的即時回調處理。本文件詳細說明回調系統的使用方法和最佳實踐。

## 🎯 支援的回調類型

### 市場資料回調

| 回調類型 | 方法名稱 | 資料類型 | 描述 |
|----------|----------|----------|------|
| **股票 Tick** | `on_tick_stk_v1` | `TickSTKv1` | 股票即時成交資料 |
| **期權 Tick** | `on_tick_fop_v1` | `TickFOPv1` | 期貨/選擇權即時成交資料 |
| **股票買賣價差** | `on_bidask_stk_v1` | `BidAskSTKv1` | 股票五檔買賣價差 |
| **期權買賣價差** | `on_bidask_fop_v1` | `BidAskFOPv1` | 期貨/選擇權五檔買賣價差 |
| **股票報價** | `on_quote_stk_v1` | `QuoteSTKv1` | 股票綜合報價資料 |
| **通用報價** | `on_quote` | `JSON Value` | 通用報價事件 |

### 系統回調

| 回調類型 | 方法名稱 | 參數 | 描述 |
|----------|----------|------|------|
| **系統事件** | `on_event` | `resp_code`, `event_code`, `info`, `event` | 系統事件通知 |
| **連線中斷** | `on_session_down` | 無參數 | 連線中斷通知 |

## 🚀 基本使用方法

### 1. 註冊單一回調

```rust
use rshioaji::{Shioaji, Exchange, TickSTKv1};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化客戶端
    let client = Shioaji::new(false, HashMap::new())?;
    client.init().await?;
    
    // 登入
    let api_key = std::env::var("SHIOAJI_API_KEY")?;
    let secret_key = std::env::var("SHIOAJI_SECRET_KEY")?;
    client.login(&api_key, &secret_key, true, 30, None, false, 30000).await?;
    
    // 註冊股票 tick 回調
    client.on_tick_stk_v1(|exchange: Exchange, tick: TickSTKv1| {
        println!("📈 股票 Tick: {} @ {:?} - 價格: {}, 量: {}", 
                tick.code, exchange, tick.close, tick.volume);
    }, false).await?;
    
    // 註冊系統事件回調
    client.on_event(|resp_code, event_code, info, event| {
        println!("🔔 系統事件: [{} {}] {} - {}", 
                resp_code, event_code, info, event);
    }).await?;
    
    // 訂閱台積電
    let tsmc = client.create_stock("2330", Exchange::TSE);
    client.subscribe(tsmc.contract, "tick").await?;
    
    println!("✅ 回調系統已啟動，正在監聽事件...");
    
    // 等待回調事件
    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
    
    Ok(())
}
```

### 2. 註冊多個回調

```rust
// 註冊期貨 tick 回調 (MXFG5)
client.on_tick_fop_v1(|exchange: Exchange, tick: TickFOPv1| {
    println!("📊 期貨 Tick: {} @ {:?} - 價格: {}, 量: {}", 
            tick.code, exchange, tick.close, tick.volume);
}, false).await?;

// 註冊股票買賣價差回調
client.on_bidask_stk_v1(|exchange: Exchange, bidask: BidAskSTKv1| {
    println!("💹 股票買賣價差: {} - 買一: {}, 賣一: {}", 
            bidask.code, bidask.bid_price[0], bidask.ask_price[0]);
}, false).await?;

// 註冊連線中斷回調
client.on_session_down(|| {
    println!("⚠️ 連線中斷！正在嘗試重連...");
}).await?;

// 訂閱多個商品
let tsmc = client.create_stock("2330", Exchange::TSE);
client.subscribe(tsmc.contract, "tick").await?;
client.subscribe(tsmc.contract, "bidask").await?;

// 訂閱期貨 (MXFG5)
let mxfg5 = client.create_future("MXFG5", Exchange::TFE);
client.subscribe(mxfg5.contract, "tick").await?;
```

## 🏗️ 進階使用 - 事件處理器模式

### 創建自定義事件處理器

```rust
use rshioaji::{
    Shioaji, TickCallback, BidAskCallback, SystemCallback,
    TickSTKv1, TickFOPv1, BidAskSTKv1, BidAskFOPv1, Exchange
};
use std::sync::Arc;

// 自定義事件處理器
#[derive(Debug)]
struct MarketDataHandler {
    name: String,
    tick_count: std::sync::atomic::AtomicUsize,
}

impl MarketDataHandler {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            tick_count: std::sync::atomic::AtomicUsize::new(0),
        }
    }
    
    fn get_tick_count(&self) -> usize {
        self.tick_count.load(std::sync::atomic::Ordering::Relaxed)
    }
}

impl TickCallback for MarketDataHandler {
    fn on_tick_stk_v1(&self, exchange: Exchange, tick: TickSTKv1) {
        let count = self.tick_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        println!("📈 [{}] 股票 Tick #{}: {} @ {:?} - 價格: {} ({})", 
                self.name, count + 1, tick.code, exchange, tick.close, 
                chrono::DateTime::from_timestamp(tick.ts as i64, 0)
                    .unwrap_or_default()
                    .format("%H:%M:%S"));
    }
    
    fn on_tick_fop_v1(&self, exchange: Exchange, tick: TickFOPv1) {
        let count = self.tick_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        println!("📊 [{}] 期權 Tick #{}: {} @ {:?} - 價格: {}", 
                self.name, count + 1, tick.code, exchange, tick.close);
    }
}

impl BidAskCallback for MarketDataHandler {
    fn on_bidask_stk_v1(&self, exchange: Exchange, bidask: BidAskSTKv1) {
        println!("💹 [{}] 股票價差: {} @ {:?} - 買: {} / 賣: {}", 
                self.name, bidask.code, exchange, 
                bidask.bid_price[0], bidask.ask_price[0]);
    }
    
    fn on_bidask_fop_v1(&self, exchange: Exchange, bidask: BidAskFOPv1) {
        println!("💰 [{}] 期權價差: {} @ {:?} - 買: {} / 賣: {}", 
                self.name, bidask.code, exchange, 
                bidask.bid_price[0], bidask.ask_price[0]);
    }
}

impl SystemCallback for MarketDataHandler {
    fn on_event(&self, resp_code: i32, event_code: i32, info: String, event: String) {
        println!("🔔 [{}] 系統事件: [{} {}] {} - {}", 
                self.name, resp_code, event_code, info, event);
    }
    
    fn on_session_down(&self) {
        println!("⚠️ [{}] 連線中斷！", self.name);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Shioaji::new(false, HashMap::new())?;
    client.init().await?;
    
    // 登入
    let api_key = std::env::var("SHIOAJI_API_KEY")?;
    let secret_key = std::env::var("SHIOAJI_SECRET_KEY")?;
    client.login(&api_key, &secret_key, true, 30, None, false, 30000).await?;
    
    // 創建事件處理器
    let handler = Arc::new(MarketDataHandler::new("主處理器"));
    
    // 註冊處理器到客戶端
    client.register_tick_callback(handler.clone()).await;
    client.register_bidask_callback(handler.clone()).await;
    client.register_system_callback(handler.clone()).await;
    
    // 設定回調系統
    client.setup_callbacks().await?;
    
    // 訂閱資料
    let tsmc = client.create_stock("2330", Exchange::TSE);
    client.subscribe(tsmc.contract, "tick").await?;
    
    println!("✅ 事件處理器已啟動");
    
    // 運行 30 秒並顯示統計
    for i in (1..=30).rev() {
        if i % 5 == 0 || i <= 5 {
            println!("⏰ 剩餘 {} 秒... (已接收 {} 個 tick)", i, handler.get_tick_count());
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
    
    println!("📊 總計接收 {} 個 tick 事件", handler.get_tick_count());
    
    Ok(())
}
```

## 📊 回調參數詳細說明

### TickSTKv1 (股票 Tick 資料)

```rust
pub struct TickSTKv1 {
    pub ts: f64,           // 時間戳
    pub code: String,      // 股票代碼
    pub close: f64,        // 成交價
    pub volume: i64,       // 成交量
    pub bid_price: f64,    // 買價
    pub ask_price: f64,    // 賣價
    pub bid_size: i64,     // 買量
    pub ask_size: i64,     // 賣量
    // ... 更多欄位
}
```

### TickFOPv1 (期貨/選擇權 Tick 資料)

```rust
pub struct TickFOPv1 {
    pub ts: f64,           // 時間戳
    pub code: String,      // 合約代碼
    pub close: f64,        // 成交價
    pub volume: i64,       // 成交量
    pub open_interest: i64, // 未平倉量
    // ... 更多欄位
}
```

### BidAskSTKv1 (股票買賣價差)

```rust
pub struct BidAskSTKv1 {
    pub ts: f64,                    // 時間戳
    pub code: String,               // 股票代碼
    pub bid_price: [f64; 5],       // 五檔買價
    pub ask_price: [f64; 5],       // 五檔賣價
    pub bid_size: [i64; 5],        // 五檔買量
    pub ask_size: [i64; 5],        // 五檔賣量
    // ... 更多欄位
}
```

## 🔧 訂閱和取消訂閱

### 訂閱市場資料

```rust
// 訂閱股票 tick 資料
let tsmc = client.create_stock("2330", Exchange::TSE);
client.subscribe(tsmc.contract, "tick").await?;

// 訂閱股票買賣價差
client.subscribe(tsmc.contract, "bidask").await?;

// 訂閱期貨 tick 資料
let mxfg5 = client.create_future("MXFG5", Exchange::TFE);
client.subscribe(mxfg5.contract, "tick").await?;

// 訂閱成功會收到系統事件：
// 🔔 系統事件: [200 16] TIC/v1/STK/*/TSE/2330 - 訂閱成功
// 🔔 系統事件: [200 16] TIC/v1/FOP/*/TFE/MXFG5 - 訂閱成功
```

### 取消訂閱

```rust
// 取消訂閱股票資料
client.unsubscribe(tsmc.contract, "tick").await?;
client.unsubscribe(tsmc.contract, "bidask").await?;

// 取消訂閱期貨資料
client.unsubscribe(mxfg5.contract, "tick").await?;
```

## ⚠️ 重要注意事項

### 1. 登入順序

```rust
// ❌ 錯誤：未登入就設定回調
let client = Shioaji::new(false, HashMap::new())?;
client.on_tick_stk_v1(|_, _| {}, false).await?; // 會失敗

// ✅ 正確：先登入再設定回調
let client = Shioaji::new(false, HashMap::new())?;
client.init().await?;
client.login(&api_key, &secret_key, true, 30, None, false, 30000).await?;
client.on_tick_stk_v1(|_, _| {}, false).await?; // 成功
```

### 2. 回調函數要求

- **線程安全**：回調函數必須實現 `Send + Sync`
- **非阻塞**：回調函數應該快速執行，避免阻塞事件處理
- **錯誤處理**：回調函數內部應處理所有可能的錯誤

### 3. 訂閱限制

- 需要先註冊對應的回調函數才能接收資料
- 訂閱成功會收到系統事件確認
- 某些資料需要相應的權限才能訂閱

## 🎯 最佳實踐

### 1. 事件計數和監控

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

static TICK_COUNT: AtomicUsize = AtomicUsize::new(0);
static EVENT_COUNT: AtomicUsize = AtomicUsize::new(0);

// 在回調中計數
client.on_tick_stk_v1(|exchange, tick| {
    let count = TICK_COUNT.fetch_add(1, Ordering::Relaxed);
    if count % 100 == 0 {
        println!("已接收 {} 個 tick 事件", count);
    }
}, false).await?;

client.on_event(|resp_code, event_code, info, event| {
    EVENT_COUNT.fetch_add(1, Ordering::Relaxed);
    println!("系統事件 #{}: [{} {}] {}", 
             EVENT_COUNT.load(Ordering::Relaxed), 
             resp_code, event_code, info);
}).await?;
```

### 2. 錯誤處理和日誌

```rust
client.on_tick_stk_v1(|exchange, tick| {
    match process_tick_data(&tick) {
        Ok(_) => log::debug!("處理 tick 成功: {}", tick.code),
        Err(e) => log::error!("處理 tick 失敗: {} - {}", tick.code, e),
    }
}, false).await?;

fn process_tick_data(tick: &TickSTKv1) -> Result<(), Box<dyn std::error::Error>> {
    // 您的商業邏輯
    if tick.close <= 0.0 {
        return Err("無效的價格".into());
    }
    // ... 其他處理
    Ok(())
}
```

### 3. 資料持久化

```rust
use tokio::sync::mpsc;

// 創建通道用於資料傳輸
let (tx, mut rx) = mpsc::channel::<TickSTKv1>(1000);

// 回調中發送資料
client.on_tick_stk_v1(move |exchange, tick| {
    if let Err(e) = tx.blocking_send(tick) {
        log::error!("發送 tick 資料失敗: {}", e);
    }
}, false).await?;

// 單獨的任務處理資料持久化
tokio::spawn(async move {
    while let Some(tick) = rx.recv().await {
        // 保存到資料庫或檔案
        save_tick_to_database(&tick).await;
    }
});
```

## 🔗 相關文件

- [登入流程說明](login_flow.md)
- [環境設定說明](environment_setup.md)
- [API 完整文件](https://docs.rs/rshioaji)

---

**注意**: 回調系統需要穩定的網路連線和有效的市場資料權限。請確保您的 API 金鑰有相應的資料訂閱權限。
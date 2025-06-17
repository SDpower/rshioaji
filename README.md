# rshioaji

[![Crates.io](https://img.shields.io/crates/v/rshioaji)](https://crates.io/crates/rshioaji)
[![Documentation](https://docs.rs/rshioaji/badge.svg)](https://docs.rs/rshioaji)
[![License](https://img.shields.io/crates/l/rshioaji)](https://github.com/stevelo/rshioaji/blob/main/LICENSE-MIT)

**🚀 rshioaji v0.4.3** - 永豐證券 Shioaji API 的高效能 Rust 封裝庫

## 🚀 主要特色

- **統一的 API 設計**：簡化的登入流程，使用統一的 `login()` 方法
- **完整的回調系統**：支援 Tick、BidAsk、Quote、Order 和系統事件回調
- **Python-Rust 事件橋接**：高效能的跨語言事件處理
- **強大的 Mock 系統**：支援無憑證的開發和測試環境
- **乾淨的使用體驗**：靜默的 Mock 系統，專業級的執行環境
- **類型安全**：完整的 Rust 類型定義，編譯時安全保證
- **跨平台支援**：支援 Linux、macOS 和 Windows

## 📦 安裝

```toml
[dependencies]
rshioaji = "0.4.2"
```

## 🔧 快速開始

### 基本使用

```rust
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
                 bidask.code, bidask.bid_price[0], bidask.ask_price[0]);
    }
    
    fn on_bidask_fop_v1(&self, _exchange: Exchange, bidask: BidAskFOPv1) {
        println!("💰 收到期貨 BidAsk: {} 買:{:.2} 賣:{:.2}", 
                 bidask.code, bidask.bid_price[0], bidask.ask_price[0]);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 建立客戶端
    let client = Shioaji::new(true, HashMap::new())?; // true = 模擬模式
    client.init().await?;
    
    // 登入
    let accounts = client.login("api_key", "secret_key", false).await?;
    println!("登入成功！找到 {} 個帳戶", accounts.len());
    
    // 註冊回調處理器
    let handler = Arc::new(MyEventHandler);
    client.register_tick_callback(handler.clone()).await;
    client.register_bidask_callback(handler.clone()).await;
    client.setup_callbacks().await?;
    
    // 訂閱市場數據
    let stock = client.create_stock("2330", Exchange::TSE);
    client.subscribe(stock.contract.clone(), rshioaji::QuoteType::Tick).await?;
    client.subscribe(stock.contract, rshioaji::QuoteType::BidAsk).await?;
    
    // 等待接收模擬數據
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    
    // 登出
    client.logout().await?;
    
    Ok(())
}
```

### 環境變數配置

```bash
# .env 檔案
SHIOAJI_API_KEY=your_api_key
SHIOAJI_SECRET_KEY=your_secret_key
SHIOAJI_SIMULATION=true
SHIOAJI_VERBOSE_MOCK=false  # 設為 true 可顯示詳細的 Mock 調試訊息
```

```rust
use rshioaji::Config;

// 從環境變數載入配置
let config = Config::from_env()?;
let client = Shioaji::new(config.simulation, HashMap::new())?;
```

## 📋 功能特色

### 統一的登入流程

v0.4.2 提供簡化且乾淨的登入 API：

```rust
// ✅ 統一使用 login() 方法
let accounts = client.login(&api_key, &secret_key, false).await?;

// 內部會根據 simulation 參數自動選擇正確的登入模式
// 不會產生大量調試訊息，提供專業級使用體驗
```

### 完整的回調系統

```rust
use rshioaji::{TickCallback, BidAskCallback, SystemCallback};

// 實現多種回調 trait
struct EventHandler;

impl TickCallback for EventHandler {
    fn on_tick_stk_v1(&self, exchange: Exchange, tick: TickSTKv1) {
        // 處理股票 Tick 數據
    }
    
    fn on_tick_fop_v1(&self, exchange: Exchange, tick: TickFOPv1) {
        // 處理期貨 Tick 數據
    }
}

impl BidAskCallback for EventHandler {
    fn on_bidask_stk_v1(&self, exchange: Exchange, bidask: BidAskSTKv1) {
        // 處理股票 BidAsk 數據
    }
    
    fn on_bidask_fop_v1(&self, exchange: Exchange, bidask: BidAskFOPv1) {
        // 處理期貨 BidAsk 數據
    }
}

impl SystemCallback for EventHandler {
    fn on_event(&self, event_type: i32, code: i32, message: String, details: String) {
        // 處理系統事件
    }
    
    fn on_session_down(&self) {
        // 處理連線中斷
    }
}

// 註冊回調
let handler = Arc::new(EventHandler);
client.register_tick_callback(handler.clone()).await;
client.register_bidask_callback(handler.clone()).await;
client.register_system_callback(handler.clone()).await;
```

### 乾淨的 Mock 系統

v0.4.2 提供專業級的 Mock 體驗：

```rust
// 建立模擬環境客戶端
let client = Shioaji::new(true, HashMap::new())?; // simulation = true

// 使用假憑證進行測試，系統會靜默運行
let accounts = client.login("demo_api_key", "demo_secret_key", false).await?;

// 正常使用所有功能進行開發測試，不會有干擾的調試訊息
// 如需詳細調試資訊，設定環境變數：SHIOAJI_VERBOSE_MOCK=true
```

## 🔄 版本更新

### v0.4.3 (2025-06)

- ✅ **真實環境回調修復**：解決用戶在真實 API 環境中無法收到市場事件的關鍵問題
- ✅ **完整的事件橋接**：修復 `setup_real_callbacks` 方法，確保 Python 回調函數正確註冊
- ✅ **程式碼庫清理**：移除開發階段的臨時測試檔案，保持程式碼整潔
- ✅ **文件更新**：完善版本記錄和使用說明

### v0.4.2 (2025-06)

- ✅ **乾淨的 Mock 體驗**：移除大量調試訊息，提供專業級執行環境
- ✅ **實際事件觸發**：修復 Mock 系統，確保回調事件能正確觸發
- ✅ **可選調試模式**：通過 `SHIOAJI_VERBOSE_MOCK` 環境變數控制詳細輸出
- ✅ **完善的範例程式**：提供乾淨使用體驗的示範程式
- ✅ **優化用戶體驗**：靜默的模擬數據生成，不干擾正常使用流程

### v0.4.1 (2024-01)

- ✅ **統一登入 API**：移除多餘的 `token_login`/`simulation_login`，統一使用 `login()` 方法
- ✅ **完善 Mock 系統**：支援完整的無憑證開發環境
- ✅ **修正初始化問題**：解決 `'NoneType' object has no attribute 'activated_ca'` 錯誤
- ✅ **改進事件橋接**：更穩定的 Python-Rust 回調系統
- ✅ **增強錯誤處理**：更好的錯誤訊息和診斷資訊

## 📚 文件

- [API 文件](https://docs.rs/rshioaji)
- [使用範例](examples/)
- [登入流程說明](docs/login_flow.md)
- [環境設定指南](docs/environment_setup.md)

## 🧪 測試

```bash
# 執行所有測試
cargo test

# 執行範例（乾淨體驗）
cargo run --example clean_usage

# 執行其他範例
cargo run --example basic_usage
cargo run --example callback_example
```

## 🤝 貢獻

歡迎提交 Issue 和 Pull Request！

## 📄 授權

此專案採用 MIT 或 Apache-2.0 雙重授權。

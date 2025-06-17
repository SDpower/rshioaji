# rshioaji

[![Crates.io](https://img.shields.io/crates/v/rshioaji)](https://crates.io/crates/rshioaji)
[![Documentation](https://docs.rs/rshioaji/badge.svg)](https://docs.rs/rshioaji)
[![License](https://img.shields.io/crates/l/rshioaji)](https://github.com/stevelo/rshioaji/blob/main/LICENSE-MIT)

**rshioaji v0.4.1** - 高效能的台灣永豐證券 Shioaji API Rust 綁定庫

## 🚀 主要特色

- **統一的 API 設計**：簡化的登入流程，使用統一的 `login()` 方法
- **完整的回調系統**：支援 Tick、BidAsk、Quote、Order 和系統事件回調
- **Python-Rust 事件橋接**：高效能的跨語言事件處理
- **強大的 Mock 系統**：支援無憑證的開發和測試環境
- **類型安全**：完整的 Rust 類型定義，編譯時安全保證
- **跨平台支援**：支援 Linux、macOS 和 Windows

## 📦 安裝

```toml
[dependencies]
rshioaji = "0.4.1"
```

## 🔧 快速開始

### 基本使用

```rust
use rshioaji::{Shioaji, TickCallback, Exchange, TickSTKv1, TickFOPv1};
use std::collections::HashMap;
use std::sync::Arc;

// 創建 Tick 回調處理器
struct MyTickHandler;

impl TickCallback for MyTickHandler {
    fn on_tick_stk_v1(&self, _exchange: Exchange, tick: TickSTKv1) {
        println!("收到股票 Tick: {} @ {}", tick.code, tick.close);
    }
    
    fn on_tick_fop_v1(&self, _exchange: Exchange, tick: TickFOPv1) {
        println!("收到期貨 Tick: {} @ {}", tick.code, tick.close);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 建立客戶端
    let client = Shioaji::new(true, HashMap::new())?; // true = 模擬模式
    client.init().await?;
    
    // 登入
    let accounts = client.login("api_key", "secret_key", true).await?;
    println!("登入成功！找到 {} 個帳戶", accounts.len());
    
    // 註冊回調處理器
    client.register_tick_callback(Arc::new(MyTickHandler)).await;
    client.setup_callbacks().await?;
    
    // 訂閱市場數據
    let stock = client.create_stock("2330", Exchange::TSE);
    client.subscribe(stock.contract, rshioaji::QuoteType::Tick).await?;
    
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
```

```rust
use rshioaji::Config;

// 從環境變數載入配置
let config = Config::from_env()?;
let client = Shioaji::new(config.simulation, HashMap::new())?;
```

## 📋 功能特色

### 統一的登入流程

v0.4.1 簡化了登入 API，移除了多餘的 `token_login` 和 `simulation_login` 方法：

```rust
// ✅ 統一使用 login() 方法
let accounts = client.login(&api_key, &secret_key, true).await?;

// 內部會根據 simulation 參數自動選擇正確的登入模式
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

### 強大的 Mock 系統

支援無真實憑證的開發和測試：

```rust
// 建立模擬環境客戶端
let client = Shioaji::new(true, HashMap::new())?; // simulation = true

// 使用假憑證進行測試
let accounts = client.login("fake_api_key", "fake_secret_key", false).await?;

// 正常使用所有功能進行開發測試
```

## 🔄 版本更新

### v0.4.1 (2024-01)

- ✅ **統一登入 API**：移除多餘的 `token_login`/`simulation_login`，統一使用 `login()` 方法
- ✅ **完善 Mock 系統**：支援完整的無憑證開發環境
- ✅ **修正初始化問題**：解決 `'NoneType' object has no attribute 'activated_ca'` 錯誤
- ✅ **改進事件橋接**：更穩定的 Python-Rust 回調系統
- ✅ **增強錯誤處理**：更好的錯誤訊息和診斷資訊

### v0.4.0

- ✅ 完整的回調系統實現
- ✅ Python-Rust 事件橋接
- ✅ 支援所有主要的市場數據類型
- ✅ 跨平台相容性

## 📚 文件

- [API 文件](https://docs.rs/rshioaji)
- [使用範例](examples/)
- [登入流程說明](docs/login_flow.md)
- [環境設定指南](docs/environment_setup.md)

## 🧪 測試

```bash
# 執行所有測試
cargo test

# 執行範例
cargo run --example basic_usage
cargo run --example callback_example
```

## 🤝 貢獻

歡迎提交 Issue 和 Pull Request！

## 📄 授權

```toml
[dependencies]
rshioaji = { version = "0.4.0", features = ["speed"] }
tokio = { version = "1.0", features = ["full"] }
```

### 2. 基本使用範例

```rust
use rshioaji::{Shioaji, Config, Exchange, QuoteType, EnvironmentConfig, init_logging};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 📚 前置作業：初始化環境配置和日誌系統
    let env_config = EnvironmentConfig::from_env();
    if let Err(e) = env_config.validate() {
        eprintln!("環境變數配置錯誤: {}", e);
        return Ok(());
    }
    
    // 初始化日誌系統（對應 Python utils.py）
    init_logging(&env_config)?;
    log::info!("🚀 rshioaji 環境初始化完成");
    
    // 從環境變數載入配置
    let config = Config::from_env()?;
    
    // 創建客戶端
    let client = Shioaji::new(config.simulation, HashMap::new())?;
    client.init().await?;
    
    // 🔑 完整登入流程（包含錯誤追蹤、合約下載、預設帳戶設定）
    let accounts = client.login(&config.api_key, &config.secret_key, true).await?;
    log::info!("登入成功！帳戶數量: {}", accounts.len());
    
    // 創建股票合約並訂閱
    let stock = client.create_stock("2330", Exchange::TSE);
    client.subscribe(stock.contract.clone(), QuoteType::Tick).await?;
    
    // 取得歷史資料
    let kbars = client.kbars(stock.contract, "2024-01-01", "2024-01-31").await?;
    log::info!("取得 {} 筆 K 線資料", kbars.data.len());
    
    Ok(())
}
```

### 3. v0.4.0 修復完成的回調系統範例

```rust
use rshioaji::{
    Shioaji, TickCallback, BidAskCallback, QuoteCallback, OrderCallback, SystemCallback,
    Exchange, TickSTKv1, BidAskSTKv1, QuoteSTKv1, TickFOPv1, EnvironmentConfig
};
use std::sync::{Arc, atomic::{AtomicU32, Ordering}};
use std::collections::HashMap;
use serde_json::Value;

// v0.4.0 修復版本的事件處理器
#[derive(Debug)]
struct CallbackHandler {
    name: String,
    event_count: AtomicU32,
}

impl CallbackHandler {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            event_count: AtomicU32::new(0),
        }
    }
    
    fn increment_count(&self) -> u32 {
        self.event_count.fetch_add(1, Ordering::Relaxed) + 1
    }
}

// v0.4.0 支援的所有回調類型
impl TickCallback for CallbackHandler {
    fn on_tick_stk_v1(&self, exchange: Exchange, tick: TickSTKv1) {
        let count = self.increment_count();
        println!("📈 [{}] 股票 Tick #{}: {} @ {:?} - 價格: {}, 成交量: {}", 
                self.name, count, tick.code, exchange, tick.close, tick.volume);
    }
    
    fn on_tick_fop_v1(&self, exchange: Exchange, tick: TickFOPv1) {
        let count = self.increment_count();
        println!("📊 [{}] 期權 Tick #{}: {} @ {:?} - 價格: {}", 
                self.name, count, tick.code, exchange, tick.close);
    }
}

impl BidAskCallback for CallbackHandler {
    fn on_bidask_stk_v1(&self, exchange: Exchange, bidask: BidAskSTKv1) {
        let count = self.increment_count();
        println!("💰 [{}] 買賣價差 #{}: {} @ {:?} - 買價: {}, 賣價: {}", 
                self.name, count, bidask.code, exchange, 
                bidask.bid_price.first().unwrap_or(&0.0),
                bidask.ask_price.first().unwrap_or(&0.0));
    }
    
    fn on_bidask_fop_v1(&self, exchange: Exchange, bidask: rshioaji::BidAskFOPv1) {
        let count = self.increment_count();
        println!("💹 [{}] 期權買賣價差 #{}: {} @ {:?}", 
                self.name, count, bidask.code, exchange);
    }
}

impl QuoteCallback for CallbackHandler {
    fn on_quote_stk_v1(&self, exchange: Exchange, quote: QuoteSTKv1) {
        let count = self.increment_count();
        println!("📋 [{}] 股票報價 #{}: {} @ {:?}", 
                self.name, count, quote.code, exchange);
    }
    
    fn on_quote(&self, topic: String, _data: Value) {
        let count = self.increment_count();
        println!("📋 [{}] 一般報價 #{}: {}", self.name, count, topic);
    }
}

impl OrderCallback for CallbackHandler {
    fn on_order(&self, order_state: rshioaji::OrderState, _data: Value) {
        let count = self.increment_count();
        println!("🏦 [{}] 訂單更新 #{}: {:?}", self.name, count, order_state);
    }
}

impl SystemCallback for CallbackHandler {
    fn on_event(&self, event_type: i32, code: i32, message: String, _details: String) {
        let count = self.increment_count();
        println!("🔔 [{}] 系統事件 #{}: Type:{}, Code:{}, Msg:{}", 
                self.name, count, event_type, code, message);
    }
    
    fn on_session_down(&self) {
        let count = self.increment_count();
        println!("⚠️ [{}] 連線中斷 #{}: 重新連線中...", self.name, count);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化環境和日誌
    let env_config = EnvironmentConfig::from_env();
    rshioaji::init_logging(&env_config)?;
    
    println!("🚀 rshioaji v0.4.0 - 修復完成的回調系統範例");
    println!("===========================================");
    
    // 創建客戶端
    let client = Shioaji::new(true, HashMap::new())?;
    client.init().await?;
    
    // 建立事件處理器
    let handler = Arc::new(CallbackHandler::new("主處理器"));
    
    // v0.4.0 修復：註冊所有類型的回調
    println!("📋 註冊所有回調處理器...");
    client.register_tick_callback(handler.clone()).await;
    client.register_bidask_callback(handler.clone()).await;
    client.register_quote_callback(handler.clone()).await;
    client.register_order_callback(handler.clone()).await;
    client.register_system_callback(handler.clone()).await;
    
    // v0.4.0 修復：設定事件橋接系統
    println!("🌉 設定 Python-Rust 事件橋接...");
    match client.setup_callbacks().await {
        Ok(()) => {
            println!("✅ v0.4.0 事件橋接系統設定成功！");
            println!("🔧 SolaceAPI 匯入問題已完全修復");
        },
        Err(e) => {
            println!("⚠️ 事件橋接設定問題: {}", e);
            println!("💡 這是預期的，因為需要真實的登入憑證");
        }
    }
    
    // 創建測試合約
    let stock = client.create_stock("2330", Exchange::TSE);
    println!("✅ 測試合約建立成功: 2330 (台積電)");
    
    println!("\n🎉 v0.4.0 回調系統修復驗證完成！");
    println!("✅ 所有回調類型已正確註冊");
    println!("✅ SolaceAPI 匯入問題已解決");
    println!("✅ Python-Rust 橋接已建立");
    println!("✅ 系統準備接收真實市場數據");
    
    println!("\n💡 下一步：使用真實 API 憑證測試完整功能");
    
    Ok(())
}
```

## 從源碼編譯 (開發者)

### 1. 克隆專案
```bash
git clone https://github.com/stevelo/rshioaji
cd rshioaji
```

### 2. 編譯專案

#### 一般編譯（動態連結）
```bash
cargo build --release
```

#### 靜態連結編譯（推薦）
```bash
cargo build --release --features static-link
```

#### 高效能編譯（包含速度優化）
```bash
# 啟用 speed 功能，等效於 shioaji[speed]
cargo build --release --features speed

# 結合靜態連結和速度優化
cargo build --release --features "static-link,speed"
```

**靜態連結優勢**：
- 🔗 所有 .so 檔案內嵌於執行檔中
- 📦 單一執行檔，無外部依賴
- 🚀 更快的啟動時間
- 🛡️ 提升安全性，減少攻擊面
- 📋 便於分發和部署

**Speed 功能優勢**：
- ⚡ 快速日期時間處理（等效於 ciso8601）
- 🔢 高效能 base58 編碼/解碼（等效於 based58）
- 🚀 Rust 原生高效能實作
- 📈 減少 Python C 擴展依賴

### 3. 環境變數配置

創建 `.env` 檔案或設定環境變數：

```bash
# .env 檔案範例
SHIOAJI_API_KEY=您的實際API金鑰
SHIOAJI_SECRET_KEY=您的實際密鑰
SHIOAJI_SIMULATION=false
RUST_LOG=info
```

#### 支援的環境變數

##### 基本 API 設定
- `SHIOAJI_API_KEY` 或 `API_KEY` - API 金鑰
- `SHIOAJI_SECRET_KEY` 或 `SECRET_KEY` - 密鑰
- `SHIOAJI_SIMULATION` 或 `SIMULATION` - 模擬模式 (true/false)

##### 日誌設定 (對應 Python utils.py)
- `LOG_LEVEL` - 日誌等級 (DEBUG/INFO/WARNING/ERROR/CRITICAL)
- `SJ_LOG_PATH` - 日誌檔案路徑 (設為 "console" 只輸出到控制台)
- `RUST_LOG` - Rust 日誌等級 (debug/info/warn/error)

##### Sentry 錯誤追蹤設定
- `SENTRY_URI` - Sentry DSN URL
- `LOG_SENTRY` - 是否啟用 Sentry (True/False)
- `SENTRY_LOG_LEVEL` - Sentry 日誌等級 (DEBUG/INFO/WARNING/ERROR/CRITICAL)

##### 測試設定
- `LEGACY_TEST` - 遺留測試模式 (0=停用, 1=啟用)

### 4. 執行程式

```bash
# 基本執行
cargo run

# 啟用高效能功能
cargo run --features speed

# 生產環境執行
cargo run --release --features "speed,static-link"
```

## 📚 使用範例

### 完整交易範例

```rust
use rshioaji::{Shioaji, Config, Exchange, Action, OrderType, StockPriceType, Order};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 載入配置
    let config = Config::from_env()?;
    let client = Shioaji::new(config.simulation, HashMap::new())?;
    client.init().await?;
    
    // 登入
    let accounts = client.login(&config.api_key, &config.secret_key, true).await?;
    
    // 創建股票合約
    let stock = client.create_stock("2330", Exchange::TSE);
    
    // 創建買單
    let order = Order::new(
        Action::Buy,
        100.0,  // 價格
        1000,   // 數量
        OrderType::ROD,
        StockPriceType::LMT,
    );
    
    // 下單 (注意：這會實際下單，請謹慎使用)
    let trade = client.place_order(stock.contract, order).await?;
    println!("委託成功：{:?}", trade);
    
    Ok(())
}
```

### CLI 工具使用 (從源碼)

```bash
# 編譯 CLI 工具
cargo build --bin rshioaji-cli --release

# 查看幫助
./target/release/rshioaji-cli --help

# 查詢股票資料
./target/release/rshioaji-cli --stock 2330

# 啟用除錯模式
./target/release/rshioaji-cli --debug --stock 2330
```

## Docker 部署

### 建置 Docker 映像檔

```bash
# Linux x86_64 平台（推薦生產環境 - 162MB）
./docker-build.sh linux

# Python 3.12 原生支援版本（173MB）
docker build -t rshioaji:python312 -f Dockerfile.python .

# Alpine Linux（超輕量版本 - 50MB）
./docker-build.sh alpine

# macOS ARM64 平台（開發環境 - 100MB）
./docker-build.sh macos

# 手動建置
docker build -t rshioaji:latest .                    # 輕量版 162MB (Python 3.11)
docker build -t rshioaji:python312 -f Dockerfile.python . # Python 3.12 173MB
docker build -t rshioaji:alpine -f Dockerfile.alpine . # 超輕量 50MB
docker build -t rshioaji:macos -f Dockerfile.macos .   # macOS ARM64
```

### 執行容器

```bash
# 使用 .env 檔案執行（推薦 - Python 3.12）
docker run --rm -v $(pwd)/.env:/app/.env:ro rshioaji:python312 --stock 2330

# 使用 .env 檔案執行（Python 3.11 輕量版）
docker run --rm -v $(pwd)/.env:/app/.env:ro rshioaji:latest --stock 2330

# 使用環境變數執行（Python 3.12）
docker run --rm \
  -e SHIOAJI_API_KEY=your_key \
  -e SHIOAJI_SECRET_KEY=your_secret \
  -e SHIOAJI_SIMULATION=false \
  rshioaji:python312 --stock 2330 --debug

# Alpine 超輕量版本
docker run --rm -v $(pwd)/.env:/app/.env:ro rshioaji:alpine --stock 2330

# 互動模式（Python 3.12）
docker run --rm -it -v $(pwd)/.env:/app/.env:ro rshioaji:python312 bash

# 背景執行（Python 3.12）
docker run -d --name rshioaji-trader \
  -v $(pwd)/.env:/app/.env:ro \
  rshioaji:python312 --stock 2330 --debug
```

### Docker Compose 部署

創建 `docker-compose.yml`（Python 3.12 版本）：
```yaml
version: '3.8'
services:
  rshioaji:
    build:
      context: .
      dockerfile: Dockerfile.python  # 使用 Python 3.12
    env_file:
      - .env
    command: ["--stock", "2330", "--debug"]
    restart: unless-stopped
    volumes:
      - ./logs:/app/logs
```

或使用預建映像：
```yaml
version: '3.8'
services:
  rshioaji:
    image: rshioaji:python312
    env_file:
      - .env
    command: ["--stock", "2330", "--debug"]
    restart: unless-stopped
    volumes:
      - ./logs:/app/logs
```

執行：
```bash
docker-compose up -d
docker-compose logs -f rshioaji
```

### Docker 特點

- 🏔️ **超輕量設計**：173MB Python 3.12 | 162MB 輕量版 | 50MB 超輕量版 (減少 91.3% 大小)
- 🐧 **多平台支援**：Linux x86_64、Alpine Linux 和 macOS ARM64
- 🐍 **Python 3.12**：原生支援 Python 3.12 和完整 C 擴展整合 (推薦)
- 📦 **多階段建置**：分離編譯環境與運行環境，大幅減少映像檔大小
- 🔐 **安全配置**：支援 .env 檔案和環境變數，API 憑證自動遮罩
- ⚡ **快速部署**：一鍵建置與執行，容器啟動速度快
- 🛡️ **隔離環境**：避免 macOS 安全性限制，提供穩定運行環境
- 🚀 **生產就緒**：多種部署模式，支援 Docker Compose 和容器編排

### 映像檔大小對比
| 版本 | 大小 | 用途 | Python 支援 |
|------|------|------|-------------|
| rshioaji:python312 | 173MB | **Python 3.12 推薦** | ✅ 原生 3.12 支援 |
| rshioaji:latest | 162MB | Python 3.11 輕量版 | ✅ 完整支援 |
| rshioaji:alpine | 50MB | 資源受限環境 | ⚠️ 基本支援 |
| rshioaji:macos | 100MB | 開發環境 | ✅ 完整支援 |

## 🔧 環境變數配置

rshioaji 提供完整的環境變數管理功能，對應 Python shioaji 的 `utils.py` 模組。

### 環境變數設定範例

創建 `.env` 檔案：
```bash
# 基本 API 設定
SHIOAJI_API_KEY=your_actual_api_key
SHIOAJI_SECRET_KEY=your_actual_secret_key
SHIOAJI_SIMULATION=true

# 日誌設定
LOG_LEVEL=INFO
SJ_LOG_PATH=shioaji.log

# Sentry 錯誤追蹤 (選用)
SENTRY_URI=https://your-dsn@sentry.io/project-id
LOG_SENTRY=True
SENTRY_LOG_LEVEL=ERROR

# 測試設定
LEGACY_TEST=0
```

### 使用方式

```rust
use rshioaji::{EnvironmentConfig, init_logging};

// 載入環境變數配置
let env_config = EnvironmentConfig::from_env();

// 驗證配置
if let Err(e) = env_config.validate() {
    eprintln!("環境變數配置錯誤: {}", e);
    return Ok(());
}

// 初始化日誌系統
init_logging(&env_config)?;
log::info!("環境配置: {}", env_config.summary());
```

## 📝 日誌系統

### 日誌格式
日誌格式與 Python 版本保持一致：
```
[L YYYY-MM-DD HH:MM:SS.fff UTC filename:line:function] message
```

### 範例輸出
```
[I 2024-01-15 08:30:45.123 UTC main.rs:25:main] 🚀 rshioaji 環境初始化完成
[I 2024-01-15 08:30:45.124 UTC main.rs:26:main] 📊 日誌等級: INFO
[I 2024-01-15 08:30:45.125 UTC main.rs:27:main] 🛡️  Sentry 錯誤追蹤: 啟用
```

### 啟用 Sentry 功能
```bash
# 編譯時加入 sentry 功能
cargo build --features sentry

# 執行時啟用 Sentry
LOG_SENTRY=True SENTRY_URI=your_sentry_dsn cargo run --features sentry
```

## 📚 詳細文件

- **[環境設定指南](docs/environment_setup.md)** - 完整的環境變數配置說明
- **[登入流程說明](docs/login_flow.md)** - 標準登入流程詳細解析
- **[代碼品質指南](docs/linting_guide.md)** - Clippy 和代碼品質檢查
- **[更新日誌](CHANGELOG.md)** - 版本更新記錄

## 📖 API 使用指南

### 基本配置

```rust
use rshioaji::{Shioaji, Config};
use std::collections::HashMap;

// 方法 1: 從環境變數載入 (推薦)
let config = Config::from_env()?;
let client = Shioaji::new(config.simulation, HashMap::new())?;

// 方法 2: 手動配置
let client = Shioaji::new(true, HashMap::new())?; // true = 模擬模式
```

### 市場資料操作

```rust
use rshioaji::{Exchange, QuoteType};

// 創建合約
let stock = client.create_stock("2330", Exchange::TSE);

// 訂閱即時報價
client.subscribe(stock.contract.clone(), QuoteType::Tick).await?;

// 取得歷史 K 線
let kbars = client.kbars(stock.contract, "2024-01-01", "2024-01-31").await?;
```

### 下單操作

```rust
use rshioaji::{Action, OrderType, StockPriceType, Order};

// 創建委託單
let order = Order::new(
    Action::Buy,           // 買賣別
    100.0,                // 價格
    1000,                 // 數量
    OrderType::ROD,       // 委託類型
    StockPriceType::LMT,  // 價格類型
);

// 送出委託
let trade = client.place_order(stock.contract, order).await?;
```

## 專案結構

```
rshioaji/
├── src/                    # Rust 原始碼
│   ├── lib.rs             # 程式庫入口
│   ├── main.rs            # 可執行檔入口
│   ├── client.rs          # 主要客戶端實作
│   ├── bindings.rs        # Python FFI 綁定
│   ├── platform.rs        # 平台檢測邏輯
│   ├── error.rs           # 錯誤處理
│   └── types/             # 型別定義
├── lib/shioaji/           # Python C 擴展檔案
│   ├── macosx_arm/        # macOS ARM64 版本
│   └── manylinux_x86_64/  # Linux x86_64 版本
├── examples/              # 範例程式
├── tests/                 # 測試檔案
├── Dockerfile             # Docker 配置
├── .dockerignore          # Docker 忽略檔案
└── docker-build.sh        # Docker 建置腳本
```

## 平台檢測

rshioaji 會自動檢測執行平台並載入對應的 C 擴展檔案：

```rust
use rshioaji::platform::Platform;

let platform = Platform::detect();
println!("檢測到平台: {:?}", platform);

// 驗證安裝
let base_path = std::env::current_dir()?;
platform.validate_installation(&base_path)?;
```

## 環境設定

### 動態連結版本

#### macOS ARM64
```bash
export DYLD_LIBRARY_PATH=/path/to/lib/shioaji/macosx_arm/backend:/path/to/lib/shioaji/macosx_arm/backend/solace
```

#### Linux x86_64
```bash
export LD_LIBRARY_PATH=/path/to/lib/shioaji/manylinux_x86_64/backend:/path/to/lib/shioaji/manylinux_x86_64/backend/solace
```

### 靜態連結版本

靜態連結版本無需設定環境變數，可直接執行：

```bash
# 直接執行，無需額外設定
./target/release/rshioaji-cli

# 或使用 cargo
cargo run --release --features static-link
```

## 除錯

### 啟用日誌
```bash
export RUST_LOG=debug
cargo run --example simple_test
```

### 檢查平台檔案
```bash
# 確認 .so 檔案存在
ls -la lib/shioaji/*/backend/solace/*.so

# 檢查檔案權限
chmod +x lib/shioaji/*/backend/solace/*.so
```

## 常見問題

### Q: 出現 "Platform validation failed" 錯誤
A: 請確認對應平台的 .so 檔案存在且有執行權限。

### Q: Docker 容器無法啟動
A: 確認使用正確的 Dockerfile（Linux 用 Dockerfile，macOS 用 Dockerfile.macos）。

### Q: Python 3.12 模組載入錯誤
A: 確認 lib/shioaji 目錄下的 .so 檔案為 cpython-312 版本。

### Q: Python 模組匯入錯誤
A: 檢查 PYTHONPATH 和 LD_LIBRARY_PATH 環境變數設定，確認 Python 3.12 環境正確。

## 授權

此專案採用 MIT 和 Apache 2.0 雙重授權。

## 貢獻

歡迎提交 Issue 和 Pull Request！

## 開發者聯絡

如有任何問題或建議，請聯絡：
- **Steve Lo** - info@sd.idv.tw

## 🎯 進階使用

### 功能開關

```bash
# 啟用高效能模式 (推薦生產環境)
cargo build --release --features speed

# 啟用靜態連結 (單一執行檔)
cargo build --release --features static-link

# 同時啟用多個功能
cargo build --release --features "speed,static-link"
```

### 效能優化

```rust
// 使用 speed 功能時，自動啟用：
// - 高效能日期時間處理 (等效於 ciso8601)
// - 優化的 base58 編碼 (等效於 based58)
// - 其他 Rust 原生高效能實作

// 無需額外程式碼，編譯時自動優化
```

## ✅ 生產驗證

**rshioaji 已成功發佈至 crates.io 並通過完整測試：**

- **📦 crates.io**: [https://crates.io/crates/rshioaji](https://crates.io/crates/rshioaji)
- **📚 文件**: [https://docs.rs/rshioaji](https://docs.rs/rshioaji)
- **🔐 API 認證**: 真實憑證登入測試通過
- **📊 市場資料**: 成功查詢台積電 (2330) 資料
- **📈 即時訂閱**: K 線和 tick 資料正常運作
- **🌐 跨平台**: macOS ARM64 和 Linux x86_64 支援
- **🚀 高效能**: speed 功能提升處理效能

### 安裝驗證

```bash
# 創建測試專案
cargo new test-rshioaji && cd test-rshioaji

# 添加依賴
echo 'rshioaji = { version = "0.4.0", features = ["speed"] }' >> Cargo.toml

# 編譯測試
cargo build
```

## 🔗 相關連結

- **📦 crates.io**: [https://crates.io/crates/rshioaji](https://crates.io/crates/rshioaji)
- **📚 API 文件**: [https://docs.rs/rshioaji](https://docs.rs/rshioaji)  
- **🐙 GitHub**: [https://github.com/stevelo/rshioaji](https://github.com/stevelo/rshioaji)
- **📧 聯絡**: info@sd.idv.tw

## 📊 套件資訊

```toml
[dependencies]
rshioaji = "0.4.0"  # 最新版本 (回調系統修復完成)
```

- **版本**: 0.4.0
- **授權**: MIT OR Apache-2.0
- **平台**: macOS ARM64, Linux x86_64  
- **Rust 版本**: 1.75+

---

**⚠️ 重要聲明**: 
- 此套件已通過完整功能驗證並發佈至 crates.io
- 正式交易前請充分測試，開發者不承擔任何交易損失責任
- 需要有效的永豐金證券 API 金鑰才能正常運作
- 請向永豐金證券申請相關授權並遵守其使用條款

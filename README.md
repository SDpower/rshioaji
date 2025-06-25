# rshioaji

一個用 Rust 封裝台灣永豐金證券 shioaji API 的高效能交易程式庫，支援多平台部署。

[![Crates.io](https://img.shields.io/crates/v/rshioaji.svg)](https://crates.io/crates/rshioaji)
[![Documentation](https://docs.rs/rshioaji/badge.svg)](https://docs.rs/rshioaji)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/stevelo/rshioaji)

**✅ 已成功發佈至 [crates.io](https://crates.io/crates/rshioaji)**

## 開發者資訊

**開發者**: Steve Lo  
**聯絡方式**: info@sd.idv.tw  
**專案性質**: 概念驗證 (Proof of Concept) 專案

## 特點

- 🚀 **高效能**：基於 Rust 實現，提供優秀的執行效能和記憶體安全
- 🔗 **相容性**：直接使用系統安裝的 Python shioaji，確保完整功能相容性
- 🌐 **多平台支援**：支援 macOS ARM64 和 Linux x86_64 平台
- 📦 **純系統整合**：無需嵌入 .so 檔案，直接使用 pip install shioaji
- 🐳 **容器化**：提供 Docker 支援，便於部署和分發
- ⚡ **非同步**：基於 tokio 實現非同步操作
- 🛡️ **型別安全**：完整的 Rust 型別定義，編譯時錯誤檢查
- 🔧 **環境變數管理**：完整的環境變數處理和驗證，對應 Python utils.py
- 📝 **日誌系統**：與 Python 版本相同格式的日誌系統
- 🔍 **錯誤追蹤**：支援 Sentry 整合和錯誤監控
- 🔑 **完整登入流程**：實現與 Python 版本相同的標準登入步驟
- 📡 **事件回調系統**：原生 Rust trait 系統，支援市場資料、訂單和系統事件回調

## 📦 安裝

### 從 crates.io 安裝 (推薦)

在您的 `Cargo.toml` 中添加：

```toml
[dependencies]
# 基本版本
rshioaji = "0.4.6"

# 啟用高效能功能 (推薦)
rshioaji = { version = "0.4.6", features = ["speed"] }

# 啟用所有功能 + 事件回調
rshioaji = { version = "0.4.6", features = ["speed"] }
```

### 可用功能 (Features)

| 功能 | 描述 | 用途 |
|------|------|------|
| `speed` | 🚀 高效能時間處理 | 等效於 Python `shioaji[speed]`，提升日期時間處理效能 |
| `sentry` | 🔍 Sentry 錯誤追蹤 | 支援 Sentry 錯誤監控和追蹤功能 |

## 🎯 新功能 v0.4.6 - 完整實現市場資料訂閱與回調系統

### 重大變更說明 (從 v0.2.0 跳躍至 v0.4.6)

由於 v0.2.0 版本在功能實現上存在問題，我們進行了大幅度的架構重構和功能完善：

- **❌ v0.2.0 問題**：回調系統未完全實現，市場資料訂閱存在問題
- **✅ v0.4.6 成果**：完整實現 Python → Rust 回調轉發，成功接收真實市場資料
- **🚀 跳躍版本**：反映重大架構改進和功能完整性

### v0.4.6 核心功能

### 支援的回調類型

| 回調類型 | 介面 | 描述 |
|----------|------|------|
| **市場資料回調** | `TickCallback` | 處理股票和期權的 tick 資料事件 |
| **買賣價差回調** | `BidAskCallback` | 處理委買委賣價差變化事件 |
| **報價回調** | `QuoteCallback` | 處理即時報價和綜合報價事件 |
| **訂單回調** | `OrderCallback` | 處理訂單狀態變更和成交事件 |
| **系統回調** | `SystemCallback` | 處理系統事件和連線狀態變化 |

### 回調系統特點

- 🔧 **原生 Rust Trait**：完全基於 Rust trait 系統，型別安全
- 🚀 **高效能事件處理**：零開銷抽象，直接函數調用
- 📡 **多重處理器支援**：可註冊多個回調處理器
- 🛡️ **線程安全**：支援多線程環境下的安全事件分發
- 🎯 **靈活組合**：可選擇性實作需要的回調介面

### 🔧 合約存取架構改進 (2025-06-25)

#### 重要變更：`get_system_contract` 方法

- **方法重新命名**：`create_system_contract` → `get_system_contract`
- **語意更準確**：反映實際功能（取得現有合約，而非建立新合約）
- **架構對齊**：與 Python shioaji 的 `api.Contracts.Stocks["2330"]` 模式一致

#### 新增安全檢查

- **必要條件**：使用前必須先呼叫 `login()` 方法
- **錯誤處理**：未登入時回傳清楚的錯誤訊息
- **安全性**：防止在未認證狀態下存取合約資料

```rust
// ❌ 錯誤用法：未登入就嘗試存取合約
let client = Shioaji::new(false, HashMap::new())?;
client.place_order(contract, order).await?; // 會失敗並提示需要登入

// ✅ 正確用法：先登入再存取合約
let client = Shioaji::new(false, HashMap::new())?;
client.init().await?;
client.login(&api_key, &secret_key, true, 30, None, false, 30000).await?;
client.place_order(contract, order).await?; // 成功，取得真實 Python 合約實例
```

### 編譯選項

```bash
# 基本編譯
cargo build

# 啟用高效能功能
cargo build --features speed

# 生產環境編譯 (推薦)
cargo build --release --features speed
```

## 支援平台

- **macOS ARM64** (`macosx_arm`)
- **Linux x86_64** (`manylinux_x86_64`)

## 開發環境需求

### 系統需求
- Rust 1.75+
- Python 3.13+ (完整支援並測試驗證)
- 系統安裝的 shioaji 套件：`pip install shioaji`

### 開發依賴
- PyO3 0.20+
- tokio 1.0+
- serde 1.0+

## 🚀 快速開始

### 1. 安裝套件

```bash
# 創建新的 Rust 專案
cargo new my-trading-app
cd my-trading-app

# 編輯 Cargo.toml 添加依賴
```

```toml
[dependencies]
rshioaji = { version = "0.4.6", features = ["speed"] }
tokio = { version = "1.0", features = ["full"] }
```

### 2. 基本使用範例

```rust
use rshioaji::{Shioaji, Exchange, Action, OrderType, Order, StockPriceType};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化環境
    dotenvy::dotenv().ok();
    env_logger::init();
    
    // 創建客戶端
    let client = Shioaji::new(false, HashMap::new())?; // false = 真實模式
    client.init().await?;
    
    // 🔑 重要：必須先登入才能存取合約
    // get_system_contract 方法會檢查登入狀態
    let api_key = std::env::var("SHIOAJI_API_KEY")?;
    let secret_key = std::env::var("SHIOAJI_SECRET_KEY")?;
    
    let accounts = client.login(
        &api_key, 
        &secret_key, 
        true,    // fetch_contract: 下載合約資料
        30,      // contracts_timeout
        None,    // contracts_cb
        false,   // subscribe_trade
        30000    // receive_window
    ).await?;
    
    println!("✅ 登入成功！帳戶數量: {}", accounts.len());
    
    // ✅ 登入後可以安全存取合約
    // get_system_contract 會從 api.Contracts.Stocks["2330"] 取得真實 Python 實例
    let stock = client.create_stock("2330", Exchange::TSE);
    let order = Order::new(Action::Buy, 500.0, 1000, OrderType::ROD, StockPriceType::LMT);
    
    match client.place_order(stock.contract, order).await {
        Ok(trade) => println!("下單成功！交易 ID: {}", trade.order_id),
        Err(e) => println!("下單失敗：{}", e),
    }
    
    // 登出
    client.logout().await?;
    
    Ok(())
}
```

#### 🛡️ 安全檢查重點

```rust
// ❌ 錯誤：未登入就嘗試下單
let client = Shioaji::new(false, HashMap::new())?;
client.place_order(contract, order).await?; 
// Error: "Must login first before accessing contracts. Please call login() method."

// ✅ 正確：先登入再操作
let client = Shioaji::new(false, HashMap::new())?;
client.init().await?;
client.login(&api_key, &secret_key, true, 30, None, false, 30000).await?;
client.place_order(contract, order).await?; // 成功
```

### 3. 事件回調系統範例 (新功能)

```rust
use rshioaji::{
    Shioaji, TickCallback, BidAskCallback, QuoteCallback, OrderCallback, SystemCallback,
    TickSTKv1, TickFOPv1, BidAskSTKv1, BidAskFOPv1, QuoteSTKv1, OrderState, Exchange
};
use std::sync::Arc;

// 實作事件處理器
#[derive(Debug)]
struct MyEventHandler {
    name: String,
}

impl TickCallback for MyEventHandler {
    fn on_tick_stk_v1(&self, exchange: Exchange, tick: TickSTKv1) {
        println!("📈 [{}] 股票 Tick: {} @ {:?} - 價格: {}", 
                self.name, tick.code, exchange, tick.close);
    }
    
    fn on_tick_fop_v1(&self, exchange: Exchange, tick: TickFOPv1) {
        println!("📊 [{}] 期權 Tick: {} @ {:?} - 價格: {}", 
                self.name, tick.code, exchange, tick.close);
    }
}

impl OrderCallback for MyEventHandler {
    fn on_order(&self, order_state: OrderState, data: serde_json::Value) {
        println!("📋 [{}] 訂單更新: {:?}", self.name, order_state);
    }
}

impl SystemCallback for MyEventHandler {
    fn on_event(&self, event_type: i32, code: i32, message: String, details: String) {
        println!("🔔 [{}] 系統事件: {}", self.name, message);
    }
    
    fn on_session_down(&self) {
        println!("⚠️ [{}] 連線中斷！", self.name);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Shioaji::new(true, HashMap::new())?;
    client.init().await?;
    
    // 建立事件處理器
    let handler = Arc::new(MyEventHandler { name: "主處理器".to_string() });
    
    // 註冊各種回調
    client.register_tick_callback(handler.clone()).await;
    client.register_order_callback(handler.clone()).await;
    client.register_system_callback(handler.clone()).await;
    
    // 設定回調系統
    client.setup_callbacks().await?;
    
    println!("✅ 事件回調系統已啟動");
    
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

#### 高效能編譯（推薦）
```bash
# 啟用 speed 功能，等效於 shioaji[speed]
cargo build --release --features speed
```

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

##### 系統需求
- **必要**: 系統安裝的 shioaji 套件：`pip install shioaji`
- **檢查**: 確認 Python 可以正確導入 shioaji 模組

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
cargo run --release --features speed
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

# Python 3.13 原生支援版本（200MB）
docker build -t rshioaji:python312 -f Dockerfile.python .

# Alpine Linux（超輕量版本 - 50MB）
./docker-build.sh alpine

# macOS ARM64 平台（開發環境 - 100MB）
./docker-build.sh macos

# 手動建置
docker build -t rshioaji:latest .                    # 輕量版 162MB (Python 3.11)
docker build -t rshioaji:python313 -f Dockerfile.python . # Python 3.13 200MB
docker build -t rshioaji:alpine -f Dockerfile.alpine . # 超輕量 50MB
docker build -t rshioaji:macos -f Dockerfile.macos .   # macOS ARM64
```

### 執行容器

```bash
# 使用 .env 檔案執行（推薦 - Python 3.13）
docker run --rm -v $(pwd)/.env:/app/.env:ro rshioaji:python313 --stock 2330

# 使用 .env 檔案執行（Python 3.11 輕量版）
docker run --rm -v $(pwd)/.env:/app/.env:ro rshioaji:latest --stock 2330

# 使用環境變數執行（Python 3.13）
docker run --rm \
  -e SHIOAJI_API_KEY=your_key \
  -e SHIOAJI_SECRET_KEY=your_secret \
  -e SHIOAJI_SIMULATION=false \
  rshioaji:python313 --stock 2330 --debug

# Alpine 超輕量版本
docker run --rm -v $(pwd)/.env:/app/.env:ro rshioaji:alpine --stock 2330

# 互動模式（Python 3.13）
docker run --rm -it -v $(pwd)/.env:/app/.env:ro rshioaji:python313 bash

# 背景執行（Python 3.12）
docker run -d --name rshioaji-trader \
  -v $(pwd)/.env:/app/.env:ro \
  rshioaji:python313 --stock 2330 --debug
```

### Docker Compose 部署

創建 `docker-compose.yml`（Python 3.13 版本）：
```yaml
version: '3.8'
services:
  rshioaji:
    build:
      context: .
      dockerfile: Dockerfile.python  # 使用 Python 3.13
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
    image: rshioaji:python313
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

- 🏔️ **超輕量設計**：200MB Python 3.13 | 180MB 輕量版 | 70MB 超輕量版 (減少 88% 大小)
- 🐧 **多平台支援**：Linux x86_64、Alpine Linux 和 macOS ARM64
- 🐍 **Python 3.13**：原生支援 Python 3.13 和 PyO3 橋接整合 (推薦)
- 📦 **多階段建置**：分離編譯環境與運行環境，大幅減少映像檔大小
- 🔐 **安全配置**：支援 .env 檔案和環境變數，API 憑證自動遮罩
- ⚡ **快速部署**：一鍵建置與執行，容器啟動速度快
- 🛡️ **隔離環境**：避免 macOS 安全性限制，提供穩定運行環境
- 🚀 **生產就緒**：多種部署模式，支援 Docker Compose 和容器編排

### 映像檔大小對比
| 版本 | 大小 | 用途 | Python 支援 |
|------|------|------|-------------|
| rshioaji:python313 | 200MB | **Python 3.13 推薦** | ✅ 原生 3.13 支援 |
| rshioaji:latest | 180MB | Python 3.13 輕量版 | ✅ 完整支援 |
| rshioaji:alpine | 70MB | 資源受限環境 | ⚠️ 基本支援 |
| rshioaji:macos | 120MB | 開發環境 | ✅ 完整支援 |

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
- **[回調系統使用指南](docs/callback_usage.md)** - 完整的事件回調系統使用說明
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
├── examples/              # 範例程式
├── tests/                 # 測試檔案
├── Dockerfile             # Docker 配置
├── .dockerignore          # Docker 忽略檔案
└── docker-build.sh        # Docker 建置腳本
```

## 平台檢測

rshioaji 會自動檢測執行平台並確認系統 shioaji 安裝：

```rust
use rshioaji::platform::Platform;

let platform = Platform::detect();
println!("檢測到平台: {:?}", platform);

// 驗證系統 shioaji 安裝
platform.validate_system_shioaji()?;
```

## 環境設定

### 系統要求

#### 安裝系統 shioaji
```bash
# 安裝 Python shioaji 套件
pip install shioaji

# 驗證安裝
python3 -c "import shioaji; print('✅ 系統 shioaji 安裝成功')"
```

### 純系統整合

v0.2.0+ 使用純系統 shioaji 整合，無需設定環境變數：

```bash
# 直接執行，自動檢測系統 shioaji
./target/release/rshioaji-cli

# 或使用 cargo
cargo run --release --features speed
```

## 除錯

### 啟用日誌
```bash
export RUST_LOG=debug
cargo run --example simple_test
```

### 檢查系統安裝
```bash
# 確認系統 shioaji 安裝
python3 -c "import shioaji; s=shioaji.Shioaji(); print('✅ 系統 shioaji 正常')"

# 檢查 Python 環境
which python3
python3 --version
```

## 常見問題

### Q: 出現 "Platform validation failed" 錯誤
A: 請確認系統已安裝 shioaji：`pip install shioaji`，並確認可以正常導入。

### Q: Docker 容器無法啟動
A: 確認使用正確的 Dockerfile（Linux 用 Dockerfile，macOS 用 Dockerfile.macos）。

### Q: Python 3.13 模組載入錯誤
A: 確認系統 Python 環境正確且已安裝 shioaji：`pip install shioaji`。

### Q: Python 模組匯入錯誤
A: 檢查系統 Python 環境，確認 shioaji 正確安裝：`python3 -c "import shioaji"`。

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

# 基本編譯 (純系統整合)
cargo build --release
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
echo 'rshioaji = { version = "0.4.6", features = ["speed"] }' >> Cargo.toml

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
rshioaji = "0.4.6"  # 最新版本 (支援事件回調)
```

- **版本**: 0.4.6
- **授權**: MIT OR Apache-2.0
- **平台**: macOS ARM64, Linux x86_64  
- **Rust 版本**: 1.75+

---

**⚠️ 重要聲明**: 
- 此套件已通過完整功能驗證並發佈至 crates.io
- 正式交易前請充分測試，開發者不承擔任何交易損失責任
- 需要有效的永豐金證券 API 金鑰才能正常運作
- 請向永豐金證券申請相關授權並遵守其使用條款

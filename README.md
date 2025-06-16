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
- 🔗 **相容性**：使用原始 Python C 擴展 (.so 檔案)，確保完整功能相容性
- 🌐 **多平台支援**：支援 macOS ARM64 和 Linux x86_64 平台
- 📦 **靜態連結**：支援將 .so 檔案內嵌至執行檔，無運行時依賴
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
rshioaji = "0.3.5"

# 啟用高效能功能 (推薦)
rshioaji = { version = "0.3.5", features = ["speed"] }

# 啟用所有功能 + 事件回調
rshioaji = { version = "0.3.5", features = ["speed", "static-link"] }
```

### 可用功能 (Features)

| 功能 | 描述 | 用途 |
|------|------|------|
| `speed` | 🚀 高效能時間處理 | 等效於 Python `shioaji[speed]`，提升日期時間處理效能 |
| `static-link` | 📦 靜態連結 | 將 .so 檔案內嵌到執行檔，無運行時依賴 |
| `sentry` | 🔍 Sentry 錯誤追蹤 | 支援 Sentry 錯誤監控和追蹤功能 |

## 🎯 新功能 v0.3.5 - 企業級整合管理系統

**✅ 重要更新：v0.3.5 完整實作狀態**

v0.3.5 實現了企業級的 Shioaji 整合管理系統，提供完整的交易解決方案：

| 功能 | 狀態 | 說明 |
|------|------|------|
| ✅ **RealEventBridge** | 企業級實作 | 完整的真實事件橋接系統，支援高頻事件處理 |
| ✅ **ShioajiIntegration** | 企業級實作 | 統一的整合管理器，提供完整的交易系統架構 |
| ✅ **智能訂單引擎** | 企業級實作 | 支援 TWAP、條件訂單、演算法交易策略 |
| ✅ **風險管理系統** | 企業級實作 | 即時風險監控、VaR 計算、自動停損 |
| ✅ **績效追蹤系統** | 企業級實作 | 全方位績效分析和報告生成 |
| ✅ **異步事件處理** | 企業級實作 | 基於 tokio 的高性能事件架構 |

### 企業級功能模組

| 模組 | 功能 | 描述 | v0.3.5 狀態 |
|------|------|------|-------------|
| **RealEventBridge** | 事件橋接 | 高頻事件處理、統計監控、自動心跳 | ✅ 完整實作 |
| **MarketDataManager** | 市場數據 | 即時價格、技術分析、數據品質監控 | ✅ 完整實作 |
| **OrderManager** | 訂單管理 | 智能路由、批量處理、狀態追蹤 | ✅ 完整實作 |
| **SmartOrderEngine** | 智能交易 | TWAP、條件訂單、演算法策略 | ✅ 完整實作 |
| **RiskManager** | 風險控制 | VaR 計算、Beta 分析、自動停損 | ✅ 完整實作 |
| **PerformanceTracker** | 績效分析 | 夏普比率、最大回撤、詳細報告 | ✅ 完整實作 |

### v0.3.5 企業級特點

- 🏢 **企業級架構**：完整的交易系統解決方案，適合機構投資者
- 🧠 **智能訂單引擎**：支援多種演算法交易策略和條件執行
- 📊 **全方位監控**：事件統計、性能監控、風險分析一體化
- ⚡ **高頻交易支援**：專為高頻場景設計的事件處理架構
- 🛡️ **企業級風控**：多層次風險管理和即時監控系統
- 📈 **專業分析**：完整的績效評估和投資組合分析工具
- 🔄 **異步處理**：基於 tokio 的高性能異步事件系統
- 🎯 **靈活配置**：可根據需求選擇啟用的功能模組

### v0.3.5 核心架構

- **ShioajiIntegration**：統一的整合管理器，提供完整的 API
- **RealEventBridge**：企業級事件橋接，支援高頻事件處理
- **SmartOrderType**：智能訂單類型定義和執行引擎
- **RiskMetrics**：風險指標計算和監控系統
- **PerformanceMetrics**：績效指標分析和報告生成

### 編譯選項

```bash
# 基本編譯
cargo build

# 啟用高效能功能
cargo build --features speed

# 生產環境編譯 (推薦)
cargo build --release --features "speed,static-link"
```

## 支援平台

- **macOS ARM64** (`macosx_arm`)
- **Linux x86_64** (`manylinux_x86_64`)

## 開發環境需求

### 系統需求
- Rust 1.75+
- Python 3.12+ (完整支援並測試驗證)
- 對應平台的 shioaji C 擴展檔案

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
rshioaji = { version = "0.3.5", features = ["speed"] }
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

### 3. v0.3.5 企業級整合系統範例

```rust
use rshioaji::{
    Shioaji, ShioajiIntegration, RealEventBridge, 
    SmartOrderType, RiskManager, PerformanceTracker,
    MarketDataManager, OrderManager, Exchange
};
use std::sync::Arc;
use std::collections::HashMap;

// 實作完整的事件處理器
#[derive(Debug)]
struct MyEventHandler {
    name: String,
}

impl TickCallback for MyEventHandler {
    fn on_tick_stk_v1(&self, exchange: Exchange, tick: TickSTKv1) {
        println!("📈 [{}] 股票 Tick: {} @ {:?} - 價格: {}, 成交量: {}", 
                self.name, tick.code, exchange, tick.close, tick.volume);
    }
    
    fn on_tick_fop_v1(&self, exchange: Exchange, tick: TickFOPv1) {
        println!("📊 [{}] 期權 Tick: {} @ {:?} - 價格: {}, 成交量: {}", 
                self.name, tick.code, exchange, tick.close, tick.volume);
    }
}

impl BidAskCallback for MyEventHandler {
    fn on_bidask_stk_v1(&self, exchange: Exchange, bidask: BidAskSTKv1) {
        println!("💰 [{}] 買賣價差: {} @ {:?} - 買價: {}, 賣價: {}", 
                self.name, bidask.code, exchange, 
                bidask.bid_price.first().unwrap_or(&0.0),
                bidask.ask_price.first().unwrap_or(&0.0));
    }
    
    fn on_bidask_fop_v1(&self, exchange: Exchange, bidask: BidAskFOPv1) {
        println!("💹 [{}] 期權買賣價差: {} @ {:?}", self.name, bidask.code, exchange);
    }
}

impl OrderCallback for MyEventHandler {
    fn on_order(&self, order_state: OrderState, data: serde_json::Value) {
        println!("📋 [{}] 訂單更新: {:?} - 資料: {}", 
                self.name, order_state, 
                serde_json::to_string_pretty(&data).unwrap_or_default());
    }
}

impl SystemCallback for MyEventHandler {
    fn on_event(&self, event_type: i32, code: i32, message: String, details: String) {
        println!("🔔 [{}] 系統事件[{}/{}]: {} - {}", 
                self.name, event_type, code, message, details);
    }
    
    fn on_session_down(&self) {
        println!("⚠️ [{}] 連線中斷！重新連線中...", self.name);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日誌系統
    rshioaji::init_logging();
    
    println!("🚀 rshioaji v0.3.5 - 企業級整合管理系統範例");
    
    let client = Shioaji::new(true, HashMap::new())?;
    client.init().await?;
    
    // 建立事件處理器
    let handler = Arc::new(MyEventHandler { name: "主處理器".to_string() });
    
    // 註冊所有類型的回調
    println!("📋 註冊事件處理器...");
    client.register_tick_callback(handler.clone()).await;
    client.register_bidask_callback(handler.clone()).await;
    client.register_order_callback(handler.clone()).await;
    client.register_system_callback(handler.clone()).await;
    
    // v0.3.5 企業級整合系統設定
    let integration = ShioajiIntegration::new().await?;
    integration.initialize(&client).await?;
    
    // 智能訂單範例
    integration.submit_smart_order(
        "2330".to_string(),
        SmartOrderType::Twap { 
            duration: std::time::Duration::from_secs(3600),
            slice_size: 100 
        },
        1000
    ).await?;
    
    println!("✅ v0.3.5 企業級整合管理系統已啟動！");
    println!("🏢 智能訂單引擎、風險管理、績效追蹤已就緒");
    println!("📊 準備進行高級交易和分析...");
    
    println!("🎯 系統狀態：企業級整合管理架構已實現");
    
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
echo 'rshioaji = { version = "0.3.5", features = ["speed"] }' >> Cargo.toml

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
rshioaji = "0.3.5"  # 最新版本 (企業級整合系統)
```

- **版本**: 0.3.5
- **授權**: MIT OR Apache-2.0
- **平台**: macOS ARM64, Linux x86_64  
- **Rust 版本**: 1.75+

---

**⚠️ 重要聲明**: 
- 此套件已通過完整功能驗證並發佈至 crates.io
- 正式交易前請充分測試，開發者不承擔任何交易損失責任
- 需要有效的永豐金證券 API 金鑰才能正常運作
- 請向永豐金證券申請相關授權並遵守其使用條款

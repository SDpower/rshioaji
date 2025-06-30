# rshioaji 環境設定說明

## 📋 概述

根據 Python shioaji `utils.py` 的功能，我們在 Rust 版本中實現了相同的環境變數處理和前置作業。

## 🔧 支援的環境變數

### 基本 API 設定
```bash
SHIOAJI_API_KEY=your_actual_api_key_here
SHIOAJI_SECRET_KEY=your_actual_secret_key_here
SHIOAJI_SIMULATION=true
```

### PyO3 橋接需求 (v0.4.8+)
```bash
# Python 版本 (建議 3.13+)
PYTHON_VERSION=3.13

# PyO3 Python 路徑 (可選，自動檢測)
PYO3_PYTHON=python3.13

# shioaji 套件安裝 (必須)
# 請確保已安裝: pip install "shioaji[speed]"
```

### 日誌設定（對應 Python utils.py）
```bash
# 日誌等級，允許值: DEBUG, INFO, WARNING, ERROR, CRITICAL
LOG_LEVEL=INFO

# 日誌檔案路徑，設定為 "console" 則只輸出到控制台
SJ_LOG_PATH=shioaji.log
```

### Sentry 錯誤追蹤設定
```bash
# Sentry DSN URL
SENTRY_URI=https://6aec6ef8db7148aa979a17453c0e44dd@sentry.io/1371618

# 是否啟用 Sentry 日誌記錄
LOG_SENTRY=True

# Sentry 日誌等級，允許值: DEBUG, INFO, WARNING, ERROR, CRITICAL
SENTRY_LOG_LEVEL=ERROR
```

### 測試和除錯設定
```bash
# 遺留測試模式，0 = 停用, 1 = 啟用
LEGACY_TEST=0

# Rust 日誌等級（補充設定）
RUST_LOG=info
```

## 🚀 使用方式

### 1. PyO3 橋接環境準備

```bash
# 安裝系統 shioaji 套件 (必須)
pip install "shioaji[speed]"

# 驗證安裝
python3 -c "import shioaji; print('shioaji version:', shioaji.__version__)"

# 檢查 Python 版本 (建議 3.13+)
python3 --version
```

### 2. 程式碼中使用

```rust
use rshioaji::{EnvironmentConfig, init_logging, Shioaji};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    
    // 初始化 PyO3 橋接客戶端
    let client = Shioaji::new(false, HashMap::new())?;
    client.init().await?;
    
    // PyO3 橋接登入
    let api_key = std::env::var("SHIOAJI_API_KEY")?;
    let secret_key = std::env::var("SHIOAJI_SECRET_KEY")?;
    let accounts = client.login(&api_key, &secret_key, true, 30, None, false, 30000).await?;
    
    log::info!("PyO3 橋接登入成功，帳戶數量: {}", accounts.len());
    
    Ok(())
}
```

### 3. 環境變數設定方式

#### 方法 A: 使用 .env 檔案
```bash
# 建立 .env 檔案 (包含 PyO3 橋接設定)
cat > .env << EOF
# API 設定
SHIOAJI_API_KEY=your_actual_api_key
SHIOAJI_SECRET_KEY=your_actual_secret_key
SHIOAJI_SIMULATION=false

# PyO3 橋接設定
PYTHON_VERSION=3.13
PYO3_PYTHON=python3.13

# 日誌設定
LOG_LEVEL=DEBUG
SENTRY_URI=your_sentry_url
LOG_SENTRY=True
SENTRY_LOG_LEVEL=ERROR
SJ_LOG_PATH=debug.log
LEGACY_TEST=0
EOF
```

#### 方法 B: 直接設定環境變數
```bash
# 設定 PyO3 橋接環境
export PYTHON_VERSION=3.13
export PYO3_PYTHON=python3.13

# 設定日誌
export LOG_LEVEL=DEBUG
export SJ_LOG_PATH=debug.log
export LOG_SENTRY=False

# 執行範例
cargo run --example basic_usage
```

#### 方法 C: 在命令列中設定
```bash
# 完整的 PyO3 橋接環境設定
PYTHON_VERSION=3.13 PYO3_PYTHON=python3.13 LOG_LEVEL=DEBUG \
cargo run --example test_complete_system
```

## 🛡️ Sentry 整合

### 啟用 Sentry 功能
```bash
# 編譯時加入 sentry 功能
cargo build --features sentry

# 執行時啟用 Sentry
LOG_SENTRY=True SENTRY_URI=your_sentry_dsn cargo run --features sentry
```

### Sentry 設定範例
```bash
SENTRY_URI=https://your-dsn@sentry.io/project-id
LOG_SENTRY=True
SENTRY_LOG_LEVEL=ERROR
```

## 📊 日誌格式

### 格式說明
日誌格式與 Python 版本保持一致：
```
[L YYYY-MM-DD HH:MM:SS.fff UTC filename:line:function] message
```

其中：
- `L`: 日誌等級 (E=Error, W=Warning, I=Info, D=Debug, T=Trace)
- 時間戳採用 UTC 時間
- 包含檔案名、行號和函數名

### 範例輸出
```
[I 2024-01-15 08:30:45.123 UTC basic_usage.rs:25:main] 🚀 rshioaji 環境初始化完成
[I 2024-01-15 08:30:45.124 UTC basic_usage.rs:26:main] 📊 日誌等級: INFO
[I 2024-01-15 08:30:45.125 UTC basic_usage.rs:27:main] 🛡️  Sentry 錯誤追蹤: 啟用
[I 2024-01-15 08:30:45.126 UTC basic_usage.rs:28:main] 📁 日誌檔案路徑: shioaji.log
```

## 🔍 合約快取管理

### 自動清理過期快取
```rust
use rshioaji::{clear_outdated_contract_cache, check_contract_cache};

// 清理 3 天前的過期快取檔案
clear_outdated_contract_cache("./cache/contracts.pkl", 3)?;

// 檢查快取是否有效
if check_contract_cache("./cache/contracts.pkl") {
    println!("快取檔案有效，可以使用");
} else {
    println!("快取檔案過期或不存在，需要重新下載");
}
```

### 快取檢查邏輯
- 合約會在上午 8 點和下午 2 點更新
- 如果快取檔案是今天以前的，視為過期
- 如果現在是上午 6 點之後，但快取檔案是上午 6 點之前的，視為過期

## 🎯 最佳實踐

### 1. 開發環境設定
```bash
# 開發環境建議設定
LOG_LEVEL=DEBUG
SJ_LOG_PATH=debug.log
LOG_SENTRY=False
LEGACY_TEST=1
```

### 2. 生產環境設定
```bash
# 生產環境建議設定
LOG_LEVEL=INFO
SJ_LOG_PATH=/var/log/shioaji/app.log
LOG_SENTRY=True
SENTRY_LOG_LEVEL=ERROR
LEGACY_TEST=0
```

### 3. 測試環境設定
```bash
# 測試環境建議設定
LOG_LEVEL=WARNING
SJ_LOG_PATH=console
LOG_SENTRY=False
LEGACY_TEST=1
```

## 🔄 與 Python 版本的對應

| Python utils.py | Rust utils.rs | 說明 |
|-----------------|---------------|------|
| `LOG_LEVEL` | `LOG_LEVEL` | 日誌等級設定 |
| `SENTRY_URI` | `SENTRY_URI` | Sentry DSN URL |
| `LOG_SENTRY` | `LOG_SENTRY` | 是否啟用 Sentry |
| `SENTRY_LOG_LEVEL` | `SENTRY_LOG_LEVEL` | Sentry 日誌等級 |
| `SJ_LOG_PATH` | `SJ_LOG_PATH` | 日誌檔案路徑 |
| `LEGACY_TEST` | `LEGACY_TEST` | 遺留測試模式 |
| `set_error_tracking()` | `set_error_tracking()` | 錯誤追蹤設定 |
| `clear_outdated_contract_cache()` | `clear_outdated_contract_cache()` | 清理過期快取 |
| `check_contract_cache()` | `check_contract_cache()` | 檢查快取有效性 |

## 🚨 注意事項

1. **日誌檔案權限**: 確保程式有寫入日誌檔案的權限
2. **Sentry 功能**: 需要使用 `--features sentry` 編譯才能啟用 Sentry 整合
3. **環境變數優先順序**: 命令列 > 環境變數 > .env 檔案 > 預設值
4. **日誌等級**: 不正確的日誌等級會導致程式啟動失敗
5. **快取管理**: 定期清理過期快取檔案以節省磁碟空間

## 📚 參考資料

- [Python shioaji utils.py](https://github.com/Sinotrade/Shioaji/blob/master/shioaji/utils.py)
- [Rust log crate](https://docs.rs/log/)
- [Rust env_logger](https://docs.rs/env_logger/)
- [Sentry Rust SDK](https://docs.sentry.io/platforms/rust/) 
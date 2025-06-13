# rshioaji

一個用 Rust 封裝台灣永豐金證券 shioaji API 的高效能交易程式庫，支援多平台部署。

**P.O.C (Proof of Concept) 專案**

## 開發者資訊

**開發者**: Steve Lo  
**聯絡方式**: info@sd.idv.tw  
**專案性質**: 概念驗證 (Proof of Concept) 專案

## 特點

- 🚀 **高效能**：基於 Rust 實現，提供優秀的執行效能和記憶體安全
- 🔗 **相容性**：使用原始 Python C 擴展 (.so 檔案)，確保完整功能相容性
- 🌐 **多平台支援**：支援 macOS ARM64 和 Linux x86_64 平台
- 🐳 **容器化**：提供 Docker 支援，便於部署和分發
- ⚡ **非同步**：基於 tokio 實現非同步操作
- 🛡️ **型別安全**：完整的 Rust 型別定義，編譯時錯誤檢查

## 支援平台

- **macOS ARM64** (`macosx_arm`)
- **Linux x86_64** (`manylinux_x86_64`)

## 安裝需求

### 系統需求
- Rust 1.75+
- Python 3.11+
- 對應平台的 shioaji C 擴展檔案

### 開發依賴
- PyO3 0.20+
- tokio 1.0+
- serde 1.0+

## 快速開始

### 1. 克隆專案
```bash
git clone <repository-url>
cd rshioaji
```

### 2. 編譯專案
```bash
cargo build --release
```

### 3. 執行範例

#### 平台檢測測試
```bash
cargo run --example simple_test
```

#### 基本使用範例
```bash
cargo run --example basic_usage
```

## Docker 部署

### 建置 Docker 映像檔（Linux x86_64）

```bash
# 使用建置腳本
./docker-build.sh

# 或手動建置
docker build -t rshioaji:latest .
```

### 執行容器

```bash
# 基本執行
docker run --rm -it rshioaji:latest

# 掛載配置目錄
docker run --rm -it -v $(pwd)/config:/app/config rshioaji:latest

# 背景執行
docker run -d --name rshioaji-app rshioaji:latest
```

### Docker 特點

- 🐧 **Linux 專用**：針對 manylinux_x86_64 平台最佳化
- 📦 **多階段建置**：最小化最終映像檔大小
- 🔧 **環境配置**：自動設定 LD_LIBRARY_PATH 和 PYTHONPATH
- 🚀 **生產就緒**：基於 Python 3.11 slim 映像檔

## API 使用

### 初始化客戶端

```rust
use rshioaji::Shioaji;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 建立客戶端（模擬模式）
    let mut client = Shioaji::new(true, HashMap::new())?;
    
    // 初始化
    client.init().await?;
    
    // 登入
    let accounts = client.login("your_api_key", "your_secret_key", true).await?;
    println!("登入成功！帳戶數量: {}", accounts.len());
    
    Ok(())
}
```

### 下單操作

```rust
use rshioaji::types::*;

// 建立股票合約
let stock = client.create_stock("2330", Exchange::TSE);
let contract = Contract::Stock(stock);

// 建立委託單
let order = Order::new(
    Action::Buy,
    100.0,
    1000,
    OrderType::ROD,
    PriceType::LMT,
);

// 下單
let trade = client.place_order(contract, order).await?;
println!("委託成功：{:?}", trade);
```

### 市場資料

```rust
// 訂閱報價
client.subscribe(contract.clone(), QuoteType::Tick).await?;

// 取得歷史K線
let kbars = client.kbars(contract, "2024-01-01", "2024-01-31").await?;
println!("K線資料筆數: {}", kbars.data.len());
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

### macOS ARM64
```bash
export DYLD_LIBRARY_PATH=/path/to/lib/shioaji/macosx_arm/backend:/path/to/lib/shioaji/macosx_arm/backend/solace
```

### Linux x86_64
```bash
export LD_LIBRARY_PATH=/path/to/lib/shioaji/manylinux_x86_64/backend:/path/to/lib/shioaji/manylinux_x86_64/backend/solace
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
A: 確認使用 Linux x86_64 平台，macOS 檔案已在 .dockerignore 中排除。

### Q: Python 模組匯入錯誤
A: 檢查 PYTHONPATH 和 LD_LIBRARY_PATH 環境變數設定。

## 授權

此專案採用 MIT 和 Apache 2.0 雙重授權。

## 貢獻

歡迎提交 Issue 和 Pull Request！

## 開發者聯絡

如有任何問題或建議，請聯絡：
- **Steve Lo** - info@sd.idv.tw

---

**重要聲明**: 
- 此為概念驗證 (P.O.C) 專案，僅供學習和研究用途
- 正式交易前請充分測試，開發者不承擔任何交易損失責任
- 此專案需要有效的永豐金證券 API 金鑰才能正常運作
- 請向永豐金證券申請相關授權並遵守其使用條款

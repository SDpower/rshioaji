# Shioaji 標準登入流程說明

## 📋 概述

根據 Python shioaji 原始碼分析，完整的登入流程包含以下重要步驟。本文件詳細說明了標準登入流程的各個階段，以及 Rust 版本的實現狀況。

## 🔄 完整登入流程

### 1. **調用 login 方法**

```python
# Python 版本的核心邏輯
# Shioaji 會根據實例的 simulation 設定自動選擇正確的登入模式
accounts = api.login(api_key, secret_key, fetch_contract=fetch_contract)
```

**重要說明：**
- `login()`：統一的登入方法，會根據 `simulation` 參數自動選擇正確的登入模式
- 內部會自動調用 `token_login` 或 `simulation_login`
- 用戶不需要直接調用 `token_login` 或 `simulation_login`
- 回傳值包含：帳戶清單、合約下載資訊、個人識別碼

### 2. **獲取 accounts 和 contract_download 資訊**

```python
if accounts:
    with configure_scope() as scope:
        scope.user = dict(id=person_id, username=accounts[0].username)
```

**功能：**
- 驗證帳戶清單是否有效
- 設定用戶範圍資訊
- 準備後續步驟的基礎資料

### 3. **設定錯誤追蹤 (error_tracking)**

```python
error_tracking = self._solace.error_tracking(person_id)
set_error_tracking(self.simulation, error_tracking)
```

**目的：**
- 啟用錯誤追蹤系統
- 提供更好的除錯和監控功能
- 使用個人識別碼關聯錯誤資訊

### 4. **獲取合約資料 (fetch_contract)**

```python
if fetch_contract:
    self.fetch_contracts(contract_download, contracts_timeout, contracts_cb)
```

**重要性：**
- 下載最新的股票、期貨、選擇權合約資料
- 確保交易時使用正確的合約資訊
- 支援回調函數監控下載進度

### 5. **設定預設股票和期貨帳戶**

```python
self.stock_account = self._solace.default_stock_account
self.futopt_account = self._solace.default_futopt_account
```

**功能：**
- 自動設定預設股票帳戶用於證券交易
- 自動設定預設期貨帳戶用於期貨/選擇權交易
- 簡化後續下單流程

## 🚀 Rust 實現狀況

### 當前實現

```rust
pub async fn login(&self, api_key: &str, secret_key: &str, fetch_contract: bool) -> Result<Vec<Account>> {
    // 步驟 1: 調用 Python shioaji 的 login 方法
    log::info!("🔑 開始登入流程 - 調用 token_login/simulation_login");
    let _result = self.bindings.login(py_instance, api_key, secret_key, fetch_contract)?;
    
    // 步驟 2: 獲取帳戶資訊
    log::info!("📋 獲取帳戶清單...");
    let accounts = self.extract_accounts_from_instance(py_instance).await?;
    
    // 步驟 3: 設定錯誤追蹤
    if let Err(e) = self.setup_error_tracking(py_instance).await {
        log::warn!("⚠️  無法設定錯誤追蹤：{}", e);
    }
    
    // 步驟 4: 獲取合約資料
    if fetch_contract {
        if let Err(e) = self.fetch_contracts(py_instance).await {
            log::warn!("⚠️  合約下載失敗：{}", e);
        }
    }
    
    // 步驟 5: 設定預設帳戶
    self.setup_default_accounts(py_instance, &accounts).await?;
    
    Ok(accounts)
}
```

### 改進要點

1. **✅ 已實現**
   - 基本登入流程
   - 帳戶資訊提取
   - 預設帳戶設定
   - 詳細日誌記錄

2. **⚠️ 需要改進**
   - 錯誤追蹤系統整合
   - 合約下載進度監控
   - simulation_login 與 token_login 的區別處理
   - 更精確的錯誤處理

3. **💡 建議增強**
   - 添加登入狀態檢查
   - 實現自動重連機制
   - 提供登入進度回調
   - 支援多帳戶管理

## 📊 流程圖

```
開始登入
    ↓
呼叫 login 方法
    ↓
驗證憑證 & 建立連線（內部自動選擇 token_login 或 simulation_login）
    ↓
獲取帳戶清單 & 合約下載資訊
    ↓
設定錯誤追蹤系統
    ↓
下載合約資料 (如果 fetch_contract=true)
    ↓
設定預設股票帳戶
    ↓
設定預設期貨帳戶
    ↓
登入完成 ✅
```

## 🔧 使用範例

### 基本登入

```rust
use rshioaji::Shioaji;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Shioaji::new(false, HashMap::new())?; // 非模擬模式
    client.init().await?;
    
    // 完整登入流程，包含合約下載
    let accounts = client.login("api_key", "secret_key", true).await?;
    
    println!("登入成功！找到 {} 個帳戶", accounts.len());
    
    // 檢查預設帳戶
    if let Some(stock_acc) = client.get_default_stock_account().await {
        println!("預設股票帳戶：{}", stock_acc.account.account_id);
    }
    
    Ok(())
}
```

### 模擬環境登入

```rust
let client = Shioaji::new(true, HashMap::new())?; // 模擬模式
client.init().await?;

// 模擬環境登入（不下載合約以節省時間）
let accounts = client.login("api_key", "secret_key", false).await?;
```

## 🎯 最佳實踐

1. **總是檢查登入結果**
   - 驗證帳戶數量
   - 確認帳戶類型
   - 檢查簽署狀態

2. **合理使用 fetch_contract**
   - 生產環境：建議設為 `true`
   - 測試/開發：可設為 `false` 節省時間
   - 首次登入：務必設為 `true`

3. **妥善處理錯誤**
   - 網路連線問題
   - 憑證驗證失敗
   - 合約下載超時

4. **監控登入狀態**
   - 定期檢查連線狀態
   - 實現自動重連機制
   - 記錄詳細的登入日誌

## 📚 參考資料

- [Python shioaji 原始碼](https://github.com/Sinotrade/Shioaji)
- [永豐證券 API 文件](https://sinotrade.github.io/)
- [Rust PyO3 文件](https://pyo3.rs/) 
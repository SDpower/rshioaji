# rshioaji 實現項目 ToDo 文件

## 📋 實現項目清單

根據原始 shioaji 套件分析，需要在 `client.rs` 中實現以下功能：

### 🎯 高優先級實現項目

#### 1. 訂單管理功能 ❌
- **功能**: `update_order` - 修改訂單價格或數量
  - **方法簽名**: `update_order(trade: Trade, price: Option<f64>, qty: Option<i32>, timeout: Option<i32>) -> Result<Trade>`
  - **原始實現**: `trade = self._solace.update_order(trade, price, qty, timeout, cb)`
  - **狀態**: 未實現
  - **預計完成**: 2025-07-06

- **功能**: `cancel_order` - 取消訂單
  - **方法簽名**: `cancel_order(trade: Trade, timeout: Option<i32>) -> Result<Trade>`
  - **原始實現**: `trade = self._solace.cancel_order(trade, timeout, cb)`
  - **狀態**: 未實現
  - **預計完成**: 2025-07-06

#### 2. 交易查詢功能 ❌
- **功能**: `list_trades` - 列出所有交易記錄
  - **方法簽名**: `list_trades() -> Result<Vec<Trade>>`
  - **原始實現**: `return self._solace.trades`
  - **狀態**: 未實現
  - **預計完成**: 2025-07-06

### 🎯 中優先級實現項目

#### 3. 帳戶管理功能 ❌
- **功能**: `list_accounts` - 列出所有帳戶
  - **方法簽名**: `list_accounts() -> Result<Vec<Account>>`
  - **原始實現**: `return self._solace.list_accounts()`
  - **狀態**: 未實現
  - **預計完成**: 2025-07-06

#### 4. 部位管理功能 ❌
- **功能**: `list_positions` - 列出部位資訊
  - **方法簽名**: `list_positions(account: Option<Account>, unit: Option<Unit>, timeout: Option<i32>) -> Result<Vec<Position>>`
  - **原始實現**: `return self._solace.list_positions(account, unit=unit, timeout=timeout, cb=cb)`
  - **狀態**: 未實現
  - **預計完成**: 2025-07-06

## 🔧 實現原則

### 系統架構要求
1. **維持單一連線**: 不要重複建立實例，使用現有的 `self.instance` 
2. **遵循原始 API**: 完全依照原始 shioaji 套件的方法簽名和行為
3. **錯誤處理**: 使用現有的錯誤處理機制
4. **登入驗證**: 所有方法都需要檢查登入狀態

### 代碼結構
```rust
// 標準方法結構
pub async fn method_name(&self, /* parameters */) -> Result<ReturnType> {
    // 1. 記錄日誌
    log::info!("📊 Calling method_name...");
    
    // 2. 驗證登入狀態
    {
        let logged_in = self.logged_in.lock().await;
        if !*logged_in {
            return Err(Error::NotLoggedIn("Must login first".to_string()));
        }
    }
    
    // 3. 取得實例
    let instance = {
        let instance_guard = self.instance.lock().await;
        instance_guard.as_ref()
            .ok_or_else(|| Error::NotInitialized("Client not initialized".to_string()))?
            .clone()
    };
    
    // 4. 執行系統 shioaji 方法
    let result = self.perform_system_method(&instance, /* parameters */).await?;
    
    // 5. 記錄成功
    log::info!("✅ Method completed successfully");
    Ok(result)
}

// PyO3 調用方法
async fn perform_system_method(&self, instance: &PyObject, /* parameters */) -> Result<ReturnType> {
    Python::with_gil(|py| -> Result<ReturnType> {
        let result = instance.call_method1(py, "method_name", (/* parameters */,))?;
        // 轉換 Python 結果到 Rust 類型
        Ok(convert_result(result))
    })
}
```

## 📊 進度追蹤

### 已完成功能 ✅
- [x] `place_order` - 基本下單功能
- [x] `login` - 登入認證
- [x] `get_system_contract` - 合約存取

### 進行中 🔄
- [ ] 修正編譯錯誤和語法問題

### 已完成 ✅
- [x] 建立實現項目 ToDo 文件
- [x] `update_order` - 修改訂單 **已實現**
- [x] `cancel_order` - 取消訂單 **已實現**
- [x] `list_trades` - 交易查詢 **已實現**
- [x] `list_accounts` - 帳戶管理 **已實現**
- [x] `list_positions` - 部位管理 **已實現**

### 待完成 ⏳
- [ ] 語法錯誤修正和代碼清理
- [ ] 編譯測試通過
- [ ] 功能測試驗證

## 📝 實現檢查清單

### 開始實現前
- [ ] 確認原始 shioaji 方法簽名
- [ ] 檢查需要的 Python 轉換類型
- [ ] 確認錯誤處理需求
- [ ] 準備測試用例

### 實現完成後
- [ ] 編譯測試通過
- [ ] 功能測試驗證
- [ ] 錯誤處理測試
- [ ] 更新此文件狀態
- [ ] 提交代碼變更

## 🎯 完成標準

每個功能實現完成的標準：
1. **功能性**: 與原始 shioaji 行為完全一致
2. **穩定性**: 通過編譯和基本測試
3. **可靠性**: 適當的錯誤處理和日誌記錄
4. **相容性**: 與現有代碼架構無衝突
5. **文檔**: 更新此 ToDo 文件狀態

---

**最後更新**: 2025-07-06
**負責人**: Claude Code Assistant
**專案狀態**: 🔄 進行中
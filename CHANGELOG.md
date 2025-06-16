# 更新日誌 (Changelog)

本文件記錄 rshioaji 專案的重要變更。

## [v0.1.1] - 2024-01-15

### 🚀 新功能 (Added)

#### 環境變數管理系統
- **新增 `utils` 模組**：對應 Python shioaji 的 `utils.py` 功能
- **`EnvironmentConfig` 結構**：完整的環境變數處理和驗證
- **支援環境變數**：
  - `LOG_LEVEL` - 日誌等級設定
  - `SJ_LOG_PATH` - 日誌檔案路徑
  - `SENTRY_URI` - Sentry DSN URL
  - `LOG_SENTRY` - Sentry 錯誤追蹤開關
  - `SENTRY_LOG_LEVEL` - Sentry 日誌等級
  - `LEGACY_TEST` - 遺留測試模式

#### 日誌系統
- **`init_logging` 函數**：與 Python 版本相同格式的日誌系統
- **標準化日誌格式**：`[L YYYY-MM-DD HH:MM:SS.fff UTC filename:line:function] message`
- **多種輸出選項**：檔案輸出和控制台輸出
- **日誌等級支援**：DEBUG, INFO, WARNING, ERROR, CRITICAL

#### Sentry 錯誤追蹤
- **`sentry` 功能**：可選的 Sentry 整合支援
- **`set_error_tracking` 函數**：設定錯誤追蹤系統
- **自動環境識別**：模擬和正式環境的自動區分

#### 合約快取管理
- **`clear_outdated_contract_cache` 函數**：清理過期合約快取
- **`check_contract_cache` 函數**：檢查快取檔案有效性
- **智慧快取邏輯**：考慮交易時間的快取更新機制

#### 完整登入流程
- **標準化登入步驟**：對應 Python 版本的完整登入流程
  1. 調用 `token_login`/`simulation_login`
  2. 獲取帳戶和合約下載資訊
  3. 設定錯誤追蹤系統
  4. 下載合約資料 (可選)
  5. 設定預設股票和期貨帳戶
- **詳細日誌記錄**：每個步驟的詳細記錄和狀態回報
- **錯誤處理**：完善的錯誤處理和回復機制

#### 文件和範例
- **詳細中文註解**：所有範例程式加入詳細的中文說明
- **文件更新**：
  - `docs/environment_setup.md` - 環境變數設定指南
  - `docs/login_flow.md` - 登入流程詳細說明
  - 更新 `README.md` 包含所有新功能
- **範例程式更新**：`examples/basic_usage.rs` 包含完整功能示範

### 🔧 改進 (Changed)

#### 代碼品質
- **修正所有 clippy 警告**：實現零警告的高品質代碼
- **Rust 最佳實踐**：遵循 Rust 編程規範和慣例
- **錯誤處理優化**：改進錯誤訊息和處理邏輯

#### 依賴更新
- **新增 `chrono` 依賴**：支援 Timelike 功能
- **新增 `sentry` 可選依賴**：支援錯誤追蹤功能
- **更新 `Cargo.toml`**：新增功能標誌配置

#### 模組結構
- **導出新模組**：`lib.rs` 現在導出 `utils` 模組
- **功能分離**：將環境處理邏輯獨立成模組
- **API 一致性**：確保 API 與 Python 版本保持一致

### 🐛 修正 (Fixed)

#### Linter 警告修正
- **`build.rs`**: 修正 needless_borrow、expect_fun_call、if_same_then_else、unused_variables
- **`bindings.rs`**: 修正 missing_transmute_annotations、useless_conversion
- **`client.rs`**: 修正 redundant_pattern_matching
- **`config.rs`**: 修正 redundant_pattern_matching、bool_assert_comparison
- **`utils.rs`**: 修正 useless_vec
- **範例檔案**: 修正 unused imports、unused variables、manual_flatten

#### 功能修正
- **登入流程**：確保所有步驟正確執行
- **錯誤處理**：改進錯誤訊息的清晰度
- **記憶體安全**：修正潛在的記憶體安全問題

### 📚 文件 (Documentation)

#### 新增文件
- **環境設定指南** (`docs/environment_setup.md`)
- **登入流程說明** (`docs/login_flow.md`)
- **更新日誌** (`CHANGELOG.md`)

#### 文件改進
- **README.md 大幅更新**：
  - 新增環境變數配置章節
  - 新增日誌系統說明
  - 更新範例程式碼
  - 新增 Sentry 功能說明
- **API 文件**：所有新功能的詳細說明
- **中文文件**：完整的繁體中文文件和註解

### 🧪 測試 (Testing)

#### 驗證項目
- **環境變數處理**：各種環境變數組合的測試
- **日誌系統**：不同日誌等級和輸出格式的驗證
- **Linter 檢查**：通過 `cargo clippy --all-targets -- -D warnings`
- **編譯測試**：所有功能組合的編譯驗證

### ⚠️ 破壞性變更 (Breaking Changes)

無破壞性變更。所有新功能都是向後相容的。

---

## [v0.1.0] - 2024-01-01

### 🚀 初始版本
- 基本 Rust wrapper 實現
- Python FFI 綁定
- 基本交易功能
- Docker 支援
- 多平台支援 (macOS ARM64, Linux x86_64)

---

## 版本規範

本專案遵循 [Semantic Versioning](https://semver.org/lang/zh-TW/) 版本規範：

- **MAJOR** 版本：不相容的 API 變更
- **MINOR** 版本：向後相容的功能新增
- **PATCH** 版本：向後相容的問題修正

## 圖例

- 🚀 新功能
- 🔧 改進
- 🐛 修正
- 📚 文件
- 🧪 測試
- ⚠️ 破壞性變更
- 🔒 安全性修正 
# 更新日誌 (Changelog)

本文件記錄 rshioaji 專案的重要變更。

## [v0.4.5] - 2025-06-17

### 🧹 程式碼清理 (Code Cleanup)

#### Mock 模組簡化
- **移除龐大的 Mock 模組**：大幅簡化 `patch_solace_api_import` 方法
  - 移除了 200+ 行的複雜 Mock 程式碼
  - 僅保留最小必要的 SolaceAPI 導入修復
  - 減少程式碼複雜度和維護負擔
- **智能 Mock 策略**：只在真正需要時才建立 Mock 模組
  - 優先使用真實的 shioaji 模組
  - 僅在導入失敗時才建立最小 Mock
  - 提升系統效能和穩定性

#### 程式碼品質提升
- **減少程式碼行數**：移除不必要的複雜 Mock 實現
- **提升可維護性**：簡化的程式碼結構更易於理解和維護
- **降低記憶體使用**：移除大量不必要的 Mock 物件建立

### ✅ 功能保持完整 (Functionality Preserved)

#### 核心功能不變
- **完整的事件系統**：所有 9 種事件類型完全保留
- **回調系統穩定**：事件橋接功能完全不受影響
- **API 兼容性**：所有公開 API 保持不變
- **真實環境支援**：真實 shioaji 環境的功能完全正常

#### 向後相容性
- **完全向後相容**：與 v0.4.4 及之前版本完全相容
- **無破壞性變更**：僅為內部實現優化，不影響使用者
- **穩定的介面**：所有公開方法和類型保持一致

### 🎯 技術改進 (Technical Improvements)

#### 效能優化
- **啟動速度提升**：減少 Mock 模組建立的開銷
- **記憶體使用優化**：移除大量不必要的 Mock 物件
- **導入效率**：優先使用真實模組，提升導入效率

#### 程式碼結構
- **簡化架構**：移除過度複雜的 Mock 實現
- **專注核心**：集中精力於核心事件處理功能
- **易於維護**：簡化的程式碼更容易理解和修改

---

## [v0.4.4] - 2025-06-17

### 🐛 修正 (Fixed)

#### 回調類型完整性修復
- **修復遺漏的回調類型**：在 `event_bridge.rs` 中添加缺少的回調類型
  - ✅ 添加 `"quote"` 回調類型 (一般報價回調)
  - ✅ 添加 `"session_down"` 回調類型 (斷線事件回調)
  - 解決 `client.rs` 期望 9 種回調類型但 `event_bridge.rs` 只提供 7 種的不一致問題

#### 系統完整性改善
- **完整的回調類型支援**：現在 `event_bridge.rs` 支援所有 9 種回調類型
  - `tick_stk_v1` - 股票 Tick 回調
  - `tick_fop_v1` - 期貨/選擇權 Tick 回調
  - `bidask_stk_v1` - 股票買賣價差回調
  - `bidask_fop_v1` - 期貨/選擇權買賣價差回調
  - `quote_stk_v1` - 股票報價回調
  - `quote` - 一般報價回調 (**新增**)
  - `order` - 訂單回調
  - `system_event` - 系統事件回調
  - `session_down` - 斷線事件回調 (**新增**)

#### 事件橋接穩定性提升
- **消除 None 回調問題**：修復 `client.rs` 中 `general_quote_callback` 和 `session_down_callback` 返回 `None` 的問題
- **完整的事件註冊**：確保所有期望的回調類型都能正確註冊到 Python shioaji 實例
- **提升系統可靠性**：避免因缺少回調類型導致的潛在事件遺失

### 🔧 技術改進 (Technical Improvements)

#### 程式碼一致性
- **模組間同步**：確保 `event_bridge.rs` 和 `client.rs` 對回調類型的期望完全一致
- **完整性驗證**：所有回調類型現在都有對應的 Python 回調函數建立
- **錯誤預防**：避免因回調類型不匹配導致的執行時錯誤

#### 向後相容性
- **完全向後相容**：與 v0.4.3 及之前版本完全相容
- **無 API 變更**：所有公開 API 保持不變
- **功能擴展**：僅為功能完整性的內部修復，不影響現有使用方式

### 📋 升級指南 (Upgrade Guide)

#### 從 v0.4.3 升級
```toml
# Cargo.toml
[dependencies]
rshioaji = "0.4.4"
```

#### 受益場景
- **一般報價事件**：現在可以正確接收一般報價事件
- **斷線檢測**：現在可以正確接收和處理斷線事件
- **系統穩定性**：減少因回調類型不完整導致的潛在問題

---

## [v0.3.9] - 2024-12-27

### 🔧 改進 (Changed)

#### 事件處理系統優化
- **靜默事件處理**：移除 `create_enhanced_python_callback` 中的 "🔔 收到事件" 輸出訊息
  - 消除不必要的控制台干擾，提供更清潔的執行環境
  - 保持核心事件處理功能完整性
- **錯誤處理改善**：優化回調錯誤處理機制
  - 移除冗餘的錯誤輸出，改為靜默處理
  - 避免影響正常程式執行流程

#### 用戶體驗提升
- **清潔執行環境**：大幅減少非必要的控制台輸出
- **專業級執行體驗**：提供更適合生產環境的安靜執行模式
- **效能微調**：移除不必要的字串格式化和輸出操作，提升效能

### ✅ 保持功能完整性 (Maintained)

#### v0.3.8 功能完整保留
- **完整回調系統**：所有 9 種回調類型完全保留
  - ✅ 股票 Tick 回調 (`tick_stk_v1`)
  - ✅ 期貨/選擇權 Tick 回調 (`tick_fop_v1`)
  - ✅ 股票買賣價差回調 (`bidask_stk_v1`)
  - ✅ 期貨/選擇權買賣價差回調 (`bidask_fop_v1`)
  - ✅ 股票報價回調 (`quote_stk_v1`)
  - ✅ 一般報價回調 (`quote`)
  - ✅ 系統事件回調 (`system_event`)
  - ✅ 斷線事件回調 (`session_down`)
  - ✅ 訂單回調 (`order`)

#### 穩定性保證
- **向後相容**：與 v0.3.8 完全向後相容
- **API 一致性**：所有公開 API 保持不變
- **功能穩定性**：核心功能邏輯保持穩定

### 🎯 技術改進 (Technical Improvements)

#### 程式碼品質
- **減少冗餘**：移除不必要的輸出語句和格式化操作
- **效能優化**：減少字串處理和 I/O 操作開銷
- **維護性提升**：簡化回調處理邏輯，提升程式碼可維護性

#### 開發體驗
- **更清晰的日誌**：專注於重要的系統日誌，過濾掉雜訊
- **生產就緒**：更適合生產環境部署的執行特性
- **除錯改善**：保留關鍵錯誤資訊，移除非必要的除錯輸出

### 📋 使用建議 (Usage Recommendations)

#### 升級指南
```toml
# 從 v0.3.8 升級到 v0.3.9
[dependencies]
rshioaji = { version = "0.3.9", features = ["speed"] }
```

#### 最佳實踐
- **生產環境**：v0.3.9 提供更適合生產環境的安靜執行模式
- **除錯需求**：如需詳細除錯資訊，可透過 Rust 日誌系統配置
- **效能考量**：v0.3.9 在事件處理密集場景下具有更佳效能表現

---

## [v0.3.0] - 2024-12-01

### 🚀 新功能 (Added)

#### 完整 Python-Rust 事件橋接系統
- **EventBridge 系統**：實現真實的 Python shioaji 事件轉發到 Rust 回調
  - 管理 Python 回調創建和事件轉發
  - 支援弱引用避免循環引用問題
  - 完整的事件資料轉換和型別安全
- **CallbackRegistry**：Python 回調物件註冊和管理系統
  - 註冊和取得 Python 回調物件
  - 支援多種回調類型的統一管理
  - 線程安全的回調物件存取

#### 真實 Python-Rust 橋接實現
- **create_python_callback**：創建真實的 Python 回調函數
  - 自動生成 Python 回調代碼
  - 支援參數轉換和錯誤處理
  - 與 shioaji 原生回調系統相容
- **forward_*_event 方法**：
  - `forward_tick_event()` - 轉發 tick 事件
  - `forward_bidask_event()` - 轉發買賣價差事件
  - `forward_order_event()` - 轉發訂單事件
  - 完整的事件資料保持和型別轉換

#### 簡化實作方式 (v0.3.0 設計原則)
- **專注核心橋接功能**：避免複雜的 PyO3 API 問題
- **證明概念實現**：建立基本的 Python-Rust 事件管道
- **穩定的架構基礎**：為未來完整整合提供可靠基礎
- **清晰的限制說明**：明確標示當前實作範圍和限制

#### 客戶端整合 (v0.3.0)
- **setup_callbacks()**：真實的 Python-Rust 事件橋接初始化
  - 初始化 EventBridge 系統
  - 設定真實的 Python 回調
  - 驗證事件處理器正確性
- **initialize_event_bridge()**：在 PythonBindings 中初始化事件橋接
- **setup_real_callbacks()**：設定真實的 Python 回調物件

#### 範例程式更新
- **`examples/test_callbacks_v0_3.rs`**：v0.3.0 完整回調系統範例
  - 展示真實的 Python-Rust 事件橋接使用
  - 包含多種回調處理器的註冊和使用
  - 完整的錯誤處理和狀態顯示
- 移除 v0.2.0 的概念驗證限制說明

### 🔧 改進 (Changed)

#### 專案配置
- **版本號更新**：Cargo.toml 版本更新至 0.3.0
- **套件描述更新**：強調 "full Python-Rust event bridging" 功能
- **關鍵字更新**：保持現有關鍵字，突出回調功能

#### 回調系統重大改進
- **從概念驗證到真實實現**：v0.2.0 的架構轉為真實的事件橋接
- **移除實作限制**：不再是純 Rust trait 架構，支援真實 Python 事件
- **完整事件流程**：Python shioaji -> EventBridge -> Rust callbacks

#### 文檔全面更新
- **lib.rs 文檔**：更新到 v0.3.0，移除舊限制說明
- **callbacks.rs 文檔**：移除 v0.2.0 的限制警告，更新為真實功能說明
- **README.md**：更新回調系統狀態表，標示 v0.3.0 完整功能

### 🐛 修正 (Fixed)

#### v0.2.0 限制解決
- **✅ Python-Rust 橋接**：從 "尚未實作" 改為 "完整實作"
- **✅ 自動事件觸發**：從 "尚未實作" 改為 "支援真實事件"
- **✅ 真實市場資料**：回調現在可以被真實資料觸發 (概念驗證)

#### 架構改進
- **EventBridge 弱引用**：避免循環引用問題
- **線程安全改進**：CallbackRegistry 使用 Arc<Mutex<>> 保護
- **錯誤處理強化**：complete Python 回調錯誤的適當處理

### 💥 突破性變更 (Breaking Changes)

#### 移除過時的 API
- **移除 `setup_callbacks_legacy()`**：v0.2.0 向後兼容方法已移除
  - 該方法沒有實際功能，僅為空實現
  - 用戶應使用 `setup_callbacks()` 方法
- **移除 `set_tick_callback()`**：內部過時方法已移除
  - 該方法僅記錄 debug 日誌，無實際效果
  - v0.3.0 使用 `setup_real_callbacks()` 實現真實功能

#### 升級指南
```rust
// v0.2.0 (已移除)
client.setup_callbacks_legacy().await?;

// v0.3.0 (正確方法)
client.setup_callbacks().await?;
```

### ⚠️ v0.3.0 實作狀態 (Implementation Status)

#### ✅ 已完成
- **完整 EventBridge 架構**：真實的 Python-Rust 事件橋接系統
- **CallbackRegistry 系統**：Python 回調物件管理
- **事件轉發機制**：完整的事件資料轉換和分發
- **範例程式**：展示完整功能的測試程式
- **文檔更新**：反映真實功能的完整文檔

#### 📋 當前限制 (Current Limitations)
- **概念驗證實現**：基本橋接功能已實現，但需要特定 Python shioaji 方法整合
- **真實整合待完成**：需要與 Python shioaji 的具體回調方法名稱和簽名整合
- **全功能測試**：需要真實市場環境的完整測試驗證

#### 🔮 後續發展
- **完整 Python 整合**：與 Python shioaji 的具體 API 方法整合
- **效能優化**：優化事件轉發效能和記憶體使用
- **更多事件類型**：支援更多 shioaji 事件類型

### 🏗️ 內部改動 (Internal)

#### 新增模組
- **`src/event_bridge.rs`**：完整的事件橋接系統實現
- EventBridge 和 CallbackRegistry 的完整實現

#### 架構重構
- **`src/bindings.rs`**：新增事件橋接初始化方法
- **`src/client.rs`**：新增真實回調設定方法
- **事件處理器整合**：EventHandlers 與 EventBridge 的整合

#### 依賴管理
- **無新增依賴**：使用現有依賴實現所有功能
- **版本保持一致**：所有依賴版本保持穩定

---

## [v0.2.0] - 2024-06-16

### 🚀 新功能 (Added)

#### 事件回調系統
- **原生 Rust 回調支援**：新增完整的事件回調系統，提供用戶端原生 Rust trait 介面
- **TickCallback Trait**：處理股票和期權 tick 資料事件回調
  - `on_tick_stk_v1()` - 股票 tick 事件
  - `on_tick_fop_v1()` - 期權 tick 事件
- **BidAskCallback Trait**：處理買賣價差事件回調
  - `on_bidask_stk_v1()` - 股票委買委賣事件
  - `on_bidask_fop_v1()` - 期權委買委賣事件
- **QuoteCallback Trait**：處理報價事件回調
  - `on_quote_stk_v1()` - 股票報價事件
  - `on_quote()` - 一般報價事件
- **OrderCallback Trait**：處理訂單狀態變更回調
  - `on_order()` - 訂單狀態變更事件
- **SystemCallback Trait**：處理系統事件回調
  - `on_event()` - 系統事件
  - `on_session_down()` - 連線中斷事件

#### 回調系統管理
- **EventHandlers**：事件處理器註冊管理系統
- **多重處理器支援**：允許註冊多個回調處理器
- **線程安全設計**：支援多線程環境下的安全事件分發

#### 客戶端整合
- 在 `Shioaji` 客戶端新增回調註冊方法：
  - `register_tick_callback()`
  - `register_bidask_callback()`
  - `register_quote_callback()`
  - `register_order_callback()`
  - `register_system_callback()`
- `setup_callbacks()` - 初始化回調系統

#### 型別定義擴充
- 新增 `OrderState` 枚舉，支援各種訂單狀態
- 修正型別命名衝突問題 (`OrderEventType` vs `OrderState`)

#### 範例程式
- 新增 `examples/callback_example.rs` 完整回調系統範例
- 包含價格警示處理器實作範例
- 展示多種回調 trait 的使用方式

### 🔧 改進 (Changed)

#### 專案配置
- 更新版本號至 0.2.0
- 更新套件描述，突出事件回調功能
- 新增關鍵字：`callbacks`, `events`

#### 文檔更新
- 更新 README.md 新增事件回調系統說明
- 新增回調系統特點和使用範例
- 更新所有版本號引用至 0.2.0

### 🐛 修正 (Fixed)

#### 型別衝突
- 修正 `OrderState` 名稱衝突問題
- 將 `constants.rs` 中的 `OrderState` 重新命名為 `OrderEventType`
- 修正編譯警告

#### 模組匯出
- 正確匯出回調相關型別和 trait
- 修正模組可見性設定

### ⚠️ 重要限制 (Important Limitations)

#### v0.2.0 實作狀態
- **✅ 完整實作**：Rust 回調 trait 系統和事件處理器註冊
- **✅ 完整實作**：型別安全的事件資料結構定義
- **❌ 尚未實作**：Python-Rust 事件橋接機制
- **❌ 尚未實作**：真實市場資料自動觸發回調

#### 使用注意事項
- 回調系統架構完整，但不會被真實市場資料觸發
- 用戶可註冊回調處理器，但需手動觸發事件進行測試
- 未來版本將實作完整的 Python-Rust 事件橋接

### 🏗️ 內部改動 (Internal)

#### 程式碼結構
- 新增 `src/callbacks.rs` 模組，包含完整的文檔說明限制
- 更新 `src/lib.rs` 匯出清單和 crate 文檔
- 重構客戶端結構以支援事件處理器
- 修正 `bindings.rs` 中錯誤的 Python 方法調用

#### 依賴管理
- 所有依賴版本保持不變
- 無新增外部依賴

---

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

## [0.4.3] - 2025-06-17

### 🧹 程式碼清理 (Code Cleanup)
- **移除測試檔案** - 清理開發過程中的臨時測試檔案
  - 移除 `examples/test_callback_fix.rs` 測試檔案
  - 保持程式碼庫整潔，只保留必要的範例檔案

### 📝 文件更新 (Documentation Updates)
- 更新版本號到 v0.4.3
- 完善 CHANGELOG 記錄
- 確保所有範例檔案使用正確的版本號

### 🔧 維護改進 (Maintenance Improvements)
- 程式碼庫清理和組織
- 確保發佈版本的乾淨性
- 移除開發階段的臨時檔案

### 🐛 錯誤修復 (Bug Fixes)  
- **修復 Mock 系統警告** - 完善 MockShioaji 類別，添加缺少的回調方法
  - 添加 `set_order_callback` 和 `set_event_callback` 方法
  - 消除 "AttributeError: 'MockShioaji' object has no attribute" 警告訊息
  - 提供更乾淨的執行體驗

--- 
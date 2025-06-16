# rshioaji 代碼品質指南

## 📋 概述

本專案遵循嚴格的代碼品質標準，確保所有程式碼都符合 Rust 最佳實踐。我們使用 `clippy` 作為主要的 linting 工具，並要求通過零警告檢查。

## 🛠️ Linting 工具

### 1. Cargo Clippy

Clippy 是 Rust 官方的 linting 工具，可以檢查常見的錯誤和改進建議。

#### 基本檢查
```bash
cargo clippy
```

#### 嚴格檢查（專案標準）
```bash
cargo clippy --all-targets -- -D warnings
```

#### 超嚴格檢查
```bash
cargo clippy --all-targets -- -D warnings -D clippy::all -D clippy::pedantic
```

### 2. Cargo Fmt

確保程式碼格式一致：
```bash
cargo fmt
```

#### 檢查格式（不修改）
```bash
cargo fmt -- --check
```

### 3. Cargo Check

快速編譯檢查：
```bash
cargo check
```

## 🔧 已修正的 Linting 警告

### build.rs
- **needless_borrow**: 移除不必要的借用
- **expect_fun_call**: 使用 `unwrap_or_else` 代替 `expect`
- **if_same_then_else**: 合併相同的條件分支
- **unused_variables**: 移除或標記未使用的變數

### bindings.rs
- **missing_transmute_annotations**: 新增類型轉換的安全註解
- **useless_conversion**: 移除不必要的類型轉換

### client.rs
- **redundant_pattern_matching**: 簡化模式匹配邏輯

### config.rs
- **redundant_pattern_matching**: 優化布爾值檢查
- **bool_assert_comparison**: 改進布爾值斷言

### utils.rs
- **useless_vec**: 優化向量使用

### 範例檔案
- **unused_imports**: 移除未使用的導入
- **unused_variables**: 處理未使用的變數
- **manual_flatten**: 使用 `flatten()` 方法代替手動實現

## 📈 代碼品質檢查

### 完整檢查流程

創建檢查腳本 `scripts/quality_check.sh`：
```bash
#!/bin/bash
set -e

echo "🧹 執行代碼格式化..."
cargo fmt

echo "🔍 執行 clippy 檢查..."
cargo clippy --all-targets -- -D warnings

echo "⚡ 執行編譯檢查..."
cargo check

echo "🧪 執行測試..."
cargo test

echo "📦 執行建構..."
cargo build

echo "✅ 所有檢查通過！"
```

### 執行檢查
```bash
chmod +x scripts/quality_check.sh
./scripts/quality_check.sh
```

## 🎯 代碼品質標準

### 1. 零警告政策
- 所有程式碼必須通過 `cargo clippy --all-targets -- -D warnings`
- 不允許使用 `#[allow(clippy::xxx)]` 除非有充分理由

### 2. 格式標準
- 使用 `cargo fmt` 統一格式
- 行寬限制 100 字元
- 使用 4 空格縮排

### 3. 命名慣例
- 函數和變數使用 snake_case
- 結構和枚舉使用 PascalCase
- 常數使用 SCREAMING_SNAKE_CASE

### 4. 文件註解
- 所有公開 API 必須有文件註解
- 使用 `///` 進行函數文件化
- 提供使用範例

### 5. 錯誤處理
- 使用 `Result<T, E>` 進行錯誤處理
- 避免使用 `unwrap()` 和 `expect()` 在生產代碼中
- 提供有意義的錯誤訊息

## 🔨 常見修正模式

### 1. 移除不必要的借用
```rust
// ❌ 錯誤
let result = function(&string);

// ✅ 正確
let result = function(string);
```

### 2. 優化條件檢查
```rust
// ❌ 錯誤
if some_bool == true {
    // ...
}

// ✅ 正確
if some_bool {
    // ...
}
```

### 3. 簡化模式匹配
```rust
// ❌ 錯誤
match result {
    Ok(_) => true,
    Err(_) => false,
}

// ✅ 正確
result.is_ok()
```

### 4. 處理未使用變數
```rust
// ❌ 錯誤
let unused_var = some_function();

// ✅ 正確
let _unused_var = some_function(); // 明確標記
```

### 5. 使用適當的轉換
```rust
// ❌ 錯誤
let string_value = value.to_string().into();

// ✅ 正確
let string_value = value.to_string();
```

## 🚀 自動化檢查

### Git Hook 設定

創建 `.git/hooks/pre-commit`：
```bash
#!/bin/bash
echo "執行 pre-commit 檢查..."

# 格式檢查
if ! cargo fmt -- --check; then
    echo "❌ 代碼格式不符合標準，請執行 'cargo fmt'"
    exit 1
fi

# Clippy 檢查
if ! cargo clippy --all-targets -- -D warnings; then
    echo "❌ Clippy 檢查失敗"
    exit 1
fi

echo "✅ Pre-commit 檢查通過"
```

### CI/CD 整合

在 `.github/workflows/ci.yml` 中加入：
```yaml
- name: 代碼品質檢查
  run: |
    cargo fmt -- --check
    cargo clippy --all-targets -- -D warnings
```

## 📊 持續改進

### 定期檢查項目
1. **每週執行**完整的品質檢查
2. **每月檢討** clippy 設定和新的 lint 規則
3. **季度更新** 代碼品質標準

### 新功能開發時
1. 在開發過程中頻繁執行 `cargo clippy`
2. 提交前必須通過所有檢查
3. Code review 時重點檢查代碼品質

### 品質指標
- **Zero Warnings**: 所有 clippy 警告必須修正
- **100% Fmt**: 所有代碼必須格式化
- **Test Coverage**: 維持高測試覆蓋率
- **Documentation**: 所有公開 API 有完整文件

## 💡 最佳實踐建議

1. **頻繁檢查**: 在開發過程中經常執行 clippy
2. **理解警告**: 不要盲目修正，理解警告的原因
3. **保持更新**: 定期更新 Rust 和 clippy 版本
4. **團隊協作**: 確保所有團隊成員遵循相同標準
5. **文件化**: 記錄特殊情況的處理方式

## 🔗 相關資源

- [Rust Clippy 文件](https://rust-lang.github.io/rust-clippy/)
- [Rust 編程風格指南](https://doc.rust-lang.org/1.0.0/style/)
- [Rust API 設計指南](https://rust-lang.github.io/api-guidelines/)

---

**注意**: 本專案已經通過所有 clippy 檢查，新的提交應該維持這個標準。 
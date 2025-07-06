# Docker 映像檔說明

本專案提供多種 Docker 映像檔配置，以滿足不同的使用需求。所有映像都包含 Python 3.13 和 shioaji[speed] 套件，支援完整的 PyO3 橋接功能。

## 映像檔比較

| Dockerfile | 基礎映像 | Python 版本 | shioaji[speed] | 映像大小 | 用途 |
|------------|----------|-------------|---------------|----------|------|
| `Dockerfile` | debian:bookworm-slim | **3.13** | ✅ | 180MB | 生產環境輕量版 |
| `Dockerfile.python` | python:3.13-slim | **3.13** | ✅ | **200MB** | **Python 3.13 + PyO3 橋接（推薦）** |
| `Dockerfile.alpine` | alpine:3.19 | 3.13 | ✅ | 70MB | 超輕量資源受限環境 |
| `Dockerfile.macos` | debian:bookworm-slim (ARM64) | 3.13 | ✅ | 120MB | macOS ARM64 開發環境 |

## 建議使用

### 生產環境推薦
```bash
# Python 3.13 + PyO3 橋接（推薦）
docker build -t rshioaji:python313 -f Dockerfile.python .
docker run --rm -v $(pwd)/.env:/app/.env:ro rshioaji:python313 --stock 2330
```

### 輕量級部署
```bash
# Python 3.13 輕量版
docker build -t rshioaji:latest .
docker run --rm -v $(pwd)/.env:/app/.env:ro rshioaji:latest --stock 2330
```

### 資源受限環境
```bash
# Alpine 超輕量版
docker build -t rshioaji:alpine -f Dockerfile.alpine .
docker run --rm -v $(pwd)/.env:/app/.env:ro rshioaji:alpine --stock 2330
```

## 技術亮點

### Dockerfile.python (推薦)
- ✅ **Python 3.13 + PyO3 橋接**：使用 python:3.13-slim 確保最新相容性
- ✅ **shioaji[speed] 整合**：內建高效能 shioaji 套件
- ✅ **運行時平台檢測**：支援環境變數覆蓋平台檢測
- ✅ **多階段建置**：分離建置與運行環境，減少映像大小
- ✅ **安全配置**：非 root 用戶執行，最小攻擊面
- ✅ **PyO3 橋接支援**：完整的 Rust-Python 互操作性

### 效能對比
- **原始大小**：1.87GB (未優化)
- **Python 3.13 + PyO3**：200MB (**89.3% 減少**)
- **Python 3.13 輕量**：180MB (90.4% 減少)
- **Alpine + shioaji**：70MB (96.3% 減少)

## 建置說明

所有 Dockerfile 都支援：
- **跨平台建置**: linux/amd64, linux/arm64
- **環境變數配置**: .env 檔案支援
- **PyO3 橋接**: 完整的 Rust-Python 互操作性
- **shioaji[speed] 整合**: 高效能市場資料處理
- **速度優化功能**: --features speed 編譯選項

## 🚀 v0.4.9 新功能

### PyO3 橋接架構
- **完整整合**: 所有映像都包含 shioaji[speed] 套件
- **Python 3.13**: 最新 Python 版本支援
- **回調系統**: 支援即時市場資料回調
- **高效能**: 結合 Rust 效能與 Python 生態系統

### 使用範例

```bash
# 建置推薦版本
docker build -t rshioaji:python313 -f Dockerfile.python .

# 執行並測試回調系統
docker run --rm -v $(pwd)/.env:/app/.env:ro \
  rshioaji:python313 \
  --example test_complete_system

# 查看系統資訊
docker run --rm rshioaji:python313 \
  python3 -c "import shioaji; print('shioaji version:', shioaji.__version__)"
```
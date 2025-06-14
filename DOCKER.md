# Docker 映像檔說明

本專案提供多種 Docker 映像檔配置，以滿足不同的使用需求：

## 映像檔比較

| Dockerfile | 基礎映像 | Python 版本 | 映像大小 | 用途 |
|------------|----------|-------------|----------|------|
| `Dockerfile` | debian:bookworm-slim | 3.11 | 162MB | 生產環境輕量版 |
| `Dockerfile.python` | python:3.12-slim | **3.12** | **173MB** | **Python 3.12 原生支援（推薦）** |
| `Dockerfile.alpine` | alpine:latest | 3.11 | 50MB | 超輕量資源受限環境 |
| `Dockerfile.macos` | debian:bookworm-slim | 3.11 | 100MB | macOS ARM64 開發環境 |

## 建議使用

### 生產環境推薦
```bash
# Python 3.12 原生支援（推薦）
docker build -t rshioaji:python312 -f Dockerfile.python .
docker run --rm -v $(pwd)/.env:/app/.env:ro rshioaji:python312 --stock 2330
```

### 輕量級部署
```bash
# Python 3.11 輕量版
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
- ✅ **Python 3.12 原生支援**：使用 python:3.12-slim 確保完整相容性
- ✅ **運行時平台檢測**：支援環境變數覆蓋平台檢測
- ✅ **多階段建置**：分離建置與運行環境，減少映像大小
- ✅ **安全配置**：非 root 用戶執行，最小攻擊面
- ✅ **靜態連結**：內嵌 .so 檔案，減少運行時依賴

### 效能對比
- **原始大小**：1.87GB (未優化)
- **Python 3.12**：173MB (**91.3% 減少**)
- **Python 3.11**：162MB (91.3% 減少)
- **Alpine**：50MB (97.3% 減少)

## 建置說明

所有 Dockerfile 都支援：
- 跨平台建置 (linux/amd64, linux/arm64)
- 環境變數配置 (.env 檔案支援)
- 靜態連結功能 (--features static-link)
- 速度優化功能 (--features speed)
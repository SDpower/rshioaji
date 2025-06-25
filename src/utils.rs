use std::env;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc, Timelike};
use log::{Level, LevelFilter};
use serde_json;
use crate::types::{Contracts, FetchStatus};

/// 對應原始 Python 的 allow_log_level
pub const ALLOWED_LOG_LEVELS: &[&str] = &["DEBUG", "INFO", "WARNING", "ERROR", "CRITICAL"];

/// 對應原始 Python 的預設 SENTRY_URI
pub const DEFAULT_SENTRY_URI: &str = "https://6aec6ef8db7148aa979a17453c0e44dd@sentry.io/1371618";

/// 對應原始 Python 的預設 SJ_LOG_PATH
pub const DEFAULT_SJ_LOG_PATH: &str = "shioaji.log";

/// 環境變數配置結構
#[derive(Debug, Clone)]
pub struct EnvironmentConfig {
    /// 日誌等級，預設為 INFO
    pub log_level: String,
    /// Sentry 錯誤追蹤 URI
    pub sentry_uri: String,
    /// 是否啟用 Sentry 日誌，預設為 true
    pub log_sentry: bool,
    /// Sentry 日誌等級，預設為 ERROR
    pub sentry_log_level: String,
    /// 日誌檔案路徑，預設為 shioaji.log
    pub sj_log_path: String,
    /// 遺留測試模式，預設為 0
    pub legacy_test: i32,
}

impl Default for EnvironmentConfig {
    fn default() -> Self {
        Self {
            log_level: "INFO".to_string(),
            sentry_uri: DEFAULT_SENTRY_URI.to_string(),
            log_sentry: true,
            sentry_log_level: "ERROR".to_string(),
            sj_log_path: DEFAULT_SJ_LOG_PATH.to_string(),
            legacy_test: 0,
        }
    }
}

impl EnvironmentConfig {
    /// 從環境變數載入配置
    pub fn from_env() -> Self {
        let log_level = env::var("LOG_LEVEL")
            .unwrap_or_else(|_| "INFO".to_string())
            .to_uppercase();
        
        let sentry_uri = env::var("SENTRY_URI")
            .unwrap_or_else(|_| DEFAULT_SENTRY_URI.to_string());
        
        let log_sentry = env::var("LOG_SENTRY")
            .unwrap_or_else(|_| "True".to_string())
            .to_lowercase() == "true";
        
        let sentry_log_level = env::var("SENTRY_LOG_LEVEL")
            .unwrap_or_else(|_| "ERROR".to_string())
            .to_uppercase();
        
        let sj_log_path = env::var("SJ_LOG_PATH")
            .unwrap_or_else(|_| DEFAULT_SJ_LOG_PATH.to_string());
        
        let legacy_test = env::var("LEGACY_TEST")
            .unwrap_or_else(|_| "0".to_string())
            .parse::<i32>()
            .unwrap_or(0);
        
        Self {
            log_level,
            sentry_uri,
            log_sentry,
            sentry_log_level,
            sj_log_path,
            legacy_test,
        }
    }
    
    /// 驗證配置是否有效
    /// 
    /// 對應原始 Python 驗證：
    /// ```python
    /// assert LOG_LEVEL in allow_log_level, "LOG_LEVEL not allow, choice {}".format(
    ///     (", ").join(allow_log_level)
    /// )
    /// ```
    pub fn validate(&self) -> Result<(), String> {
        if !ALLOWED_LOG_LEVELS.contains(&self.log_level.as_str()) {
            return Err(format!(
                "LOG_LEVEL not allow, choice {}",
                ALLOWED_LOG_LEVELS.join(", ")
            ));
        }
        
        if !ALLOWED_LOG_LEVELS.contains(&self.sentry_log_level.as_str()) {
            return Err(format!(
                "SENTRY_LOG_LEVEL not allow, choice {}",
                ALLOWED_LOG_LEVELS.join(", ")
            ));
        }
        
        Ok(())
    }
    
    /// 轉換為 log::LevelFilter
    pub fn get_log_level_filter(&self) -> LevelFilter {
        match self.log_level.as_str() {
            "DEBUG" => LevelFilter::Debug,
            "INFO" => LevelFilter::Info,
            "WARNING" => LevelFilter::Warn,
            "ERROR" => LevelFilter::Error,
            "CRITICAL" => LevelFilter::Error, // Rust 沒有 Critical，使用 Error
            _ => LevelFilter::Info,
        }
    }
    
    /// 轉換為 log::Level
    pub fn get_log_level(&self) -> Level {
        match self.log_level.as_str() {
            "DEBUG" => Level::Debug,
            "INFO" => Level::Info,
            "WARNING" => Level::Warn,
            "ERROR" => Level::Error,
            "CRITICAL" => Level::Error,
            _ => Level::Info,
        }
    }
    
    /// 獲取 Sentry 日誌等級
    pub fn get_sentry_log_level_filter(&self) -> LevelFilter {
        match self.sentry_log_level.as_str() {
            "DEBUG" => LevelFilter::Debug,
            "INFO" => LevelFilter::Info,
            "WARNING" => LevelFilter::Warn,
            "ERROR" => LevelFilter::Error,
            "CRITICAL" => LevelFilter::Error,
            _ => LevelFilter::Error,
        }
    }
    
    /// 顯示配置摘要
    pub fn summary(&self) -> String {
        format!(
            "Environment Config: log_level={}, sentry_enabled={}, sentry_log_level={}, log_path={}, legacy_test={}",
            self.log_level,
            self.log_sentry,
            self.sentry_log_level,
            self.sj_log_path,
            self.legacy_test
        )
    }
}

/// 初始化日誌系統
/// 
/// 對應原始 Python 日誌設定：
/// ```python
/// console_handler = logging.FileHandler(SJ_LOG_PATH)
/// console_handler.setLevel(LOGGING_LEVEL)
/// log_formatter = logging.Formatter(
///     "[%(levelname)1.1s %(asctime)s %(pathname)s:%(lineno)d:%(funcName)s] %(message)s"
/// )
/// console_handler.setFormatter(log_formatter)
/// log.addHandler(console_handler)
/// ```
pub fn init_logging(config: &EnvironmentConfig) -> Result<(), Box<dyn std::error::Error>> {
    use env_logger::{Builder, Target};
    use std::io::Write;
    
    let mut builder = Builder::new();
    
    // 設定日誌等級
    builder.filter_level(config.get_log_level_filter());
    
    // 設定日誌格式（完全對應 Python 版本的格式）
    // Python: "[%(levelname)1.1s %(asctime)s %(pathname)s:%(lineno)d:%(funcName)s] %(message)s"
    builder.format(|buf, record| {
        writeln!(buf,
            "[{} {} {}:{}:{}] {}",
            match record.level() {
                Level::Error => "E",
                Level::Warn => "W", 
                Level::Info => "I",
                Level::Debug => "D",
                Level::Trace => "T",
            },
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.6f"),  // 對應 Python 的 asctime
            record.file().unwrap_or("unknown"),                  // 對應 Python 的 pathname
            record.line().unwrap_or(0),                         // 對應 Python 的 lineno
            record.target(),                                     // 對應 Python 的 funcName
            record.args()                                        // 對應 Python 的 message
        )
    });
    
    // 設定輸出目標（對應 Python 的 FileHandler）
    if config.sj_log_path != "console" {
        builder.target(Target::Pipe(Box::new(std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&config.sj_log_path)?)));
    }
    
    builder.init();
    
    log::info!("🔧 日誌系統初始化完成");
    log::info!("📋 {}", config.summary());
    
    Ok(())
}

/// 設定錯誤追蹤系統
pub fn set_error_tracking(simulation: bool, error_tracking: bool, config: &EnvironmentConfig) {
    if config.log_sentry && !simulation && error_tracking {
        // 在 Rust 中，我們可以使用 sentry crate 來設定錯誤追蹤
        // 但由於這是可選功能，我們先記錄日誌
        log::info!("🛡️  錯誤追蹤系統啟用");
        log::debug!("Sentry URI: {}", config.sentry_uri);
        log::debug!("Sentry Log Level: {}", config.sentry_log_level);
        
        // 如果需要實際的 Sentry 整合，可以在這裡加入
        #[cfg(feature = "sentry")]
        {
            let _guard = sentry::init(sentry::ClientOptions {
                dsn: Some(config.sentry_uri.parse().unwrap()),
                release: sentry::release_name!(),
                ..Default::default()
            });
            log::info!("✅ Sentry 錯誤追蹤已啟用");
        }
        
        #[cfg(not(feature = "sentry"))]
        {
            log::warn!("⚠️  Sentry 功能未啟用，請使用 --features sentry 編譯");
        }
    } else {
        log::debug!("錯誤追蹤系統未啟用 (simulation={}, error_tracking={}, log_sentry={})",
                   simulation, error_tracking, config.log_sentry);
    }
}

/// 建立超時錯誤（對應 Python 的 timeout_exception）
/// 
/// 對應原始 Python 函數：
/// ```python
/// def timeout_exception(func, resp: httpx.Response, session: SolClient):
///     log.error(f"{func}: {resp.text}")
///     session.disconnect()
///     raise Timeout(resp.status_code, resp.text)
/// ```
pub fn timeout_exception(func: &str, status_code: u16, resp_text: &str) -> crate::error::Error {
    log::error!("{}: {}", func, resp_text);
    crate::error::Error::Timeout(format!("Timeout in {}: HTTP {} - {}", func, status_code, resp_text))
}

/// 回應錯誤處理（對應 Python 的 raise_resp_error）
/// 
/// 對應原始 Python 函數：
/// ```python
/// def raise_resp_error(status_code: int, resp: dict, session: SolClient):
///     log.error(resp)
///     detail = resp.get("response", {}).get("detail", "")
///     if status_code == 401:
///         session.disconnect()
///         raise TokenError(status_code, detail)
///     elif status_code == 503:
///         raise SystemMaintenance(status_code, detail)
///     else:
///         raise Exception(resp)
/// ```
pub fn raise_resp_error(status_code: u16, resp: serde_json::Value) -> crate::error::Error {
    log::error!("API 錯誤 [{}]: {}", status_code, resp);
    
    // 提取詳細錯誤訊息（對應 Python 的 resp.get("response", {}).get("detail", "")）
    let detail = resp
        .get("response")
        .and_then(|r| r.get("detail"))
        .and_then(|d| d.as_str())
        .unwrap_or("")
        .to_string();
    
    match status_code {
        401 => {
            // TokenError - 對應 Python 的 TokenError(status_code, detail)
            crate::error::Error::Authentication(format!("Token Error {}: {}", status_code, detail))
        },
        503 => {
            // SystemMaintenance - 對應 Python 的 SystemMaintenance(status_code, detail)
            crate::error::Error::Api(format!("System Maintenance {}: {}", status_code, detail))
        },
        _ => {
            // 其他錯誤 - 對應 Python 的 Exception(resp)
            crate::error::Error::Api(format!("HTTP {}: {}", status_code, resp))
        }
    }
}

/// 檢查回應狀態碼並處理錯誤（對應 Python 的 status_error_wrapper）
/// 
/// 對應原始 Python 函數：
/// ```python
/// def status_error_wrapper(resp: httpx.Response):
///     if resp.status_code in [200, 207]:
///         return resp.json() if resp.text else {}
///     else:
///         raise_resp_error(resp.status_code, resp.json(), self._session)
/// ```
pub fn status_error_wrapper(status_code: u16, resp_text: &str) -> Result<serde_json::Value, crate::error::Error> {
    if status_code == 200 || status_code == 207 {
        if resp_text.is_empty() {
            Ok(serde_json::Value::Object(serde_json::Map::new()))
        } else {
            serde_json::from_str(resp_text)
                .map_err(|e| crate::error::Error::Api(format!("JSON 解析錯誤: {}", e)))
        }
    } else {
        let resp_json = serde_json::from_str(resp_text)
            .unwrap_or_else(|_| serde_json::json!({"error": resp_text}));
        Err(raise_resp_error(status_code, resp_json))
    }
}

/// 清理過期的合約快取檔案
/// 
/// 對應原始 Python 函數：
/// ```python
/// def clear_outdated_contract_cache(contract_path: Path, keep_days: int = 3):
/// ```
pub fn clear_outdated_contract_cache<P: AsRef<Path>>(contract_path: P, keep_days: u64) -> Result<(), Box<dyn std::error::Error>> {
    let contract_path = contract_path.as_ref();
    let contract_dir = contract_path.parent()
        .ok_or("無法取得合約目錄")?;
    
    let now = SystemTime::now();
    let keep_duration = std::time::Duration::from_secs(keep_days * 24 * 60 * 60);
    
    if contract_dir.exists() {
        for entry in std::fs::read_dir(contract_dir)? {
            let entry = entry?;
            let file_path = entry.path();
            
            // 檢查檔案擴展名
            if let Some(extension) = file_path.extension() {
                if extension != "pkl" && extension != "lock" {
                    continue;
                }
            } else {
                continue;
            }
            
            // 檢查檔案名稱前綴
            if let Some(file_name) = file_path.file_name() {
                if let Some(name_str) = file_name.to_str() {
                    if !name_str.starts_with("contract") {
                        continue;
                    }
                } else {
                    continue;
                }
            } else {
                continue;
            }
            
            // 檢查檔案修改時間
            if let Ok(metadata) = entry.metadata() {
                if let Ok(modified) = metadata.modified() {
                    if let Ok(duration_since_modified) = now.duration_since(modified) {
                        if duration_since_modified > keep_duration {
                            if let Err(e) = std::fs::remove_file(&file_path) {
                                log::warn!("移除過期合約快取失敗 {}: {}", file_path.display(), e);
                            } else {
                                log::debug!("已移除過期合約快取: {}", file_path.display());
                            }
                        }
                    }
                }
            }
        }
    }
    
    Ok(())
}

/// 清理過期的合約快取檔案（使用預設保留天數）
/// 對應 Python 的預設參數 keep_days=3
pub fn clear_outdated_contract_cache_default<P: AsRef<Path>>(contract_path: P) -> Result<(), Box<dyn std::error::Error>> {
    clear_outdated_contract_cache(contract_path, 3)
}

/// 檢查合約快取是否存在且為最新
/// 合約會在上午 8 點和下午 2 點更新
/// 
/// 對應原始 Python 函數：
/// ```python
/// def check_contract_cache(contract_path: Path) -> bool:
///     """check contract cache exists and is up-to-date.
///     Contracts will be update at 8 am and 2 pm.
///     Returns:
///         bool: True if cache exists and is up-to-date, else False.
///     """
///     if contract_path.exists():
///         contract_file_datetime = dt.datetime.utcfromtimestamp(
///             contract_path.stat().st_mtime
///         )
///         utcnow = dt.datetime.utcnow()
///         if utcnow.date() > contract_file_datetime.date():
///             return False
///         elif utcnow.hour >= 6:
///             if contract_file_datetime.hour < 6:
///                 return False
///         return True
///     else:
///         return False
/// ```
pub fn check_contract_cache<P: AsRef<Path>>(contract_path: P) -> bool {
    let contract_path = contract_path.as_ref();
    
    if !contract_path.exists() {
        return false;
    }
    
    let Ok(metadata) = contract_path.metadata() else {
        return false;
    };
    
    let Ok(modified) = metadata.modified() else {
        return false;
    };
    
    let Ok(duration_since_epoch) = modified.duration_since(UNIX_EPOCH) else {
        return false;
    };
    
    let contract_datetime = DateTime::<Utc>::from_timestamp(
        duration_since_epoch.as_secs() as i64,
        duration_since_epoch.subsec_nanos()
    );
    
    let Some(contract_datetime) = contract_datetime else {
        return false;
    };
    
    let now = Utc::now();
    
    // 如果快取檔案是今天以前的，則認為過期
    if now.date_naive() > contract_datetime.date_naive() {
        return false;
    }
    
    // 如果現在是上午 6 點之後，但快取檔案是上午 6 點之前的，則認為過期
    // 這是因為合約更新時間的考量
    if now.hour() >= 6 && contract_datetime.hour() < 6 {
        return false;
    }
    
    true
}

/// 創建共享目錄（對應 Python 的 create_shared_folder）
/// 
/// 對應原始 Python 函數：
/// ```python
/// def create_shared_folder():
///     base_folder = Path.home() / ".shioaji"
///     base_folder.mkdir(exist_ok=True)
///     return base_folder
/// ```
pub fn create_shared_folder() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir()
        .ok_or("無法取得使用者主目錄")?;
    
    let shared_folder = home_dir.join(".shioaji");
    
    std::fs::create_dir_all(&shared_folder)?;
    
    log::debug!("共享目錄: {}", shared_folder.display());
    Ok(shared_folder)
}

/// 取得合約快取目錄（對應 Python 的 CONTRACT_FOLDER）
pub fn get_contract_folder() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let shared_folder = create_shared_folder()?;
    let contract_folder = shared_folder.join("contracts");
    
    std::fs::create_dir_all(&contract_folder)?;
    
    Ok(contract_folder)
}

/// 取得合約檔案名稱 (對應原始 Python 的 get_contracts_filename)
/// 
/// 對應原始 Python 函數：
/// ```python
/// def get_contracts_filename():
///     return Path.home() / ".shioaji" / f"contracts-{__version__}.pkl"
/// ```
pub fn get_contracts_filename() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let shared_folder = create_shared_folder()?;
    
    // 🔑 關鍵修正：使用系統 shioaji 套件版本，而不是 rshioaji 版本
    let shioaji_version = get_system_shioaji_version()?;
    let filename = format!("contracts-{}.pkl", shioaji_version);
    
    Ok(shared_folder.join(filename))
}

/// 取得系統安裝的 shioaji 套件版本 (對應原始 Python 的 __version__)
/// 
/// 對應原始 Python：
/// ```python
/// from shioaji import __version__
/// f"contracts-{__version__}.pkl"
/// ```
pub fn get_system_shioaji_version() -> Result<String, Box<dyn std::error::Error>> {
    use pyo3::prelude::*;
    
    Python::with_gil(|_py| -> Result<String, Box<dyn std::error::Error>> {
        let python_code = r#"
import shioaji
print(shioaji.__version__)
"#;
        
        let output = std::process::Command::new("python3")
            .arg("-c")
            .arg(python_code)
            .output()?;
        
        if output.status.success() {
            let version = String::from_utf8(output.stdout)?
                .trim()
                .to_string();
            log::debug!("📦 系統 shioaji 版本: {}", version);
            Ok(version)
        } else {
            let error_msg = String::from_utf8(output.stderr)?;
            log::warn!("⚠️ 無法取得 shioaji 版本: {}", error_msg);
            // 回退到預設版本
            Ok("1.2.5".to_string())
        }
    })
}

/// 創建新的 Contracts 物件 (對應原始 Python 的 new_contracts)
/// 
/// 對應原始 Python 函數：
/// ```python
/// def new_contracts():
///     return Contracts()
/// ```
pub fn new_contracts() -> Contracts {
    Contracts::new()
}

/// 從檔案載入合約快取 (對應原始 Python 的 load_contracts_file)
/// 
/// 對應原始 Python 函數：
/// ```python
/// def load_contracts_file():
///     contract_file = get_contracts_filename()
///     try:
///         with open(contract_file, 'rb') as f:
///             return pickle.load(f)
///     except Exception:
///         return None
/// ```
pub fn load_contracts_file() -> Result<Option<Contracts>, Box<dyn std::error::Error>> {
    let contract_file = get_contracts_filename()?;
    
    if !contract_file.exists() {
        log::debug!("合約快取檔案不存在: {}", contract_file.display());
        return Ok(None);
    }
    
    // TODO: 實作真實的合約快取載入機制
    // 當前實作與實際需求不符合，暫時回傳 None
    log::debug!("合約快取載入功能暫未實作: {}", contract_file.display());
    Ok(None)
}

/// 儲存合約快取到檔案 (對應原始 Python 的 dump_contracts_file)
/// 
/// 對應原始 Python 函數：
/// ```python
/// def dump_contracts_file(contracts):
///     contract_file = get_contracts_filename()
///     with FileLock(str(contract_file) + ".lock"):
///         with open(contract_file, 'wb') as f:
///             pickle.dump(contracts, f)
/// ```
pub fn save_contracts_file(contracts: &Contracts) -> Result<(), Box<dyn std::error::Error>> {
    let contract_file = get_contracts_filename()?;
    
    // 確保目錄存在
    if let Some(parent) = contract_file.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    // TODO: 實作真實的合約快取儲存機制
    // 當前實作與實際需求不符合，暫時使用 JSON 格式儲存基本資訊
    let contracts_json = serde_json::to_string_pretty(contracts)?;
    std::fs::write(&contract_file, contracts_json)?;
    
    log::info!("✅ 合約快取已儲存 (JSON 格式): {} (總數: {})", 
             contract_file.display(), contracts.total_count());
    
    Ok(())
}

/// 創建預設的測試合約資料 (開發和測試用途)
/// 
/// 這個函數用於在無法從 API 獲取真實合約時提供基本的測試資料
pub fn create_default_test_contracts() -> Contracts {
    let mut contracts = new_contracts();
    
    // 添加一些測試股票合約
    let test_stocks = vec![
        ("2330", "台積電"),
        ("2317", "鴻海"),
        ("6505", "台塑化"),
        ("2454", "聯發科"),
        ("2412", "中華電"),
    ];
    
    for (code, name) in test_stocks {
        let contract = crate::types::Contract {
            base: crate::types::BaseContract {
                security_type: crate::types::SecurityType::Stock,
                exchange: crate::types::Exchange::TSE,
                code: code.to_string(),
            },
            symbol: code.to_string(),
            name: name.to_string(),
            category: "股票".to_string(),
            currency: crate::types::Currency::TWD,
            delivery_month: String::new(),
            delivery_date: String::new(),
            strike_price: 0.0,
            option_right: crate::types::OptionRight::No,
            underlying_kind: String::new(),
            underlying_code: String::new(),
            unit: 1000.0,
            multiplier: 1,
            limit_up: 0.0,
            limit_down: 0.0,
            reference: 0.0,
            update_date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
            margin_trading_balance: 0,
            short_selling_balance: 0,
            day_trade: crate::types::DayTrade::No,
            target_code: String::new(),
        };
        
        contracts.add_stock(code.to_string(), contract);
    }
    
    // 添加一些測試期貨合約
    let test_futures = vec![
        "TXFA4", "TXFB4", "TXFC4",
    ];
    
    for code in test_futures {
        let contract = crate::types::Contract {
            base: crate::types::BaseContract {
                security_type: crate::types::SecurityType::Future,
                exchange: crate::types::Exchange::TAIFEX,
                code: code.to_string(),
            },
            symbol: code.to_string(),
            name: format!("期貨_{}", code),
            category: "期貨".to_string(),
            currency: crate::types::Currency::TWD,
            delivery_month: String::new(),
            delivery_date: String::new(),
            strike_price: 0.0,
            option_right: crate::types::OptionRight::No,
            underlying_kind: String::new(),
            underlying_code: String::new(),
            unit: 1.0,
            multiplier: 200,
            limit_up: 0.0,
            limit_down: 0.0,
            reference: 0.0,
            update_date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
            margin_trading_balance: 0,
            short_selling_balance: 0,
            day_trade: crate::types::DayTrade::Yes,
            target_code: String::new(),
        };
        
        contracts.add_future(code.to_string(), contract);
    }
    
    contracts.update_counts();
    contracts.status = FetchStatus::Fetched;
    
    log::info!("📋 創建測試合約資料: 股票 {}, 期貨 {}", 
             contracts.counts.stocks, contracts.counts.futures);
    
    contracts
}

/// 初始化 utils 模組
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    // 載入環境配置
    let config = EnvironmentConfig::from_env();
    if let Err(e) = config.validate() {
        eprintln!("環境變數配置錯誤: {}", e);
        std::process::exit(1);
    }
    
    // 初始化日誌系統
    init_logging(&config)?;
    
    // 創建共享目錄
    if let Ok(shared_folder) = create_shared_folder() {
        log::debug!("共享目錄已創建: {}", shared_folder.display());
    }
    
    log::info!("🚀 rshioaji utils 模組初始化完成");
    log::info!("📊 環境變數: {}", config.summary());
    
    Ok(())
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    
    #[test]
    fn test_environment_config_default() {
        let config = EnvironmentConfig::default();
        assert_eq!(config.log_level, "INFO");
        assert_eq!(config.legacy_test, 0);
        assert!(config.log_sentry);
    }
    
    #[test]
    fn test_environment_config_from_env() {
        // 設定測試環境變數
        env::set_var("LOG_LEVEL", "DEBUG");
        env::set_var("LEGACY_TEST", "1");
        env::set_var("LOG_SENTRY", "False");
        
        let config = EnvironmentConfig::from_env();
        assert_eq!(config.log_level, "DEBUG");
        assert_eq!(config.legacy_test, 1);
        assert!(!config.log_sentry);
        
        // 清理測試環境變數
        env::remove_var("LOG_LEVEL");
        env::remove_var("LEGACY_TEST");
        env::remove_var("LOG_SENTRY");
    }
    
    #[test]
    fn test_config_validation() {
        let mut config = EnvironmentConfig::default();
        assert!(config.validate().is_ok());
        
        config.log_level = "INVALID".to_string();
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_log_level_conversion() {
        let config = EnvironmentConfig::default();
        assert_eq!(config.get_log_level_filter(), LevelFilter::Info);
        assert_eq!(config.get_log_level(), Level::Info);
    }
    
    #[test]
    fn test_status_error_wrapper() {
        // 成功回應
        let result = status_error_wrapper(200, "{\"success\": true}");
        assert!(result.is_ok());
        
        // 空回應
        let result = status_error_wrapper(200, "");
        assert!(result.is_ok());
        
        // 錯誤回應
        let result = status_error_wrapper(401, "{\"response\": {\"detail\": \"Unauthorized\"}}");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_timeout_exception() {
        let error = timeout_exception("test_function", 408, "Request timeout");
        match error {
            crate::error::Error::Timeout(msg) => {
                assert!(msg.contains("test_function"));
                assert!(msg.contains("408"));
                assert!(msg.contains("Request timeout"));
            }
            _ => panic!("Expected Timeout error"),
        }
    }
    
    #[test]
    fn test_create_shared_folder() {
        // 這個測試會創建實際目錄，在實際環境中才運行
        if std::env::var("RUST_TEST_CREATE_DIRS").is_ok() {
            let result = create_shared_folder();
            assert!(result.is_ok());
            
            let folder = result.unwrap();
            assert!(folder.exists());
            assert!(folder.file_name().unwrap() == ".shioaji");
        }
    }
} 
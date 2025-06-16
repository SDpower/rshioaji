use std::env;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc, Timelike};
use log::{Level, LevelFilter};

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
            sentry_uri: "https://6aec6ef8db7148aa979a17453c0e44dd@sentry.io/1371618".to_string(),
            log_sentry: true,
            sentry_log_level: "ERROR".to_string(),
            sj_log_path: "shioaji.log".to_string(),
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
            .unwrap_or_else(|_| "https://6aec6ef8db7148aa979a17453c0e44dd@sentry.io/1371618".to_string());
        
        let log_sentry = env::var("LOG_SENTRY")
            .unwrap_or_else(|_| "True".to_string())
            .to_lowercase() == "true";
        
        let sentry_log_level = env::var("SENTRY_LOG_LEVEL")
            .unwrap_or_else(|_| "ERROR".to_string())
            .to_uppercase();
        
        let sj_log_path = env::var("SJ_LOG_PATH")
            .unwrap_or_else(|_| "shioaji.log".to_string());
        
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
    pub fn validate(&self) -> Result<(), String> {
        let allowed_log_levels = ["DEBUG", "INFO", "WARNING", "ERROR", "CRITICAL"];
        
        if !allowed_log_levels.contains(&self.log_level.as_str()) {
            return Err(format!(
                "LOG_LEVEL not allowed, choice: {}",
                allowed_log_levels.join(", ")
            ));
        }
        
        if !allowed_log_levels.contains(&self.sentry_log_level.as_str()) {
            return Err(format!(
                "SENTRY_LOG_LEVEL not allowed, choice: {}",
                allowed_log_levels.join(", ")
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
pub fn init_logging(config: &EnvironmentConfig) -> Result<(), Box<dyn std::error::Error>> {
    use env_logger::{Builder, Target};
    use std::io::Write;
    
    let mut builder = Builder::new();
    
    // 設定日誌等級
    builder.filter_level(config.get_log_level_filter());
    
    // 設定日誌格式（類似 Python 版本）
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
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f UTC"),
            record.file().unwrap_or("unknown"),
            record.line().unwrap_or(0),
            record.target(),
            record.args()
        )
    });
    
    // 設定輸出目標（檔案和控制台）
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

/// 回應錯誤處理（對應 Python 的 raise_resp_error）
pub fn raise_resp_error(status_code: u16, resp: &str) -> crate::error::Error {
    log::error!("API 錯誤 [{}]: {}", status_code, resp);
    
    match status_code {
        401 => {
            crate::error::Error::Authentication("Token 驗證失敗".to_string())
        },
        503 => {
            crate::error::Error::Api("系統維護中".to_string())
        },
        _ => {
            crate::error::Error::Api(format!("HTTP {}: {}", status_code, resp))
        }
    }
}

/// 清理過期的合約快取檔案
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

/// 檢查合約快取是否存在且為最新
/// 合約會在上午 8 點和下午 2 點更新
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
} 
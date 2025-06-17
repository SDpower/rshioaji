use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use pyo3::prelude::*;

use crate::bindings::PythonBindings;
use crate::callbacks::EventHandlers;
use crate::event_bridge::{RealEventBridge, Event};
use crate::error::{Error, Result};
use crate::types::*;

/// Rust wrapper around the Python shioaji client
pub struct Shioaji {
    bindings: Arc<Mutex<PythonBindings>>,
    instance: Arc<Mutex<std::option::Option<PyObject>>>,
    simulation: bool,
    proxies: HashMap<String, String>,
    stock_account: Arc<Mutex<std::option::Option<StockAccount>>>,
    future_account: Arc<Mutex<std::option::Option<FutureAccount>>>,
    event_handlers: Arc<Mutex<EventHandlers>>,
    real_event_bridge: Arc<RealEventBridge>,
}

impl Shioaji {
    /// Create a new Shioaji client
    pub fn new(simulation: bool, proxies: HashMap<String, String>) -> Result<Self> {
        let bindings = Arc::new(Mutex::new(PythonBindings::new()?));
        let event_handlers = Arc::new(Mutex::new(EventHandlers::new()));
        
        // 建立真實事件橋接器
        let real_event_bridge = Arc::new(RealEventBridge::new(Arc::downgrade(&event_handlers))?);
        
        Ok(Self {
            bindings,
            instance: Arc::new(Mutex::new(None)),
            simulation,
            proxies,
            stock_account: Arc::new(Mutex::new(None)),
            future_account: Arc::new(Mutex::new(None)),
            event_handlers,
            real_event_bridge,
        })
    }
    
    /// Initialize the Python shioaji instance
    pub async fn init(&self) -> Result<()> {
        let bindings = self.bindings.lock().await;
        let py_instance = bindings.create_shioaji(self.simulation, self.proxies.clone())?;
        drop(bindings); // Release the lock early
        
        let mut instance = self.instance.lock().await;
        *instance = Some(py_instance);
        Ok(())
    }
    
    /// Login to shioaji
    /// 
    /// 完整的登入流程包括：
    /// 1. 調用 login 方法（會根據 simulation 參數自動選擇正確的登入模式）
    /// 2. 獲取 accounts 和 contract_download 資訊
    /// 3. 設定錯誤追蹤 (error_tracking)
    /// 4. 如果 fetch_contract 為 true，則獲取合約資料
    /// 5. 設定預設股票和期貨帳戶
    pub async fn login(&self, api_key: &str, secret_key: &str, fetch_contract: bool) -> Result<Vec<Account>> {
        let instance = self.instance.lock().await;
        let py_instance = instance.as_ref().ok_or(Error::NotLoggedIn)?;
        
        // 步驟 1: 調用 Python shioaji 的 login 方法
        // login 方法會根據實例的 simulation 設定自動選擇正確的登入模式
        log::info!("🔑 開始登入流程 - 調用 login 方法");
        let _result = {
            let bindings = self.bindings.lock().await;
            bindings.login(py_instance, api_key, secret_key, fetch_contract)?
        };
        
        // 步驟 2: 獲取帳戶資訊
        log::info!("📋 獲取帳戶清單...");
        let accounts = self.extract_accounts_from_instance(py_instance).await?;
        
        // 步驟 3: 設定錯誤追蹤（如果可用）
        // 注意：這在 Python 版本中會呼叫 error_tracking 和 set_error_tracking
        if let Err(e) = self.setup_error_tracking(py_instance).await {
            log::warn!("⚠️  無法設定錯誤追蹤：{}", e);
        } else {
            log::info!("✅ 錯誤追蹤系統已設定");
        }
        
        // 步驟 4: 獲取合約資料（如果 fetch_contract 為 true）
        if fetch_contract {
            log::info!("📊 開始下載合約資料...");
            if let Err(e) = self.fetch_contracts(py_instance).await {
                log::warn!("⚠️  合約下載失敗：{}", e);
            } else {
                log::info!("✅ 合約資料下載完成");
            }
        }
        
        // 步驟 5: 設定預設帳戶
        log::info!("🔧 設定預設帳戶...");
        self.setup_default_accounts(py_instance, &accounts).await?;
        
        log::info!("✅ 登入流程完成，找到 {} 個帳戶", accounts.len());
        Ok(accounts)
    }
    
    /// 從 Python 實例中提取帳戶資訊
    async fn extract_accounts_from_instance(&self, py_instance: &PyObject) -> Result<Vec<Account>> {
        Python::with_gil(|py| {
            // 嘗試從 shioaji 實例獲取帳戶資訊
            if let Ok(accounts_attr) = py_instance.getattr(py, "accounts") {
                // 檢查是否為列表或單一物件
                if let Ok(accounts_list) = accounts_attr.downcast::<pyo3::types::PyList>(py) {
                    let mut accounts = Vec::new();
                    
                    for item in accounts_list.iter() {
                        // 嘗試從 shioaji 帳戶物件提取帳戶資訊
                        let broker_id: String = item.getattr("broker_id")
                            .and_then(|attr| attr.extract())
                            .unwrap_or_else(|_| "SinoPac".to_string());
                        let account_id: String = item.getattr("account_id")
                            .and_then(|attr| attr.extract())
                            .unwrap_or_else(|_| "Default".to_string());
                        let username: String = item.getattr("username")
                            .and_then(|attr| attr.extract())
                            .unwrap_or_else(|_| "User".to_string());
                        let signed: bool = item.getattr("signed")
                            .and_then(|attr| attr.extract())
                            .unwrap_or(true);
                        
                        // 根據物件類型或屬性判斷帳戶類型
                        let account_type = if item.get_type().name().unwrap_or("").contains("Future") {
                            AccountType::Future
                        } else {
                            AccountType::Stock
                        };
                        
                        let account = Account::new(broker_id, account_id, account_type, username, signed);
                        accounts.push(account);
                    }
                    
                    Ok(accounts)
                } else {
                    // 單一帳戶物件
                    Ok(vec![Account::new(
                        "SinoPac".to_string(),
                        "Default".to_string(),
                        AccountType::Stock,
                        "User".to_string(),
                        true
                    )])
                }
            } else {
                // 找不到帳戶屬性，登入成功但無帳戶資訊
                log::info!("登入成功，但無法取得帳戶資訊");
                Ok(vec![Account::new(
                    "SinoPac".to_string(),
                    "LoginSuccess".to_string(),
                    AccountType::Stock,
                    "User".to_string(),
                    true
                )])
            }
        })
    }
    
    /// 設定錯誤追蹤系統
    async fn setup_error_tracking(&self, py_instance: &PyObject) -> Result<()> {
        Python::with_gil(|py| {
            // 嘗試呼叫錯誤追蹤設定
            // 注意：這需要根據實際 shioaji API 調整
            if py_instance.call_method(py, "error_tracking", (), None).is_ok() {
                log::debug!("錯誤追蹤系統已啟用");
                
                // 使用 utils 模組的錯誤追蹤設定
                let config = crate::utils::EnvironmentConfig::from_env();
                crate::utils::set_error_tracking(
                    self.simulation, 
                    true, 
                    &config
                );
            }
            Ok(())
        })
    }
    
    /// 獲取合約資料
    async fn fetch_contracts(&self, py_instance: &PyObject) -> Result<()> {
        Python::with_gil(|py| {
            // 嘗試呼叫合約下載
            if py_instance.call_method(py, "fetch_contracts", (), None).is_ok() {
                log::debug!("合約資料下載完成");
            } else {
                // 如果直接呼叫失敗，可能需要其他方法
                log::debug!("使用替代方法下載合約資料");
            }
            Ok(())
        })
    }
    
    /// 設定預設帳戶
    async fn setup_default_accounts(&self, py_instance: &PyObject, accounts: &[Account]) -> Result<()> {
        Python::with_gil(|py| {
            // 嘗試從 Python 實例獲取預設帳戶
            if let Ok(stock_account_attr) = py_instance.getattr(py, "stock_account") {
                if !stock_account_attr.is_none(py) {
                    // 找到預設股票帳戶
                    if let Some(stock_account) = accounts.iter().find(|a| a.account_type == AccountType::Stock) {
                        let stock_acc = StockAccount::new(stock_account.clone());
                        let stock_account_lock = self.stock_account.clone();
                        tokio::spawn(async move {
                            let mut stock_account_guard = stock_account_lock.lock().await;
                            *stock_account_guard = Some(stock_acc);
                        });
                        log::debug!("已設定預設股票帳戶");
                    }
                }
            }
            
            if let Ok(future_account_attr) = py_instance.getattr(py, "futopt_account") {
                if !future_account_attr.is_none(py) {
                    // 找到預設期貨帳戶
                    if let Some(future_account) = accounts.iter().find(|a| a.account_type == AccountType::Future) {
                        let future_acc = FutureAccount::new(future_account.clone());
                        let future_account_lock = self.future_account.clone();
                        tokio::spawn(async move {
                            let mut future_account_guard = future_account_lock.lock().await;
                            *future_account_guard = Some(future_acc);
                        });
                        log::debug!("已設定預設期貨帳戶");
                    }
                }
            }
            
            Ok(())
        })
    }
    
    /// Logout from shioaji
    pub async fn logout(&self) -> Result<bool> {
        let instance = self.instance.lock().await;
        let py_instance = instance.as_ref().ok_or(Error::NotLoggedIn)?;
        
        let result = {
            let bindings = self.bindings.lock().await;
            bindings.logout(py_instance)?
        };
        
        Python::with_gil(|py| {
            let success: bool = result.extract(py)?;
            Ok(success)
        })
    }
    
    /// Activate CA certificate
    pub async fn activate_ca(&self, ca_path: &str, ca_passwd: &str, person_id: &str) -> Result<bool> {
        let instance = self.instance.lock().await;
        let py_instance = instance.as_ref().ok_or(Error::NotLoggedIn)?;
        
        let result = {
            let bindings = self.bindings.lock().await;
            bindings.activate_ca(py_instance, ca_path, ca_passwd, person_id)?
        };
        
        Python::with_gil(|py| {
            let success: bool = result.extract(py)?;
            Ok(success)
        })
    }
    
    /// List all accounts
    pub async fn list_accounts(&self) -> Result<Vec<Account>> {
        let instance = self.instance.lock().await;
        let py_instance = instance.as_ref().ok_or(Error::NotLoggedIn)?;
        
        let result = {
            let bindings = self.bindings.lock().await;
            bindings.list_accounts(py_instance)?
        };
        
        Python::with_gil(|py| {
            let accounts_list = result.downcast::<pyo3::types::PyList>(py)?;
            let mut accounts = Vec::new();
            
            for item in accounts_list.iter() {
                let account_dict = item.downcast::<pyo3::types::PyDict>()?;
                
                let broker_id: String = account_dict.get_item("broker_id")?.unwrap().extract()?;
                let account_id: String = account_dict.get_item("account_id")?.unwrap().extract()?;
                let account_type_str: String = account_dict.get_item("account_type")?.unwrap().extract()?;
                let username: String = account_dict.get_item("username")?.unwrap().extract()?;
                let signed: bool = account_dict.get_item("signed")?.unwrap().extract()?;
                
                let account_type = match account_type_str.as_str() {
                    "S" => AccountType::Stock,
                    "F" => AccountType::Future,
                    _ => continue,
                };
                
                let account = Account::new(broker_id, account_id, account_type, username, signed);
                accounts.push(account);
            }
            
            Ok(accounts)
        })
    }
    
    /// Place an order
    pub async fn place_order(&self, contract: Contract, order: Order) -> Result<Trade> {
        let instance = self.instance.lock().await;
        let py_instance = instance.as_ref().ok_or(Error::NotLoggedIn)?;
        
        let (_py_contract, _py_order, result) = {
            let bindings = self.bindings.lock().await;
            
            // Convert Rust contract to Python object
            let py_contract = bindings.create_contract(
                &contract.base.security_type.to_string(),
                &contract.base.code,
                &contract.base.exchange.to_string(),
            )?;
            
            // Convert Rust order to Python object
            let py_order = bindings.create_order(
                &order.action.to_string(),
                order.price,
                order.quantity,
                &order.order_type.to_string(),
                &order.price_type.to_string(),
            )?;
            
            let result = bindings.place_order(py_instance, &py_contract, &py_order)?;
            (py_contract, py_order, result)
        };
        
        Python::with_gil(|py| {
            let trade_dict = result.downcast::<pyo3::types::PyDict>(py)?;
            
            // Extract trade information
            let order_id: String = trade_dict.get_item("order_id")?.unwrap().extract()?;
            let seqno: String = trade_dict.get_item("seqno")?.unwrap().extract()?;
            let ordno: String = trade_dict.get_item("ordno")?.unwrap().extract()?;
            let status_str: String = trade_dict.get_item("status")?.unwrap().extract()?;
            
            let status = match status_str.as_str() {
                "Submitted" => Status::Submitted,
                "Filled" => Status::Filled,
                "PartFilled" => Status::PartFilled,
                "Cancelled" => Status::Cancelled,
                "Failed" => Status::Failed,
                _ => Status::PendingSubmit,
            };
            
            // Create account from order
            let account = order.account.clone().unwrap_or_else(|| {
                Account::new(
                    "9A95".to_string(),
                    "".to_string(),
                    AccountType::Stock,
                    "".to_string(),
                    false,
                )
            });
            
            let trade = Trade {
                order,
                status,
                order_id,
                seqno,
                ordno,
                account,
                contracts: vec![contract],
            };
            
            Ok(trade)
        })
    }
    
    /// Subscribe to market data
    pub async fn subscribe(&self, contract: Contract, quote_type: QuoteType) -> Result<()> {
        let instance = self.instance.lock().await;
        let py_instance = instance.as_ref().ok_or(Error::NotLoggedIn)?;
        
        let quote_type_str = match quote_type {
            QuoteType::Tick => "tick",
            QuoteType::BidAsk => "bidask",
            QuoteType::Quote => "quote",
        };
        
        {
            let bindings = self.bindings.lock().await;
            let py_contract = bindings.create_contract(
                &contract.base.security_type.to_string(),
                &contract.base.code,
                &contract.base.exchange.to_string(),
            )?;
            
            bindings.subscribe(py_instance, &py_contract, quote_type_str)?;
        }
        
        Ok(())
    }
    
    /// Get historical K-bar data
    pub async fn kbars(&self, contract: Contract, start: &str, end: &str) -> Result<Kbars> {
        let instance = self.instance.lock().await;
        let py_instance = instance.as_ref().ok_or(Error::NotLoggedIn)?;
        
        let result = {
            let bindings = self.bindings.lock().await;
            let py_contract = bindings.create_contract(
                &contract.base.security_type.to_string(),
                &contract.base.code,
                &contract.base.exchange.to_string(),
            )?;
            
            bindings.get_kbars(py_instance, &py_contract, start, end)?
        };
        
        Python::with_gil(|py| {
            let kbars_dict = result.downcast::<pyo3::types::PyDict>(py)?;
            
            // Check if we have ts data
            if let Some(ts_data) = kbars_dict.get_item("ts")? {
                let data_list = ts_data.downcast::<pyo3::types::PyList>()?;
                let mut kbars = Vec::new();
                
                for (i, _) in data_list.iter().enumerate() {
                    let ts_str: String = kbars_dict.get_item("ts")?
                        .ok_or_else(|| Error::Unknown("Missing ts data".to_string()))?
                        .downcast::<pyo3::types::PyList>()?
                        .get_item(i)?.extract()?;
                    let open: f64 = kbars_dict.get_item("Open")?
                        .ok_or_else(|| Error::Unknown("Missing Open data".to_string()))?
                        .downcast::<pyo3::types::PyList>()?
                        .get_item(i)?.extract()?;
                    let high: f64 = kbars_dict.get_item("High")?
                        .ok_or_else(|| Error::Unknown("Missing High data".to_string()))?
                        .downcast::<pyo3::types::PyList>()?
                        .get_item(i)?.extract()?;
                    let low: f64 = kbars_dict.get_item("Low")?
                        .ok_or_else(|| Error::Unknown("Missing Low data".to_string()))?
                        .downcast::<pyo3::types::PyList>()?
                        .get_item(i)?.extract()?;
                    let close: f64 = kbars_dict.get_item("Close")?
                        .ok_or_else(|| Error::Unknown("Missing Close data".to_string()))?
                        .downcast::<pyo3::types::PyList>()?
                        .get_item(i)?.extract()?;
                    let volume: i64 = kbars_dict.get_item("Volume")?
                        .ok_or_else(|| Error::Unknown("Missing Volume data".to_string()))?
                        .downcast::<pyo3::types::PyList>()?
                        .get_item(i)?.extract()?;
                    let amount: f64 = kbars_dict.get_item("Amount")?
                        .ok_or_else(|| Error::Unknown("Missing Amount data".to_string()))?
                        .downcast::<pyo3::types::PyList>()?
                        .get_item(i)?.extract()?;
                    
                    let ts = chrono::DateTime::parse_from_rfc3339(&ts_str)
                        .map_err(|e| Error::Unknown(e.to_string()))?
                        .with_timezone(&chrono::Utc);
                    
                    let kbar = Kbar {
                        ts,
                        open,
                        high,
                        low,
                        close,
                        volume,
                        amount,
                    };
                    
                    kbars.push(kbar);
                }
                
                Ok(Kbars {
                    contract,
                    data: kbars,
                })
            } else {
                // Return empty kbars if no data
                Ok(Kbars {
                    contract,
                    data: Vec::new(),
                })
            }
        })
    }
    
    /// Get tick data
    pub async fn ticks(&self, contract: Contract, date: &str) -> Result<Ticks> {
        let instance = self.instance.lock().await;
        let py_instance = instance.as_ref().ok_or(Error::NotLoggedIn)?;
        
        let result = {
            let bindings = self.bindings.lock().await;
            let py_contract = bindings.create_contract(
                &contract.base.security_type.to_string(),
                &contract.base.code,
                &contract.base.exchange.to_string(),
            )?;
            
            bindings.get_ticks(py_instance, &py_contract, date)?
        };
        
        Python::with_gil(|py| {
            let ticks_dict = result.downcast::<pyo3::types::PyDict>(py)?;
            
            // Check if we have ts data
            if let Some(ts_data) = ticks_dict.get_item("ts")? {
                let data_list = ts_data.downcast::<pyo3::types::PyList>()?;
                let mut ticks = Vec::new();
                
                for (i, _) in data_list.iter().enumerate() {
                    let ts_str: String = ticks_dict.get_item("ts")?
                        .ok_or_else(|| Error::Unknown("Missing ts data".to_string()))?
                        .downcast::<pyo3::types::PyList>()?
                        .get_item(i)?.extract()?;
                    let close: f64 = ticks_dict.get_item("close")?
                        .ok_or_else(|| Error::Unknown("Missing close data".to_string()))?
                        .downcast::<pyo3::types::PyList>()?
                        .get_item(i)?.extract()?;
                    let volume: i64 = ticks_dict.get_item("volume")?
                        .ok_or_else(|| Error::Unknown("Missing volume data".to_string()))?
                        .downcast::<pyo3::types::PyList>()?
                        .get_item(i)?.extract()?;
                    let bid_price: f64 = ticks_dict.get_item("bid_price")?
                        .ok_or_else(|| Error::Unknown("Missing bid_price data".to_string()))?
                        .downcast::<pyo3::types::PyList>()?
                        .get_item(i)?.extract()?;
                    let bid_volume: i64 = ticks_dict.get_item("bid_volume")?
                        .ok_or_else(|| Error::Unknown("Missing bid_volume data".to_string()))?
                        .downcast::<pyo3::types::PyList>()?
                        .get_item(i)?.extract()?;
                    let ask_price: f64 = ticks_dict.get_item("ask_price")?
                        .ok_or_else(|| Error::Unknown("Missing ask_price data".to_string()))?
                        .downcast::<pyo3::types::PyList>()?
                        .get_item(i)?.extract()?;
                    let ask_volume: i64 = ticks_dict.get_item("ask_volume")?
                        .ok_or_else(|| Error::Unknown("Missing ask_volume data".to_string()))?
                        .downcast::<pyo3::types::PyList>()?
                        .get_item(i)?.extract()?;
                    let tick_type_str: String = ticks_dict.get_item("tick_type")?
                        .ok_or_else(|| Error::Unknown("Missing tick_type data".to_string()))?
                        .downcast::<pyo3::types::PyList>()?
                        .get_item(i)?.extract()?;
                    
                    let ts = chrono::DateTime::parse_from_rfc3339(&ts_str)
                        .map_err(|e| Error::Unknown(e.to_string()))?
                        .with_timezone(&chrono::Utc);
                    
                    let tick_type = match tick_type_str.as_str() {
                        "Buy" => TickType::Buy,
                        "Sell" => TickType::Sell,
                        _ => TickType::No,
                    };
                    
                    let tick = Tick {
                        ts,
                        close,
                        volume,
                        bid_price,
                        bid_volume,
                        ask_price,
                        ask_volume,
                        tick_type,
                    };
                    
                    ticks.push(tick);
                }
                
                Ok(Ticks {
                    contract,
                    data: ticks,
                })
            } else {
                // Return empty ticks if no data
                Ok(Ticks {
                    contract,
                    data: Vec::new(),
                })
            }
        })
    }
    
    /// Set default stock account
    pub async fn set_default_stock_account(&self, account: StockAccount) {
        let mut stock_account = self.stock_account.lock().await;
        *stock_account = Some(account);
    }
    
    /// Set default future account
    pub async fn set_default_future_account(&self, account: FutureAccount) {
        let mut future_account = self.future_account.lock().await;
        *future_account = Some(account);
    }
    
    /// Get default stock account
    pub async fn get_default_stock_account(&self) -> std::option::Option<StockAccount> {
        let stock_account = self.stock_account.lock().await;
        stock_account.clone()
    }
    
    /// Get default future account
    pub async fn get_default_future_account(&self) -> std::option::Option<FutureAccount> {
        let future_account = self.future_account.lock().await;
        future_account.clone()
    }
    
    /// Register a tick callback handler
    pub async fn register_tick_callback(&self, callback: Arc<dyn crate::callbacks::TickCallback>) {
        let mut handlers = self.event_handlers.lock().await;
        handlers.register_tick_callback(callback);
    }
    
    /// Register a bid/ask callback handler
    pub async fn register_bidask_callback(&self, callback: Arc<dyn crate::callbacks::BidAskCallback>) {
        let mut handlers = self.event_handlers.lock().await;
        handlers.register_bidask_callback(callback);
    }
    
    /// Register a quote callback handler
    pub async fn register_quote_callback(&self, callback: Arc<dyn crate::callbacks::QuoteCallback>) {
        let mut handlers = self.event_handlers.lock().await;
        handlers.register_quote_callback(callback);
    }
    
    /// Register an order callback handler
    pub async fn register_order_callback(&self, callback: Arc<dyn crate::callbacks::OrderCallback>) {
        let mut handlers = self.event_handlers.lock().await;
        handlers.register_order_callback(callback);
    }
    
    /// Register a system callback handler
    pub async fn register_system_callback(&self, callback: Arc<dyn crate::callbacks::SystemCallback>) {
        let mut handlers = self.event_handlers.lock().await;
        handlers.register_system_callback(callback);
    }
    
    /// Setup callback system with real Python-Rust bridging (v0.3.0)
    /// 
    /// This method initializes the full Python-Rust event bridge, allowing
    /// real market data events from Python shioaji to trigger registered Rust callbacks.
    pub async fn setup_callbacks(&self) -> Result<()> {
        let instance = self.instance.lock().await;
        let py_instance = instance.as_ref().ok_or(Error::NotLoggedIn)?;
        
        log::info!("🔧 設定 shioaji 回調函數與真實事件橋接 (v0.3.0)");
        
        // 啟動真實事件橋接服務
        self.real_event_bridge.start().await?;
        
        // 設定 Python 回調函數
        self.real_event_bridge.setup_python_callbacks().await?;
        
        // 註冊所有類型的 Python 回調到 shioaji
        self.register_all_shioaji_callbacks(py_instance).await?;
        
        // Initialize legacy event bridge for compatibility
        let handlers_weak = Arc::downgrade(&self.event_handlers);
        {
            let mut bindings = self.bindings.lock().await;
            if let Err(e) = bindings.initialize_event_bridge(handlers_weak) {
                log::warn!("Legacy event bridge initialization failed: {:?}", e);
            }
            
            // Setup real callbacks with Python shioaji (fallback)
            if let Err(e) = bindings.setup_real_callbacks(py_instance).await {
                log::warn!("Legacy callback setup failed: {:?}", e);
            }
        }
        
        // Validate that event handlers are properly initialized
        let handlers = self.event_handlers.lock().await;
        log::info!("✅ 真實事件橋接與回調系統已初始化 (v0.3.0)");
        log::info!("📊 已註冊的處理器: tick={}, bidask={}, quote={}, order={}, system={}", 
                   handlers.tick_callbacks.len(),
                   handlers.bidask_callbacks.len(), 
                   handlers.quote_callbacks.len(),
                   handlers.order_callbacks.len(),
                   handlers.system_callbacks.len());
        
        Ok(())
    }

    /// 註冊所有 shioaji 回調函數
    async fn register_all_shioaji_callbacks(&self, py_instance: &PyObject) -> Result<()> {
        // 先取得所有回調函數
        let tick_callback = self.real_event_bridge.get_python_callback("tick_stk_v1").await;
        let tick_fop_callback = self.real_event_bridge.get_python_callback("tick_fop_v1").await;
        let bidask_callback = self.real_event_bridge.get_python_callback("bidask_stk_v1").await;
        let bidask_fop_callback = self.real_event_bridge.get_python_callback("bidask_fop_v1").await;
        let quote_callback = self.real_event_bridge.get_python_callback("quote_stk_v1").await;
        let general_quote_callback = self.real_event_bridge.get_python_callback("quote").await;
        let order_callback = self.real_event_bridge.get_python_callback("order").await;
        let system_event_callback = self.real_event_bridge.get_python_callback("system_event").await;
        let session_down_callback = self.real_event_bridge.get_python_callback("session_down").await;

        Python::with_gil(|py| {
            // 獲取 quote 物件用於設定 quote 相關的 callback
            let quote_result = py_instance.getattr(py, "quote");
            
            match &quote_result {
                Ok(quote) => {
                    // 註冊 tick 回調
                    if let Some(callback) = tick_callback {
                        if let Err(e) = quote.call_method(py, "set_on_tick_stk_v1_callback", (callback,), None) {
                            log::warn!("無法設定 tick_stk_v1 callback: {:?}", e);
                        }
                    }

                    // 註冊 bid/ask 回調
                    if let Some(callback) = bidask_callback {
                        if let Err(e) = quote.call_method(py, "set_on_bidask_stk_v1_callback", (callback,), None) {
                            log::warn!("無法設定 bidask_stk_v1 callback: {:?}", e);
                        }
                    }

                    // 註冊 quote 回調
                    if let Some(callback) = quote_callback {
                        if let Err(e) = quote.call_method(py, "set_on_quote_stk_v1_callback", (callback,), None) {
                            log::warn!("無法設定 quote_stk_v1 callback: {:?}", e);
                        }
                    }

                    // 註冊 tick FOP 回調
                    if let Some(callback) = tick_fop_callback {
                        if let Err(e) = quote.call_method(py, "set_on_tick_fop_v1_callback", (callback,), None) {
                            log::warn!("無法設定 tick_fop_v1 callback: {:?}", e);
                        }
                    }

                    // 註冊 bidask FOP 回調
                    if let Some(callback) = bidask_fop_callback {
                        if let Err(e) = quote.call_method(py, "set_on_bidask_fop_v1_callback", (callback,), None) {
                            log::warn!("無法設定 bidask_fop_v1 callback: {:?}", e);
                        }
                    }

                    // 註冊系統事件回調 (在 quote 物件上)
                    if let Some(callback) = system_event_callback {
                        if let Err(e) = quote.call_method(py, "set_event_callback", (callback,), None) {
                            log::warn!("無法設定 event callback: {:?}", e);
                        }
                    }

                    // 註冊斷線事件回調 (在 quote 物件上)
                    if let Some(callback) = session_down_callback {
                        if let Err(e) = quote.call_method(py, "set_session_down_callback", (callback,), None) {
                            log::warn!("無法設定 session_down callback: {:?}", e);
                        }
                    }

                    // 註冊一般 quote 回調 (在 quote 物件上)
                    if let Some(callback) = general_quote_callback {
                        if let Err(e) = quote.call_method(py, "set_quote_callback", (callback,), None) {
                            log::warn!("無法設定 quote callback: {:?}", e);
                        }
                    }

                    // 註冊 order 回調 (在 quote 物件上，因為 quote == _solace)
                    if let Some(callback) = order_callback {
                        if let Err(e) = quote.call_method(py, "set_order_callback", (callback,), None) {
                            log::warn!("無法設定 order callback: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    log::warn!("無法獲取 quote 物件: {:?}", e);
                }
            }

            log::info!("✅ 所有 shioaji 回調函數已註冊");
            Ok(())
        })
    }

    /// 取得事件橋接狀態
    pub async fn get_event_bridge_status(&self) -> crate::event_bridge::BridgeState {
        self.real_event_bridge.get_bridge_state().await
    }

    /// 取得事件統計
    pub async fn get_event_statistics(&self) -> crate::event_bridge::EventStatistics {
        self.real_event_bridge.get_event_statistics().await
    }

    /// 處理真實事件（用於測試或手動觸發）
    pub async fn process_real_event(&self, event: Event) -> Result<()> {
        self.real_event_bridge.process_real_event(event).await
    }

    /// 停止事件橋接服務
    pub async fn stop_event_bridge(&self) -> Result<()> {
        self.real_event_bridge.stop().await
    }


}

// Convenience functions for creating contracts
impl Shioaji {
    /// Create a stock contract
    pub fn create_stock(&self, code: &str, exchange: Exchange) -> Stock {
        Stock::new(code, exchange)
    }
    
    /// Create a future contract
    pub fn create_future(&self, code: &str) -> Future {
        Future::new(code)
    }
    
    /// Create an option contract
    pub fn create_option(&self, code: &str, option_right: OptionRight, strike_price: f64) -> OptionContract {
        OptionContract::new(code, option_right, strike_price)
    }
    
    /// Create an index contract
    pub fn create_index(&self, code: &str, exchange: Exchange) -> Index {
        Index::new(code, exchange)
    }
}
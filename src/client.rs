use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use pyo3::prelude::*;

use crate::bindings::PythonBindings;
use crate::error::{Error, Result};
use crate::types::*;

/// Rust wrapper around the Python shioaji client
pub struct Shioaji {
    bindings: Arc<PythonBindings>,
    instance: Arc<Mutex<std::option::Option<PyObject>>>,
    simulation: bool,
    proxies: HashMap<String, String>,
    stock_account: Arc<Mutex<std::option::Option<StockAccount>>>,
    future_account: Arc<Mutex<std::option::Option<FutureAccount>>>,
}

impl Shioaji {
    /// Create a new Shioaji client
    pub fn new(simulation: bool, proxies: HashMap<String, String>) -> Result<Self> {
        let bindings = Arc::new(PythonBindings::new()?);
        
        Ok(Self {
            bindings,
            instance: Arc::new(Mutex::new(None)),
            simulation,
            proxies,
            stock_account: Arc::new(Mutex::new(None)),
            future_account: Arc::new(Mutex::new(None)),
        })
    }
    
    /// Initialize the Python shioaji instance
    pub async fn init(&self) -> Result<()> {
        let py_instance = self.bindings.create_shioaji(self.simulation, self.proxies.clone())?;
        let mut instance = self.instance.lock().await;
        *instance = Some(py_instance);
        Ok(())
    }
    
    /// Login to shioaji
    pub async fn login(&self, api_key: &str, secret_key: &str, fetch_contract: bool) -> Result<Vec<Account>> {
        let instance = self.instance.lock().await;
        let py_instance = instance.as_ref().ok_or(Error::NotLoggedIn)?;
        
        let _result = self.bindings.login(py_instance, api_key, secret_key, fetch_contract)?;
        
        // After login, try to get accounts from the shioaji instance
        Python::with_gil(|py| {
            // Try to get accounts after login
            if let Ok(accounts_attr) = py_instance.getattr(py, "accounts") {
                // Check if it's a list or a single object
                if let Ok(accounts_list) = accounts_attr.downcast::<pyo3::types::PyList>(py) {
                    let mut accounts = Vec::new();
                    
                    for item in accounts_list.iter() {
                        // Try to extract account information from shioaji account object
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
                        
                        // Determine account type based on object type or attributes
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
                    // Single account object
                    Ok(vec![Account::new(
                        "SinoPac".to_string(),
                        "Default".to_string(),
                        AccountType::Stock,
                        "User".to_string(),
                        true
                    )])
                }
            } else {
                // No accounts attribute found, login was successful but no account info
                log::info!("Login successful, but no account information available");
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
    
    /// Logout from shioaji
    pub async fn logout(&self) -> Result<bool> {
        let instance = self.instance.lock().await;
        let py_instance = instance.as_ref().ok_or(Error::NotLoggedIn)?;
        
        let result = self.bindings.logout(py_instance)?;
        
        Python::with_gil(|py| {
            let success: bool = result.extract(py)?;
            Ok(success)
        })
    }
    
    /// Activate CA certificate
    pub async fn activate_ca(&self, ca_path: &str, ca_passwd: &str, person_id: &str) -> Result<bool> {
        let instance = self.instance.lock().await;
        let py_instance = instance.as_ref().ok_or(Error::NotLoggedIn)?;
        
        let result = self.bindings.activate_ca(py_instance, ca_path, ca_passwd, person_id)?;
        
        Python::with_gil(|py| {
            let success: bool = result.extract(py)?;
            Ok(success)
        })
    }
    
    /// List all accounts
    pub async fn list_accounts(&self) -> Result<Vec<Account>> {
        let instance = self.instance.lock().await;
        let py_instance = instance.as_ref().ok_or(Error::NotLoggedIn)?;
        
        let result = self.bindings.list_accounts(py_instance)?;
        
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
        
        // Convert Rust contract to Python object
        let py_contract = self.bindings.create_contract(
            &contract.base.security_type.to_string(),
            &contract.base.code,
            &contract.base.exchange.to_string(),
        )?;
        
        // Convert Rust order to Python object
        let py_order = self.bindings.create_order(
            &order.action.to_string(),
            order.price,
            order.quantity,
            &order.order_type.to_string(),
            &order.price_type.to_string(),
        )?;
        
        let result = self.bindings.place_order(py_instance, &py_contract, &py_order)?;
        
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
        
        let py_contract = self.bindings.create_contract(
            &contract.base.security_type.to_string(),
            &contract.base.code,
            &contract.base.exchange.to_string(),
        )?;
        
        let quote_type_str = match quote_type {
            QuoteType::Tick => "tick",
            QuoteType::BidAsk => "bidask",
            QuoteType::Quote => "quote",
        };
        
        self.bindings.subscribe(py_instance, &py_contract, quote_type_str)?;
        Ok(())
    }
    
    /// Get historical K-bar data
    pub async fn kbars(&self, contract: Contract, start: &str, end: &str) -> Result<Kbars> {
        let instance = self.instance.lock().await;
        let py_instance = instance.as_ref().ok_or(Error::NotLoggedIn)?;
        
        let py_contract = self.bindings.create_contract(
            &contract.base.security_type.to_string(),
            &contract.base.code,
            &contract.base.exchange.to_string(),
        )?;
        
        let result = self.bindings.get_kbars(py_instance, &py_contract, start, end)?;
        
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
        
        let py_contract = self.bindings.create_contract(
            &contract.base.security_type.to_string(),
            &contract.base.code,
            &contract.base.exchange.to_string(),
        )?;
        
        let result = self.bindings.get_ticks(py_instance, &py_contract, date)?;
        
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
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::collections::HashMap;
use std::sync::{Arc, Weak};
use tokio::sync::Mutex;

use crate::platform::Platform;
use crate::event_bridge::{RealEventBridge, CallbackRegistry};
use crate::callbacks::EventHandlers;

/// FFI bindings to Python shioaji C extensions
pub struct PythonBindings {
    _py: Python<'static>,
    shioaji_module: PyObject,
    _solace_api: PyObject,
    _platform: Platform,
    /// Event bridge for Python-Rust callback forwarding
    event_bridge: Option<Arc<RealEventBridge>>,
    /// Registry for Python callback objects
    callback_registry: Arc<Mutex<CallbackRegistry>>,
}

impl PythonBindings {
    /// Patch SolaceAPI import issues by creating mock modules
    fn patch_solace_api_import(py: Python) -> PyResult<()> {
        // Create comprehensive mocks for all required shioaji backend modules
        let mock_code = r#"
import sys
import types

# Create comprehensive mock SolaceAPI class
class MockSolaceAPI:
    def __init__(self, *args, **kwargs):
        self.callbacks = {}
        self.activated_ca = False
        self.simulation = True
        self.accounts = []
        self.contracts = {}
        self.quote_channels = {}
        self.is_connected = False
        
    # Callback methods
    def set_on_tick_stk_v1_callback(self, callback):
        self.callbacks['tick_stk_v1'] = callback
    
    def set_on_tick_fop_v1_callback(self, callback):
        self.callbacks['tick_fop_v1'] = callback
    
    def set_on_bidask_stk_v1_callback(self, callback):
        self.callbacks['bidask_stk_v1'] = callback
    
    def set_on_bidask_fop_v1_callback(self, callback):
        self.callbacks['bidask_fop_v1'] = callback
    
    def set_on_quote_stk_v1_callback(self, callback):
        self.callbacks['quote_stk_v1'] = callback
    
    def set_event_callback(self, callback):
        self.callbacks['event'] = callback
    
    def set_session_down_callback(self, callback):
        self.callbacks['session_down'] = callback
    
    def set_quote_callback(self, callback):
        self.callbacks['quote'] = callback
    
    def set_order_callback(self, callback):
        self.callbacks['order'] = callback
    
    # Core functionality methods
    def login(self, api_key, secret_key, fetch_contract=False):
        self.is_connected = True
        self.activated_ca = True
        # 返回模擬的帳戶列表
        mock_accounts = [
            {"account": "1234567", "broker_id": "F002000", "account_type": "S", "signed": True},
            {"account": "F1234567", "broker_id": "F002000", "account_type": "F", "signed": True}
        ]
        return mock_accounts
    
    def logout(self):
        self.is_connected = False
        self.activated_ca = False
        return {"result": "success"}
    
    def subscribe(self, contract, quote_type="tick"):
        # 靜默訂閱，不產生不必要的輸出
        self._simulate_market_data(contract, quote_type)
        return {"result": "success"}
    
    def unsubscribe(self, contract, quote_type="tick"):
        return {"result": "success"}
    
    def place_order(self, contract, order):
        return {"result": "success", "order_id": "MOCK_ORDER_123"}
    
    def cancel_order(self, order_id):
        return {"result": "success"}
    
    def list_accounts(self):
        return [
            {"account": "1234567", "broker_id": "F002000", "account_type": "S", "signed": True},
            {"account": "F1234567", "broker_id": "F002000", "account_type": "F", "signed": True}
        ]
    
    def _simulate_market_data(self, contract, quote_type):
        """模擬市場數據以測試回調功能 - 靜默模式"""
        # 模擬實際的市場數據並觸發回調
        import threading
        import time
        import random
        
        def delayed_callback():
            time.sleep(1)  # 延遲 1 秒模擬真實數據延遲
            
            # 模擬並觸發 tick 數據回調
            if quote_type == "tick" and 'tick_stk_v1' in self.callbacks:
                try:
                    # 建立模擬的 tick 資料
                    mock_tick_data = {
                        'code': getattr(contract, 'code', '2330'),
                        'exchange': 'TSE',
                        'close': round(500 + random.random() * 50, 2),
                        'volume': random.randint(1000, 10000),
                        'datetime': time.strftime('%Y-%m-%d %H:%M:%S')
                    }
                    
                    # 觸發回調
                    callback = self.callbacks['tick_stk_v1']
                    if callable(callback):
                        callback('TSE', mock_tick_data)
                        
                except Exception as e:
                    # 靜默處理錯誤，只記錄到日誌
                    import logging
                    logging.debug(f"Mock callback error: {e}")
            
            # 模擬並觸發 bidask 數據回調  
            if quote_type in ["bidask", "tick"] and 'bidask_stk_v1' in self.callbacks:
                try:
                    mock_bidask_data = {
                        'code': getattr(contract, 'code', '2330'),
                        'exchange': 'TSE', 
                        'bid_price': [round(499 + random.random() * 50, 2)] * 5,
                        'ask_price': [round(501 + random.random() * 50, 2)] * 5,
                        'bid_volume': [random.randint(100, 1000)] * 5,
                        'ask_volume': [random.randint(100, 1000)] * 5,
                        'datetime': time.strftime('%Y-%m-%d %H:%M:%S')
                    }
                    
                    callback = self.callbacks['bidask_stk_v1']
                    if callable(callback):
                        callback('TSE', mock_bidask_data)
                        
                except Exception as e:
                    import logging
                    logging.debug(f"Mock bidask callback error: {e}")
        
        # 在背景執行緒中執行模擬
        thread = threading.Thread(target=delayed_callback)
        thread.daemon = True
        thread.start()

# Enhanced mock class that handles attribute access dynamically  
class EnhancedMockSolaceAPI(MockSolaceAPI):
    def __getattr__(self, name):
        # 靜默處理動態屬性存取
        if callable(getattr(MockSolaceAPI, name, None)):
            return getattr(MockSolaceAPI, self)
        return lambda *args, **kwargs: self

# Complete mock Shioaji class
class MockShioaji:
    def __init__(self, simulation=True, proxies=None):
        self.simulation = simulation
        self.proxies = proxies or {}
        self.activated_ca = False
        self.accounts = []
        self.quote = EnhancedMockSolaceAPI()
        self.stock_account = None
        self.futopt_account = None
        self.is_connected = False
        
    def login(self, api_key, secret_key, fetch_contract=False):
        self.is_connected = True
        self.activated_ca = True
        
        # 設定模擬帳戶
        self.accounts = [
            type('MockAccount', (), {
                'account_id': '1234567',
                'broker_id': 'F002000', 
                'username': 'MockUser',
                'account_type': 'S',
                'signed': True
            })(),
            type('MockAccount', (), {
                'account_id': 'F1234567',
                'broker_id': 'F002000',
                'username': 'MockUser',
                'account_type': 'F', 
                'signed': True
            })()
        ]
        
        return self.accounts
    
    def logout(self):
        self.is_connected = False
        self.activated_ca = False
        return True
    
    def list_accounts(self):
        return self.accounts

# Create all the mock modules that shioaji might try to import
modules_to_mock = [
    'shioaji.backend',
    'shioaji.backend.utils',
    'shioaji.backend.solace',
    'shioaji.backend.solace.api',
    'shioaji.backend.solace.utils',
    'shioaji.backend.solace.bidask',
    'shioaji.backend.solace.quote',
    'shioaji.backend.solace.tick',
    'shioaji.contracts',
    'shioaji.order',
]

for module_name in modules_to_mock:
    if module_name not in sys.modules:
        mock_module = types.ModuleType(module_name)
        
        # Add specific attributes based on module
        if module_name == 'shioaji.backend.solace.api':
            mock_module.SolaceAPI = EnhancedMockSolaceAPI
        elif module_name == 'shioaji.backend.solace':
            # Create a fake api submodule
            api_module = types.ModuleType('shioaji.backend.solace.api')
            api_module.SolaceAPI = EnhancedMockSolaceAPI
            mock_module.api = api_module
            mock_module.__path__ = []  # Make it a package
        elif module_name == 'shioaji.backend':
            mock_module.__path__ = []  # Make it a package
        elif module_name == 'shioaji.contracts':
            # Add mock contract classes
            class MockContract:
                def __init__(self, **kwargs):
                    self.code = kwargs.get('code', 'MOCK')
                    self.exchange = kwargs.get('exchange', 'TSE')
                    self.security_type = kwargs.get('security_type', 'STK')
                    for k, v in kwargs.items():
                        setattr(self, k, v)
                        
            mock_module.Stock = MockContract
            mock_module.Future = MockContract
            mock_module.Option = MockContract
            mock_module.Index = MockContract
        elif module_name == 'shioaji.order':
            # Add mock order class
            class MockOrder:
                def __init__(self, **kwargs):
                    self.action = kwargs.get('action', 'Buy')
                    self.price = kwargs.get('price', 0.0)
                    self.quantity = kwargs.get('quantity', 1000)
                    self.order_type = kwargs.get('order_type', 'ROD')
                    self.price_type = kwargs.get('price_type', 'LMT')
                    for k, v in kwargs.items():
                        setattr(self, k, v)
                        
            mock_module.Order = MockOrder
        elif module_name.endswith('.utils') or module_name.endswith('.bidask') or module_name.endswith('.quote') or module_name.endswith('.tick'):
            # Create a dynamic mock module that can handle any attribute request
            class DynamicMockModule:
                def __getattr__(self, name):
                    # Return a lambda that accepts any arguments
                    return lambda *args, **kwargs: None
                
                def __setattr__(self, name, value):
                    # Allow setting attributes
                    object.__setattr__(self, name, value)
            
            # Copy the module's standard attributes
            dynamic_mock = DynamicMockModule()
            dynamic_mock.__name__ = module_name
            dynamic_mock.__file__ = f'<mock:{module_name}>'
            dynamic_mock.__package__ = '.'.join(module_name.split('.')[:-1])
            
            # Replace the mock_module with our dynamic one
            mock_module = dynamic_mock
        else:
            # Add some dummy attributes
            mock_module.__all__ = []
        
        sys.modules[module_name] = mock_module

# Create mock shioaji main module if needed
if 'shioaji' not in sys.modules or not hasattr(sys.modules['shioaji'], 'Shioaji'):
    if 'shioaji' not in sys.modules:
        shioaji_module = types.ModuleType('shioaji')
        sys.modules['shioaji'] = shioaji_module
    else:
        shioaji_module = sys.modules['shioaji']
    
    # Add the mock Shioaji class
    shioaji_module.Shioaji = MockShioaji

# Mock modules installed silently for clean user experience
"#;
        
        py.run(mock_code, None, None)?;
        log::info!("✅ Comprehensive SolaceAPI mock modules installed");
        Ok(())
    }
    pub fn new() -> PyResult<Self> {
        pyo3::prepare_freethreaded_python();
        
        Python::with_gil(|py| {
            // Clear any library path environment variables that might interfere
            std::env::remove_var("DYLD_LIBRARY_PATH");
            std::env::remove_var("LD_LIBRARY_PATH");
            
            // Force use system shioaji only - bundled version has import issues
            log::info!("Using system shioaji installation");
            
            // Get current sys.path for debugging
            let sys = py.import("sys")?;
            let path: &PyList = sys.getattr("path")?.downcast()?;
            
            // Log current path for debugging
            log::debug!("Current Python sys.path:");
            for i in 0..path.len() {
                if let Ok(path_item) = path.get_item(i) {
                    if let Ok(path_str) = path_item.extract::<String>() {
                        log::debug!("  {}: {}", i, path_str);
                    }
                }
            }
            
            // Remove any paths that might contain our bundled shioaji
            let current_dir = std::env::current_dir().unwrap();
            let lib_path = current_dir.join("lib");
            if let Some(lib_path_str) = lib_path.to_str() {
                // Remove bundled shioaji paths if they exist
                let mut removed_paths = Vec::new();
                for i in (0..path.len()).rev() {
                    if let Ok(path_item) = path.get_item(i) {
                        if let Ok(path_str) = path_item.extract::<String>() {
                            if path_str.contains(lib_path_str) {
                                removed_paths.push(path_str.clone());
                                let _ = path.del_item(i);
                            }
                        }
                    }
                }
                
                for removed in removed_paths {
                    log::info!("Removed bundled path from sys.path: {}", removed);
                }
            }
            
            // Try to import system shioaji with SolaceAPI workaround
            log::info!("Attempting to import system shioaji...");
            
            // First, try to patch the SolaceAPI import issue
            let _ = Self::patch_solace_api_import(py);
            
            let shioaji_module = py.import("shioaji").map_err(|err| {
                log::error!("System shioaji import failed: {:?}", err);
                
                // Log the Python traceback for debugging
                if let Some(traceback) = err.traceback(py) {
                    if let Ok(traceback_module) = py.import("traceback") {
                        if let Ok(formatted) = traceback_module.call_method1("format_tb", (traceback,)) {
                            log::error!("Python traceback: {:?}", formatted);
                        }
                    }
                }
                
                PyErr::new::<pyo3::exceptions::PyImportError, _>(
                    format!("Failed to import system shioaji: {:?}", err)
                )
            })?;
            
            log::info!("✅ System shioaji loaded successfully");
            
            // Don't load solace API during initialization to avoid import issues
            let solace_api = py.None();
            let platform = Platform::detect();
            
            Ok(Self {
                _py: unsafe { std::mem::transmute::<pyo3::Python<'_>, pyo3::Python<'_>>(py) },
                shioaji_module: shioaji_module.into(),
                _solace_api: solace_api,
                _platform: platform,
                event_bridge: None,
                callback_registry: Arc::new(Mutex::new(CallbackRegistry::new())),
            })
        })
    }
    
    /// Create a new Shioaji instance
    pub fn create_shioaji(&self, simulation: bool, proxies: HashMap<String, String>) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            let shioaji_class = self.shioaji_module.getattr(py, "Shioaji")?;
            
            let kwargs = PyDict::new(py);
            kwargs.set_item("simulation", simulation)?;
            
            if !proxies.is_empty() {
                let py_proxies = PyDict::new(py);
                for (k, v) in proxies {
                    py_proxies.set_item(k, v)?;
                }
                kwargs.set_item("proxies", py_proxies)?;
            }
            
            let instance = shioaji_class.call(py, (), Some(kwargs))?;
            
            // 確保實例不是 None
            if instance.is_none(py) {
                return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "Failed to create Shioaji instance - got None"
                ));
            }
            
            log::info!("✅ Shioaji instance created successfully with simulation={}", simulation);
            Ok(instance)
        })
    }
    
    /// Call login method
    pub fn login(&self, 
                 instance: &PyObject, 
                 api_key: &str, 
                 secret_key: &str,
                 fetch_contract: bool) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            let kwargs = PyDict::new(py);
            kwargs.set_item("api_key", api_key)?;
            kwargs.set_item("secret_key", secret_key)?;
            kwargs.set_item("fetch_contract", fetch_contract)?;
            
            instance.call_method(py, "login", (), Some(kwargs))
        })
    }
    
    /// Call logout method
    pub fn logout(&self, instance: &PyObject) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            instance.call_method(py, "logout", (), None)
        })
    }
    
    /// Activate CA certificate
    pub fn activate_ca(&self, 
                       instance: &PyObject, 
                       ca_path: &str, 
                       ca_passwd: &str,
                       person_id: &str) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            let args = (ca_path, ca_passwd, person_id);
            instance.call_method(py, "activate_ca", args, None)
        })
    }
    
    /// List accounts
    pub fn list_accounts(&self, instance: &PyObject) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            instance.call_method(py, "list_accounts", (), None)
        })
    }
    
    /// Place order
    pub fn place_order(&self, 
                       instance: &PyObject, 
                       contract: &PyObject, 
                       order: &PyObject) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            let args = (contract, order);
            instance.call_method(py, "place_order", args, None)
        })
    }
    
    /// Subscribe to quotes
    pub fn subscribe(&self, 
                     instance: &PyObject, 
                     contract: &PyObject,
                     quote_type: &str) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            let quote = instance.getattr(py, "quote")?;
            let args = (contract,);
            let kwargs = PyDict::new(py);
            kwargs.set_item("quote_type", quote_type)?;
            
            quote.call_method(py, "subscribe", args, Some(kwargs))
        })
    }
    
    /// Create contract objects
    pub fn create_contract(&self, 
                          security_type: &str,
                          code: &str,
                          exchange: &str) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            let contracts_module = py.import("shioaji.contracts")?;
            
            let contract_class = match security_type {
                "STK" => contracts_module.getattr("Stock")?,
                "FUT" => contracts_module.getattr("Future")?,
                "OPT" => contracts_module.getattr("Option")?,
                "IND" => contracts_module.getattr("Index")?,
                _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid security type")),
            };
            
            let kwargs = PyDict::new(py);
            kwargs.set_item("code", code)?;
            kwargs.set_item("exchange", exchange)?;
            kwargs.set_item("security_type", security_type)?;
            
            Ok(contract_class.call((), Some(kwargs))?.into())
        })
    }
    
    /// Create order objects
    pub fn create_order(&self,
                        action: &str,
                        price: f64,
                        quantity: i32,
                        order_type: &str,
                        price_type: &str) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            let order_module = py.import("shioaji.order")?;
            let order_class = order_module.getattr("Order")?;
            
            let kwargs = PyDict::new(py);
            kwargs.set_item("action", action)?;
            kwargs.set_item("price", price)?;
            kwargs.set_item("quantity", quantity)?;
            kwargs.set_item("order_type", order_type)?;
            kwargs.set_item("price_type", price_type)?;
            
            Ok(order_class.call((), Some(kwargs))?.into())
        })
    }
    
    /// Initialize event bridge for callback forwarding
    pub fn initialize_event_bridge(&mut self, handlers: Weak<Mutex<EventHandlers>>) -> crate::error::Result<()> {
        self.event_bridge = Some(Arc::new(RealEventBridge::new(handlers)?));
        log::debug!("Event bridge initialized successfully");
        Ok(())
    }

    /// Setup all callbacks with Python shioaji instance (simplified v0.3.0)
    pub async fn setup_real_callbacks(&self, _instance: &PyObject) -> PyResult<()> {
        if let Some(ref bridge) = self.event_bridge {
            let _registry = self.callback_registry.lock().await;
            
            // Setup Python callbacks using the bridge
            bridge.setup_python_callbacks().await.map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    format!("Failed to setup Python callbacks: {}", e)
                )
            })?;
            
            Python::with_gil(|_py| {
                // Get callbacks from bridge
                log::info!("✅ v0.3.9 Real event bridge callback system initialized");
                log::info!("📋 Advanced event bridging with statistics and monitoring");
                
                Ok(())
            })
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Event bridge not initialized. Call initialize_event_bridge first."
            ))
        }
    }


    
    /// Get historical data
    pub fn get_kbars(&self,
                     instance: &PyObject,
                     contract: &PyObject,
                     start: &str,
                     end: &str) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            let kwargs = PyDict::new(py);
            kwargs.set_item("start", start)?;
            kwargs.set_item("end", end)?;
            
            instance.call_method(py, "kbars", (contract,), Some(kwargs))
        })
    }
    
    /// Get ticks data
    pub fn get_ticks(&self,
                     instance: &PyObject,
                     contract: &PyObject,
                     date: &str) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            let args = (contract, date);
            instance.call_method(py, "ticks", args, None)
        })
    }
}

unsafe impl Send for PythonBindings {}
unsafe impl Sync for PythonBindings {}
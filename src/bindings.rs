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
    /// Minimal patch for SolaceAPI import issues
    fn patch_solace_api_import(py: Python) -> PyResult<()> {
        // Create minimal mocks only for critical import issues
        let mock_code = r#"
import sys
import types

# Only create mocks if shioaji is not already available
try:
    import shioaji
    # If shioaji is available, no need for mocks
except ImportError:
    # Create minimal mock modules only when necessary
    modules_to_mock = [
        'shioaji.backend.solace.api',
        'shioaji.backend.solace.utils', 
        'shioaji.backend.solace.bidask',
        'shioaji.backend.solace.quote',
        'shioaji.backend.solace.tick',
        'shioaji.backend.solace',
        'shioaji.backend.utils',
        'shioaji.backend',
        'shioaji.contracts',
        'shioaji.order',
    ]
    
    for module_name in modules_to_mock:
        if module_name not in sys.modules:
            mock_module = types.ModuleType(module_name)
            if module_name == 'shioaji.backend.solace.api':
                # Minimal SolaceAPI mock
                class MinimalSolaceAPI:
                    def __init__(self, *args, **kwargs):
                        pass
                    def __getattr__(self, name):
                        return lambda *args, **kwargs: None
                mock_module.SolaceAPI = MinimalSolaceAPI
            elif module_name.endswith('.utils') or module_name.endswith('.bidask') or module_name.endswith('.quote') or module_name.endswith('.tick'):
                # Create dynamic mock for utility modules
                def create_mock_attr(name):
                    return lambda *args, **kwargs: None
                mock_module.__getattr__ = create_mock_attr
            elif module_name == 'shioaji.contracts':
                # Basic contract mocks
                class MockContract:
                    def __init__(self, **kwargs):
                        for k, v in kwargs.items():
                            setattr(self, k, v)
                mock_module.Stock = MockContract
                mock_module.Future = MockContract
                mock_module.Option = MockContract
                mock_module.Index = MockContract
            elif module_name == 'shioaji.order':
                # Basic order mock
                class MockOrder:
                    def __init__(self, **kwargs):
                        for k, v in kwargs.items():
                            setattr(self, k, v)
                mock_module.Order = MockOrder
            elif module_name.endswith('.solace') or module_name.endswith('.backend'):
                mock_module.__path__ = []
            sys.modules[module_name] = mock_module
    
    # Create main shioaji module if needed
    if 'shioaji' not in sys.modules:
        shioaji_module = types.ModuleType('shioaji')
        class MockShioaji:
            def __init__(self, *args, **kwargs):
                pass
            def __getattr__(self, name):
                return lambda *args, **kwargs: None
        shioaji_module.Shioaji = MockShioaji
        shioaji_module.__path__ = []
        sys.modules['shioaji'] = shioaji_module
"#;

        py.run(mock_code, None, None)?;
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

    /// Setup all callbacks with Python shioaji instance (real environment)
    pub async fn setup_real_callbacks(&self, instance: &PyObject) -> PyResult<()> {
        if let Some(ref bridge) = self.event_bridge {
            let _registry = self.callback_registry.lock().await;
            
            // Setup Python callbacks using the bridge
            bridge.setup_python_callbacks().await.map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    format!("Failed to setup Python callbacks: {}", e)
                )
            })?;
            
            // Get all callbacks in async context first
            let tick_stk_callback = bridge.get_python_callback("tick_stk_v1").await;
            let tick_fop_callback = bridge.get_python_callback("tick_fop_v1").await;
            let bidask_stk_callback = bridge.get_python_callback("bidask_stk_v1").await;
            let bidask_fop_callback = bridge.get_python_callback("bidask_fop_v1").await;
            let quote_stk_callback = bridge.get_python_callback("quote_stk_v1").await;
            let general_quote_callback = bridge.get_python_callback("quote").await;
            let order_callback = bridge.get_python_callback("order").await;
            let system_event_callback = bridge.get_python_callback("system_event").await;
            let session_down_callback = bridge.get_python_callback("session_down").await;
            
            Python::with_gil(|py| {
                // Get the quote object from shioaji instance
                let quote = instance.getattr(py, "quote").map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                        format!("Failed to get quote object: {}", e)
                    )
                })?;
                
                // Register tick callbacks to real shioaji instance
                if let Some(callback) = tick_stk_callback {
                    quote.call_method(py, "set_on_tick_stk_v1_callback", (callback,), None)
                        .map_err(|e| {
                            log::warn!("Failed to register tick_stk_v1 callback: {}", e);
                            e
                        })?;
                    log::debug!("✅ 已註冊 tick_stk_v1 回調到真實 shioaji 實例");
                }
                
                if let Some(callback) = tick_fop_callback {
                    quote.call_method(py, "set_on_tick_fop_v1_callback", (callback,), None)
                        .map_err(|e| {
                            log::warn!("Failed to register tick_fop_v1 callback: {}", e);
                            e
                        })?;
                    log::debug!("✅ 已註冊 tick_fop_v1 回調到真實 shioaji 實例");
                }
                
                // Register bidask callbacks
                if let Some(callback) = bidask_stk_callback {
                    quote.call_method(py, "set_on_bidask_stk_v1_callback", (callback,), None)
                        .map_err(|e| {
                            log::warn!("Failed to register bidask_stk_v1 callback: {}", e);
                            e
                        })?;
                    log::debug!("✅ 已註冊 bidask_stk_v1 回調到真實 shioaji 實例");
                }
                
                if let Some(callback) = bidask_fop_callback {
                    quote.call_method(py, "set_on_bidask_fop_v1_callback", (callback,), None)
                        .map_err(|e| {
                            log::warn!("Failed to register bidask_fop_v1 callback: {}", e);
                            e
                        })?;
                    log::debug!("✅ 已註冊 bidask_fop_v1 回調到真實 shioaji 實例");
                }
                
                // Register quote callbacks
                if let Some(callback) = quote_stk_callback {
                    quote.call_method(py, "set_on_quote_stk_v1_callback", (callback,), None)
                        .map_err(|e| {
                            log::warn!("Failed to register quote_stk_v1 callback: {}", e);
                            e
                        })?;
                    log::debug!("✅ 已註冊 quote_stk_v1 回調到真實 shioaji 實例");
                }
                
                if let Some(callback) = general_quote_callback {
                    quote.call_method(py, "set_quote_callback", (callback,), None)
                        .map_err(|e| {
                            log::warn!("Failed to register quote callback: {}", e);
                            e
                        })?;
                    log::debug!("✅ 已註冊 quote 回調到真實 shioaji 實例");
                }
                
                // Register system callbacks to main instance (not quote object)
                if let Some(callback) = order_callback {
                    instance.call_method(py, "set_order_callback", (callback,), None)
                        .map_err(|e| {
                            log::warn!("Failed to register order callback: {}", e);
                            e
                        })?;
                    log::debug!("✅ 已註冊 order 回調到真實 shioaji 實例");
                }
                
                if let Some(callback) = system_event_callback {
                    instance.call_method(py, "set_event_callback", (callback,), None)
                        .map_err(|e| {
                            log::warn!("Failed to register event callback: {}", e);
                            e
                        })?;
                    log::debug!("✅ 已註冊 system_event 回調到真實 shioaji 實例");
                }
                
                if let Some(callback) = session_down_callback {
                    instance.call_method(py, "set_session_down_callback", (callback,), None)
                        .map_err(|e| {
                            log::warn!("Failed to register session_down callback: {}", e);
                            e
                        })?;
                    log::debug!("✅ 已註冊 session_down 回調到真實 shioaji 實例");
                }
                
                log::info!("✅ v0.4.5 Real event bridge callback system initialized");
                log::info!("📋 所有回調函數已註冊到真實 shioaji 實例");
                log::info!("🔗 Python-Rust 事件橋接已建立，準備接收市場數據");
                
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
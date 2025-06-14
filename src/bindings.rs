use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::collections::HashMap;
use crate::platform::Platform;

/// FFI bindings to Python shioaji C extensions
pub struct PythonBindings {
    py: Python<'static>,
    shioaji_module: PyObject,
    solace_api: PyObject,
    platform: Platform,
}

impl PythonBindings {
    pub fn new() -> PyResult<Self> {
        pyo3::prepare_freethreaded_python();
        
        Python::with_gil(|py| {
            // Detect platform and validate installation
            let platform = Platform::detect();
            let base_path = std::env::current_dir().unwrap();
            
            // Validate that the required files exist for this platform
            if let Err(e) = platform.validate_installation(&base_path) {
                return Err(PyErr::new::<pyo3::exceptions::PyImportError, _>(
                    format!("Platform validation failed: {}", e)
                ));
            }
            
            // Get platform-specific shioaji path
            let shioaji_path = platform.get_shioaji_path(&base_path)
                .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyImportError, _>(
                    "Unsupported platform"
                ))?;
            
            // Set up environment variables if needed
            for (key, value) in platform.get_env_vars(&shioaji_path) {
                std::env::set_var(key, value);
            }
            
            // Add the platform-specific lib path to Python sys.path
            let sys = py.import("sys")?;
            let path: &PyList = sys.getattr("path")?.downcast()?;
            
            // Since we fixed the system shioaji, just use it directly
            log::info!("Using system shioaji installation");
            let shioaji_module = py.import("shioaji")?;
            
            // Don't load solace API during initialization to avoid import issues
            let solace_api = py.None();
            
            let platform_dir = platform.directory_name().unwrap();
            log::info!("Successfully loaded shioaji for platform: {}", platform_dir);
            
            Ok(Self {
                py: unsafe { std::mem::transmute(py) },
                shioaji_module: shioaji_module.into(),
                solace_api: solace_api.into(),
                platform,
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
            
            shioaji_class.call(py, (), Some(kwargs))
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
    
    /// Set callback for tick data (simplified version without actual callback)
    pub fn set_tick_callback(&self, instance: &PyObject) -> PyResult<()> {
        Python::with_gil(|py| {
            // For now, we'll just set a placeholder callback
            // In a real implementation, you'd need to create a proper Python callable
            let py_none = py.None();
            instance.call_method(py, "set_on_tick_stk_v1_callback", (py_none,), None)?;
            Ok(())
        })
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
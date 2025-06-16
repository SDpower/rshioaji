//! # Simplified Python-Rust Event Bridge for v0.3.0
//! 
//! This module provides a simplified event bridge implementation that focuses
//! on establishing the basic Python-Rust callback infrastructure.

use std::sync::Weak;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use tokio::sync::Mutex;
use chrono::Utc;
use serde_json::Value;

use crate::callbacks::EventHandlers;
use crate::types::{Exchange, TickSTKv1, BidAskSTKv1};
use crate::types::orders::OrderState;
use crate::error::Result;

/// Simplified bridge that manages Python-Rust event forwarding
pub struct EventBridge {
    /// Weak reference to event handlers to avoid circular references
    handlers: Weak<Mutex<EventHandlers>>,
}

impl EventBridge {
    /// Create a new event bridge
    pub fn new(handlers: Weak<Mutex<EventHandlers>>) -> Result<Self> {
        Ok(Self { handlers })
    }

    /// Create a Python callback function that forwards to Rust handlers
    pub fn create_python_callback(&self, callback_type: &str) -> PyResult<PyObject> {
        let handlers = self.handlers.clone();
        let callback_type = callback_type.to_string();
        
        Python::with_gil(|py| {
            // Create a simple Python function that will be called by shioaji
            let callback_code = format!(r#"
def shioaji_callback(*args, **kwargs):
    """Callback function that forwards shioaji events to Rust"""
    import json
    try:
        # Convert arguments to JSON-serializable format
        args_dict = {{}}
        if len(args) >= 2:
            # First arg is usually exchange, second is data
            args_dict['exchange'] = str(args[0]) if args[0] else 'TSE'
            args_dict['data'] = dict(args[1]) if hasattr(args[1], '__dict__') else str(args[1])
        
        # Call the Rust handler through the registered function
        # This will be implemented in the binding layer
        print(f"[{}] Callback triggered with args: {{args_dict}}")
        
    except Exception as e:
        print(f"[{}] Callback error: {{e}}")

shioaji_callback
"#, callback_type, callback_type);

            let module = PyModule::from_code(py, &callback_code, "callback.py", "callback")?;
            let callback = module.getattr("shioaji_callback")?;
            Ok(callback.into())
        })
    }

    /// Forward tick data to registered Rust handlers
    pub async fn forward_tick_event(&self, exchange: Exchange, tick_data: TickSTKv1) {
        if let Some(handlers_arc) = self.handlers.upgrade() {
            let handlers = handlers_arc.lock().await;
            handlers.trigger_tick_stk_v1(exchange, tick_data);
        }
    }

    /// Forward bid/ask data to registered Rust handlers
    pub async fn forward_bidask_event(&self, exchange: Exchange, bidask_data: BidAskSTKv1) {
        if let Some(handlers_arc) = self.handlers.upgrade() {
            let handlers = handlers_arc.lock().await;
            handlers.trigger_bidask_stk_v1(exchange, bidask_data);
        }
    }

    /// Forward order events to registered Rust handlers
    pub async fn forward_order_event(&self, order_state: OrderState, order_data: Value) {
        if let Some(handlers_arc) = self.handlers.upgrade() {
            let handlers = handlers_arc.lock().await;
            handlers.trigger_order(order_state, order_data);
        }
    }
}

/// Registry for managing Python callback objects
pub struct CallbackRegistry {
    /// Available callback objects
    pub callbacks: std::collections::HashMap<String, PyObject>,
}

impl CallbackRegistry {
    pub fn new() -> Self {
        Self {
            callbacks: std::collections::HashMap::new(),
        }
    }

    pub fn register_callback(&mut self, name: String, callback: PyObject) {
        self.callbacks.insert(name, callback);
    }

    pub fn get_callback(&self, name: &str) -> Option<&PyObject> {
        self.callbacks.get(name)
    }
}

impl Default for CallbackRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper functions for data conversion
pub mod conversion {
    use super::*;

    /// Convert Python dict to minimal TickSTKv1
    pub fn dict_to_tick_stk(data: &PyDict) -> PyResult<TickSTKv1> {
        Ok(TickSTKv1 {
            code: data.get_item("code")
                .unwrap_or_default()
                .and_then(|item| item.extract().ok())
                .unwrap_or_else(|| "UNKNOWN".to_string()),
            datetime: Utc::now(), // Simplified - use current time
            open: extract_f64(data, "open"),
            avg_price: extract_f64(data, "avg_price"),
            close: extract_f64(data, "close"),
            high: extract_f64(data, "high"),
            low: extract_f64(data, "low"),
            amount: extract_f64(data, "amount"),
            total_amount: extract_f64(data, "total_amount"),
            volume: extract_i64(data, "volume"),
            total_volume: extract_i64(data, "total_volume"),
            tick_type: crate::types::constants::TickType::Buy,
            chg_type: crate::types::constants::ChangeType::Up,
            price_chg: extract_f64(data, "price_chg"),
            pct_chg: extract_f64(data, "pct_chg"),
            bid_side_total_vol: extract_i64(data, "bid_side_total_vol"),
            ask_side_total_vol: extract_i64(data, "ask_side_total_vol"),
            bid_side_total_cnt: extract_i64(data, "bid_side_total_cnt"),
            ask_side_total_cnt: extract_i64(data, "ask_side_total_cnt"),
            suspend: extract_bool(data, "suspend"),
            simtrade: extract_bool(data, "simtrade"),
        })
    }

    fn extract_f64(data: &PyDict, key: &str) -> f64 {
        data.get_item(key)
            .unwrap_or_default()
            .and_then(|item| item.extract().ok())
            .unwrap_or(0.0)
    }

    fn extract_i64(data: &PyDict, key: &str) -> i64 {
        data.get_item(key)
            .unwrap_or_default()
            .and_then(|item| item.extract().ok())
            .unwrap_or(0)
    }

    fn extract_bool(data: &PyDict, key: &str) -> bool {
        data.get_item(key)
            .unwrap_or_default()
            .and_then(|item| item.extract().ok())
            .unwrap_or(false)
    }

    /// Parse exchange string
    pub fn parse_exchange(exchange_str: &str) -> Exchange {
        match exchange_str {
            "TSE" => Exchange::TSE,
            "OTC" => Exchange::OTC,
            "OES" => Exchange::OES,
            "TAIFEX" => Exchange::TAIFEX,
            _ => Exchange::TSE, // Default
        }
    }
}
//! # rshioaji - Rust Wrapper for Taiwan Shioaji API
//! 
//! A high-performance trading library with native event callbacks for Taiwan Shioaji API.
//! 
//! ## Features
//! 
//! - 🚀 **High Performance**: Built with Rust for excellent performance and memory safety
//! - 📡 **Native Event Callbacks**: Full Rust trait-based event callback system
//! - 🌐 **Multi-platform**: Supports macOS ARM64 and Linux x86_64
//! - ⚡ **Async Support**: Built on tokio for async operations
//! - 🛡️ **Type Safety**: Complete Rust type definitions with compile-time checks
//! 
//! ## Version 0.2.0 - Event Callback System
//! 
//! This version introduces a comprehensive event callback system with native Rust traits:
//! 
//! - **TickCallback** - Handle stock and futures tick data events
//! - **BidAskCallback** - Handle bid/ask spread events  
//! - **QuoteCallback** - Handle quote events
//! - **OrderCallback** - Handle order status changes
//! - **SystemCallback** - Handle system events and connection status
//! 
//! ## Quick Start
//! 
//! ```no_run
//! use rshioaji::{Shioaji, TickCallback, Exchange, TickSTKv1};
//! use std::sync::Arc;
//! 
//! struct MyHandler;
//! 
//! impl TickCallback for MyHandler {
//!     fn on_tick_stk_v1(&self, exchange: Exchange, tick: TickSTKv1) {
//!         println!("Received tick for {}: {}", tick.code, tick.close);
//!     }
//!     
//!     fn on_tick_fop_v1(&self, exchange: Exchange, tick: rshioaji::TickFOPv1) {
//!         // Handle futures/options tick
//!     }
//! }
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Shioaji::new(true, std::collections::HashMap::new())?;
//!     client.init().await?;
//!     
//!     // Register callback
//!     let handler = Arc::new(MyHandler);
//!     client.register_tick_callback(handler).await;
//!     client.setup_callbacks().await?;
//!     
//!     Ok(())
//! }
//! ```

pub mod bindings;
pub mod callbacks;
pub mod client;
pub mod config;
pub mod error;
pub mod platform;
pub mod types;
pub mod utils;

// Re-export commonly used types and functions
pub use callbacks::{TickCallback, BidAskCallback, QuoteCallback, OrderCallback, SystemCallback, EventHandlers};
pub use client::Shioaji;
pub use config::Config;
pub use error::{Error, Result};
pub use platform::Platform;
pub use utils::{EnvironmentConfig, init_logging, set_error_tracking, clear_outdated_contract_cache, check_contract_cache};

// Re-export all types from the types module
pub use types::{
    accounts::*,
    constants::*,
    contracts::*,
    market_data::*,
    orders::*,
    positions::*,
};
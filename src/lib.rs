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
//! ## Version 0.4.7 - Complete Callback System
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
//! use rshioaji::{Shioaji, Exchange, TickSTKv1};
//! use std::collections::HashMap;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Shioaji::new(true, HashMap::new())?;
//!     client.init().await?;
//!     
//!     // Register stock tick callback
//!     client.on_tick_stk_v1(|exchange: Exchange, tick: TickSTKv1| {
//!         println!("Received tick for {}: {}", tick.code, tick.close);
//!     }, false).await?;
//!     
//!     // Register system event callback
//!     client.on_event(|resp_code, event_code, info, event| {
//!         println!("System event: {} {} - {} {}", resp_code, event_code, info, event);
//!     }).await?;
//!     
//!     Ok(())
//! }
//! ```

// pub mod bindings; // Removed - using pure system shioaji architecture
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
pub use utils::{
    EnvironmentConfig, 
    init_logging, 
    set_error_tracking, 
    clear_outdated_contract_cache, 
    check_contract_cache,
    create_shared_folder,
    get_contract_folder,
    raise_resp_error,
    timeout_exception,
    status_error_wrapper
};

// Re-export all types from the types module
pub use types::{
    accounts::*,
    constants::*,
    contracts::*,
    market_data::*,
    orders::*,
    positions::*,
};
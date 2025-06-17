//! # rshioaji - Rust Wrapper for Taiwan Shioaji API
//! 
//! A high-performance trading library with full Python-Rust event bridging for Taiwan Shioaji API.
//! 
//! ## Features
//! 
//! - 🚀 **High Performance**: Built with Rust for excellent performance and memory safety
//! - 📡 **Native Event Callbacks**: Full Rust trait-based event callback system
//! - 🌐 **Multi-platform**: Supports macOS ARM64 and Linux x86_64
//! - ⚡ **Async Support**: Built on tokio for async operations
//! - 🛡️ **Type Safety**: Complete Rust type definitions with compile-time checks
//! 
//! ## Version 0.3.9 - Advanced Real Event Bridging & Complete Shioaji Integration
//! 
//! This version introduces advanced real event bridging and complete shioaji integration:
//! 
//! - **RealEventBridge** - Advanced real-time event processing with high-frequency support
//! - **ShioajiIntegration** - Enterprise-grade trading system integration
//! - **SmartOrderEngine** - TWAP, conditional orders, and algorithmic trading
//! - **RiskManager** - Real-time risk monitoring and automatic risk controls
//! - **PerformanceTracker** - Comprehensive trading analytics and reporting
//! - **MarketDataManager** - Advanced market data subscription and processing
//! - **Complete Event System** - Full Python-Rust event bridging with statistics and monitoring
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
pub mod event_bridge;
pub mod platform;
pub mod shioaji_integration;
pub mod types;
pub mod utils;

// Re-export commonly used types and functions
pub use callbacks::{TickCallback, BidAskCallback, QuoteCallback, OrderCallback, SystemCallback, EventHandlers};
pub use client::Shioaji;
pub use config::Config;
pub use error::{Error, Result};
pub use event_bridge::{RealEventBridge, Event, BridgeState, EventStatistics};
pub use platform::Platform;
pub use shioaji_integration::{ShioajiIntegration, SmartOrderType, PriceSnapshot, IntegrationState};
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
pub mod bindings;
pub mod client;
pub mod config;
pub mod error;
pub mod platform;
pub mod types;
pub mod utils;

// Re-export commonly used types and functions
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
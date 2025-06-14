pub mod bindings;
pub mod types;
pub mod client;
pub mod error;
pub mod platform;
pub mod config;

pub use client::Shioaji;
pub use error::{Error, Result};
pub use types::*;
pub use config::{Config, ConfigError};

// Re-export for easier access
pub use platform::Platform;
pub mod bindings;
pub mod types;
pub mod client;
pub mod error;
pub mod platform;

pub use client::Shioaji;
pub use error::{Error, Result};
pub use types::*;

// Re-export for easier access
pub use platform::Platform;
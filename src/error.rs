use thiserror::Error;
use pyo3::PyDowncastError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Python error: {0}")]
    Python(#[from] pyo3::PyErr),
    
    #[error("Python downcast error: {0}")]
    PyDowncast(String),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Authentication failed: {0}")]
    Authentication(String),
    
    #[error("Invalid contract: {0}")]
    InvalidContract(String),
    
    #[error("Invalid order: {0}")]
    InvalidOrder(String),
    
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("API error: {0}")]
    Api(String),
    
    #[error("Not logged in")]
    NotLoggedIn,
    
    #[error("Account not found")]
    AccountNotFound,
    
    #[error("Insufficient balance")]
    InsufficientBalance,
    
    #[error("Market closed")]
    MarketClosed,
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl<'a> From<PyDowncastError<'a>> for Error {
    fn from(err: PyDowncastError<'a>) -> Self {
        Error::PyDowncast(err.to_string())
    }
}
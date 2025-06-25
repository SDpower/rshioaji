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
    
    #[error("Not logged in: {0}")]
    NotLoggedIn(String),
    
    #[error("Client not initialized: {0}")]
    NotInitialized(String),
    
    #[error("Initialization error: {0}")]
    Initialization(String),
    
    #[error("Contract fetch error: {0}")]
    ContractFetch(String),
    
    #[error("Trading error: {0}")]
    Trading(String),
    
    #[error("Subscription error: {0}")]
    Subscription(String),
    
    #[error("Callback error: {0}")]
    Callback(String),
    
    #[error("Connection error: {0}")]
    Connection(String),
    
    #[error("Data fetch error: {0}")]
    DataFetch(String),
    
    #[error("File system error: {0}")]
    FileSystem(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Account not found")]
    AccountNotFound,
    
    #[error("Insufficient balance")]
    InsufficientBalance,
    
    #[error("Market closed")]
    MarketClosed,
    
    #[error("Timeout error: {0}")]
    Timeout(String),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
    
    #[error("System error: {0}")]
    System(String),
}

impl<'a> From<PyDowncastError<'a>> for Error {
    fn from(err: PyDowncastError<'a>) -> Self {
        Error::PyDowncast(err.to_string())
    }
}
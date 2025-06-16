use std::env;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Environment variable not found: {0}")]
    EnvVarNotFound(String),
    #[error("Failed to load .env file: {0}")]
    DotenvError(String),
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

/// Configuration for Shioaji client
#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: String,
    pub secret_key: String,
    pub simulation: bool,
    pub env_file_path: Option<String>,
}

impl Config {
    /// Create a new config with required fields
    pub fn new(api_key: String, secret_key: String, simulation: bool) -> Self {
        Self {
            api_key,
            secret_key,
            simulation,
            env_file_path: None,
        }
    }

    /// Load configuration from environment variables and optional .env file
    pub fn from_env() -> Result<Self, ConfigError> {
        // Try to load .env file from current directory
        if Self::load_dotenv_file(".env").is_ok() {
            log::info!("Successfully loaded .env file");
        } else {
            log::debug!("No .env file found or failed to load, using environment variables only");
        }
        
        let api_key = env::var("SHIOAJI_API_KEY")
            .or_else(|_| env::var("API_KEY"))
            .map_err(|_| ConfigError::EnvVarNotFound("SHIOAJI_API_KEY or API_KEY".to_string()))?;

        let secret_key = env::var("SHIOAJI_SECRET_KEY")
            .or_else(|_| env::var("SECRET_KEY"))
            .map_err(|_| ConfigError::EnvVarNotFound("SHIOAJI_SECRET_KEY or SECRET_KEY".to_string()))?;

        let simulation = env::var("SHIOAJI_SIMULATION")
            .or_else(|_| env::var("SIMULATION"))
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()
            .unwrap_or(true);

        Ok(Self {
            api_key,
            secret_key,
            simulation,
            env_file_path: Some(".env".to_string()),
        })
    }

    /// Load configuration from a specific .env file path
    pub fn from_env_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let path_str = path.as_ref().to_string_lossy().to_string();
        Self::load_dotenv_file(&path_str)?;

        let api_key = env::var("SHIOAJI_API_KEY")
            .or_else(|_| env::var("API_KEY"))
            .map_err(|_| ConfigError::EnvVarNotFound("SHIOAJI_API_KEY or API_KEY".to_string()))?;

        let secret_key = env::var("SHIOAJI_SECRET_KEY")
            .or_else(|_| env::var("SECRET_KEY"))
            .map_err(|_| ConfigError::EnvVarNotFound("SHIOAJI_SECRET_KEY or SECRET_KEY".to_string()))?;

        let simulation = env::var("SHIOAJI_SIMULATION")
            .or_else(|_| env::var("SIMULATION"))
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()
            .unwrap_or(true);

        Ok(Self {
            api_key,
            secret_key,
            simulation,
            env_file_path: Some(path_str),
        })
    }

    /// Load .env file and set environment variables
    fn load_dotenv_file(path: &str) -> Result<(), ConfigError> {
        if !Path::new(path).exists() {
            return Err(ConfigError::DotenvError(format!("File not found: {}", path)));
        }

        dotenvy::from_filename(path)
            .map_err(|e| ConfigError::DotenvError(format!("Failed to load {}: {}", path, e)))?;

        log::info!("Successfully loaded environment variables from {}", path);
        Ok(())
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.api_key.is_empty() {
            return Err(ConfigError::InvalidConfig("API key cannot be empty".to_string()));
        }

        if self.secret_key.is_empty() {
            return Err(ConfigError::InvalidConfig("Secret key cannot be empty".to_string()));
        }

        // Basic API key format validation (alphanumeric and common symbols)
        if !self.api_key.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            return Err(ConfigError::InvalidConfig("API key contains invalid characters".to_string()));
        }

        if !self.secret_key.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            return Err(ConfigError::InvalidConfig("Secret key contains invalid characters".to_string()));
        }

        Ok(())
    }

    /// Get a summary of the configuration (without revealing secrets)
    pub fn summary(&self) -> String {
        format!(
            "Config {{ api_key: {}***, secret_key: {}***, simulation: {}, env_file: {:?} }}",
            &self.api_key[..std::cmp::min(4, self.api_key.len())],
            &self.secret_key[..std::cmp::min(4, self.secret_key.len())],
            self.simulation,
            self.env_file_path
        )
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::from_env().unwrap_or_else(|_| Self {
            api_key: String::new(),
            secret_key: String::new(),
            simulation: true,
            env_file_path: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_config_creation() {
        let config = Config::new(
            "test_api_key".to_string(),
            "test_secret_key".to_string(),
            true,
        );
        
        assert_eq!(config.api_key, "test_api_key");
        assert_eq!(config.secret_key, "test_secret_key");
        assert!(config.simulation);
    }

    #[test]
    fn test_config_validation() {
        let valid_config = Config::new(
            "testkey123".to_string(),
            "testsecret456".to_string(),
            true,
        );
        assert!(valid_config.validate().is_ok());

        let invalid_config = Config::new(
            "".to_string(),
            "testsecret456".to_string(),
            true,
        );
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_config_summary() {
        let config = Config::new(
            "testkey123".to_string(),
            "testsecret456".to_string(),
            false,
        );
        
        let summary = config.summary();
        assert!(summary.contains("test***"));
        assert!(summary.contains("simulation: false"));
        assert!(!summary.contains("testkey123")); // Should not expose full key
    }

    #[test]
    fn test_config_from_env() {
        // Set test environment variables
        env::set_var("SHIOAJI_API_KEY", "env_test_key");
        env::set_var("SHIOAJI_SECRET_KEY", "env_test_secret");
        env::set_var("SHIOAJI_SIMULATION", "false");

        let config = Config::from_env();
        
        // Clean up
        env::remove_var("SHIOAJI_API_KEY");
        env::remove_var("SHIOAJI_SECRET_KEY");
        env::remove_var("SHIOAJI_SIMULATION");

        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.api_key, "env_test_key");
        assert_eq!(config.secret_key, "env_test_secret");
        assert!(!config.simulation);
    }
}
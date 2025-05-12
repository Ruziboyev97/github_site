
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Configuration error: {0}")]
    Missing(&'static str),
}

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub github_client_id: String,
    pub github_client_secret: String,
    pub session_secret: String,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        Ok(Self {
            database_url: "postgres://user".into(),
            github_client_id: "your github client_id".into(),
            github_client_secret: "Your secret".into(),
            session_secret: "super_secret_key_123".into()
        })
    }
}
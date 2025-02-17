use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub log: LogConfig,
    pub database: DatabaseConfig,
    pub tinkoff_api: TinkoffApiConfig,
    pub client_rest: ClientRestConfig,
    pub share_updater: ShareUpdaterConfig,
}
#[derive(Debug, Deserialize)]
pub struct ShareUpdaterConfig {
    pub enabled: bool,
    pub interval_seconds: u64,
    pub max_retries: u32,
    pub retry_delay_seconds: u64,
}

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    pub level: String,
    pub format: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub max_connections: u32,
    pub timeout_seconds: u64,
    pub pool_size: u32,
}

#[derive(Debug, Deserialize)]
pub struct TinkoffApiConfig {
    pub base_url: String,
    pub domain: String,
}

#[derive(Debug, Deserialize)]
pub struct ClientRestConfig {
    pub timeout: u64,
    pub keepalive: u64,
}

impl ClientRestConfig {
    pub fn timeout(&self) -> Duration {
        Duration::from_secs(self.timeout)
    }

    pub fn keepalive(&self) -> Duration {
        Duration::from_secs(self.keepalive)
    }
}

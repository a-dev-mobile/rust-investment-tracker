use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct LogConfig {
    pub level: String,
    pub format: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub address: String,
    pub port: u16,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub pool_size: u32,
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GrpcTinkoffConfig {
    pub base_url: String,
    pub domain: String,
    pub token: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GrpcClientConfig {
    pub keepalive: u64,
    pub timeout: u64,
}
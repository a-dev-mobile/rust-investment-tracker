use super::types::{DatabaseConfig, GrpcClientConfig, GrpcTinkoffConfig, LogConfig, ServerConfig};
use super::Environment;
use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::env;
use std::str::FromStr;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    #[serde(skip)]
    pub environment: Environment,
    #[allow(dead_code)]
    pub app_env: String,
    pub server: ServerConfig,
    pub log: LogConfig,
    pub database: DatabaseConfig,
    pub grpc_tinkoff: GrpcTinkoffConfig,
    pub grpc_cient: GrpcClientConfig,
}

impl Settings {
    pub fn environment(&self) -> &Environment {
        &self.environment
    }

    pub fn is_dev(&self) -> bool {
        self.environment.is_dev()
    }
}

fn get_env_var(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| panic!("ENV -> {} is not set", name))
}

pub fn get_settings() -> Result<Settings, ConfigError> {
    let app_env = get_env_var("APP_ENV");
    let config_path = format!("{}/{}", "config", app_env);

    let config = Config::builder()
        .add_source(File::with_name(&config_path))
        .build()?;

    let mut settings: Settings = config.try_deserialize()?;

    // Override with environment variables
    settings.database.url = get_env_var("APP_DATABASE_URL");
    settings.grpc_tinkoff.token = get_env_var("APP_GRPC_TINKOFF_TOKEN");

    // Set environment from app_env
    settings.environment = Environment::from_str(&app_env).expect("Invalid environment");

    Ok(settings)
}

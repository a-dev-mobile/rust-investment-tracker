use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Deserialize, Clone)]
pub struct AppEnv {
    pub env: Env,
    pub database_url: String,
    pub tinkoff_token: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub enum Env {
    Local,
    Development,
    Production,
}

impl FromStr for Env {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Env::Local),
            "dev" | "development" => Ok(Env::Development),
            "prod" | "production" => Ok(Env::Production),
            _ => Err(format!("Unknown environment: {}", s)),
        }
    }
}

impl ToString for Env {
    fn to_string(&self) -> String {
        match self {
            Env::Local => "local".to_string(),
            Env::Development => "dev".to_string(),
            Env::Production => "prod".to_string(),
        }
    }
}
impl Env {
    pub fn is_dev(env: &Env) -> bool {
        matches!(env, Env::Local | Env::Development)
    }
}

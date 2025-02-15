use std::str::FromStr;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Environment {
    #[default]
    Local,
    Development,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Development => "dev",
            Environment::Production => "prod",
        }
    }

    pub fn is_dev(&self) -> bool {
        matches!(self, Environment::Development | Environment::Local)
    }
}

impl FromStr for Environment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Environment::Local),
            "dev" | "development" => Ok(Environment::Development),
            "prod" | "production" => Ok(Environment::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either 'local', 'dev', or 'prod'",
                other
            )),
        }
    }
}

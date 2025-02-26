use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing::info;

use crate::env_config::models::app_setting::AppSettings;

#[derive(Clone)]
pub struct PostgresDb {
    pub pool: PgPool,
}

impl PostgresDb {
    pub async fn connect(settings: &AppSettings) -> PostgresDb {
        info!("Connecting to PostgreSQL database...");

        let pool = PgPoolOptions::new()
            .max_connections(settings.app_config.postgres_db.max_connections)
            .acquire_timeout(std::time::Duration::from_secs(
                settings.app_config.postgres_db.timeout_seconds,
            ))
            .connect(&settings.app_env.postgres_url)
            .await
            .expect("Failed to connect to PostgreSQL database");

        info!("Successfully connected to PostgreSQL database");
        PostgresDb { pool }
    }
}

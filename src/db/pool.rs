use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing::info;

use crate::env_config::models::app_setting::AppSettings;

#[derive(Clone)]
pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn connect(settings: &AppSettings) -> Database {
        info!("Connecting to the database...");

        let pool = PgPoolOptions::new()
            .max_connections(settings.app_config.database.max_connections)
            .connect(&settings.app_env.database_url)
            .await
            .expect("Failed to connect to the database");

        info!("Successfully connected to the database");
        Database { pool }
    }
}

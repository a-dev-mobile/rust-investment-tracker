use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing::info;

use crate::config::Settings;

#[derive(Clone)]
pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn connect(settings: &Settings) -> Result<Database, sqlx::Error> {
        info!("Connecting to the database...");

        let pool = PgPoolOptions::new()
            .max_connections(settings.database.max_connections)
            .connect(&settings.database.url)
            .await?;

        info!("Successfully connected to the database");
        Ok(Database { pool })
    }
}

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use tracing::{debug, error, info};

use crate::features::stream::models::watched_instrument::WatchedInstrument;

pub struct StreamRepository {
    db_pool: Arc<PgPool>,
}

impl StreamRepository {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self { db_pool }
    }

    pub async fn get_active_instruments(&self) -> Vec<WatchedInstrument> {
        debug!("Fetching active watched instruments");

        let result: Result<Vec<WatchedInstrument>, sqlx::Error> =
            sqlx::query_as::<_, WatchedInstrument>(
                "
SELECT figi, watched_instruments.subscription_interval_id
FROM instrument_services.watched_instruments
WHERE is_active = true
        ",
            )
            .fetch_all(&*self.db_pool)
            .await;

        match result {
            Ok(v) => {
                debug!("Found {} active watched instruments", v.len());
                return v;
            }
            Err(e) => {
                error!("Failed to fetch active watched instruments: {}", e);
                return vec![];
            }
        }
    }
}

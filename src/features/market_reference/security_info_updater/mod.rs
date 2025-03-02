mod scheduler;
mod tracking_service;



use crate::{
    db::MongoDb,
    env_config::models::app_setting::AppSettings,
    services::tinkoff::client_grpc::TinkoffClient,
};
use std::sync::Arc;
use sqlx::PgPool;
use tracing::info;

pub struct CandlesTrackingUpdater {
    client: Arc<TinkoffClient>,
    mongo_db: Arc<MongoDb>,
    settings: Arc<AppSettings>,
}

impl CandlesTrackingUpdater {
    pub async fn new(
        #[allow(unused_variables)] db_pool: Arc<PgPool>,
        mongo_db: Arc<MongoDb>,
        settings: Arc<AppSettings>,
        client: Arc<TinkoffClient>,
    ) -> Self {
        CandlesTrackingUpdater {
            client,
            mongo_db,
            settings,
        }
    }

    pub async fn start_update_loop(&self) {
        if !self.settings.app_config.candles_tracking_updater.enabled {
            info!("Candles tracking updater is disabled in configuration");
            return;
        }

        // Call the scheduler's start_update_loop method
        self.start_scheduler_loop().await;
    }
}
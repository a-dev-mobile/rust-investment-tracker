mod bonds_service;
mod client;
mod converter;
mod etfs_service;
mod futures_service;
mod scheduler;
mod shares_service;
mod status;

use crate::{
    db::MongoDb, env_config::models::app_setting::AppSettings,
    services::tinkoff::client_grpc::TinkoffClient,
};
use sqlx::PgPool;
use std::sync::Arc;
use tracing::info;

pub struct TinkoffInstrumentsUpdater {
    client: Arc<TinkoffClient>,
    mongo_db: Arc<MongoDb>,
    settings: Arc<AppSettings>,
}

impl TinkoffInstrumentsUpdater {
    pub async fn new(
        #[allow(unused_variables)] db_pool: Arc<PgPool>,
        mongo_db: Arc<MongoDb>,
        settings: Arc<AppSettings>,
        client: Arc<TinkoffClient>,
    ) -> Self {
        TinkoffInstrumentsUpdater {
            client,
            mongo_db,
            settings,
        }
    }

    pub async fn start_update_loop(&self) {
        if !self.settings.app_config.tinkoff_market_data_updater.enabled {
            info!("Instruments updater is disabled in configuration");
            return;
        }

        // Call the scheduler's start_update_loop method directly rather than using ::
        self.start_scheduler_loop().await;
    }
}

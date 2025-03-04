use std::sync::Arc;
use tracing::{error, info};

use super::service::HistoricalCandleDataService;
use super::updater::start_historical_candle_updater;
use crate::env_config::models::app_setting::AppSettings;
use crate::features::db::MongoDb;
use crate::services::tinkoff::client_grpc::TinkoffClient;

impl HistoricalCandleDataService {
    pub async fn run_at_startup(&self) {
        info!("Running historical candle data service at application startup");
        
        // The main processing logic is encapsulated in the start method
        self.start().await;
    }
}

pub async fn start_historical_candle_service(service: Arc<HistoricalCandleDataService>) {
    // Run the service in a separate task to not block application startup
    tokio::spawn(async move {
        service.run_at_startup().await;
    });
}

pub async fn initialize_historical_candle_services(
    client: Arc<TinkoffClient>,
    mongo_db: Arc<MongoDb>,
    settings: Arc<AppSettings>,
) {
    // Initialize the one-time service for historical data loading at startup
    if settings.app_config.historical_candle_data.enabled &&
       settings.app_config.historical_candle_data.run_on_startup {
        let historical_candle_service = Arc::new(HistoricalCandleDataService::new(
            client.clone(),
            mongo_db.clone(),
            settings.clone(),
        ));
        
        start_historical_candle_service(historical_candle_service).await;
    }
    
    // Initialize the periodic updater service
    if settings.app_config.historical_candle_updater.enabled {
        start_historical_candle_updater(client, mongo_db, settings).await;
    }
}
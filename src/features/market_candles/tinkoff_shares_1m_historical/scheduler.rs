use std::sync::Arc;
use tracing::{error, info};

use super::service::HistoricalCandleDataService;


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
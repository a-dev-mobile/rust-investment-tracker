use std::time::Duration;
use tokio::time;
use tracing::{error, info};

use super::TinkoffInstrumentsUpdater;

impl TinkoffInstrumentsUpdater {
    pub(super) async fn start_scheduler_loop(&self) {
        info!(
            "Starting instruments update loop with {} second interval (timezone: {})",
            self.settings
                .app_config
                .tinkoff_instruments_updater
                .interval_seconds,
            self.settings
                .app_config
                .tinkoff_instruments_updater
                .timezone
        );

        // Initialize status collection on startup
        match self.initialize_status_collection().await {
            Ok(_) => info!("Status collection initialized successfully"),
            Err(e) => error!("Failed to initialize status collection: {}", e),
        }

        let mut interval = time::interval(Duration::from_secs(
            self.settings
                .app_config
                .tinkoff_instruments_updater
                .interval_seconds,
        ));

        loop {
            interval.tick().await;

            if !self
                .settings
                .app_config
                .tinkoff_instruments_updater
                .is_update_time()
            {
                info!(
                    "Current time outside update window ({}-{})",
                    self.settings
                        .app_config
                        .tinkoff_instruments_updater
                        .update_start_time,
                    self.settings
                        .app_config
                        .tinkoff_instruments_updater
                        .update_end_time
                );
                continue;
            }

            info!("Fetching updated instruments data");

            // Update shares
            match self.update_shares().await {
                Ok(_) => info!("Successfully updated shares data"),
                Err(e) => error!("Failed to update shares: {}", e),
            }

            // Update bonds
            match self.update_bonds().await {
                Ok(_) => info!("Successfully updated bonds data"),
                Err(e) => error!("Failed to update bonds: {}", e),
            }
            // Update ETFs
            match self.update_etfs().await {
                Ok(_) => info!("Successfully updated ETFs data"),
                Err(e) => error!("Failed to update ETFs: {}", e),
            }
            // Update futures
            match self.update_futures().await {
                Ok(_) => info!("Successfully updated futures data"),
                Err(e) => error!("Failed to update futures: {}", e),
            }
        }
    }
}

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
        }
    }
}
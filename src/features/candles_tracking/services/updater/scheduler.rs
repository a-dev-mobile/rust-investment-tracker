use std::time::Duration;
use tokio::time;
use tracing::{error, info};

use super::CandlesTrackingUpdater;

impl CandlesTrackingUpdater {
    pub(super) async fn start_scheduler_loop(&self) {
        info!(
            "Starting candles tracking update loop with {} second interval (timezone: {})",
            self.settings
                .app_config
                .candles_tracking_updater
                .interval_seconds,
            self.settings
                .app_config
                .candles_tracking_updater
                .timezone
        );



        let mut interval = time::interval(Duration::from_secs(
            self.settings
                .app_config
                .candles_tracking_updater
                .interval_seconds,
        ));

        loop {
            interval.tick().await;

            if !self
                .settings
                .app_config
                .candles_tracking_updater
                .is_update_time()
            {
                info!(
                    "Current time outside candles tracking update window ({}-{})",
                    self.settings
                        .app_config
                        .candles_tracking_updater
                        .update_start_time,
                    self.settings
                        .app_config
                        .candles_tracking_updater
                        .update_end_time
                );
                continue;
            }

            info!("Updating candles tracking information");

            // Update the tracking data
            match self.update_candles_tracking().await {
                Ok(_) => info!("Successfully updated candles tracking data"),
                Err(e) => error!("Failed to update candles tracking: {}", e),
            }
        }
    }
}
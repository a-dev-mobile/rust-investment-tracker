use crate::{env_config::models::app_setting::AppSettings, features::market_reference::currency_rates::repository::CurrencyRatesRepository};
use crate::features::moex_api::client::MoexApiClient;

use std::sync::Arc;
use tracing::{error, info};

pub struct CurrencyRatesUpdater {
    api_client: MoexApiClient,
    repository: Arc<CurrencyRatesRepository>,
    settings: Arc<AppSettings>,
}

impl CurrencyRatesUpdater {
    pub fn new(
        api_client: MoexApiClient, 
        repository: Arc<CurrencyRatesRepository>,
        settings: Arc<AppSettings>,
    ) -> Self {
        Self {
            api_client,
            repository,
            settings,
        }
    }

    /// Запуск цикла обновления курсов валют
    pub async fn start_update_loop(self) {
        info!("Starting currency rates update scheduler");

        if !self.settings.app_config.currency_rates_updater.enabled {
            info!("Currency rates updater is disabled in configuration");
            return;
        }

        info!(
            "Starting currency rates update loop with {} second interval (timezone: {})",
            self.settings.app_config.currency_rates_updater.interval_seconds,
            self.settings.app_config.currency_rates_updater.timezone
        );

        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(
            self.settings.app_config.currency_rates_updater.interval_seconds
        ));

        loop {
            interval.tick().await;

            if !self.settings.app_config.currency_rates_updater.is_update_time() {
                info!(
                    "Current time outside currency rates update window ({}-{})",
                    self.settings.app_config.currency_rates_updater.update_start_time,
                    self.settings.app_config.currency_rates_updater.update_end_time
                );
                continue;
            }

            info!("Updating currency rates information");
            self.schedule_update().await;
        }
    }

    /// Запланировать одно обновление курсов валют
    async fn schedule_update(&self) {
        info!("Scheduling currency rates update");
        
        // Получаем данные от API
        match self.api_client.get_currency_rates().await {
            Ok(moex_rates) => {
                // Передаем данные в репозиторий для обработки и сохранения
                match self.repository.save_moex_rates(&moex_rates).await {
                    Ok(currency_rates) => {
                        info!("Currency rates updated successfully. Date: {}", currency_rates.date);
                    },
                    Err(e) => {
                        error!("Failed to process and save currency rates: {}", e);
                    }
                }
            },
            Err(e) => {
                error!("Failed to fetch currency rates from API: {}", e);
            }
        }
    }
}
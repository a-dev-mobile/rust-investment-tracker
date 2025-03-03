use crate::features::moex_api::models::CurrencyRatesResponse;
use crate::features::moex_api::repositories::CurrencyRatesRepository;
use crate::features::moex_api::services::MoexApiService;
use std::sync::Arc;
use tracing::{error, info};
const UPDATE_INTERVAL_SECS: u64 = 60;
// const UPDATE_INTERVAL_SECS: u64 = 4 * 60 * 60; // 4 hours
pub struct CurrencyRatesUpdater {
    api_service: MoexApiService,
    repository: Arc<CurrencyRatesRepository>,
}

impl CurrencyRatesUpdater {
    pub fn new(api_service: MoexApiService, repository: Arc<CurrencyRatesRepository>) -> Self {
        Self {
            api_service,
            repository,
        }
    }

    /// Получает обновленные курсы валют и сохраняет их в базу данных
    pub async fn update_currency_rates(
        &self,
    ) -> Result<CurrencyRatesResponse, Box<dyn std::error::Error>> {
        info!("Updating currency rates...");

        // Получаем курсы валют через API
        let rates = self.api_service.get_currency_rates().await?;

        // Сохраняем их в базу данных
        match self.repository.save_currency_rates(&rates).await {
            Ok(_) => info!("Currency rates successfully saved to database"),
            Err(e) => error!("Failed to save currency rates to database: {}", e),
        }

        Ok(rates)
    }

    /// Запускает периодическое обновление курсов валют
    pub async fn start_update_loop(self) {
        info!("Starting currency rates update loop");

        loop {
            match self.update_currency_rates().await {
                Ok(rates) => info!("Currency rates updated. Date: {}", rates.date),
                Err(e) => error!("Failed to update currency rates: {}", e),
            }

            // Ждем  до следующего обновления

            tokio::time::sleep(tokio::time::Duration::from_secs(UPDATE_INTERVAL_SECS)).await;
        }
    }
}

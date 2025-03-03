use crate::features::moex_api::models::{MoexRatesResponse, CurrencyRatesResponse};
use crate::features::moex_api::mappers::MoexRatesMapper;
use reqwest::Client;
use std::time::Duration;
use tracing::{info, error};

const MOEX_CURRENCY_RATES_URL: &str = 
    "https://iss.moex.com/iss/statistics/engines/currency/markets/selt/rates.json?iss.meta=off";
const REQUEST_TIMEOUT: u64 = 10; // секунд

pub struct MoexApiService {
    http_client: Client,
}

impl MoexApiService {
    pub fn new() -> Self {
        // Создаем HTTP клиент с настроенным таймаутом
        let client = Client::builder()
            .timeout(Duration::from_secs(REQUEST_TIMEOUT))
            .build()
            .unwrap_or_else(|_| Client::new());
            
        Self {
            http_client: client,
        }
    }

    /// Получает данные о курсах валют от API MOEX
    pub async fn get_currency_rates(&self) -> Result<CurrencyRatesResponse, Box<dyn std::error::Error>> {
        info!("Fetching currency rates from MOEX API");
        
        // Получаем данные с API MOEX
        let response = self.http_client.get(MOEX_CURRENCY_RATES_URL)
            .send()
            .await?;
            
        // Проверяем статус ответа
        if !response.status().is_success() {
            let status = response.status();
            error!("API returned error status: {}", status);
            return Err(format!("API error: {}", status).into());
        }
        
        // Десериализуем ответ в MoexRatesResponse
        let moex_response = response.json::<MoexRatesResponse>().await?;
        info!("Successfully received MOEX currency rates data");
        
        // Используем маппер для преобразования ответа MOEX в нашу модель данных
        MoexRatesMapper::map_to_currency_rates(moex_response)
    }
}
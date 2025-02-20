use crate::{
    env_config::models::app_setting::AppSettings, features::share_updater::models::Share, gen::tinkoff_public_invest_api_contract_v1::{
        InstrumentStatus, InstrumentsRequest, SharesResponse,
    }, services::tinkoff::client_grpc::TinkoffClient
};

use serde_json::json;
use sqlx::PgPool;

use std::sync::Arc;
use std::time::Duration;
use tokio::time;
use tracing::{error, info};

pub struct ShareUpdater {
    client: Arc<TinkoffClient>,
    db_pool: Arc<PgPool>,
    settings: Arc<AppSettings>,
}


impl ShareUpdater {
    pub async fn new(db_pool: Arc<PgPool>, settings: Arc<AppSettings>,    client: Arc<TinkoffClient>) -> Self {

        ShareUpdater {
            client,
            db_pool,
            settings,
        }
    }
    async fn fetch_shares(&self) -> SharesResponse {
        let request = self
            .client
            .create_request(InstrumentsRequest {
                instrument_status: InstrumentStatus::All as i32,
            })
            .expect("Failed to create request");

        // Клонируем клиента, чтобы получить мутабельную копию для вызова метода `shares`
        let mut instruments_client = self.client.instruments.clone();
        let response = instruments_client
            .shares(request)
            .await
            .map(|response| response.into_inner())
            .expect("Failed to get shares");
        // print!("{:?}", response);
        response
    }

    async fn update_shares(&self) -> Result<(), sqlx::Error> {
        // Получаем данные акций
        let shares_response = self.fetch_shares().await;
    
        // Преобразуем gRPC модели в Serde модели
        let shares: Vec<Share> = shares_response
            .instruments
            .into_iter()
            .map(Share::from)
            .collect();
    
        // Создаем JSON массив для передачи в функцию PostgreSQL
        let shares_json = json!(shares);
    
        // Вызываем функцию обновления
        let result = sqlx::query!(
            "SELECT instrument_services.update_shares($1) as affected_rows",
            &shares_json
        )
        .fetch_one(&*self.db_pool)
        .await?;
    
        tracing::info!("Updated {:?} share records", result.affected_rows);
    
        Ok(())
    }

    pub async fn start_update_loop(&self) {
        if !self.settings.app_config.share_updater.enabled {
            info!("Share updater is disabled in configuration");
            return;
        }
    
        info!(
            "Starting share update loop with {} second interval (timezone: {})",
            self.settings.app_config.share_updater.interval_seconds,
            self.settings.app_config.share_updater.timezone
        );
    
        let mut interval = time::interval(Duration::from_secs(
            self.settings.app_config.share_updater.interval_seconds,
        ));
    
        loop {
            interval.tick().await;
            
            if self.settings.app_config.share_updater.is_night_time() {
                info!("Skipping update during night time ({}-{} {})", 
                    self.settings.app_config.share_updater.night_start_time,
                    self.settings.app_config.share_updater.night_end_time,
                    self.settings.app_config.share_updater.timezone
                );
                continue;
            }
    
            info!("Fetching updated share data");
    
            match self.update_shares().await {
                Ok(_) => info!("Successfully updated shares data"),
                Err(e) => error!("Failed to update shares: {}", e),
            }
        }
    }
}

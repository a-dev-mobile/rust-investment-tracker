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
    client: TinkoffClient,
    db_pool: Arc<PgPool>,
    settings: Arc<AppSettings>,
}


impl ShareUpdater {
    pub async fn new(db_pool: Arc<PgPool>, settings: Arc<AppSettings>) -> Self {
        let client = TinkoffClient::new(settings.clone())
            .await
            .expect("Failed to initialize Tinkoff client - cannot proceed without API access");

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
    
        // // Debug logging
        // tracing::debug!("Shares JSON sample (first item): {}", 
        //     serde_json::to_string_pretty(&shares.first())
        //         .unwrap_or_else(|_| "Failed to serialize".to_string())
        // );
    
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
            "Starting share update loop with {} second interval",
            self.settings.app_config.share_updater.interval_seconds
        );

        let mut interval = time::interval(Duration::from_secs(
            self.settings.app_config.share_updater.interval_seconds,
        ));

        loop {
            interval.tick().await;
            info!("Fetching updated share data");

            match self.update_shares().await {
                Ok(_) => info!("Successfully updated shares data"),
                Err(e) => error!("Failed to update shares: {}", e),
            }
        }
    }
}

use crate::{
    env_config::models::app_setting::AppSettings,
    gen::tinkoff_public_invest_api_contract_v1::{
        GetCandlesRequest, GetCandlesResponse, InstrumentStatus, InstrumentsRequest, SharesResponse,
    },
    services::tinkoff::client_grpc::TinkoffClient,
};

use prost_types::Timestamp;
use serde_json::json;
use sqlx::PgPool;

use std::sync::Arc;
use std::time::Duration;
use tokio::time;
use tracing::{error, info};

pub struct CandlesUpdater {
    client: Arc<TinkoffClient>, 
    db_pool: Arc<PgPool>,
    settings: Arc<AppSettings>,
}

impl CandlesUpdater {
    pub async fn new(db_pool: Arc<PgPool>, settings: Arc<AppSettings>,    client: Arc<TinkoffClient>) -> Self {
    
        CandlesUpdater {
            client,
            db_pool,
            settings,
        }
    }
    async fn fetch_candles(&self) -> GetCandlesResponse {
        let request = self
            .client
            .create_request(GetCandlesRequest {
                instrument_id: "BBG004730N88".to_string(),
                from: Some(Timestamp {
                    seconds: 1738713600,
                    nanos: 0,
                }),
                to: Some(Timestamp {
                    seconds: 1738800000,
                    nanos: 0,
                }),
                interval: 1,
                figi: "BBG004730N88".to_string(),
            })
            .expect("Failed to create request");

        // Клонируем клиента, чтобы получить мутабельную копию для вызова метода `candles`
        let mut market_data_client = self.client.market_data.clone();
        let response = market_data_client
            .get_candles(request)
            .await
            .map(|response| response.into_inner())
            .expect("Failed to get candles");
        // print!("{:?}", response);
        response
    }

    async fn update_candles(&self) -> Result<(), sqlx::Error> {
        // Получаем данные акций
        let candles_response = self.fetch_candles().await;

        // // Преобразуем gRPC модели в Serde модели
        // let candles: Vec<Share> = candles_response
        //     .instruments
        //     .into_iter()
        //     .map(Share::from)
        //     .collect();

        // // Создаем JSON массив для передачи в функцию PostgreSQL
        // let candles_json = json!(candles);

        // // Вызываем функцию обновления
        // let result = sqlx::query!(
        //     "SELECT instrument_services.update_candles($1) as affected_rows",
        //     &candles_json
        // )
        // .fetch_one(&*self.db_pool)
        // .await?;

        // tracing::info!("Updated {:?} share records", result.affected_rows);

        Ok(())
    }

    pub async fn start_update_loop(&self) {
        // if !self.settings.app_config.share_updater.enabled {
        //     info!("Share updater is disabled in configuration");
        //     return;
        // }

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

            // if self.settings.app_config.share_updater.is_night_time() {
            //     info!(
            //         "Skipping update during night time ({}-{} {})",
            //         self.settings.app_config.share_updater.night_start_time,
            //         self.settings.app_config.share_updater.night_end_time,
            //         self.settings.app_config.share_updater.timezone
            //     );
            //     continue;
            // }

            info!("Fetching updated share data");

            match self.update_candles().await {
                Ok(_) => info!("Successfully updated candles data"),
                Err(e) => error!("Failed to update candles: {}", e),
            }
        }
    }
}

use super::super::models::{Share, ShareResponse};
use crate::env_config::models::app_setting::AppSettings;
use reqwest::Client;
use sqlx::PgPool;
use std::sync::Arc;
use std::time::Duration;
use tokio::time;
use tracing::{error, info};

pub struct ShareUpdater {
    client: Client,
    db_pool: Arc<PgPool>,
    settings: Arc<AppSettings>,
}

impl ShareUpdater {
    pub fn new(db_pool: Arc<PgPool>, settings: Arc<AppSettings>) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(settings.app_config.client_rest.timeout))
            .build()
            .expect("Failed to create HTTP client");

        ShareUpdater {
            client,
            db_pool,
            settings,
        }
    }

    async fn fetch_shares(&self) -> Result<Vec<Share>, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!(
            "{}/rest/tinkoff.public.invest.api.contract.v1.InstrumentsService/Shares",
            self.settings.app_config.tinkoff_api.base_url
        );

        let response = self
            .client
            .post(&url)
            .header(
                "Authorization",
                format!("Bearer {}", self.settings.app_env.tinkoff_token),
            )
            .json(&serde_json::json!({
                "instrumentStatus": "INSTRUMENT_STATUS_ALL"
            }))
            .send()
            .await?;

        // Check if the request was successful
        if !response.status().is_success() {
            let error_msg = format!(
                "API request failed with status: {}, body: {}",
                response.status(),
                response.text().await?
            );
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                error_msg,
            )));
        }

        // Try to get the response body and print it for debugging
        let body = response.text().await?;
        println!("Response body: {}", body);

        // Parse the JSON
        let share_response: ShareResponse = serde_json::from_str(&body)?;

        Ok(share_response.instruments)
    }

    async fn update_shares(&self, shares: Vec<Share>) -> Result<(), sqlx::Error> {
        let mut transaction = self.db_pool.begin().await?;

        // Commit the transaction
        transaction.commit().await?;

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

            match self.fetch_shares().await {
                Ok(shares) => {
                    info!("Successfully fetched {} shares", shares.len());
                    match self.update_shares(shares).await {
                        Ok(_) => info!("Successfully updated shares in database"),
                        Err(e) => error!("Failed to update shares in database: {}", e),
                    }
                }
                Err(e) => error!("Failed to fetch shares: {}", e),
            }
        }
    }
}

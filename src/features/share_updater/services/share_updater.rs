use crate::{
    db::MongoDb, env_config::models::app_setting::AppSettings, features::core::models::share::HumanShare, gen::tinkoff_public_invest_api_contract_v1::{
        InstrumentStatus, InstrumentsRequest, Quotation, Share, SharesResponse
    }, services::tinkoff::client_grpc::TinkoffClient
};
use mongodb::bson::{doc, Document};
use prost_types::Timestamp;
use sqlx::PgPool;
use std::sync::Arc;
use std::time::Duration;
use tokio::time;
use tracing::{error, info};

pub struct ShareUpdater {
    client: Arc<TinkoffClient>,
    mongo_db: Arc<MongoDb>,
    settings: Arc<AppSettings>,
}

impl ShareUpdater {
    pub async fn new(
        #[allow(unused_variables)] db_pool: Arc<PgPool>,
        mongo_db: Arc<MongoDb>,
        settings: Arc<AppSettings>,
        client: Arc<TinkoffClient>,
    ) -> Self {
        ShareUpdater {
            client,
            mongo_db,
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

        let mut instruments_client = self.client.instruments.clone();
        let response = instruments_client
            .shares(request)
            .await
            .map(|response| response.into_inner())
            .expect("Failed to get shares");

        response
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

            if !self.settings.app_config.share_updater.is_update_time() {
                info!(
                    "Current time outside update window ({}-{})",
                    self.settings.app_config.share_updater.update_start_time,
                    self.settings.app_config.share_updater.update_end_time
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

    async fn set_status_updating(&self) -> Result<(), mongodb::error::Error> {
        let status_collection = self.mongo_db.status_collection();
        status_collection.delete_many(doc! {}).await?;
        status_collection
            .insert_one(doc! {
                "tinkoff_shares": "updating",
            })
            .await?;
        info!("Status set to 'updating' for tinkoff_shares collection");
        Ok(())
    }

    async fn set_status_ready(&self) -> Result<(), mongodb::error::Error> {
        let status_collection = self.mongo_db.status_collection();
        status_collection.delete_many(doc! {}).await?;
        status_collection
            .insert_one(doc! {
                "tinkoff_shares": "ready",
            })
            .await?;
        info!("Status set to 'ready' for tinkoff_shares collection");
        Ok(())
    }

    fn convert_share_to_document(&self, share: &Share) -> Document {
        // Convert to the human-readable model first
        let human_share = HumanShare::from(share);
        
        // Convert to BSON Document using serde
        match mongodb::bson::to_document(&human_share) {
            Ok(doc) => doc,
            Err(e) => {
                error!("Failed to convert share {} to document: {}", share.ticker, e);
                // Return empty document when conversion fails
                // This will be skipped during insertion
                doc! {}
            }
        }
    }

    async fn update_shares(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Fetch shares data via gRPC
        let shares_response = self.fetch_shares().await;
        let total_shares = shares_response.instruments.len();
        info!("Starting shares update: total {} records", total_shares);

        // Get collections
        let collection = self.mongo_db.shares_collection();

        // Set status to updating
        self.set_status_updating().await?;

        // Clear existing data
        collection.delete_many(doc! {}).await?;
        info!("Previous records deleted from collection");

        // Create documents for batch insertion
        let mut documents = Vec::with_capacity(total_shares);

        // Convert each share to MongoDB document
        for share in &shares_response.instruments {
            let doc = self.convert_share_to_document(share);
            // Only add non-empty documents
            if !doc.is_empty() {
                documents.push(doc);
            }
        }

        // Skip insertion if all documents failed to convert
        if documents.is_empty() {
            error!("Failed to convert any shares to documents, skipping database update");
            return Err("No valid documents to insert".into());
        }

        // Batch insert documents
        collection.insert_many(documents).await?;

        // Update status to ready
        self.set_status_ready().await?;
        info!(
            "Update completed: {} share records successfully processed",
            total_shares
        );

        Ok(())
    }
}
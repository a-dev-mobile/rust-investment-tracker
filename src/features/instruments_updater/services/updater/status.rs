use mongodb::bson::doc;
use tracing::info;

use super::TinkoffInstrumentsUpdater;

impl TinkoffInstrumentsUpdater {
    // New method to initialize the status collection
    pub(super) async fn initialize_status_collection(&self) -> Result<(), mongodb::error::Error> {
        let status_collection = self.mongo_db.status_collection();

        // Check if the collection has any documents
        let count = status_collection.count_documents(doc! {}).await?;

        if count == 0 {
            // If no documents exist, create an initial status document
            status_collection
                .insert_one(doc! {
                    "initialized_at": chrono::Utc::now().to_rfc3339(),
                    "tinkoff_shares": "not_started",
                    "tinkoff_bonds": "not_started"
                })
                .await?;
            info!("Status collection initialized with a base document");
        } else {
            info!("Status collection already exists with {} documents", count);
        }

        Ok(())
    }

    pub(super) async fn set_status_updating(
        &self,
        collection_name: &str,
    ) -> Result<(), mongodb::error::Error> {
        // First ensure the status collection exists
        self.initialize_status_collection().await?;

        let status_collection = self.mongo_db.status_collection();
        status_collection
            .update_one(
                doc! {},
                doc! {
                    "$set": { collection_name: "updating" }
                },
            )
            .await?;
        info!(
            "Status set to 'updating' for {} collection",
            collection_name
        );
        Ok(())
    }

    pub(super) async fn set_status_ready(
        &self,
        collection_name: &str,
    ) -> Result<(), mongodb::error::Error> {
        // First ensure the status collection exists
        self.initialize_status_collection().await?;

        let status_collection = self.mongo_db.status_collection();
        status_collection
            .update_one(
                doc! {},
                doc! {
                    "$set": { collection_name: "ready" }
                },
            )
            .await?;
        info!("Status set to 'ready' for {} collection", collection_name);
        Ok(())
    }
}

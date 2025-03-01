use mongodb::bson::{doc, Document};
use chrono::Utc;
use serde::{Serialize, Deserialize};
use tracing::info;

use super::TinkoffInstrumentsUpdater;


impl TinkoffInstrumentsUpdater {
    // New method to initialize the status collection
    pub(super) async fn initialize_status_collection(&self) -> Result<(), mongodb::error::Error> {
        let status_collection = self.mongo_db.status_collection();

        // Check if the collection has any documents
        let count = status_collection.count_documents(doc! {}).await?;

        if count == 0 {
            // Initialize with an empty document
            status_collection
                .insert_one(doc! {
                    "initialized_at": Utc::now().to_rfc3339(),
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
        let now = Utc::now().to_rfc3339();
        
        status_collection
            .update_one(
                doc! {},
                doc! {
                    "$set": {
                        collection_name: {
                            "status": "updating",
                            "updated_at": now.clone()
                        }
                    }
                },
            )
            .await?;
        info!(
            "Status set to 'updating' for {} collection at {}",
            collection_name,
            now
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
        let now = Utc::now().to_rfc3339();
        
        status_collection
            .update_one(
                doc! {},
                doc! {
                    "$set": {
                        collection_name: {
                            "status": "ready",
                            "updated_at": now.clone()
                        }
                    }
                },
            )
            .await?;
        info!(
            "Status set to 'ready' for {} collection at {}",
            collection_name,
            &now
        );
        Ok(())
    }
}

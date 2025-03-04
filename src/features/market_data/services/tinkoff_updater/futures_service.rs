use mongodb::bson::doc;
use tracing::{error, info};

use crate::features::db::mongo_db::Collections;

use super::TinkoffInstrumentsUpdater;

impl TinkoffInstrumentsUpdater {
    pub(super) async fn update_futures(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Fetch futures data via gRPC
        let futures_response = self.fetch_futures().await;
        let total_futures = futures_response.instruments.len();
        info!("Starting futures update: total {} records", total_futures);

        // Get collections
        let collection = self.mongo_db.futures_collection();

        // Set status to updating
        self.set_status_updating(Collections::TINKOFF_FUTURES)
            .await?;

        // Clear existing data
        collection.delete_many(doc! {}).await?;
        info!("Previous records deleted from futures collection");

        // Create documents for batch insertion
        let mut documents = Vec::with_capacity(total_futures);

        // Convert each future to MongoDB document
        for future in &futures_response.instruments {
            let doc = self.convert_future_to_document(future);
            // Only add non-empty documents
            if !doc.is_empty() {
                documents.push(doc);
            }
        }

        // Skip insertion if all documents failed to convert
        if documents.is_empty() {
            error!("Failed to convert any futures to documents, skipping database update");
            return Err("No valid future documents to insert".into());
        }

        // Batch insert documents
        collection.insert_many(documents).await?;

        // Update status to ready
        self.set_status_ready(Collections::TINKOFF_FUTURES).await?;
        info!(
            "Update completed: {} futures records successfully processed",
            total_futures
        );

        Ok(())
    }
}

use mongodb::bson::doc;
use tracing::{error, info};

use crate::db::mongo_db::Collections;

use super::TinkoffInstrumentsUpdater;

impl TinkoffInstrumentsUpdater {
    pub(super) async fn update_shares(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Fetch shares data via gRPC
        let shares_response = self.fetch_shares().await;
        let total_shares = shares_response.instruments.len();
        info!("Starting shares update: total {} records", total_shares);

        // Get collections
        let collection = self.mongo_db.shares_collection();

        // Set status to updating
        self.set_status_updating(Collections::TINKOFF_SHARES)
            .await?;

        // Clear existing data
        collection.delete_many(doc! {}).await?;
        info!("Previous records deleted from shares collection");

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
            return Err("No valid share documents to insert".into());
        }

        // Batch insert documents
        collection.insert_many(documents).await?;

        // Update status to ready
        self.set_status_ready(Collections::TINKOFF_SHARES).await?;
        info!(
            "Update completed: {} share records successfully processed",
            total_shares
        );

        Ok(())
    }
}

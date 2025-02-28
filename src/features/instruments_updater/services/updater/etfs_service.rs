use mongodb::bson::doc;
use tracing::{error, info};

use super::TinkoffInstrumentsUpdater;

impl TinkoffInstrumentsUpdater {
    pub(super) async fn update_etfs(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Fetch ETFs data via gRPC
        let etfs_response = self.fetch_etfs().await;
        let total_etfs = etfs_response.instruments.len();
        info!("Starting ETFs update: total {} records", total_etfs);

        // Get collections
        let collection = self.mongo_db.etfs_collection();

        // Set status to updating
        self.set_status_updating("tinkoff_etfs").await?;

        // Clear existing data
        collection.delete_many(doc! {}).await?;
        info!("Previous records deleted from ETFs collection");

        // Create documents for batch insertion
        let mut documents = Vec::with_capacity(total_etfs);

        // Convert each ETF to MongoDB document
        for etf in &etfs_response.instruments {
            let doc = self.convert_etf_to_document(etf);
            // Only add non-empty documents
            if !doc.is_empty() {
                documents.push(doc);
            }
        }

        // Skip insertion if all documents failed to convert
        if documents.is_empty() {
            error!("Failed to convert any ETFs to documents, skipping database update");
            return Err("No valid ETF documents to insert".into());
        }

        // Batch insert documents
        collection.insert_many(documents).await?;

        // Update status to ready
        self.set_status_ready("tinkoff_etfs").await?;
        info!(
            "Update completed: {} ETF records successfully processed",
            total_etfs
        );

        Ok(())
    }
}
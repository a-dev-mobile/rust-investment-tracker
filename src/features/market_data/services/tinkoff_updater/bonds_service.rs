use mongodb::bson::doc;
use tracing::{error, info};



use crate::features::db::mongo_db::Collections;

use super::TinkoffInstrumentsUpdater;

impl TinkoffInstrumentsUpdater {
    pub(super) async fn update_bonds(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Fetch bonds data via gRPC
        let bonds_response = self.fetch_bonds().await;
        let total_bonds = bonds_response.instruments.len();
        info!("Starting bonds update: total {} records", total_bonds);

        // Get collections
        let collection = self.mongo_db.bonds_collection();

        // Set status to updating
        self.set_status_updating(Collections::TINKOFF_BONDS).await?;

        // Clear existing data
        collection.delete_many(doc! {}).await?;
        info!("Previous records deleted from bonds collection");

        // Create documents for batch insertion
        let mut documents = Vec::with_capacity(total_bonds);

        // Convert each bond to MongoDB document
        for bond in &bonds_response.instruments {
            let doc = self.convert_bond_to_document(bond);
            // Only add non-empty documents
            if !doc.is_empty() {
                documents.push(doc);
            }
        }

        // Skip insertion if all documents failed to convert
        if documents.is_empty() {
            error!("Failed to convert any bonds to documents, skipping database update");
            return Err("No valid bond documents to insert".into());
        }

        // Batch insert documents
        collection.insert_many(documents).await?;

        // Update status to ready
        self.set_status_ready(Collections::TINKOFF_BONDS).await?;
        info!(
            "Update completed: {} bond records successfully processed",
            total_bonds
        );

        Ok(())
    }
}

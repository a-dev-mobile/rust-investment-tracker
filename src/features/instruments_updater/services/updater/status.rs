use mongodb::bson::doc;
use tracing::info;

use super::TinkoffInstrumentsUpdater;

impl TinkoffInstrumentsUpdater {
    pub(super) async fn set_status_updating(&self, collection_name: &str) -> Result<(), mongodb::error::Error> {
        let status_collection = self.mongo_db.status_collection();
        status_collection
            .update_one(
                doc! {},
                doc! {
                    "$set": { collection_name: "updating" }
                },
            )
            .await?;
        info!("Status set to 'updating' for {} collection", collection_name);
        Ok(())
    }

    pub(super) async fn set_status_ready(&self, collection_name: &str) -> Result<(), mongodb::error::Error> {
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
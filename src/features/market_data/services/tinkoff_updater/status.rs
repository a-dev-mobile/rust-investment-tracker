use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};
use tracing::info;

use super::TinkoffInstrumentsUpdater;

/// Converts RFC3339 timestamp to Moscow time in a human-readable format
fn to_human_readable(rfc3339_time: &str) -> String {
    // Parse the RFC3339 string into a DateTime
    match DateTime::parse_from_rfc3339(rfc3339_time) {
        Ok(utc_time) => {
            // Moscow timezone is UTC+3
            let moscow_offset = FixedOffset::east_opt(3 * 3600).unwrap();
            let moscow_time = utc_time.with_timezone(&moscow_offset);


            moscow_time
                .format("%d.%m.%Y %H:%M:%S")
                .to_string()
        }
        Err(_) => format!("Invalid time format: {}", rfc3339_time),
    }
}

impl TinkoffInstrumentsUpdater {
    // New method to initialize the status collection

    /// Updates a collection's status field
    async fn update_collection_status(
        &self,
        collection_name: &str,
        status: &str,
    ) -> Result<(), mongodb::error::Error> {

        let status_collection = self.mongo_db.status_collection();
        let rfc3339_now = Utc::now().to_rfc3339();
        let moscow_readable = to_human_readable(&rfc3339_now);

        // Define field names for better readability
        let status_field = format!("{}_status", collection_name);
        let update_at_field = format!("{}_update_at", collection_name);

        // Log status change
        info!(
            "Status set to '{}' for {} collection at {}",
            status, collection_name, moscow_readable
        );

        // Update document with flattened structure
        status_collection
            .update_one(
                doc! {},
                doc! {
                    "$set": {
                        status_field: status,
                        update_at_field: moscow_readable,

                    }
                },
            )
            .await?;

        Ok(())
    }

    pub(super) async fn set_status_updating(
        &self,
        collection_name: &str,
    ) -> Result<(), mongodb::error::Error> {
        self.update_collection_status(collection_name, "updating")
            .await
    }

    pub(super) async fn set_status_ready(
        &self,
        collection_name: &str,
    ) -> Result<(), mongodb::error::Error> {
        self.update_collection_status(collection_name, "ready")
            .await
    }
}

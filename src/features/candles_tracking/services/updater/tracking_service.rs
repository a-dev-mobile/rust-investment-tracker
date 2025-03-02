use std::collections::HashSet;
use std::vec;

use crate::db::mongo_db::{Collections, DbNames};
use crate::features::core::models::bond::BondModel;
use crate::features::core::models::etf::EtfModel;
use crate::features::core::models::future::FutureModel;
use crate::features::core::models::instrument::InstrumentEnum;
use crate::features::core::models::share::ShareModel;

use super::CandlesTrackingUpdater;
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use futures::stream::TryStreamExt;
use mongodb::bson::DateTime as BsonDateTime;
use mongodb::bson::{doc, Document};
use mongodb::options::FindOptions;
use mongodb::Collection;
use serde::{Deserialize, Serialize};
use tokio_stream::StreamExt;
use tracing::{error, info};

// Define the struct to match the returned data format
#[derive(Debug, Serialize, Deserialize)]
struct TrackingDocument {
    #[serde(rename = "_id")]
    id: ObjectId,
    figi: String,
}
impl CandlesTrackingUpdater {
    pub(super) async fn update_candles_tracking(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Get the candles tracking collection
        let tracking_collection: Collection<Document> = self
            .mongo_db
            .database(DbNames::MARKET_SERVICES)
            .collection(Collections::CANDLES_TRACKING);

        // Get all documents with enabled tracking
        let enabled_documents = self
            .get_enabled_tracking_documents(&tracking_collection)
            .await?;

        // Process each document and update it
        let updated_count = self
            .process_enabled_documents(enabled_documents, &tracking_collection)
            .await;

        info!(
            "Candles tracking update completed: {} documents updated",
            updated_count
        );

        Ok(())
    }

    /// Получает все документы с включенным отслеживанием
    async fn get_enabled_tracking_documents(
        &self,
        collection: &Collection<Document>,
    ) -> Result<Vec<TrackingDocument>, mongodb::error::Error> {
        let documents: Vec<Document> = collection
            .aggregate([
                doc! {
                    "$match": doc! {
                        "user_setting.enabled": true,
                        "user_setting.figi": doc! {
                            "$exists": true,
                            "$type": "string"
                        }
                    }
                },
                doc! {
                    "$group": doc! {
                        "_id": "$user_setting.figi",
                        "original_id": doc! {
                            "$first": "$_id"
                        }
                    }
                },
                doc! {
                    "$addFields": doc! {
                        "figi": "$_id",
                        "_id": "$original_id"
                    }
                },
                doc! {
                    "$project": doc! {
                        "original_id": 0
                    }
                },
            ])
            .await?
            .try_collect()
            .await?;

        // Convert documents to TrackingDocument structs
        let tracking_documents = documents
            .into_iter()
            .filter_map(|doc| {
                let id = doc.get_object_id("_id").ok()?;
                let figi = doc.get_str("figi").ok()?.to_string();
                Some(TrackingDocument { id, figi })
            })
            .collect();

        Ok(tracking_documents)
    }

    async fn process_enabled_documents(
        &self,
        tracking_documents: Vec<TrackingDocument>,
        collection: &Collection<Document>,
    ) -> i32 {
        let mut updated_count = 0;

        for doc in tracking_documents {
            // Get the original document first
            if let Ok(Some(original_doc)) = collection.find_one(doc! { "_id": doc.id }).await {
                let figi = doc.figi.clone(); // Clone the figi before it's moved
                                             // Try to update the document with instrument data
                match self
                    .update_document_with_instrument_data(&original_doc, doc.figi, collection)
                    .await
                {
                    Ok(_) => updated_count += 1,
                    Err(e) => error!("Failed to update document for FIGI {}: {}", figi, e),
                }
            }
        }

        updated_count
    }

    async fn update_document_with_instrument_data(
        &self,
        doc: &Document,
        figi: String,
        collection: &Collection<Document>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Find instrument by FIGI
        if let Some(instrument) = self.find_instrument_by_figi(&figi).await {
            // Create updated document
            match self
                .create_updated_tracking_document(doc.clone(), instrument)
                .await
            {
                Ok(updated_doc) => {
                    // Update document in collection
                    match collection
                        .replace_one(doc! { "_id": doc.get("_id").unwrap() }, updated_doc)
                        .await
                    {
                        Ok(_) => {
                            info!("Updated tracking data for instrument {}", figi);
                            Ok(())
                        }
                        Err(e) => {
                            error!("Failed to update document for {}: {}", figi, e);
                            Err(Box::new(e))
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to create updated document for {}: {}", figi, e);
                    Err(Box::new(e))
                }
            }
        } else {
            error!("Instrument with FIGI {} not found in any collection", figi);
            Err(format!("Instrument with FIGI {} not found", figi).into())
        }
    }

    /// Поиск инструмента по UID во всех коллекциях
    ///
    /// Последовательно проверяет коллекции акций, облигаций, ETF и фьючерсов
    async fn find_instrument_by_figi(&self, figi: &str) -> Option<InstrumentEnum> {
        // Filter for searching by FIGI
        let filter = doc! { "figi": figi };

        // Check shares collection
        if let Ok(Some(doc)) = self
            .mongo_db
            .shares_collection()
            .find_one(filter.clone())
            .await
        {
            // Convert BSON document to HumanShare
            match bson::from_document::<ShareModel>(doc) {
                Ok(share) => return Some(InstrumentEnum::Share(share)),
                Err(e) => {
                    error!("Failed to deserialize Share document: {}", e);
                    // Continue checking other collections
                }
            }
        }

        // Check bonds collection
        if let Ok(Some(doc)) = self
            .mongo_db
            .bonds_collection()
            .find_one(filter.clone())
            .await
        {
            match bson::from_document::<BondModel>(doc) {
                Ok(bond) => return Some(InstrumentEnum::Bond(bond)),
                Err(e) => {
                    error!("Failed to deserialize Bond document: {}", e);
                }
            }
        }

        // Check ETFs collection
        if let Ok(Some(doc)) = self
            .mongo_db
            .etfs_collection()
            .find_one(filter.clone())
            .await
        {
            match bson::from_document::<EtfModel>(doc) {
                Ok(etf) => return Some(InstrumentEnum::Etf(etf)),
                Err(e) => {
                    error!("Failed to deserialize ETF document: {}", e);
                }
            }
        }

        // Check futures collection
        if let Ok(Some(doc)) = self
            .mongo_db
            .futures_collection()
            .find_one(filter.clone())
            .await
        {
            match bson::from_document::<FutureModel>(doc) {
                Ok(future) => return Some(InstrumentEnum::Future(future)),
                Err(e) => {
                    error!("Failed to deserialize Future document: {}", e);
                }
            }
        }

        None
    }

    // Then you would need to update the create_updated_tracking_document method to work with the enum
    async fn create_updated_tracking_document(
        &self,
        original_doc: Document,
        instrument: InstrumentEnum,
    ) -> Result<Document, mongodb::error::Error> {
        // Extract data based on the instrument type
        let instrument_data = match &instrument {
            InstrumentEnum::Share(share) => {
                doc! {
                    "figi": &share.figi,
                    "ticker": &share.ticker,
                    "name": &share.name,
                    "instrument_type": "share",
                    "first_available_date": share.first_1day_candle_date.as_ref().map(|d| d.timestamp_utc.clone()),
                    "currency": &share.currency,
                    "lot": share.lot,
                }
            }
            InstrumentEnum::Bond(bond) => {
                doc! {
                    "figi": &bond.figi,
                    "ticker": &bond.ticker,
                    "name": &bond.name,
                    "instrument_type": "bond",
                    "first_available_date": bond.first_1day_candle_date.as_ref().map(|d| d.timestamp_utc.clone()),
                    "currency": &bond.currency,
                    "lot": bond.lot,
                }
            }
            InstrumentEnum::Etf(etf) => {
                doc! {
                    "figi": &etf.figi,
                    "ticker": &etf.ticker,
                    "name": &etf.name,
                    "instrument_type": "etf",
                    "first_available_date": etf.first_1day_candle_date.as_ref().map(|d| d.timestamp_utc.clone()),
                    "currency": &etf.currency,
                    "lot": etf.lot,
                }
            }
            InstrumentEnum::Future(future) => {
                doc! {
                    "figi": &future.figi,
                    "ticker": &future.ticker,
                    "name": &future.name,
                    "instrument_type": "future",
                    "first_available_date": future.first_1day_candle_date.as_ref().map(|d| d.timestamp_utc.clone()),
                    "currency": &future.currency,
                    "lot": future.lot,
                }
            }
        };

        // Current time for tracking updates
        let now = Utc::now().to_rfc3339();

        // Create the data document with current time
        let mut data_doc = instrument_data.clone();
        data_doc.insert("last_update", now);

        // Add data to original document
        let mut result = original_doc.clone();
        result.insert("data", data_doc);

        Ok(result)
    }

    // The extract_instrument_data and extract_first_available_date methods are no longer needed
    // since we're working with typed data models now
}

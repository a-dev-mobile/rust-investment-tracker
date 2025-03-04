// src/features/market_candles/tinkoff_shares_1m_historical/status_tracker.rs

use mongodb::bson::{doc, Document};
use chrono::{Duration, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use futures::TryStreamExt;

#[derive(Debug, Serialize, Deserialize)]
pub struct CandleHistoryStatus {
    pub figi: String,
    pub first_candle_date_seconds: i64,
    pub last_candle_date_seconds: i64,
    pub first_candle_date_moscow: String,
    pub last_candle_date_moscow: String,
    pub candle_count: i64,
    pub last_updated: String,
}

impl super::service::HistoricalCandleDataService {
    /// Updates the candle history status for a specific FIGI
    pub async fn update_candle_history_status(&self, figi: &str) -> Result<(), Box<dyn std::error::Error>> {
        info!("Updating candle history status for {}", figi);
        
        // Get the historical collection
        let historical_collection = self.get_historical_collection();
        let status_collection = self.get_status_collection();
        
        // Find the min and max dates for this FIGI
        let pipeline = vec![
            doc! {
                "$match": {
                    "figi": figi
                }
            },
            doc! {
                "$group": {
                    "_id": "$figi",
                    "first_candle_date": { "$min": "$time.seconds" },
                    "last_candle_date": { "$max": "$time.seconds" },
                    "candle_count": { "$sum": 1 }
                }
            }
        ];
        
        let mut cursor = historical_collection.aggregate(pipeline).await?;
        let mut documents = Vec::new();
        
        while let Some(doc) = cursor.try_next().await? {
            documents.push(doc);
        }
        
        if documents.is_empty() {
            info!("No candles found for {}", figi);
            return Ok(());
        }
        
        if let Some(doc) = documents.first() {
            // Extract data from aggregation result with better error handling
            let first_candle_date_seconds = match doc.get("first_candle_date") {
                Some(bson) => {
                    if let Some(val) = bson.as_i64() {
                        val
                    } else if let Some(val) = bson.as_i32() {
                        val as i64
                    } else {
                        error!("first_candle_date has unexpected type: {:?}", bson);
                        return Err(format!("first_candle_date has unexpected type: {:?}", bson).into());
                    }
                },
                None => {
                    error!("first_candle_date field not found in aggregation result");
                    return Err("first_candle_date field not found".into());
                }
            };
            
            let last_candle_date_seconds = match doc.get("last_candle_date") {
                Some(bson) => {
                    if let Some(val) = bson.as_i64() {
                        val
                    } else if let Some(val) = bson.as_i32() {
                        val as i64
                    } else {
                        error!("last_candle_date has unexpected type: {:?}", bson);
                        return Err(format!("last_candle_date has unexpected type: {:?}", bson).into());
                    }
                },
                None => {
                    error!("last_candle_date field not found in aggregation result");
                    return Err("last_candle_date field not found".into());
                }
            };
            
            let candle_count = match doc.get("candle_count") {
                Some(bson) => {
                    if let Some(val) = bson.as_i64() {
                        val
                    } else if let Some(val) = bson.as_i32() {
                        val as i64
                    } else if let Some(val) = bson.as_f64() {
                        val as i64
                    } else {
                        // Добавляем дополнительную диагностику
                        error!("candle_count has unexpected type: {:?}", bson);
                        info!("Full document: {:?}", doc);
                        return Err(format!("candle_count has unexpected type: {:?}", bson).into());
                    }
                },
                None => {
                    error!("candle_count field not found in aggregation result");
                    return Err("candle_count field not found".into());
                }
            };
            
            // Convert to UTC datetime
            let first_utc = Utc.timestamp_opt(first_candle_date_seconds, 0).unwrap();
            let last_utc = Utc.timestamp_opt(last_candle_date_seconds, 0).unwrap();
            
            // Convert to Moscow time (UTC+3)
            let first_moscow = first_utc + Duration::hours(3);
            let last_moscow = last_utc + Duration::hours(3);
            
            // Format as human-readable string
            let first_moscow_str = first_moscow.format("%Y-%m-%d %H:%M:%S").to_string();
            let last_moscow_str = last_moscow.format("%Y-%m-%d %H:%M:%S").to_string();
            
            // Current time
            let now = Utc::now().to_rfc3339();
            
            // Create status document
            let status = CandleHistoryStatus {
                figi: figi.to_string(),
                first_candle_date_seconds,
                last_candle_date_seconds,
                first_candle_date_moscow: first_moscow_str.clone(),
                last_candle_date_moscow: last_moscow_str.clone(),
                candle_count,
                last_updated: now,
            };
            
            // Convert to BSON document
            let status_doc = mongodb::bson::to_document(&status)?;
            
            // Upsert into status collection
            let options = mongodb::options::UpdateOptions::builder().upsert(true).build();
            status_collection
                .update_one(
                    doc! { "figi": figi },
                    doc! { "$set": status_doc },
                 
                ).with_options(   options)
                .await?;
            
            info!(
                "Updated candle history status for {}: {} candles from {} to {}",
                figi, candle_count, first_moscow_str, last_moscow_str
            );
        }
        
        Ok(())
    }
    
    /// Get the existing candle history status for a FIGI
    pub async fn get_candle_history_status(&self, figi: &str) -> Option<CandleHistoryStatus> {
        let status_collection = self.get_status_collection();
        
        match status_collection.find_one(doc! { "figi": figi }).await {
            Ok(Some(doc)) => {
                match mongodb::bson::from_document::<CandleHistoryStatus>(doc) {
                    Ok(status) => Some(status),
                    Err(e) => {
                        error!("Failed to deserialize candle history status for {}: {}", figi, e);
                        None
                    }
                }
            }
            Ok(None) => None,
            Err(e) => {
                error!("Failed to query candle history status for {}: {}", figi, e);
                None
            }
        }
    }
    
    /// Check if we need to fetch historical data for a specific FIGI and date range
    pub async fn check_historical_data_needed(
        &self,
        figi: &str,
        start_date: chrono::DateTime<Utc>,
        end_date: chrono::DateTime<Utc>,
    ) -> bool {
    if let Some(status) = self.get_candle_history_status(figi).await {
        // For historical data that never changes at the beginning:
        // If we already have data starting from or before the requested start date,
        // we only need to fetch if the end date is beyond what we have

        // Check if we already have data that covers the start date
        let have_start_covered = status.first_candle_date_seconds <= start_date.timestamp();
        
        // Check if we need to extend to a later end date
        let need_to_extend_end = end_date.timestamp() > status.last_candle_date_seconds;
        
        if have_start_covered && !need_to_extend_end {
            // We already have all the data we need for this range
            info!(
                "Skipping historical data fetch for {} - already have data from {} to {} which covers requested period ({} to {})",
                figi, 
                status.first_candle_date_moscow, 
                status.last_candle_date_moscow,
                start_date.format("%Y-%m-%d %H:%M:%S"),
                end_date.format("%Y-%m-%d %H:%M:%S")
            );
            return false;
        }
        
        // Log the reason we're fetching
        if !have_start_covered {
            info!(
                "Need to fetch earlier data for {}: current earliest is {}, requested {}",
                figi,
                status.first_candle_date_moscow,
                start_date.format("%Y-%m-%d %H:%M:%S")
            );
        }
        
        if need_to_extend_end {
            info!(
                "Need to fetch more recent data for {}: current latest is {}, requested {}",
                figi,
                status.last_candle_date_moscow,
                end_date.format("%Y-%m-%d %H:%M:%S")
            );
        }
        
        return !have_start_covered || need_to_extend_end;
    }
    
    // No status found, so we need to fetch data
    info!(
        "No existing history status for {}, need to fetch full range",
        figi
    );
        true
    }
    
    // Helper functions to avoid direct access to private fields
    fn get_historical_collection(&self) -> mongodb::Collection<Document> {
        self.mongo_db.get_historical_collection()
    }
    
    fn get_status_collection(&self) -> mongodb::Collection<Document> {
        self.mongo_db.market_candles_status_collection()
    }
}
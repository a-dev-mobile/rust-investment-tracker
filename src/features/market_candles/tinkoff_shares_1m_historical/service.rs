use crate::{

    env_config::models::app_setting::AppSettings, features::db::MongoDb, gen::tinkoff_public_invest_api_contract_v1::{
        CandleInterval, GetCandlesRequest, HistoricCandle,
    }, services::tinkoff::client_grpc::TinkoffClient
};

use chrono::{Duration, TimeZone, Utc};
use futures::stream::TryStreamExt;
use mongodb::bson::{doc, Document};
use mongodb::options::{FindOptions, IndexOptions};
use prost_types::Timestamp;
use std::sync::Arc;
use tracing::{error, info, warn};

pub struct HistoricalCandleDataService {
    pub(crate) client: Arc<TinkoffClient>,
    pub(crate) mongo_db: Arc<MongoDb>,
    pub(crate) settings: Arc<AppSettings>,
}

impl HistoricalCandleDataService {
    pub fn new(
        client: Arc<TinkoffClient>,
        mongo_db: Arc<MongoDb>,
        settings: Arc<AppSettings>,
    ) -> Self {
        Self {
            client,
            mongo_db,
            settings,
        }
    }

    async fn initialize_status_collection(&self) {
        info!("Initializing historical candle status collection");
        
        let status_collection = self.mongo_db.market_candles_status_collection();
        
        // Проверяем, есть ли документы в коллекции
        let count = match status_collection.count_documents(doc! {}).await {
            Ok(count) => count,
            Err(e) => {
                error!("Failed to count documents in status collection: {}", e);
                0
            }
        };
        
        if count == 0 {
            info!("Status collection is empty, creating initial status document");
            
            // Создаем начальный документ статуса
            let initial_status = doc! {
                "_id": "system_status",
                "initialized": true,
                "last_initialized": chrono::Utc::now().to_rfc3339(),
                "version": "1.0"
            };
            
            // Вставляем начальный документ
            match status_collection.insert_one(initial_status ).await {
                Ok(_) => info!("Initial status document created successfully"),
                Err(e) => error!("Failed to create initial status document: {}", e)
            }
        } else {
            info!("Status collection already contains {} documents", count);
        }
    }
  
    pub async fn start(&self) {
        info!("Starting historical candle data service");

        // Check if service is enabled
        if !self.settings.app_config.historical_candle_data.enabled {
            info!("Historical candle data service is disabled in configuration");
            return;
        }

        // Initialize status collection
        self.initialize_status_collection().await;
        // Initialize indexes on the status collection
        self.ensure_status_collection_indexes().await;

        // Сразу определяем период для запроса на основе max_days_history
        let (start_date, end_date) = self.calculate_fetch_period();

        // Получение всех уникальных FIGI из коллекции tinkoff_shares
        let figis = self.mongo_db.get_unique_figis().await;

        if figis.is_empty() {
            info!("No FIGI found in tinkoff_shares collection");
            return;
        }

        let total_figis = figis.len();
        info!(
            "Found {} unique FIGI for historical data fetch",
            total_figis
        );
        
        // Расчет примерного времени
        let total_requests = total_figis * self.settings.app_config.historical_candle_data.max_days_history as usize;
        let estimated_time_seconds = (total_requests as u64 * self.settings.app_config.historical_candle_data.request_delay_ms) / 1000;
        let estimated_hours = estimated_time_seconds / 3600;
        let estimated_minutes = (estimated_time_seconds % 3600) / 60;
        let estimated_seconds = estimated_time_seconds % 60;
        
        info!(
            "Estimated completion time: ~{:02}:{:02}:{:02} (hh:mm:ss), total requests: {}",
            estimated_hours, estimated_minutes, estimated_seconds, total_requests
        );

        // Process each instrument
        for (idx, figi) in figis.iter().enumerate() {
            // Простой прогресс
            info!(
                "Progress: {}/{}",
                idx + 1, total_figis
            );
            
            info!("Processing historical data for {}", figi);

            // Check if we already have data for this FIGI and date range
            let should_fetch = self.check_historical_data_needed(figi, start_date, end_date).await;
            if !should_fetch && !self.settings.app_config.historical_candle_data.force_update {
                info!("Skipping {} - already have data for the requested period", figi);
                continue;
            }

            info!(
                "Fetching historical data for {} from {} to {}",
                figi,
                start_date.format("%Y-%m-%d %H:%M:%S"),
                end_date.format("%Y-%m-%d %H:%M:%S")
            );

            // Fetch data day by day
            self.fetch_historical_data_by_day(&figi, start_date, end_date)
                .await;
                
            // Update status after fetching
            if let Err(e) = self.update_candle_history_status(figi).await {
                error!("Failed to update candle history status for {}: {}", figi, e);
            }
        }

        info!("Historical candle data service completed");
    }

    async fn ensure_status_collection_indexes(&self) {
        let status_collection = self.mongo_db.market_candles_status_collection();
        
        // Create index on FIGI for quick lookups
        match status_collection
            .create_index(
                mongodb::IndexModel::builder()
                    .keys(doc! { "figi": 1 })
                    .options(IndexOptions::builder().unique(true).build())
                    .build(),
            )
            .await
        {
            Ok(_) => info!("Created FIGI index for candle history status collection"),
            Err(e) => error!("Failed to create FIGI index for status collection: {}", e),
        }
    }

    // Упрощенный метод расчета периода для запроса данных
    fn calculate_fetch_period(&self) -> (chrono::DateTime<Utc>, chrono::DateTime<Utc>) {
        // Конечная дата - вчерашний день (чтобы избежать неполных данных за сегодня)
        let end_date = Utc::now()
            .date()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .with_timezone(&Utc)
            - Duration::days(1);
        
        // Начальная дата - отступаем назад на max_days_history от конечной даты
        let start_date = end_date - Duration::days(
            self.settings
                .app_config
                .historical_candle_data
                .max_days_history as i64,
        );
        
        info!(
            "Setting historical data fetch period to {} days (from {} to {})",
            self.settings.app_config.historical_candle_data.max_days_history,
            start_date.format("%Y-%m-%d"),
            end_date.format("%Y-%m-%d"),
        );
        
        (start_date, end_date)
    }

    async fn fetch_historical_data_by_day(
        &self,
        figi: &str,
    mut start_date: chrono::DateTime<Utc>,
        end_date: chrono::DateTime<Utc>,
    ) {
        let collection = self.mongo_db.get_historical_collection();
    // Check if we already have some data for this FIGI
    if let Some(status) = self.get_candle_history_status(figi).await {
        // If we already have data, we can optimize by only fetching what we're missing
        
        // If our existing data starts earlier than requested start_date, 
        // we can start from the day after our existing latest data
        if status.first_candle_date_seconds <= start_date.timestamp() {
            let existing_end = Utc.timestamp_opt(status.last_candle_date_seconds, 0).unwrap();
            
            // Start from the day after our existing latest data
            start_date = (existing_end + Duration::days(1))
                .date()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .with_timezone(&Utc);
            
            info!(
                "Optimizing fetch for {}: Starting from {} (after existing data end)",
                figi,
                start_date.format("%Y-%m-%d")
            );
            
            // If optimized start_date is already beyond our end_date, we have nothing to fetch
            if start_date >= end_date {
                info!(
                    "No new data needed for {}: already have data up to {}",
                    figi,
                    existing_end.format("%Y-%m-%d")
                );
                return;
            }
        }
    }
        let mut current_date = start_date;

        // Process one day at a time
        while current_date < end_date {
            // Calculate the end of this day
            let day_end =
                (current_date.date().and_hms_opt(23, 59, 59).unwrap()).with_timezone(&Utc);

            // Convert dates to Timestamp for the gRPC request
            let from_ts = Timestamp {
                seconds: current_date.timestamp(),
                nanos: 0,
            };

            let to_ts = Timestamp {
                seconds: day_end.timestamp(),
                nanos: 0,
            };

            // Create the request
            let request = GetCandlesRequest {
                from: Some(from_ts),
                to: Some(to_ts),
                interval: CandleInterval::CandleInterval1Min as i32,
                instrument_id: figi.to_string(),
                #[allow(deprecated)]
                figi: figi.to_string(),
            };

            // Make the gRPC call
            match self.client.create_request(request) {
                Ok(grpc_request) => {
                    let mut market_data_client = self.client.market_data.clone();
                    match market_data_client.get_candles(grpc_request).await {
                        Ok(response) => {
                            let candles_response = response.into_inner();
                            let candle_count = candles_response.candles.len();

                            if candle_count > 0 {
                                // Convert candles to MongoDB documents
                                let mut documents = Vec::with_capacity(candle_count);

                                for candle in candles_response.candles {
                                    let doc = self.historic_candle_to_document(figi, candle);
                                    documents.push(doc);
                                }

                                // Batch insert the documents
                                match collection.insert_many(documents).await {
                                    Ok(result) => {
                                        info!(
                                            "Inserted {} historical candles for {} on {}",
                                            result.inserted_ids.len(),
                                            figi,
                                            current_date.format("%Y-%m-%d")
                                        );
                                    }
                                    Err(e) => {
                                        error!(
                                            "Failed to insert historical candles for {}: {}",
                                            figi, e
                                        );
                                    }
                                }
                            } else {
                                info!(
                                    "No historical candles found for {} on {}",
                                    figi,
                                    current_date.format("%Y-%m-%d")
                                );
                            }
                        }
                        Err(e) => {
                            error!(
                                "Failed to get historical candles for {} on {}: {}",
                                figi,
                                current_date.format("%Y-%m-%d"),
                                e
                            );
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to create request for historical candles: {}", e);
                }
            }

            // Add a delay to avoid rate limiting
            tokio::time::sleep(tokio::time::Duration::from_millis(
                self.settings
                    .app_config
                    .historical_candle_data
                    .request_delay_ms,
            ))
            .await;

            // Move to the next day
            current_date = (current_date + Duration::days(1))
                .date()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .with_timezone(&Utc);
        }
    }

    fn historic_candle_to_document(&self, figi: &str, candle: HistoricCandle) -> Document {
        // Extract seconds and nanos from the candle time
        let seconds = candle.time.as_ref().map_or(0, |t| t.seconds);
        let nanos = candle.time.as_ref().map_or(0, |t| t.nanos);

        // Convert to UTC datetime
        let utc_datetime = Utc.timestamp_opt(seconds, nanos as u32).unwrap();

        // Convert to Moscow time (UTC+3)
        let moscow_datetime = utc_datetime + Duration::hours(3);

        // Format as human-readable string
        let moscow_time_str = moscow_datetime.format("%Y-%m-%d %H:%M:%S").to_string();

        doc! {
            "figi": figi,
            "volume": candle.volume,
            "display_time": moscow_time_str,
            "open": {
                "units": candle.open.as_ref().map_or(0, |q| q.units),
                "nano": candle.open.as_ref().map_or(0, |q| q.nano)
            },
            "high": {
                "units": candle.high.as_ref().map_or(0, |q| q.units),
                "nano": candle.high.as_ref().map_or(0, |q| q.nano)
            },
            "low": {
                "units": candle.low.as_ref().map_or(0, |q| q.units),
                "nano": candle.low.as_ref().map_or(0, |q| q.nano)
            },
            "close": {
                "units": candle.close.as_ref().map_or(0, |q| q.units),
                "nano": candle.close.as_ref().map_or(0, |q| q.nano)
            },
            "time": {
                "seconds": candle.time.as_ref().map_or(0, |t| t.seconds),
                "nanos": candle.time.as_ref().map_or(0, |t| t.nanos)
            },
            // Note: HistoricCandle doesn't have last_trade_ts field
        }
    }
}

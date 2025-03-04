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
    client: Arc<TinkoffClient>,
    mongo_db: Arc<MongoDb>,
    settings: Arc<AppSettings>,
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

    pub async fn start(&self) {
        info!("Starting historical candle data service");

        // Check if service is enabled
        if !self.settings.app_config.historical_candle_data.enabled {
            info!("Historical candle data service is disabled in configuration");
            return;
        }

        // Ensure collection with proper indexes exists
        self.ensure_collection_setup().await;

        // Очистка коллекции перед запуском
        self.clear_historical_collection().await;

        // Сразу определяем период для запроса на основе max_days_history
        let (start_date, end_date) = self.calculate_fetch_period();

        // Получение всех уникальных FIGI из коллекции tinkoff_shares
        let figis = self.mongo_db.get_unique_figis().await;

        if figis.is_empty() {
            info!("No FIGI found in tinkoff_shares collection");
            return;
        }

        info!(
            "Found {} unique FIGI for historical data fetch",
            figis.len()
        );

        // Process each instrument
        for figi in figis {
            info!("Processing historical data for {}", figi);

            info!(
                "Fetching historical data for {} from {} to {}",
                figi,
                start_date.format("%Y-%m-%d %H:%M:%S"),
                end_date.format("%Y-%m-%d %H:%M:%S")
            );

            // Fetch data day by day
            self.fetch_historical_data_by_day(&figi, start_date, end_date)
                .await;
        }

        info!("Historical candle data service completed");
    }

    // Метод для очистки коллекции исторических данных
    async fn clear_historical_collection(&self) {
        info!("Clearing historical candles collection before starting data collection");
        
        let collection = self.mongo_db.get_historical_collection();
        
        match collection.delete_many(doc! {}).await {
            Ok(result) => {
                info!("Deleted {} documents from historical candles collection", result.deleted_count);
            },
            Err(e) => {
                error!("Failed to clear historical candles collection: {}", e);
            }
        }
    }

    async fn ensure_collection_setup(&self) {
        // Используем новое имя коллекции
        let collection = self.mongo_db.get_historical_collection();

        // Create compound index on figi + time.seconds for efficient queries
        let index_options = IndexOptions::builder().unique(false).build();
        let index_model = mongodb::IndexModel::builder()
            .keys(doc! {
                "figi": 1,
                "time.seconds": 1
            })
            .options(index_options)
            .build();

        match collection.create_index(index_model).await {
            Ok(result) => info!(
                "Ensured index {} on historical candles collection",
                result.index_name
            ),
            Err(e) => error!(
                "Failed to create index on historical candles collection: {}",
                e
            ),
        }
    }

    async fn get_last_historical_date(&self, figi: &str) -> chrono::DateTime<Utc> {
        // Используем новое имя коллекции
        let collection = self.mongo_db.get_historical_collection();

        // Query for the most recent record for this figi
        let filter = doc! { "figi": figi };
        let options = FindOptions::builder()
            .sort(doc! { "time.seconds": -1 })
            .limit(1)
            .build();

        match collection.find(filter).with_options(options).await {
            Ok(mut cursor) => {
                if let Ok(Some(doc)) = cursor.try_next().await {
                    if let Ok(time_doc) = doc.get_document("time") {
                        if let (Ok(seconds), Ok(nanos)) =
                            (time_doc.get_i64("seconds"), time_doc.get_i32("nanos"))
                        {
                            return Utc.timestamp_opt(seconds, nanos as u32).unwrap();
                        }
                    }
                }
                // If no records found or couldn't parse, return a default date (30 days ago)
                Utc::now() - Duration::days(30)
            }
            Err(e) => {
                warn!("Failed to query last historical date for {}: {}", figi, e);
                // Return a default date (30 days ago)
                Utc::now() - Duration::days(30)
            }
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
        start_date: chrono::DateTime<Utc>,
        end_date: chrono::DateTime<Utc>,
    ) {
        let collection = self.mongo_db.get_historical_collection();
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
            "moscow_time": moscow_time_str,
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

use chrono::{TimeZone, Utc};
use mongodb::bson::{doc, Document};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{metadata::MetadataValue, Request};
use tracing::{debug, error, info};
use std::collections::HashSet;
use std::sync::Mutex;

use crate::{
    db::MongoDb, env_config::models::app_setting::AppSettings, features::{
        core::models::candle_interval::MyCandleInterval,
        user_config::watchlists::models::DbUserConfigWatchlist,
    }, gen::tinkoff_public_invest_api_contract_v1::{
        market_data_request, market_data_response, Candle, CandleInstrument, InfoInstrument,
        MarketDataRequest, MarketDataResponse, OrderBookInstrument, SubscribeCandlesRequest,
        SubscribeInfoRequest, SubscribeOrderBookRequest,
        SubscribeTradesRequest, TradeInstrument,
    }, services::tinkoff::client_grpc::TinkoffClient,
    db::mongo_db::{Collections, DbNames}
};

pub struct MarketDataStreamer {
    client: Arc<TinkoffClient>,
    settings: Arc<AppSettings>,
    mongo_db: Arc<MongoDb>,
    figi_list: Vec<String>,
    indexed_collections: Mutex<HashSet<String>>, // Для отслеживания коллекций, где индекс уже создан
}

impl MarketDataStreamer {
    pub fn new(
        settings: Arc<AppSettings>,
        client: Arc<TinkoffClient>,
        mongo_db: Arc<MongoDb>,
        watchlists: Vec<DbUserConfigWatchlist>,
    ) -> Self {
        // Extract FIGIs from watchlists
        let figi_list = watchlists
            .into_iter()
            .filter(|watchlist| watchlist.enabled)
            .map(|watchlist| watchlist.figi)
            .collect();

        Self {
            client,
            settings,
            mongo_db,
            figi_list,
            indexed_collections: Mutex::new(HashSet::new()),
        }
    }

    pub async fn start_streaming(&self) {
        info!("Starting market data stream...");
        if !self.settings.app_config.tinkoff_market_data_stream.enabled {
            info!("Streaming is disabled in configuration");
            return;
        }
        info!(
            "Starting stream - timezone: {})",
            self.settings.app_config.tinkoff_market_data_stream.timezone
        );

        if self.figi_list.is_empty() {
            info!("No active instruments found in watchlists");
            return;
        }

        info!("Found {} active instruments to stream", self.figi_list.len());

        // Create subscription request for candles
        let request = self.create_candles_subscription_request();

        // Create channel for streaming request
        let (tx, rx) = mpsc::channel(1);
        let request_stream = ReceiverStream::new(rx);

        // Send request to channel
        if let Err(e) = tx.send(request).await {
            error!("Failed to send request to stream: {}", e);
            return;
        }

        // Create stream
        let mut client = self.client.market_data_stream.clone();
        // Add authentication token to the request
        let mut request = Request::new(request_stream);
        let auth_header_value = MetadataValue::try_from(&format!("Bearer {}", self.client.token))
            .expect("Invalid token format");
        request
            .metadata_mut()
            .insert("authorization", auth_header_value);

        match client.market_data_stream(request).await {
            Ok(streaming_response) => {
                info!("Successfully connected to market data stream");
                let mut stream = streaming_response.into_inner();

                while let Ok(Some(response)) = stream.message().await {
                    self.handle_market_data_response(response).await;
                }

                error!("Market data stream ended unexpectedly");
            }
            Err(e) => {
                error!("Failed to create market data stream: {}", e);
            }
        }
    }

    fn create_candles_subscription_request(&self) -> MarketDataRequest {
        // Use default 1-minute interval for all instruments
        let default_interval = 1; // 1-minute interval

        // Create candle instruments for each FIGI
        let candle_instruments: Vec<CandleInstrument> = self
            .figi_list
            .iter()
            .map(|figi| CandleInstrument {
                instrument_id: figi.clone(),
                interval: default_interval,
                #[allow(deprecated)]
                figi: figi.clone(),
            })
            .collect();

        info!(
            "Created subscription for {} instruments",
            candle_instruments.len()
        );

        MarketDataRequest {
            payload: Some(market_data_request::Payload::SubscribeCandlesRequest(
                SubscribeCandlesRequest {
                    subscription_action: 1, // SUBSCRIPTION_ACTION_SUBSCRIBE
                    instruments: candle_instruments,
                    waiting_close: false,
                },
            )),
        }
    }

    async fn handle_market_data_response(&self, response: MarketDataResponse) {
        match response.payload {
            Some(payload) => match payload {
                market_data_response::Payload::SubscribeCandlesResponse(candles) => {
                    info!("Received candles subscription response: {:?}", candles);
                }
                market_data_response::Payload::Candle(candle) => {
                    debug!("Received candle update for FIGI {}", candle.figi);

                    // Save candle data to MongoDB
                    self.save_candle_to_mongodb(&candle).await;
                }
                _ => {
                    debug!("Received other market data");
                }
            },
            None => {
                error!("Received empty market data response");
            }
        }
    }
    async fn save_candle_to_mongodb(&self, candle: &Candle) {
        // Получаем FIGI для названия коллекции
        let figi = &candle.figi;

        // Получаем коллекцию для 1-минутных свечей
        let collection_name = format!("tinkoff_1m_{}", figi);

        // Конвертируем время свечи в формат DateTime
        let needs_index = {
            let mut indexed_collections = self.indexed_collections.lock().unwrap();
            if !indexed_collections.contains(&collection_name) {
                indexed_collections.insert(collection_name.clone());
                true
            } else {
                false
            }
        };

        // Создаем документ для MongoDB
        if needs_index {
            self.ensure_time_index(&collection_name).await;
        }

        // Получаем коллекцию для данного FIGI
        let collection = self.mongo_db
            .client
            .database(DbNames::MARKET_CANDLES)
            .collection::<Document>(&collection_name);

        // Просто вставляем документ без проверки существования
        let doc = doc! {
            "volume": candle.volume,
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
            "last_trade_ts": candle.last_trade_ts.as_ref().map(|t| doc! {
                "seconds": t.seconds,
                "nanos": t.nanos
            }),
        };
        if let Err(e) = collection.insert_one(doc).await {
            error!("Failed to save candle for {}: {}", figi, e);
        }
    }
    async fn ensure_time_index(&self, collection_name: &str) {
        let collection = self.mongo_db
            .client
            .database(DbNames::MARKET_CANDLES)
            .collection::<Document>(collection_name);
        match collection.create_index(
            mongodb::IndexModel::builder()
                .keys(doc! { "time.seconds": 1 })
                .build(),
            
        ).await {
            Ok(_) => info!("Created time index for collection {}", collection_name),
            Err(e) => error!("Failed to create time index for {}: {}", collection_name, e),
        }
    }
}

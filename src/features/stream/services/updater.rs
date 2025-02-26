// use chrono::{TimeZone, Utc};
// use sqlx::PgPool;
// use std::sync::Arc;
// use tokio::sync::mpsc;
// use tokio_stream::wrappers::ReceiverStream;
// use tonic::{metadata::MetadataValue, Request};
// use tracing::{debug, error, info};
// // use futures::Stream;
// use crate::{
//     env_config::models::app_setting::AppSettings,
//     features::{
//         core::models::candle_interval::MyCandleInterval,
//         stream::models::{candle::MyCandle, watched_instrument::WatchedInstrument},
//     },
//     gen::tinkoff_public_invest_api_contract_v1::{
//         market_data_request, market_data_response, Candle, CandleInstrument, InfoInstrument,
//         MarketDataRequest, MarketDataResponse, OrderBookInstrument, SubscribeCandlesRequest,
//         SubscribeCandlesResponse, SubscribeInfoRequest, SubscribeOrderBookRequest,
//         SubscribeTradesRequest, TradeInstrument,
//     },
//     services::tinkoff::client_grpc::TinkoffClient,
// };

// use super::repository::StreamRepository;

// pub struct MarketDataStreamer {
//     db_pool: Arc<PgPool>,
//     client: Arc<TinkoffClient>,
//     settings: Arc<AppSettings>,
//     repository: StreamRepository,
// }

// impl MarketDataStreamer {
//     pub fn new(
//         db_pool: Arc<PgPool>,
//         settings: Arc<AppSettings>,
//         client: Arc<TinkoffClient>,
//     ) -> Self {
//         let repository = StreamRepository::new(db_pool.clone());

//         Self {
//             db_pool,
//             client,
//             settings,
//             repository,
//         }
//     }

//     pub async fn start_streaming(self) {
//         info!("Starting market data stream...");
//         if !self.settings.app_config.stream_updater.enabled {
//             info!("streaming is disabled in configuration");
//             return;
//         }
//         info!(
//             "Starting stream - timezone: {})",
//             self.settings.app_config.stream_updater.timezone
//         );

//         let active_instruments = self.repository.get_active_instruments().await;

//         if active_instruments.is_empty() {
//             info!("No active instruments found");
//             return;
//         }

//         // Create subscription request for candles
//         let request = self.create_candles_subscription_request(&active_instruments);

//         // Create channel for streaming request
//         let (tx, rx) = mpsc::channel(1);
//         let request_stream = ReceiverStream::new(rx);

//         // Send request to channel
//         if let Err(e) = tx.send(request).await {
//             error!("Failed to send request to stream: {}", e);
//             return;
//         }

//         // Create stream
//         let mut client = self.client.market_data_stream.clone();
//         // Add authentication token to the request
//         let mut request = Request::new(request_stream);
//         let auth_header_value = MetadataValue::try_from(&format!("Bearer {}", self.client.token))
//             .expect("Invalid token format");
//         request
//             .metadata_mut()
//             .insert("authorization", auth_header_value);

//         match client.market_data_stream(request).await {
//             Ok(streaming_response) => {
//                 info!("Successfully connected to market data stream");
//                 let mut stream = streaming_response.into_inner();

//                 while let Ok(Some(response)) = stream.message().await {
//                     self.handle_market_data_response(response);
//                 }

//                 error!("Market data stream ended unexpectedly");
//             }
//             Err(e) => {
//                 error!("Failed to create market data stream: {}", e);
//             }
//         }
//     }

//     fn create_candles_subscription_request(
//         &self,
//         active_instruments: &[WatchedInstrument],
//     ) -> MarketDataRequest {
//         // Group instruments by subscription interval
//         let grouped_instruments: std::collections::HashMap<i32, Vec<&WatchedInstrument>> =
//             active_instruments.iter().fold(
//                 std::collections::HashMap::new(),
//                 |mut acc, instrument| {
//                     acc.entry(instrument.subscription_interval_id)
//                         .or_insert_with(Vec::new)
//                         .push(instrument);
//                     acc
//                 },
//             );

//         // Create candle instruments for each interval group
//         let mut all_candle_instruments = Vec::new();

//         for (interval, instruments) in grouped_instruments {
//             let mut candle_instruments: Vec<CandleInstrument> = instruments
//                 .iter()
//                 .map(|instrument| CandleInstrument {
//                     instrument_id: instrument.figi.clone(),
//                     interval: interval,
//                     #[allow(deprecated)]
//                     figi: instrument.figi.clone(),
//                 })
//                 .collect();

//             all_candle_instruments.append(&mut candle_instruments);
//         }

//         info!(
//             "Created subscription for {} instruments",
//             all_candle_instruments.len()
//         );

//         MarketDataRequest {
//             payload: Some(market_data_request::Payload::SubscribeCandlesRequest(
//                 SubscribeCandlesRequest {
//                     subscription_action: 1, // SUBSCRIPTION_ACTION_SUBSCRIBE
//                     instruments: all_candle_instruments,
//                     waiting_close: false,
//                 },
//             )),
//         }
//     }

//     fn handle_market_data_response(&self, response: MarketDataResponse) {
//         match response.payload {
//             Some(payload) => match payload {
//                 market_data_response::Payload::SubscribeCandlesResponse(candles) => {
//                     info!("Received candles subscription response: {:?}", candles);
//                 }
//                 market_data_response::Payload::Candle(candle) => {
//                     info!("Received candle update: {:?}", candle);
//                     self.handle_candle_update(candle);
//                 }
//                 _ => {
//                     info!("Received other market data: {:?}", payload);
//                 }
//             },
//             None => {
//                 error!("Received empty market data response");
//             }
//         }
//     }

//     fn handle_candle_update(&self, candle: Candle) {
//         let runtime = tokio::runtime::Handle::current();
//         let repository = self.repository.clone();

//         runtime.spawn(async move {
//             // Преобразуем Candle в MyCandle
//             let my_candle = MyCandle::from(candle);
//             repository.save_candle(my_candle).await 
//         });
//     }
// }

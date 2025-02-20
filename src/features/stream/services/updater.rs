use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{metadata::MetadataValue, Request};
use tracing::{error, info};
// use futures::Stream;
use crate::{
    env_config::models::app_setting::AppSettings,
    gen::tinkoff_public_invest_api_contract_v1::{
        market_data_request, CandleInstrument, InfoInstrument, MarketDataRequest,
        MarketDataResponse, OrderBookInstrument, SubscribeCandlesRequest, SubscribeInfoRequest,
        SubscribeOrderBookRequest, SubscribeTradesRequest, TradeInstrument,
    },
    services::tinkoff::client_grpc::TinkoffClient,
};

pub struct MarketDataStreamer {
    client: Arc<TinkoffClient>,
    settings: Arc<AppSettings>,
}

impl MarketDataStreamer {
    pub fn new(settings: Arc<AppSettings>, client: Arc<TinkoffClient>) -> Self {
        Self { client, settings }
    }

    pub async fn start_streaming(self) {
        info!("Starting market data stream...");

        let candle_instrument = CandleInstrument {
            instrument_id: "BBG004730N88".to_string(),
            interval: 1, // SUBSCRIPTION_INTERVAL_ONE_MINUTE
            #[allow(deprecated)]
            figi: "BBG004730N88".to_string(),
        };

        let subscribe_candles = SubscribeCandlesRequest {
            subscription_action: 1, // SUBSCRIPTION_ACTION_SUBSCRIBE
            instruments: vec![candle_instrument],
            waiting_close: false,
        };

        let order_book_instrument = OrderBookInstrument {
            figi: String::new(),
            depth: 20,
            instrument_id: "BBG004730N88".to_string(),
        };

        let subscribe_order_book = SubscribeOrderBookRequest {
            subscription_action: 1,
            instruments: vec![order_book_instrument],
        };

        let trade_instrument = TradeInstrument {
            figi: String::new(),
            instrument_id: "BBG004730N88".to_string(),
        };

        let subscribe_trades = SubscribeTradesRequest {
            subscription_action: 1,
            instruments: vec![trade_instrument],
        };

        let info_instrument = InfoInstrument {
            figi: String::new(),
            instrument_id: "BBG004730N88".to_string(),
        };

        let subscribe_info = SubscribeInfoRequest {
            subscription_action: 1,
            instruments: vec![info_instrument],
        };

        // Создаем канал для стриминга запросов
        let (tx, rx) = mpsc::channel(4); // Увеличиваем буфер для всех запросов
        let request_stream = ReceiverStream::new(rx);

        // Отправляем все запросы в канал
        let requests = vec![
            MarketDataRequest {
                payload: Some(market_data_request::Payload::SubscribeCandlesRequest(
                    subscribe_candles,
                )),
            },
            MarketDataRequest {
                payload: Some(market_data_request::Payload::SubscribeOrderBookRequest(
                    subscribe_order_book,
                )),
            },
            MarketDataRequest {
                payload: Some(market_data_request::Payload::SubscribeTradesRequest(
                    subscribe_trades,
                )),
            },
            MarketDataRequest {
                payload: Some(market_data_request::Payload::SubscribeInfoRequest(
                    subscribe_info,
                )),
            },
        ];

        for request in requests {
            if let Err(e) = tx.send(request).await {
                error!("Failed to send request to stream: {}", e);
                return;
            }
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
        let response = client.market_data_stream(request).await;

        match response {
            Ok(streaming_response) => {
                info!("Successfully connected to market data stream");
                let mut stream = streaming_response.into_inner();

                while let Ok(Some(response)) = stream.message().await {
                    self.handle_market_data_response(response);
                }

                error!("Market data stream ended unexpectedly");
            }
            Err(e) => {
                error!("Failed to create market data stream: {}", e);
            }
        }
    }

    fn handle_market_data_response(&self, response: MarketDataResponse) {
        match response.payload {
            Some(payload) => {
                info!("Received market data: {:?}", payload);
            }
            None => {
                error!("Received empty market data response");
            }
        }
    }
}

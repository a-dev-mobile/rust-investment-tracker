use crate::{
    config::Settings,
    gen::tinkoff_public_invest_api_contract_v1::{
        instruments_service_client::InstrumentsServiceClient,
        market_data_service_client::MarketDataServiceClient,
        operations_service_client::OperationsServiceClient,
        users_service_client::UsersServiceClient, BondsResponse, InstrumentStatus,
        InstrumentsRequest, SharesResponse,
    },
};
use rustls::crypto::aws_lc_rs;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;
use std::{sync::Arc, time::Duration};
use tonic::{
    metadata::MetadataValue,
    transport::{Channel, ClientTlsConfig},
    Request, Status,
};

#[derive(Clone)]
pub struct TinkoffClient {
    instruments: InstrumentsServiceClient<Channel>,
    market_data: MarketDataServiceClient<Channel>,
    operations: OperationsServiceClient<Channel>,
    users: UsersServiceClient<Channel>,
    token: String,
}

impl TinkoffClient {
    /// Создает новый экземпляр клиента с заданными настройками
    pub async fn new(settings: Arc<Settings>) -> Result<Self> {
        // Инициализация криптографического провайдера
        let provider = aws_lc_rs::default_provider();
        provider.install_default();

        // Настройка TLS
        let tls_config = ClientTlsConfig::new()
            .domain_name(&settings.grpc_tinkoff.domain)
            .with_enabled_roots();

        // Создание канала с настроенной конфигурацией
        let channel = Channel::from_shared(settings.grpc_tinkoff.base_url.clone().into_bytes())
            .expect("Invalid URI format")
            .tls_config(tls_config)
            .expect("TLS configuration failed")
            .tcp_keepalive(Some(Duration::from_secs(settings.grpc_cient.keepalive)))
            .timeout(Duration::from_secs(settings.grpc_cient.timeout))
            .connect()
            .await
            .expect("Failed to connect to gRPC server");

        Ok(Self {
            instruments: InstrumentsServiceClient::new(channel.clone()),
            market_data: MarketDataServiceClient::new(channel.clone()),
            operations: OperationsServiceClient::new(channel.clone()),
            users: UsersServiceClient::new(channel.clone()),
            token: settings.grpc_tinkoff.token.clone(),
        })
    }

    /// Создает новый gRPC запрос с добавлением токена авторизации
   pub fn create_request<T>(&self, request: T) -> Result<Request<T>> {
        let mut request = Request::new(request);
        let auth_header_value = MetadataValue::try_from(&format!("Bearer {}", self.token))
            .expect("Invalid token format");
        request
            .metadata_mut()
            .insert("authorization", auth_header_value);
        Ok(request)
    }

    // /// Получает список акций
    // pub async fn get_shares(&mut self) -> Result<SharesResponse>  {
    //     let request = self.create_request(InstrumentsRequest {
    //         instrument_status: InstrumentStatus::All as i32,
    //     }).expect("Failed to create request");

    //     self.instruments
    //         .shares(request)
    //         .await
    //         .map(|response| response.into_inner())
    //         .map_err(|e| AppError::ExternalServiceError(format!("Failed to get shares: {}", e)))
    // }

    // Получает список облигаций
    // pub async fn get_bonds(&mut self) -> Result<BondsResponse> {
    //     let request = self.create_request(InstrumentsRequest {
    //         instrument_status: InstrumentStatus::Base as i32,
    //     })?;

    //     self.instruments
    //         .bonds(request)
    //         .await
    //         .map(|response| response.into_inner())
    //         .map_err(|e| AppError::ExternalServiceError(format!("Failed to get bonds: {}", e)))
    // }

    // // Геттеры для доступа к отдельным сервисам
    // pub fn instruments(&mut self) -> &mut InstrumentsServiceClient<Channel> {
    //     &mut self.instruments
    // }

    // pub fn market_data(&mut self) -> &mut MarketDataServiceClient<Channel> {
    //     &mut self.market_data
    // }

    // pub fn operations(&mut self) -> &mut OperationsServiceClient<Channel> {
    //     &mut self.operations
    // }

    // pub fn users(&mut self) -> &mut UsersServiceClient<Channel> {
    //     &mut self.users
    // }
}




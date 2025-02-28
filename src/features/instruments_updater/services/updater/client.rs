use crate::gen::tinkoff_public_invest_api_contract_v1::{
    BondsResponse, EtfsResponse, FuturesResponse, InstrumentStatus, InstrumentsRequest, SharesResponse
};
use tracing::info;

use super::TinkoffInstrumentsUpdater;

impl TinkoffInstrumentsUpdater {
    pub(super) async fn fetch_shares(&self) -> SharesResponse {
        let request = self
            .client
            .create_request(InstrumentsRequest {
                instrument_status: InstrumentStatus::All as i32,
            })
            .expect("Failed to create request");

        let mut instruments_client = self.client.instruments.clone();
        let response = instruments_client
            .shares(request)
            .await
            .map(|response| response.into_inner())
            .expect("Failed to get shares");

        response
    }

    pub(super) async fn fetch_bonds(&self) -> BondsResponse {
        let request = self
            .client
            .create_request(InstrumentsRequest {
                instrument_status: InstrumentStatus::All as i32,
            })
            .expect("Failed to create request");

        let mut instruments_client = self.client.instruments.clone();
        let response = instruments_client
            .bonds(request)
            .await
            .map(|response| response.into_inner())
            .expect("Failed to get bonds");

        response
    }
    pub(super) async fn fetch_etfs(&self) -> EtfsResponse {
        let request = self
            .client
            .create_request(InstrumentsRequest {
                instrument_status: InstrumentStatus::All as i32,
            })
            .expect("Failed to create request");

        let mut instruments_client = self.client.instruments.clone();
        let response = instruments_client
            .etfs(request)
            .await
            .map(|response| response.into_inner())
            .expect("Failed to get etfs");

        response
    }
    pub(super) async fn fetch_futures(&self) -> FuturesResponse {
        let request = self
            .client
            .create_request(InstrumentsRequest {
                instrument_status: InstrumentStatus::All as i32,
            })
            .expect("Failed to create request");
    
        let mut instruments_client = self.client.instruments.clone();
        let response = instruments_client
            .futures(request)
            .await
            .map(|response| response.into_inner())
            .expect("Failed to get futures");
    
        response
    }
}

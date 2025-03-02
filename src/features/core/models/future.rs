use serde::{Deserialize, Serialize};

use crate::features::core::models::{
    quotation::QuotationModel, real_exchange::RealExchangeModel, time_stamp::TimestampModel,
    trading_status::TradingStatusModel,
};
use crate::gen::tinkoff_public_invest_api_contract_v1::Future;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FutureModel {
    // Basic fields
    pub figi: String,
    pub ticker: String,
    pub class_code: String,
    pub lot: i32,
    pub currency: String,
    pub uid: String,
    pub position_uid: String,
    pub name: String,
    pub exchange: String,

    // Flag fields
    pub short_enabled_flag: bool,
    pub otc_flag: bool,
    pub buy_available_flag: bool,
    pub sell_available_flag: bool,
    pub for_iis_flag: bool,
    pub for_qual_investor_flag: bool,
    pub weekend_flag: bool,
    pub blocked_tca_flag: bool,
    pub api_trade_available_flag: bool,

    // Enhanced enum fields
    pub trading_status: TradingStatusModel,
    pub real_exchange: RealExchangeModel,

    // Future specific fields
    pub first_trade_date: Option<TimestampModel>,
    pub last_trade_date: Option<TimestampModel>,
    pub futures_type: String,
    pub asset_type: String,
    pub basic_asset: String,
    pub basic_asset_size: Option<QuotationModel>,
    pub country_of_risk: String,
    pub country_of_risk_name: String,
    pub sector: String,
    pub expiration_date: Option<TimestampModel>,
    pub basic_asset_position_uid: String,

    // Optional fields with enhanced types
    pub klong: Option<QuotationModel>,
    pub kshort: Option<QuotationModel>,
    pub dlong: Option<QuotationModel>,
    pub dshort: Option<QuotationModel>,
    pub dlong_min: Option<QuotationModel>,
    pub dshort_min: Option<QuotationModel>,
    pub min_price_increment: Option<QuotationModel>,
    pub first_1min_candle_date: Option<TimestampModel>,
    pub first_1day_candle_date: Option<TimestampModel>,
}

impl From<&Future> for FutureModel {
    fn from(future: &Future) -> Self {
        FutureModel {
            figi: future.figi.clone(),
            ticker: future.ticker.clone(),
            class_code: future.class_code.clone(),
            lot: future.lot,
            currency: future.currency.clone(),
            uid: future.uid.clone(),
            position_uid: future.position_uid.clone(),
            name: future.name.clone(),
            exchange: future.exchange.clone(),

            short_enabled_flag: future.short_enabled_flag,
            otc_flag: future.otc_flag,
            buy_available_flag: future.buy_available_flag,
            sell_available_flag: future.sell_available_flag,
            for_iis_flag: future.for_iis_flag,
            for_qual_investor_flag: future.for_qual_investor_flag,
            weekend_flag: future.weekend_flag,
            blocked_tca_flag: future.blocked_tca_flag,
            api_trade_available_flag: future.api_trade_available_flag,

            trading_status: TradingStatusModel::from(future.trading_status),
            real_exchange: RealExchangeModel::from(future.real_exchange),

            first_trade_date: future.first_trade_date.as_ref().map(TimestampModel::from),
            last_trade_date: future.last_trade_date.as_ref().map(TimestampModel::from),
            futures_type: future.futures_type.clone(),
            asset_type: future.asset_type.clone(),
            basic_asset: future.basic_asset.clone(),
            basic_asset_size: future.basic_asset_size.as_ref().map(QuotationModel::from),
            country_of_risk: future.country_of_risk.clone(),
            country_of_risk_name: future.country_of_risk_name.clone(),
            sector: future.sector.clone(),
            expiration_date: future.expiration_date.as_ref().map(TimestampModel::from),
            basic_asset_position_uid: future.basic_asset_position_uid.clone(),

            klong: future.klong.as_ref().map(QuotationModel::from),
            kshort: future.kshort.as_ref().map(QuotationModel::from),
            dlong: future.dlong.as_ref().map(QuotationModel::from),
            dshort: future.dshort.as_ref().map(QuotationModel::from),
            dlong_min: future.dlong_min.as_ref().map(QuotationModel::from),
            dshort_min: future.dshort_min.as_ref().map(QuotationModel::from),
            min_price_increment: future
                .min_price_increment
                .as_ref()
                .map(QuotationModel::from),
            first_1min_candle_date: future
                .first_1min_candle_date
                .as_ref()
                .map(TimestampModel::from),
            first_1day_candle_date: future
                .first_1day_candle_date
                .as_ref()
                .map(TimestampModel::from),
        }
    }
}

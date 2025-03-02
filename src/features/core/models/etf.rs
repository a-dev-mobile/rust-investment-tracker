use serde::{Deserialize, Serialize};

use crate::features::core::models::{
    money_value::MoneyValueModel, quotation::QuotationModel, real_exchange::RealExchangeModel,
    time_stamp::TimestampModel, trading_status::TradingStatusModel,
};
use crate::gen::tinkoff_public_invest_api_contract_v1::Etf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtfModel {
    // Basic fields
    pub figi: String,
    pub ticker: String,
    pub class_code: String,
    pub isin: String,
    pub uid: String,
    pub position_uid: String,
    pub name: String,
    pub lot: i32,
    pub currency: String,
    pub exchange: String,

    // Flag fields
    pub short_enabled_flag: bool,
    pub otc_flag: bool,
    pub buy_available_flag: bool,
    pub sell_available_flag: bool,
    pub api_trade_available_flag: bool,
    pub for_iis_flag: bool,
    pub for_qual_investor_flag: bool,
    pub weekend_flag: bool,
    pub blocked_tca_flag: bool,
    pub liquidity_flag: bool,

    // Enhanced enum fields
    pub trading_status: TradingStatusModel,
    pub real_exchange: RealExchangeModel,

    // ETF specific fields
    pub fixed_commission: Option<QuotationModel>,
    pub focus_type: String,
    pub released_date: Option<TimestampModel>,
    pub num_shares: Option<QuotationModel>,
    pub country_of_risk: String,
    pub country_of_risk_name: String,
    pub sector: String,
    pub rebalancing_freq: String,

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

impl From<&Etf> for EtfModel {
    fn from(etf: &Etf) -> Self {
        EtfModel {
            figi: etf.figi.clone(),
            ticker: etf.ticker.clone(),
            class_code: etf.class_code.clone(),
            isin: etf.isin.clone(),
            uid: etf.uid.clone(),
            position_uid: etf.position_uid.clone(),
            name: etf.name.clone(),
            lot: etf.lot,
            currency: etf.currency.clone(),
            exchange: etf.exchange.clone(),

            short_enabled_flag: etf.short_enabled_flag,
            otc_flag: etf.otc_flag,
            buy_available_flag: etf.buy_available_flag,
            sell_available_flag: etf.sell_available_flag,
            api_trade_available_flag: etf.api_trade_available_flag,
            for_iis_flag: etf.for_iis_flag,
            for_qual_investor_flag: etf.for_qual_investor_flag,
            weekend_flag: etf.weekend_flag,
            blocked_tca_flag: etf.blocked_tca_flag,
            liquidity_flag: etf.liquidity_flag,

            trading_status: TradingStatusModel::from(etf.trading_status),
            real_exchange: RealExchangeModel::from(etf.real_exchange),

            fixed_commission: etf.fixed_commission.as_ref().map(QuotationModel::from),
            focus_type: etf.focus_type.clone(),
            released_date: etf.released_date.as_ref().map(TimestampModel::from),
            num_shares: etf.num_shares.as_ref().map(QuotationModel::from),
            country_of_risk: etf.country_of_risk.clone(),
            country_of_risk_name: etf.country_of_risk_name.clone(),
            sector: etf.sector.clone(),
            rebalancing_freq: etf.rebalancing_freq.clone(),

            klong: etf.klong.as_ref().map(QuotationModel::from),
            kshort: etf.kshort.as_ref().map(QuotationModel::from),
            dlong: etf.dlong.as_ref().map(QuotationModel::from),
            dshort: etf.dshort.as_ref().map(QuotationModel::from),
            dlong_min: etf.dlong_min.as_ref().map(QuotationModel::from),
            dshort_min: etf.dshort_min.as_ref().map(QuotationModel::from),
            min_price_increment: etf.min_price_increment.as_ref().map(QuotationModel::from),
            first_1min_candle_date: etf
                .first_1min_candle_date
                .as_ref()
                .map(TimestampModel::from),
            first_1day_candle_date: etf
                .first_1day_candle_date
                .as_ref()
                .map(TimestampModel::from),
        }
    }
}

use serde::{Deserialize, Serialize};

use crate::features::core::models::{
    money_value::HumanMoneyValue, quotation::HumanQuotation, real_exchange::HumanRealExchange,
    time_stamp::HumanTimestamp, trading_status::HumanTradingStatus,
};
use crate::gen::tinkoff_public_invest_api_contract_v1::Etf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanEtf {
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
    pub trading_status: HumanTradingStatus,
    pub real_exchange: HumanRealExchange,

    // ETF specific fields
    pub fixed_commission: Option<HumanQuotation>,
    pub focus_type: String,
    pub released_date: Option<HumanTimestamp>,
    pub num_shares: Option<HumanQuotation>,
    pub country_of_risk: String,
    pub country_of_risk_name: String,
    pub sector: String,
    pub rebalancing_freq: String,

    // Optional fields with enhanced types
    pub klong: Option<HumanQuotation>,
    pub kshort: Option<HumanQuotation>,
    pub dlong: Option<HumanQuotation>,
    pub dshort: Option<HumanQuotation>,
    pub dlong_min: Option<HumanQuotation>,
    pub dshort_min: Option<HumanQuotation>,
    pub min_price_increment: Option<HumanQuotation>,
    pub first_1min_candle_date: Option<HumanTimestamp>,
    pub first_1day_candle_date: Option<HumanTimestamp>,
}

impl From<&Etf> for HumanEtf {
    fn from(etf: &Etf) -> Self {
        HumanEtf {
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

            trading_status: HumanTradingStatus::from(etf.trading_status),
            real_exchange: HumanRealExchange::from(etf.real_exchange),

            fixed_commission: etf.fixed_commission.as_ref().map(HumanQuotation::from),
            focus_type: etf.focus_type.clone(),
            released_date: etf.released_date.as_ref().map(HumanTimestamp::from),
            num_shares: etf.num_shares.as_ref().map(HumanQuotation::from),
            country_of_risk: etf.country_of_risk.clone(),
            country_of_risk_name: etf.country_of_risk_name.clone(),
            sector: etf.sector.clone(),
            rebalancing_freq: etf.rebalancing_freq.clone(),

            klong: etf.klong.as_ref().map(HumanQuotation::from),
            kshort: etf.kshort.as_ref().map(HumanQuotation::from),
            dlong: etf.dlong.as_ref().map(HumanQuotation::from),
            dshort: etf.dshort.as_ref().map(HumanQuotation::from),
            dlong_min: etf.dlong_min.as_ref().map(HumanQuotation::from),
            dshort_min: etf.dshort_min.as_ref().map(HumanQuotation::from),
            min_price_increment: etf.min_price_increment.as_ref().map(HumanQuotation::from),
            first_1min_candle_date: etf.first_1min_candle_date.as_ref().map(HumanTimestamp::from),
            first_1day_candle_date: etf.first_1day_candle_date.as_ref().map(HumanTimestamp::from),
        }
    }
}
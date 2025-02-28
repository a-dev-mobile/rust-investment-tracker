use serde::{Deserialize, Serialize};

use crate::gen::tinkoff_public_invest_api_contract_v1::Share;

use super::{money_value::HumanMoneyValue, quotation::HumanQuotation, real_exchange::HumanRealExchange, share_type::HumanShareType, time_stamp::HumanTimestamp, trading_status::HumanTradingStatus};
/// Complete human-readable share model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanShare {
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
    pub div_yield_flag: bool,
    pub api_trade_available_flag: bool,
    pub for_iis_flag: bool,
    pub for_qual_investor_flag: bool,
    pub weekend_flag: bool,
    pub blocked_tca_flag: bool,
    pub liquidity_flag: bool,

    // Enhanced enum fields
    pub trading_status: HumanTradingStatus,
    pub share_type: HumanShareType,
    pub real_exchange: HumanRealExchange,

    // Other fields
    pub issue_size: i64,
    pub issue_size_plan: i64,
    pub country_of_risk: String,
    pub country_of_risk_name: String,
    pub sector: String,

    // Optional fields with enhanced types
    pub klong: Option<HumanQuotation>,
    pub kshort: Option<HumanQuotation>,
    pub dlong: Option<HumanQuotation>,
    pub dshort: Option<HumanQuotation>,
    pub dlong_min: Option<HumanQuotation>,
    pub dshort_min: Option<HumanQuotation>,
    pub min_price_increment: Option<HumanQuotation>,
    pub nominal: Option<HumanMoneyValue>,
    pub ipo_date: Option<HumanTimestamp>,
    pub first_1min_candle_date: Option<HumanTimestamp>,
    pub first_1day_candle_date: Option<HumanTimestamp>,
}

impl From<&Share> for HumanShare {
    fn from(share: &Share) -> Self {
        HumanShare {
            figi: share.figi.clone(),
            ticker: share.ticker.clone(),
            class_code: share.class_code.clone(),
            isin: share.isin.clone(),
            uid: share.uid.clone(),
            position_uid: share.position_uid.clone(),
            name: share.name.clone(),
            lot: share.lot,
            currency: share.currency.clone(),
            exchange: share.exchange.clone(),

            short_enabled_flag: share.short_enabled_flag,
            otc_flag: share.otc_flag,
            buy_available_flag: share.buy_available_flag,
            sell_available_flag: share.sell_available_flag,
            div_yield_flag: share.div_yield_flag,
            api_trade_available_flag: share.api_trade_available_flag,
            for_iis_flag: share.for_iis_flag,
            for_qual_investor_flag: share.for_qual_investor_flag,
            weekend_flag: share.weekend_flag,
            blocked_tca_flag: share.blocked_tca_flag,
            liquidity_flag: share.liquidity_flag,

            trading_status: HumanTradingStatus::from(share.trading_status),
            share_type: HumanShareType::from(share.share_type),
            real_exchange: HumanRealExchange::from(share.real_exchange),

            issue_size: share.issue_size,
            issue_size_plan: share.issue_size_plan,
            country_of_risk: share.country_of_risk.clone(),
            country_of_risk_name: share.country_of_risk_name.clone(),
            sector: share.sector.clone(),

            klong: share.klong.as_ref().map(HumanQuotation::from),
            kshort: share.kshort.as_ref().map(HumanQuotation::from),
            dlong: share.dlong.as_ref().map(HumanQuotation::from),
            dshort: share.dshort.as_ref().map(HumanQuotation::from),
            dlong_min: share.dlong_min.as_ref().map(HumanQuotation::from),
            dshort_min: share.dshort_min.as_ref().map(HumanQuotation::from),
            min_price_increment: share.min_price_increment.as_ref().map(HumanQuotation::from),
            nominal: share.nominal.as_ref().map(HumanMoneyValue::from),
            ipo_date: share.ipo_date.as_ref().map(HumanTimestamp::from),
            first_1min_candle_date: share
                .first_1min_candle_date
                .as_ref()
                .map(HumanTimestamp::from),
            first_1day_candle_date: share
                .first_1day_candle_date
                .as_ref()
                .map(HumanTimestamp::from),
        }
    }
}

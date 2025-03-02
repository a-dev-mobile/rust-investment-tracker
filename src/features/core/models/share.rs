use serde::{Deserialize, Serialize};

use crate::gen::tinkoff_public_invest_api_contract_v1::Share;

use super::{
    money_value::MoneyValueModel, quotation::QuotationModel, real_exchange::RealExchangeModel,
    share_type::ShareTypeModel, time_stamp::TimestampModel, trading_status::TradingStatusModel,
};
/// Complete human-readable share model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareModel {
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
    pub trading_status: TradingStatusModel,
    pub share_type: ShareTypeModel,
    pub real_exchange: RealExchangeModel,

    // Other fields
    pub issue_size: i64,
    pub issue_size_plan: i64,
    pub country_of_risk: String,
    pub country_of_risk_name: String,
    pub sector: String,

    // Optional fields with enhanced types
    pub klong: Option<QuotationModel>,
    pub kshort: Option<QuotationModel>,
    pub dlong: Option<QuotationModel>,
    pub dshort: Option<QuotationModel>,
    pub dlong_min: Option<QuotationModel>,
    pub dshort_min: Option<QuotationModel>,
    pub min_price_increment: Option<QuotationModel>,
    pub nominal: Option<MoneyValueModel>,
    pub ipo_date: Option<TimestampModel>,
    pub first_1min_candle_date: Option<TimestampModel>,
    pub first_1day_candle_date: Option<TimestampModel>,
}

impl From<&Share> for ShareModel {
    fn from(share: &Share) -> Self {
        ShareModel {
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

            trading_status: TradingStatusModel::from(share.trading_status),
            share_type: ShareTypeModel::from(share.share_type),
            real_exchange: RealExchangeModel::from(share.real_exchange),

            issue_size: share.issue_size,
            issue_size_plan: share.issue_size_plan,
            country_of_risk: share.country_of_risk.clone(),
            country_of_risk_name: share.country_of_risk_name.clone(),
            sector: share.sector.clone(),

            klong: share.klong.as_ref().map(QuotationModel::from),
            kshort: share.kshort.as_ref().map(QuotationModel::from),
            dlong: share.dlong.as_ref().map(QuotationModel::from),
            dshort: share.dshort.as_ref().map(QuotationModel::from),
            dlong_min: share.dlong_min.as_ref().map(QuotationModel::from),
            dshort_min: share.dshort_min.as_ref().map(QuotationModel::from),
            min_price_increment: share.min_price_increment.as_ref().map(QuotationModel::from),
            nominal: share.nominal.as_ref().map(MoneyValueModel::from),
            ipo_date: share.ipo_date.as_ref().map(TimestampModel::from),
            first_1min_candle_date: share
                .first_1min_candle_date
                .as_ref()
                .map(TimestampModel::from),
            first_1day_candle_date: share
                .first_1day_candle_date
                .as_ref()
                .map(TimestampModel::from),
        }
    }
}

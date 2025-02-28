use serde::{Deserialize, Serialize};

use crate::features::core::models::{
    money_value::HumanMoneyValue, real_exchange::HumanRealExchange, time_stamp::HumanTimestamp,
    trading_status::HumanTradingStatus,
};
use crate::gen::tinkoff_public_invest_api_contract_v1::Bond;

use super::quotation::HumanQuotation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanBond {
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
    pub floating_coupon_flag: bool,
    pub perpetual_flag: bool,
    pub amortization_flag: bool,
    pub api_trade_available_flag: bool,
    pub for_iis_flag: bool,
    pub for_qual_investor_flag: bool,
    pub weekend_flag: bool,
    pub blocked_tca_flag: bool,
    pub subordinated_flag: bool,
    pub liquidity_flag: bool,

    // Enhanced enum fields
    pub trading_status: HumanTradingStatus,
    pub real_exchange: HumanRealExchange,

    // Specific bond fields
    pub issue_size: i64,
    pub issue_size_plan: i64,
    pub nominal: Option<HumanMoneyValue>,
    pub initial_nominal: Option<HumanMoneyValue>,
    pub placement_price: Option<HumanMoneyValue>,
    pub aci_value: Option<HumanMoneyValue>,
    pub country_of_risk: String,
    pub country_of_risk_name: String,
    pub sector: String,
    pub issue_kind: String,
    pub coupon_quantity_per_year: i32,

    // Dates
    pub maturity_date: Option<HumanTimestamp>,
    pub state_reg_date: Option<HumanTimestamp>,
    pub placement_date: Option<HumanTimestamp>,
    pub first_1min_candle_date: Option<HumanTimestamp>,
    pub first_1day_candle_date: Option<HumanTimestamp>,

    // Optional fields with enhanced types
    pub klong: Option<HumanQuotation>,
    pub kshort: Option<HumanQuotation>,
    pub dlong: Option<HumanQuotation>,
    pub dshort: Option<HumanQuotation>,
    pub dlong_min: Option<HumanQuotation>,
    pub dshort_min: Option<HumanQuotation>,
    pub min_price_increment: Option<HumanQuotation>,
    pub risk_level: Option<String>,
}

impl From<&Bond> for HumanBond {
    fn from(bond: &Bond) -> Self {
        HumanBond {
            figi: bond.figi.clone(),
            ticker: bond.ticker.clone(),
            class_code: bond.class_code.clone(),
            isin: bond.isin.clone(),
            uid: bond.uid.clone(),
            position_uid: bond.position_uid.clone(),
            name: bond.name.clone(),
            lot: bond.lot,
            currency: bond.currency.clone(),
            exchange: bond.exchange.clone(),

            short_enabled_flag: bond.short_enabled_flag,
            otc_flag: bond.otc_flag,
            buy_available_flag: bond.buy_available_flag,
            sell_available_flag: bond.sell_available_flag,
            floating_coupon_flag: bond.floating_coupon_flag,
            perpetual_flag: bond.perpetual_flag,
            amortization_flag: bond.amortization_flag,
            api_trade_available_flag: bond.api_trade_available_flag,
            for_iis_flag: bond.for_iis_flag,
            for_qual_investor_flag: bond.for_qual_investor_flag,
            weekend_flag: bond.weekend_flag,
            blocked_tca_flag: bond.blocked_tca_flag,
            subordinated_flag: bond.subordinated_flag,
            liquidity_flag: bond.liquidity_flag,

            trading_status: HumanTradingStatus::from(bond.trading_status),
            real_exchange: HumanRealExchange::from(bond.real_exchange),

            issue_size: bond.issue_size,
            issue_size_plan: bond.issue_size_plan,
            nominal: bond.nominal.as_ref().map(HumanMoneyValue::from),
            initial_nominal: bond.initial_nominal.as_ref().map(HumanMoneyValue::from),
            placement_price: bond.placement_price.as_ref().map(HumanMoneyValue::from),
            aci_value: bond.aci_value.as_ref().map(HumanMoneyValue::from),
            country_of_risk: bond.country_of_risk.clone(),
            country_of_risk_name: bond.country_of_risk_name.clone(),
            sector: bond.sector.clone(),
            issue_kind: bond.issue_kind.clone(),
            coupon_quantity_per_year: bond.coupon_quantity_per_year,

            maturity_date: bond.maturity_date.as_ref().map(HumanTimestamp::from),
            state_reg_date: bond.state_reg_date.as_ref().map(HumanTimestamp::from),
            placement_date: bond.placement_date.as_ref().map(HumanTimestamp::from),
            first_1min_candle_date: bond
                .first_1min_candle_date
                .as_ref()
                .map(HumanTimestamp::from),
            first_1day_candle_date: bond
                .first_1day_candle_date
                .as_ref()
                .map(HumanTimestamp::from),

            klong: bond.klong.as_ref().map(HumanQuotation::from),
            kshort: bond.kshort.as_ref().map(HumanQuotation::from),
            dlong: bond.dlong.as_ref().map(HumanQuotation::from),
            dshort: bond.dshort.as_ref().map(HumanQuotation::from),
            dlong_min: bond.dlong_min.as_ref().map(HumanQuotation::from),
            dshort_min: bond.dshort_min.as_ref().map(HumanQuotation::from),
            min_price_increment: bond.min_price_increment.as_ref().map(HumanQuotation::from),
            risk_level: match bond.risk_level {
                0 => Some("RISK_LEVEL_HIGH".to_string()),
                1 => Some("RISK_LEVEL_MODERATE".to_string()),
                2 => Some("RISK_LEVEL_LOW".to_string()),
                _ => None,
            },
        }
    }
}

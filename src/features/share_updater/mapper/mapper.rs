use crate::features::core::models::money_value::MyMoneyValue;
use crate::features::core::models::quotation::MyQuotation;
use crate::features::share_updater::models::{
    MyShare, RealExchange, SecurityTradingStatus, ShareType,
};
use crate::gen::tinkoff_public_invest_api_contract_v1::{
    MoneyValue as TinkoffMoneyValue, Quotation as TinkoffQuotation, Share as TinkoffShare,
};
use chrono::{DateTime, Utc};
use num_traits::FromPrimitive;
use prost_types::Timestamp;

impl From<TinkoffQuotation> for MyQuotation {
    fn from(q: TinkoffQuotation) -> Self {
        MyQuotation {
            units: q.units,
            nano: q.nano,
        }
    }
}

impl From<TinkoffMoneyValue> for MyMoneyValue {
    fn from(m: TinkoffMoneyValue) -> Self {
        MyMoneyValue {
            currency: m.currency,
            units: m.units,
            nano: m.nano,
        }
    }
}

impl From<i32> for ShareType {
    fn from(value: i32) -> Self {
        ShareType::from_i32(value).unwrap_or(ShareType::Unspecified)
    }
}

impl From<i32> for SecurityTradingStatus {
    fn from(value: i32) -> Self {
        SecurityTradingStatus::from_i32(value).unwrap_or(SecurityTradingStatus::Unspecified)
    }
}

impl From<i32> for RealExchange {
    fn from(value: i32) -> Self {
        RealExchange::from_i32(value).unwrap_or(RealExchange::Unspecified)
    }
}

impl From<TinkoffShare> for MyShare {
    fn from(s: TinkoffShare) -> Self {
        MyShare {
            figi: s.figi,
            ticker: s.ticker,
            class_code: s.class_code,
            isin: s.isin,
            lot: s.lot,
            currency: s.currency,
            klong: s.klong.map(|q| q.into()),
            kshort: s.kshort.map(|q| q.into()),
            dlong: s.dlong.map(|q| q.into()),
            dshort: s.dshort.map(|q| q.into()),
            dlong_min: s.dlong_min.map(|q| q.into()),
            dshort_min: s.dshort_min.map(|q| q.into()),
            short_enabled_flag: s.short_enabled_flag,
            name: s.name,
            exchange: s.exchange,
            ipo_date: s.ipo_date.map(timestamp_to_datetime),
            issue_size: s.issue_size,
            country_of_risk: s.country_of_risk,
            country_of_risk_name: s.country_of_risk_name,
            sector: s.sector,
            issue_size_plan: s.issue_size_plan,
            nominal: s.nominal.map(|m| m.into()),
            trading_status: s.trading_status.into(),
            otc_flag: s.otc_flag,
            buy_available_flag: s.buy_available_flag,
            sell_available_flag: s.sell_available_flag,
            div_yield_flag: s.div_yield_flag,
            share_type: s.share_type.into(),
            min_price_increment: s.min_price_increment.map(|q| q.into()),
            api_trade_available_flag: s.api_trade_available_flag,
            uid: s.uid,
            real_exchange: s.real_exchange.into(),
            position_uid: s.position_uid,
            for_iis_flag: s.for_iis_flag,
            for_qual_investor_flag: s.for_qual_investor_flag,
            weekend_flag: s.weekend_flag,
            blocked_tca_flag: s.blocked_tca_flag,
            liquidity_flag: s.liquidity_flag,
            first_1min_candle_date: s.first_1min_candle_date.map(timestamp_to_datetime),
            first_1day_candle_date: s.first_1day_candle_date.map(timestamp_to_datetime),
        }
    }
}

fn timestamp_to_datetime(ts: Timestamp) -> DateTime<Utc> {
    DateTime::<Utc>::from_utc(
        chrono::NaiveDateTime::from_timestamp_opt(ts.seconds, ts.nanos as u32).unwrap(),
        Utc,
    )
}

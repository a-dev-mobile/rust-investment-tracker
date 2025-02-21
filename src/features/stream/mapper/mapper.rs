use crate::features::{core::models::quotation::MyQuotation, stream::models::candle::MyCandle};
use crate::gen::tinkoff_public_invest_api_contract_v1::Candle as TinkoffCandle;
use chrono::{DateTime, TimeZone, Utc};
use prost_types::Timestamp;

impl From<TinkoffCandle> for MyCandle {
    fn from(c: TinkoffCandle) -> Self {
        MyCandle {
            figi: c.figi,
            interval: c.interval,
            open: c.open.map(|q| q.into()),
            high: c.high.map(|q| q.into()),
            low: c.low.map(|q| q.into()),
            close: c.close.map(|q| q.into()),
            volume: c.volume,
            time: c.time.map(|ts| timestamp_to_datetime(ts)),
            last_trade_ts: c.last_trade_ts.map(|ts| timestamp_to_datetime(ts)),
            instrument_uid: c.instrument_uid,
        }
    }
}

fn timestamp_to_datetime(ts: Timestamp) -> DateTime<Utc> {
    DateTime::<Utc>::from_utc(
        chrono::NaiveDateTime::from_timestamp_opt(ts.seconds, ts.nanos as u32).unwrap(),
        Utc,
    )
}
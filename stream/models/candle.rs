// use chrono::{DateTime, Utc};
// use serde::{Deserialize, Serialize};


// #[derive(Debug, Serialize, Deserialize)]
// pub struct MyCandle {
//     pub figi: String,
//     pub interval: i32,
//     pub open: Option<MyQuotation>,
//     pub high: Option<MyQuotation>,
//     pub low: Option<MyQuotation>,
//     pub close: Option<MyQuotation>,
//     pub volume: i64,
//     pub time: Option<DateTime<Utc>>,
//     pub last_trade_ts: Option<DateTime<Utc>>,
//     pub instrument_uid: String,
// }
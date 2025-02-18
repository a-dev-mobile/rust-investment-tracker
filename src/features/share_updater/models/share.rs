
// use serde::{Deserialize, Serialize};
// use time::OffsetDateTime;
// use uuid::Uuid;
// use sqlx::Type;  

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Brand {
//     #[serde(rename = "logoName")]
//     pub logo_name: String,
//     #[serde(rename = "logoBaseColor")]
//     pub logo_base_color: String,
//     #[serde(rename = "textColor")]
//     pub text_color: String,
// }

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
// pub enum TradingStatus {
//     #[serde(rename = "SECURITY_TRADING_STATUS_UNSPECIFIED")]
//     Unspecified,
//     #[serde(rename = "SECURITY_TRADING_STATUS_NOT_AVAILABLE_FOR_TRADING")]
//     NotAvailableForTrading,
//     #[serde(rename = "SECURITY_TRADING_STATUS_OPENING_PERIOD")]
//     OpeningPeriod,
//     #[serde(rename = "SECURITY_TRADING_STATUS_CLOSING_PERIOD")]
//     ClosingPeriod,
//     #[serde(rename = "SECURITY_TRADING_STATUS_BREAK_IN_TRADING")]
//     BreakInTrading,
//     #[serde(rename = "SECURITY_TRADING_STATUS_NORMAL_TRADING")]
//     NormalTrading,
//     #[serde(rename = "SECURITY_TRADING_STATUS_CLOSING_AUCTION")]
//     ClosingAuction,
//     #[serde(rename = "SECURITY_TRADING_STATUS_DARK_POOL_AUCTION")]
//     DarkPoolAuction,
//     #[serde(rename = "SECURITY_TRADING_STATUS_DISCRETE_AUCTION")]
//     DiscreteAuction,
//     #[serde(rename = "SECURITY_TRADING_STATUS_OPENING_AUCTION_PERIOD")]
//     OpeningAuctionPeriod,
//     #[serde(rename = "SECURITY_TRADING_STATUS_TRADING_AT_CLOSING_AUCTION_PRICE")]
//     TradingAtClosingAuctionPrice,
//     #[serde(rename = "SECURITY_TRADING_STATUS_SESSION_ASSIGNED")]
//     SessionAssigned,
//     #[serde(rename = "SECURITY_TRADING_STATUS_SESSION_CLOSE")]
//     SessionClose,
//     #[serde(rename = "SECURITY_TRADING_STATUS_SESSION_OPEN")]
//     SessionOpen,
//     #[serde(rename = "SECURITY_TRADING_STATUS_DEALER_NORMAL_TRADING")]
//     DealerNormalTrading,
//     #[serde(rename = "SECURITY_TRADING_STATUS_DEALER_BREAK_IN_TRADING")]
//     DealerBreakInTrading,
//     #[serde(rename = "SECURITY_TRADING_STATUS_DEALER_NOT_AVAILABLE_FOR_TRADING")]
//     DealerNotAvailableForTrading,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub enum ShareType {
//     #[serde(rename = "SHARE_TYPE_UNSPECIFIED")]
//     Unspecified,
//     #[serde(rename = "SHARE_TYPE_COMMON")]
//     Common,
//     #[serde(rename = "SHARE_TYPE_PREFERRED")]
//     Preferred,
//     #[serde(rename = "SHARE_TYPE_ADR")]
//     Adr,
//     #[serde(rename = "SHARE_TYPE_GDR")]
//     Gdr,
//     #[serde(rename = "SHARE_TYPE_MLP")]
//     Mlp,
//     #[serde(rename = "SHARE_TYPE_NY_REG_SHRS")]
//     NyRegShrs,
//     #[serde(rename = "SHARE_TYPE_CLOSED_END_FUND")]
//     ClosedEndFund,
//     #[serde(rename = "SHARE_TYPE_REIT")]
//     Reit,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub enum RealExchange {
//     #[serde(rename = "REAL_EXCHANGE_UNSPECIFIED")]
//     Unspecified,
//     #[serde(rename = "REAL_EXCHANGE_MOEX")]
//     Moex,
//     #[serde(rename = "REAL_EXCHANGE_RTS")]
//     Rts,
//     #[serde(rename = "REAL_EXCHANGE_OTC")]
//     Otc,
//     #[serde(rename = "REAL_EXCHANGE_DEALER")]
//     Dealer,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub enum InstrumentExchange {
//     #[serde(rename = "INSTRUMENT_EXCHANGE_UNSPECIFIED")]
//     Unspecified,
//     #[serde(rename = "INSTRUMENT_EXCHANGE_DEALER")]
//     Dealer,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct ShareResponse {
//     pub instruments: Vec<Share>,
// }


// #[derive(Debug, Serialize, Deserialize)]
// pub struct MonetaryValue {
//     pub currency: Option<String>,
//     #[serde(with = "string_or_number")]
//     pub units: i64,
//     #[serde(with = "string_or_number")]
//     pub nano: i32,
// }
// #[derive(Debug, Serialize, Deserialize)]
// pub struct Share {
//     pub figi: String,
//     pub ticker: String,
//     #[serde(rename = "classCode")]
//     pub class_code: String,
//     pub isin: String,
//     #[serde(with = "string_or_number")]
//     pub lot: i32,
//     pub currency: String,
//     #[serde(rename = "shortEnabledFlag")]
//     pub short_enabled_flag: bool,
//     pub name: String,
//     pub exchange: Option<String>,
//     #[serde(rename = "ipoDate")]
//     #[serde(default)]
//     #[serde(with = "time_format")]
//     pub ipo_date: Option<OffsetDateTime>,
//     #[serde(rename = "issueSize")]
//     #[serde(with = "string_or_number_opt")]
//     pub issue_size: Option<i64>,
//     #[serde(rename = "countryOfRisk")]
//     pub country_of_risk: Option<String>,
//     #[serde(rename = "countryOfRiskName")]
//     pub country_of_risk_name: Option<String>,
//     pub sector: Option<String>,
//     #[serde(rename = "issueSizePlan")]
//     #[serde(with = "string_or_number_opt")]
//     pub issue_size_plan: Option<i64>,
//     pub nominal: Option<MonetaryValue>,
//     #[serde(rename = "tradingStatus")]
//     pub trading_status: TradingStatus,
//     #[serde(rename = "otcFlag")]
//     pub otc_flag: bool,
//     #[serde(rename = "buyAvailableFlag")]
//     pub buy_available_flag: bool,
//     #[serde(rename = "sellAvailableFlag")]
//     pub sell_available_flag: bool,
//     #[serde(rename = "divYieldFlag")]
//     pub div_yield_flag: bool,
//     #[serde(rename = "shareType")]
//     pub share_type: ShareType,
//     #[serde(rename = "minPriceIncrement")]
//     pub min_price_increment: Option<MonetaryValue>,
//     #[serde(rename = "apiTradeAvailableFlag")]
//     pub api_trade_available_flag: bool,
//     pub uid: Uuid,
//     #[serde(rename = "realExchange")]
//     pub real_exchange: RealExchange,
//     #[serde(rename = "positionUid")]
//     pub position_uid: Uuid,
//     #[serde(rename = "assetUid")]
//     pub asset_uid: Uuid,
//     #[serde(rename = "instrumentExchange")]
//     pub instrument_exchange: InstrumentExchange,
//     #[serde(rename = "forIisFlag")]
//     pub for_iis_flag: bool,
//     #[serde(rename = "forQualInvestorFlag")]
//     pub for_qual_investor_flag: bool,
//     #[serde(rename = "weekendFlag")]
//     pub weekend_flag: bool,
//     #[serde(rename = "blockedTcaFlag")]
//     pub blocked_tca_flag: bool,
//     #[serde(rename = "liquidityFlag")]
//     pub liquidity_flag: bool,
//     pub brand: Option<Brand>,
//     #[serde(rename = "dlongClient")]
//     pub dlong_client: Option<MonetaryValue>,
//     #[serde(rename = "dshortClient")]
//     pub dshort_client: Option<MonetaryValue>,
// }

// // Custom serialization module for time
// mod time_format {
//     use serde::{self, Deserialize, Deserializer, Serializer};
//     use time::{format_description::well_known::Rfc3339, OffsetDateTime};

//     pub fn serialize<S>(date: &Option<OffsetDateTime>, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         match date {
//             Some(date) => {
//                 let s = date.format(&Rfc3339).map_err(serde::ser::Error::custom)?;
//                 serializer.serialize_str(&s)
//             }
//             None => serializer.serialize_none(),
//         }
//     }

//     pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let s: Option<String> = Option::deserialize(deserializer)?;
//         match s {
//             Some(s) => {
//                 let date = OffsetDateTime::parse(&s, &Rfc3339).map_err(serde::de::Error::custom)?;
//                 Ok(Some(date))
//             }
//             None => Ok(None),
//         }
//     }
// }

// // Updated string_or_number module to handle both i32 and i64
// mod string_or_number {
//     use serde::{self, Deserialize, Deserializer, Serializer};
//     use std::str::FromStr;

//     pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//         T: std::fmt::Display,
//     {
//         serializer.serialize_str(&value.to_string())
//     }

//     pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
//     where
//         D: Deserializer<'de>,
//         T: FromStr + serde::Deserialize<'de>,
//         T::Err: std::fmt::Display,
//     {
//         #[derive(Deserialize)]
//         #[serde(untagged)]
//         enum StringOrNumber<T> {
//             String(String),
//             Number(T),
//         }

//         match StringOrNumber::<T>::deserialize(deserializer)? {
//             StringOrNumber::String(s) => T::from_str(&s).map_err(serde::de::Error::custom),
//             StringOrNumber::Number(n) => Ok(n),
//         }
//     }
// }

// // Optional values handler
// mod string_or_number_opt {
//     use serde::{self, Deserialize, Deserializer, Serializer};
//     use std::str::FromStr;

//     pub fn serialize<S, T>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//         T: std::fmt::Display,
//     {
//         match value {
//             Some(v) => serializer.serialize_str(&v.to_string()),
//             None => serializer.serialize_none(),
//         }
//     }

//     pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
//     where
//         D: Deserializer<'de>,
//         T: FromStr + serde::Deserialize<'de>,
//         T::Err: std::fmt::Display,
//     {
//         #[derive(Deserialize)]
//         #[serde(untagged)]
//         enum StringOrNumber<T> {
//             String(String),
//             Number(T),
//         }

//         match Option::<StringOrNumber<T>>::deserialize(deserializer)? {
//             Some(StringOrNumber::String(s)) => {
//                 Ok(Some(T::from_str(&s).map_err(serde::de::Error::custom)?))
//             }
//             Some(StringOrNumber::Number(n)) => Ok(Some(n)),
//             None => Ok(None),
//         }
//     }
// }

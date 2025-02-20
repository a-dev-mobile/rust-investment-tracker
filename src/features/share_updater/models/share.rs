use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use num_derive::FromPrimitive;


#[derive(Debug, Serialize, Deserialize)]
pub struct ShareQuotation {
    pub units: i64,
    pub nano: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShareMoneyValue {
    pub currency: String,
    pub units: i64,
    pub nano: i32,
}

#[derive(Debug, Serialize, Deserialize, FromPrimitive)]
pub enum ShareType {
    #[serde(rename = "SHARE_TYPE_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "SHARE_TYPE_COMMON")]
    Common,
    #[serde(rename = "SHARE_TYPE_PREFERRED")]
    Preferred,
    #[serde(rename = "SHARE_TYPE_ADR")]
    Adr,
    #[serde(rename = "SHARE_TYPE_GDR")]
    Gdr,
    #[serde(rename = "SHARE_TYPE_MLP")]
    Mlp,
    #[serde(rename = "SHARE_TYPE_NY_REG_SHRS")]
    NyRegShrs,
    #[serde(rename = "SHARE_TYPE_CLOSED_END_FUND")]
    ClosedEndFund,
    #[serde(rename = "SHARE_TYPE_REIT")]
    Reit,
}

#[derive(Debug, Serialize, Deserialize, FromPrimitive)]
pub enum SecurityTradingStatus {
    #[serde(rename = "SECURITY_TRADING_STATUS_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "SECURITY_TRADING_STATUS_NOT_AVAILABLE_FOR_TRADING")]
    NotAvailableForTrading,
    #[serde(rename = "SECURITY_TRADING_STATUS_OPENING_PERIOD")]
    OpeningPeriod,
    #[serde(rename = "SECURITY_TRADING_STATUS_CLOSING_PERIOD")]
    ClosingPeriod,
    #[serde(rename = "SECURITY_TRADING_STATUS_BREAK_IN_TRADING")]
    BreakInTrading,
    #[serde(rename = "SECURITY_TRADING_STATUS_NORMAL_TRADING")]
    NormalTrading,
    #[serde(rename = "SECURITY_TRADING_STATUS_CLOSING_AUCTION")]
    ClosingAuction,
    #[serde(rename = "SECURITY_TRADING_STATUS_DARK_POOL_AUCTION")]
    DarkPoolAuction,
    #[serde(rename = "SECURITY_TRADING_STATUS_DISCRETE_AUCTION")]
    DiscreteAuction,
    #[serde(rename = "SECURITY_TRADING_STATUS_OPENING_AUCTION_PERIOD")]
    OpeningAuctionPeriod,
    #[serde(rename = "SECURITY_TRADING_STATUS_TRADING_AT_CLOSING_AUCTION_PRICE")]
    TradingAtClosingAuctionPrice,
    #[serde(rename = "SECURITY_TRADING_STATUS_SESSION_ASSIGNED")]
    SessionAssigned,
    #[serde(rename = "SECURITY_TRADING_STATUS_SESSION_CLOSE")]
    SessionClose,
    #[serde(rename = "SECURITY_TRADING_STATUS_SESSION_OPEN")]
    SessionOpen,
    #[serde(rename = "SECURITY_TRADING_STATUS_DEALER_NORMAL_TRADING")]
    DealerNormalTrading,
    #[serde(rename = "SECURITY_TRADING_STATUS_DEALER_BREAK_IN_TRADING")]
    DealerBreakInTrading,
    #[serde(rename = "SECURITY_TRADING_STATUS_DEALER_NOT_AVAILABLE_FOR_TRADING")]
    DealerNotAvailableForTrading,
}

#[derive(Debug, Serialize, Deserialize, FromPrimitive)]
pub enum RealExchange {
    #[serde(rename = "REAL_EXCHANGE_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "REAL_EXCHANGE_MOEX")]
    Moex,
    #[serde(rename = "REAL_EXCHANGE_RTS")]
    Rts,
    #[serde(rename = "REAL_EXCHANGE_OTC")]
    Otc,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Share {
    pub figi: String,
    pub ticker: String,
    pub class_code: String,
    pub isin: String,
    pub lot: i32,
    pub currency: String,
    pub klong: Option<ShareQuotation>,
    pub kshort: Option<ShareQuotation>,
    pub dlong: Option<ShareQuotation>,
    pub dshort: Option<ShareQuotation>,
    pub dlong_min: Option<ShareQuotation>,
    pub dshort_min: Option<ShareQuotation>,
    pub short_enabled_flag: bool,
    pub name: String,
    pub exchange: String,
    pub ipo_date: Option<DateTime<Utc>>,
    pub issue_size: i64,
    pub country_of_risk: String,
    pub country_of_risk_name: String,
    pub sector: String,
    pub issue_size_plan: i64,
    pub nominal: Option<ShareMoneyValue>,
    pub trading_status: SecurityTradingStatus,
    pub otc_flag: bool,
    pub buy_available_flag: bool,
    pub sell_available_flag: bool,
    pub div_yield_flag: bool,
    pub share_type: ShareType,
    pub min_price_increment: Option<ShareQuotation>,
    pub api_trade_available_flag: bool,
    pub uid: String,
    pub real_exchange: RealExchange,
    pub position_uid: String,
    pub for_iis_flag: bool,
    pub for_qual_investor_flag: bool,
    pub weekend_flag: bool,
    pub blocked_tca_flag: bool,
    pub liquidity_flag: bool,
    pub first_1min_candle_date: Option<DateTime<Utc>>,
    pub first_1day_candle_date: Option<DateTime<Utc>>,
}
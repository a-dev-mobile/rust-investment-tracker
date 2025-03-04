use std::collections::HashMap;
use serde::{Serialize, Deserialize};

// Структуры для преобразованного ответа
#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyRatesResponse {
    pub date: String,
    pub today_volume: Option<TradingVolume>, // Объемы торгов за сегодня
    pub currencies: HashMap<String, CurrencyInfo>,
    pub display_info: HashMap<String, CurrencyDisplayInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradingVolume {
    pub rubles: f64,  // TODAY_VALTODAY
    pub usd: f64,     // TODAY_VALTODAY_USD
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyInfo {
    pub name: String,
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub central_bank: Option<RateInfo>,       // Курс ЦБ РФ
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange: Option<ExchangeRateInfo>,   // Биржевой курс (например, USDTOM)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wap_rate: Option<WapRateInfo>,        // Средневзвешенный курс (из wap_rates)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RateInfo {
    pub current_rate: f64,
    pub previous_rate: f64,
    pub change: RateChange,
    pub date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeRateInfo {
    pub current_rate: f64,
    pub previous_rate: f64,
    pub change: RateChange,
    pub date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WapRateInfo {
    pub current_rate: f64,        // Средневзвешенная цена (price)
    pub change_percent: f64,      // Процент изменения (lasttoprevprice)
    pub previous_rate: f64,       // Вычисляется на основе текущей цены и процента
    pub date: String,             // tradedate
    pub time: String,             // tradetime
    pub nominal: f64,             // Номинал
    pub precision: u8,            // Количество знаков после запятой (decimals)
    pub security_id: String,      // secid (например, CNYRUB_TOM)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RateChange {
    pub absolute: f64,
    pub percent: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyDisplayInfo {
    pub text: String,
    pub trend: String,
    pub change_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wap_text: Option<String>, // Отображение средневзвешенного курса
}
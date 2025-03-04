
use serde::{Serialize, Deserialize};

// Структуры для десериализации ответа от API MOEX
#[derive(Debug, Serialize, Deserialize)]
pub struct MoexRatesResponse {
    pub cbrf: CbrfRates,
    pub wap_rates: WapRates,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CbrfRates {
    pub columns: Vec<String>,
    pub data: Vec<Vec<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WapRates {
    pub columns: Vec<String>,
    pub data: Vec<Vec<serde_json::Value>>,
}
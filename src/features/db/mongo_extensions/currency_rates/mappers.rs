

use crate::features::moex_api::models::MoexRatesResponse;

use std::collections::HashMap;
use tracing::{info, warn};

use super::models::{CurrencyDisplayInfo, CurrencyInfo, CurrencyRatesResponse, ExchangeRateInfo, RateChange, RateInfo, TradingVolume, WapRateInfo};

pub struct MoexRatesMapper;

#[derive(Debug)]
struct CurrencyConfig {
    code: &'static str,
    name: &'static str,
    symbol: &'static str,
    cbrf_key: &'static str,
    cbrf_change_key: &'static str,
    cbrf_date_key: &'static str,
    exchange_key: Option<(&'static str, &'static str, &'static str)>,
    wap_security_id: Option<&'static str>, // Для привязки к данным wap_rates
}

impl MoexRatesMapper {
    const CURRENCIES: [CurrencyConfig; 3] = [
        CurrencyConfig {
            code: "USD",
            name: "Доллар США",
            symbol: "$",
            cbrf_key: "CBRF_USD_LAST",
            cbrf_change_key: "CBRF_USD_LASTCHANGEPRCNT",
            cbrf_date_key: "CBRF_USD_TRADEDATE",
            exchange_key: Some((
                "USDTOM_UTS_CLOSEPRICE",
                "USDTOM_UTS_CLOSEPRICETOPREVPRCN",
                "USDTOM_UTS_TRADEDATE",
            )),
            wap_security_id: None,
        },
        CurrencyConfig {
            code: "EUR",
            name: "Евро",
            symbol: "€",
            cbrf_key: "CBRF_EUR_LAST",
            cbrf_change_key: "CBRF_EUR_LASTCHANGEPRCNT",
            cbrf_date_key: "CBRF_EUR_TRADEDATE",
            exchange_key: None,
            wap_security_id: None,
        },
        CurrencyConfig {
            code: "CNY",
            name: "Китайский юань",
            symbol: "¥",
            cbrf_key: "",
            cbrf_change_key: "",
            cbrf_date_key: "",
            exchange_key: None,
            wap_security_id: Some("CNYRUB_TOM"),
        },
    ];

    pub fn map_to_currency_rates(
        response: &MoexRatesResponse,
    ) -> Result<CurrencyRatesResponse, Box<dyn std::error::Error + Send + Sync>>  {
        let mut currencies = HashMap::new();
        let mut display_info = HashMap::new();

        if response.cbrf.data.is_empty() {
            warn!("CBRF data is empty");
            return Ok(CurrencyRatesResponse {
                date: String::new(),
                today_volume: None,
                currencies,
                display_info,
            });
        }

        let cbrf_indices = Self::build_indices(&response.cbrf.columns);
        let wap_indices = Self::build_indices(&response.wap_rates.columns);
        let today_date = response.cbrf.data[0][cbrf_indices["TODAY_DATE"]]
            .as_str()
            .unwrap_or("")
            .to_string();

        // Объемы торгов
        let today_volume = Some(TradingVolume {
            rubles: response.cbrf.data[0][cbrf_indices["TODAY_VALTODAY"]]
                .as_f64()
                .unwrap_or(0.0),
            usd: response.cbrf.data[0][cbrf_indices["TODAY_VALTODAY_USD"]]
                .as_f64()
                .unwrap_or(0.0),
        });

        // Обработка основных валют
        for config in Self::CURRENCIES.iter() {
            Self::map_currency_data(
                config,
                &response,
                &cbrf_indices,
                &wap_indices,
                &mut currencies,
                &mut display_info,
            )?;
        }

        Ok(CurrencyRatesResponse {
            date: today_date,
            today_volume,
            currencies,
            display_info,
        })
    }

    fn build_indices(columns: &[String]) -> HashMap<&str, usize> {
        columns
            .iter()
            .enumerate()
            .map(|(idx, col)| (col.as_str(), idx))
            .collect()
    }

    fn map_currency_data(
        config: &CurrencyConfig,
        response: &MoexRatesResponse,
        cbrf_indices: &HashMap<&str, usize>,
        wap_indices: &HashMap<&str, usize>,
        currencies: &mut HashMap<String, CurrencyInfo>,
        display_info: &mut HashMap<String, CurrencyDisplayInfo>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>  {
        let mut central_bank = None;
        let mut exchange = None;
        let mut wap_rate = None;

        // Central Bank Rate (CBRF)
        if !config.cbrf_key.is_empty() {
            let current_rate = response.cbrf.data[0][cbrf_indices[config.cbrf_key]]
                .as_f64()
                .unwrap_or(0.0);
            let change_percent = response.cbrf.data[0][cbrf_indices[config.cbrf_change_key]]
                .as_f64()
                .unwrap_or(0.0);
            let previous_rate = current_rate / (1.0 + change_percent / 100.0);
            let date = response.cbrf.data[0][cbrf_indices[config.cbrf_date_key]]
                .as_str()
                .unwrap_or("")
                .to_string();

            central_bank = Some(RateInfo {
                current_rate,
                previous_rate,
                change: RateChange {
                    absolute: current_rate - previous_rate,
                    percent: change_percent,
                },
                date,
            });
        }

        // Exchange Rate
        if let Some((price_key, change_key, date_key)) = config.exchange_key {
            let current_rate = response.cbrf.data[0][cbrf_indices[price_key]]
                .as_f64()
                .unwrap_or(0.0);
            let change_percent = response.cbrf.data[0][cbrf_indices[change_key]]
                .as_f64()
                .unwrap_or(0.0);
            let previous_rate = current_rate / (1.0 + change_percent / 100.0);
            let date = response.cbrf.data[0][cbrf_indices[date_key]]
                .as_str()
                .unwrap_or("")
                .to_string();

            exchange = Some(ExchangeRateInfo {
                current_rate,
                previous_rate,
                change: RateChange {
                    absolute: current_rate - previous_rate,
                    percent: change_percent,
                },
                date,
                precision: None,
            });
        }

        // WAP Rate
        if let Some(security_id) = config.wap_security_id {
            if let Some(wap_data) = response.wap_rates.data.iter().find(|row| {
                row[wap_indices["secid"]]
                    .as_str()
                    .unwrap_or("") == security_id
            }) {
                let current_rate = wap_data[wap_indices["price"]].as_f64().unwrap_or(0.0);
                let change_percent = wap_data[wap_indices["lasttoprevprice"]]
                    .as_f64()
                    .unwrap_or(0.0);
                let previous_rate = current_rate / (1.0 + change_percent / 100.0);

                wap_rate = Some(WapRateInfo {
                    current_rate,
                    change_percent,
                    previous_rate,
                    date: wap_data[wap_indices["tradedate"]]
                        .as_str()
                        .unwrap_or("")
                        .to_string(),
                    time: wap_data[wap_indices["tradetime"]]
                        .as_str()
                        .unwrap_or("")
                        .to_string(),
                    nominal: wap_data[wap_indices["nominal"]]
                        .as_f64()
                        .unwrap_or(1.0),
                    precision: wap_data[wap_indices["decimals"]]
                        .as_u64()
                        .unwrap_or(0) as u8,
                    security_id: security_id.to_string(),
                });
            }
        }

        // Сначала заполняем display_info
        let mut display = None;
        if let Some(cb) = &central_bank {
            let (trend, change_sign) = if cb.change.percent > 0.0 {
                ("рост", "+")
            } else {
                ("снижение", "")
            };
            display = Some(CurrencyDisplayInfo {
                text: format!(
                    "{:.2} ₽ за {}1 (вчера: {:.2} ₽)",
                    cb.current_rate, config.symbol, cb.previous_rate
                ),
                trend: trend.to_string(),
                change_text: format!(
                    "{}{}% ({}{:.2} ₽)",
                    change_sign,
                    cb.change.percent,
                    change_sign,
                    cb.change.absolute.abs()
                ),
                wap_text: wap_rate.as_ref().map(|wap| {
                    format!(
                        "WAP: {:.2} ₽ ({}%, вчера: {:.2} ₽)",
                        wap.current_rate, wap.change_percent, wap.previous_rate
                    )
                }),
            });
        } else if let Some(wap) = &wap_rate {
            let (trend, change_sign) = if wap.change_percent > 0.0 {
                ("рост", "+")
            } else {
                ("снижение", "")
            };
            display = Some(CurrencyDisplayInfo {
                text: format!(
                    "WAP: {:.2} ₽ за {}1 (вчера: {:.2} ₽)",
                    wap.current_rate, config.symbol, wap.previous_rate
                ),
                trend: trend.to_string(),
                change_text: format!(
                    "{}{}% ({}{:.2} ₽)",
                    change_sign,
                    wap.change_percent,
                    change_sign,
                    (wap.current_rate - wap.previous_rate).abs()
                ),
                wap_text: None,
            });
        }

        // Затем заполняем currencies
        currencies.insert(
            config.code.to_string(),
            CurrencyInfo {
                name: config.name.to_string(),
                symbol: config.symbol.to_string(),
                central_bank,
                exchange,
                wap_rate,
            },
        );

        // Если есть display_info, добавляем его
        if let Some(display) = display {
            display_info.insert(config.code.to_string(), display);
        }

        Ok(())
    }
}
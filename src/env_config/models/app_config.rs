use chrono::{NaiveTime, Utc};
use chrono_tz::Tz;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub log: LogConfig,
    pub database: DatabaseConfig,
    pub tinkoff_api: TinkoffApiConfig,
    pub share_updater: ShareUpdaterConfig,
}
#[derive(Debug, Deserialize)]
pub struct ShareUpdaterConfig {
    pub enabled: bool,
    pub interval_seconds: u64,
    pub max_retries: u32,
    pub retry_delay_seconds: u64,
    pub night_updates_disabled: bool,
    pub night_start_time: String,
    pub night_end_time: String,
    pub timezone: String,
}

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    pub level: String,
    pub format: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub max_connections: u32,
    pub timeout_seconds: u64,
    pub pool_size: u32,
}

#[derive(Debug, Deserialize)]
pub struct TinkoffApiConfig {
    pub base_url: String,
    pub domain: String,
    pub timeout: u64,
    pub keepalive: u64,
}

impl ShareUpdaterConfig {
    pub fn is_night_time(&self) -> bool {
        if !self.night_updates_disabled {
            return false;
        }

        // Парсим временную зону
        let timezone: Tz = self.timezone.parse().expect("Invalid timezone");

        // Получаем текущее время в UTC и конвертируем его в московское время
        let moscow_time = Utc::now().with_timezone(&timezone).time();

        let start_time = NaiveTime::parse_from_str(&self.night_start_time, "%H:%M")
            .expect("Invalid night_start_time format");
        let end_time = NaiveTime::parse_from_str(&self.night_end_time, "%H:%M")
            .expect("Invalid night_end_time format");

        if start_time <= end_time {
            moscow_time >= start_time && moscow_time <= end_time
        } else {
            // Обработка случая, когда ночной период пересекает полночь
            moscow_time >= start_time || moscow_time <= end_time
        }
    }
}

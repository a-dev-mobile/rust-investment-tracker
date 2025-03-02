use chrono::{NaiveTime, Utc};
use chrono_tz::Tz;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub log: LogConfig,
    pub postgres_db: PostgresDbConfig,
    pub mongo_db: MongoDbConfig,
    pub tinkoff_api: TinkoffApiConfig,
    pub tinkoff_market_data_updater: TinkoffMarketDataUpdaterConfig,
    pub candles_tracking_updater: CandlesTrackingUpdaterConfig,
    pub stream_updater: StreamConfig,
}
#[derive(Debug, Deserialize)]
pub struct CandlesTrackingUpdaterConfig {
    pub enabled: bool,
    pub interval_seconds: u64,
    pub max_retries: u32,
    pub retry_delay_seconds: u64,
    pub update_start_time: String,
    pub update_end_time: String,
    pub timezone: String,
}
#[derive(Debug, Deserialize)]
pub struct TinkoffMarketDataUpdaterConfig {
    pub enabled: bool,
    pub interval_seconds: u64,
    pub max_retries: u32,
    pub retry_delay_seconds: u64,
    pub update_start_time: String,
    pub update_end_time: String,
    pub timezone: String,
}

#[derive(Debug, Deserialize)]
pub struct StreamConfig {
    pub enabled: bool,
    pub retry_attempts: u32,
    pub retry_delay_seconds: u64,
    pub trading_start_time: String,
    pub trading_end_time: String,
    pub timezone: String,
}

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    pub level: String,
    pub format: String,
}

#[derive(Debug, Deserialize)]
pub struct PostgresDbConfig {
    pub max_connections: u32,
    pub timeout_seconds: u64,
    pub pool_size: u32,
}

#[derive(Debug, Deserialize)]
pub struct MongoDbConfig {
    pub timeout_seconds: u64,
    pub pool_size: u32,
    pub retry_writes: bool,
    pub write_concern: String,
    pub read_concern: String,
}

#[derive(Debug, Deserialize)]
pub struct TinkoffApiConfig {
    pub base_url: String,
    pub domain: String,
    pub timeout: u64,
    pub keepalive: u64,
}

// Implement is_update_time for CandlesTrackingUpdaterConfig
impl CandlesTrackingUpdaterConfig {
    pub fn is_update_time(&self) -> bool {
        // Parse the timezone
        let timezone: Tz = self.timezone.parse().expect("Invalid timezone");

        // Get the current time in UTC and convert to the specified timezone
        let current_time = Utc::now().with_timezone(&timezone).time();

        let start_time = NaiveTime::parse_from_str(&self.update_start_time, "%H:%M")
            .expect("Invalid update_start_time format");
        let end_time = NaiveTime::parse_from_str(&self.update_end_time, "%H:%M")
            .expect("Invalid update_end_time format");

        if start_time <= end_time {
            current_time >= start_time && current_time <= end_time
        } else {
            // Handle case when update period crosses midnight
            current_time >= start_time || current_time <= end_time
        }
    }
}
impl TinkoffMarketDataUpdaterConfig {
    pub fn is_update_time(&self) -> bool {
        // Парсим временную зону
        let timezone: Tz = self.timezone.parse().expect("Invalid timezone");

        // Получаем текущее время в UTC и конвертируем его в указанную временную зону
        let current_time = Utc::now().with_timezone(&timezone).time();

        let start_time = NaiveTime::parse_from_str(&self.update_start_time, "%H:%M")
            .expect("Invalid update_start_time format");
        let end_time = NaiveTime::parse_from_str(&self.update_end_time, "%H:%M")
            .expect("Invalid update_end_time format");

        if start_time <= end_time {
            current_time >= start_time && current_time <= end_time
        } else {
            // Обработка случая, когда период обновления пересекает полночь
            current_time >= start_time || current_time <= end_time
        }
    }
}

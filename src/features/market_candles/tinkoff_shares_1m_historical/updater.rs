// src/features/market_candles/tinkoff_shares_1m_historical/updater.rs
use std::sync::Arc;
use tokio::time;
use std::time::Duration;
use chrono::Timelike;
use tracing::{error, info};

use super::service::HistoricalCandleDataService;
use crate::env_config::models::app_setting::AppSettings;
use crate::features::db::MongoDb;
use crate::services::tinkoff::client_grpc::TinkoffClient;

pub struct HistoricalCandleUpdater {
    service: Arc<HistoricalCandleDataService>,
    settings: Arc<AppSettings>,
}

impl HistoricalCandleUpdater {
    pub fn new(
        client: Arc<TinkoffClient>,
        mongo_db: Arc<MongoDb>,
        settings: Arc<AppSettings>,
    ) -> Self {
        let service = Arc::new(HistoricalCandleDataService::new(
            client,
            mongo_db,
            settings.clone(),
        ));
        
        Self {
            service,
            settings,
        }
    }
    
    pub async fn start_update_loop(self) {
        info!("Starting historical candle updater");
        
        if !self.settings.app_config.historical_candle_updater.enabled {
            info!("Historical candle updater is disabled in configuration");
            return;
        }
        
        info!(
            "Starting historical candle update service (scheduled from {} to {}, timezone: {})",
            self.settings.app_config.historical_candle_updater.update_start_time,
            self.settings.app_config.historical_candle_updater.update_end_time,
            self.settings.app_config.historical_candle_updater.timezone
        );
        
        // Run immediately on startup if configured
        if self.settings.app_config.historical_candle_updater.run_on_startup {
            info!("Running historical candle update on startup");
            self.update_historical_candles().await;
        }
        
        // Проверка каждую минуту, не пора ли запускать обновление
        let mut interval = time::interval(Duration::from_secs(60));
        
        loop {
            interval.tick().await;
            
            // Проверяем, находимся ли мы в начале временного окна для запуска
            let should_start = self.should_start_update();
            
            if should_start {
                info!("Starting scheduled historical candle update");
                self.update_historical_candles().await;
            }
        }
    }
    
    // Проверяет, нужно ли начинать обновление прямо сейчас
    fn should_start_update(&self) -> bool {
        let update_config = &self.settings.app_config.historical_candle_updater;
        
        // Парсим временную зону и текущее время
        let timezone: chrono_tz::Tz = update_config.timezone.parse().expect("Invalid timezone");
        let current_time = chrono::Utc::now().with_timezone(&timezone);
        
        // Парсим время начала и конца обновления
        let start_parts: Vec<&str> = update_config.update_start_time.split(':').collect();
        let end_parts: Vec<&str> = update_config.update_end_time.split(':').collect();
        
        if start_parts.len() != 2 || end_parts.len() != 2 {
            error!("Invalid time format in configuration");
            return false;
        }
        
        let start_hour: u32 = match start_parts[0].parse() {
            Ok(h) => h,
            Err(e) => {
                error!("Failed to parse start hour: {}", e);
                return false;
            }
        };
        
        let start_minute: u32 = match start_parts[1].parse() {
            Ok(m) => m,
            Err(e) => {
                error!("Failed to parse start minute: {}", e);
                return false;
            }
        };
        
        let end_hour: u32 = match end_parts[0].parse() {
            Ok(h) => h,
            Err(e) => {
                error!("Failed to parse end hour: {}", e);
                return false;
            }
        };
        
        let end_minute: u32 = match end_parts[1].parse() {
            Ok(m) => m,
            Err(e) => {
                error!("Failed to parse end minute: {}", e);
                return false;
            }
        };
        
        // Преобразуем все времена в минуты для простоты сравнения
        let current_time_minutes = current_time.hour() * 60 + current_time.minute();
        let start_time_minutes = start_hour * 60 + start_minute;
        let end_time_minutes = end_hour * 60 + end_minute;
        
        // Проверка, находится ли текущее время в диапазоне обновления
        let in_update_window = if start_time_minutes <= end_time_minutes {
            // Обычный случай: 00:00 - 05:00
            current_time_minutes >= start_time_minutes && current_time_minutes <= end_time_minutes
        } else {
            // Случай перехода через полночь: 22:00 - 03:00
            current_time_minutes >= start_time_minutes || current_time_minutes <= end_time_minutes
        };
        
        // Проверяем, прошла ли минута с последнего запуска
        // Можно добавить состояние для отслеживания последнего запуска
        if in_update_window {
            // Добавьте дополнительную логику здесь, чтобы не запускать слишком часто
            // Например, сохраняйте время последнего запуска и проверяйте, прошло ли хотя бы 60 минут
            
            info!(
                "Current time ({:02}:{:02}) is within update window ({:02}:{:02} - {:02}:{:02})",
                current_time.hour(), current_time.minute(),
                start_hour, start_minute,
                end_hour, end_minute
            );
            
            return true;
        }
        
        false
    }
    
    async fn update_historical_candles(&self) {
        info!("Starting historical candle update");
        
        // Выполняем обновление исторических данных
        self.service.start().await;
        
        info!("Historical candle update completed");
    }
}

pub async fn start_historical_candle_updater(
    client: Arc<TinkoffClient>,
    mongo_db: Arc<MongoDb>,
    settings: Arc<AppSettings>,
) {
    let updater = HistoricalCandleUpdater::new(client, mongo_db, settings);
    
    tokio::spawn(async move {
        updater.start_update_loop().await;
    });
}
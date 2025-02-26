use chrono::{DateTime, TimeZone, Utc};
use prost_types::Timestamp;
use serde::{Deserialize, Serialize};


/// Human-readable Timestamp model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanTimestamp {
    pub seconds: i64,
    pub nanos: i32,
    pub iso_string: String,
}

impl From<&Timestamp> for HumanTimestamp {
    fn from(ts: &Timestamp) -> Self {
        let seconds = ts.seconds;
        let nanos = ts.nanos;
        
        // Используем рекомендуемую функцию DateTime::from_timestamp
        let iso_string = DateTime::from_timestamp(seconds, nanos as u32)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| {
                // Для дат, которые не могут быть обработаны стандартным методом,
                // используем альтернативный подход через TimeZone
                if seconds < 0 {
                    // Для дат до 1970 года
                    match Utc.timestamp_opt(seconds, nanos as u32) {
                        chrono::offset::LocalResult::Single(dt) => dt.format("%Y-%m-%d %H:%M:%S").to_string(),
                        _ => "Invalid date".to_string()
                    }
                } else {
                    "Invalid date".to_string()
                }
            });
            
        Self {
            seconds,
            nanos,
            iso_string,
        }
    }
}
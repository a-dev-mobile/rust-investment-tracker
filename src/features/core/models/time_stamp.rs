use chrono::{DateTime, Utc};
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
        let iso_string = DateTime::<Utc>::from_timestamp(seconds, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "Invalid date".to_string());
            
        Self {
            seconds,
            nanos,
            iso_string,
        }
    }
}
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct WatchedInstrument {
    pub figi: String,
    pub subscription_interval_id: i32,
}

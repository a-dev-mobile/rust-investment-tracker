// use chrono::{DateTime, NaiveDateTime, Utc};
// use serde::{Deserialize, Serialize};
// use serde_json::json;
// use sqlx::PgPool;
// use std::{result, sync::Arc};
// use tracing::{debug, error, info};

// use crate::features::stream::models::{candle::MyCandle, watched_instrument::WatchedInstrument};
// #[derive(Clone)]
// pub struct StreamRepository {
//     db_pool: Arc<PgPool>,
// }

// impl StreamRepository {
//     pub fn new(db_pool: Arc<PgPool>) -> Self {
//         Self { db_pool }
//     }

//     pub async fn get_active_instruments(&self) -> Vec<WatchedInstrument> {
//         debug!("Fetching active watched instruments");

//         let result: Result<Vec<WatchedInstrument>, sqlx::Error> =
//             sqlx::query_as::<_, WatchedInstrument>(
//                 "
// SELECT figi, watched_instruments.subscription_interval_id
// FROM instrument_services.watched_instruments
// WHERE is_active = true
//         ",
//             )
//             .fetch_all(&*self.db_pool)
//             .await;

//         match result {
//             Ok(v) => {
//                 debug!("Found {} active watched instruments", v.len());
//                 return v;
//             }
//             Err(e) => {
//                 error!("Failed to fetch active watched instruments: {}", e);
//                 return vec![];
//             }
//         }
//     }

//     pub async fn save_candle(&self, candle: MyCandle)  {
//         let candles_json = json!([candle]); // Сразу сериализуем MyCandle в JSON

//         // match sqlx::query!(
//         //     "SELECT instrument_services.update_candles($1) as affected_rows",
//         //     &candles_json
//         // )
//         // .fetch_one(&*self.db_pool)
//         // .await {
//         //     Ok(result) => debug!("Successfully saved candle, affected rows: {:?}", result.affected_rows),
//         //     Err(e) => error!("Failed to save candle: {}", e),
//         // }
//     }
// }

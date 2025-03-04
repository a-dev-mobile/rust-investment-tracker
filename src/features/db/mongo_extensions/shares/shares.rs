// src/features/db/mongo_extensions/shares/shares.rs
use futures::TryStreamExt;

use crate::features::db::{MongoDb, mongo_db::{DbNames, Collections}};

use mongodb::bson::{doc, Document};
use tracing::{info, error};

impl MongoDb {
    /// Получает список всех уникальных FIGI из коллекции tinkoff_shares
    pub async fn get_unique_figis(&self) -> Vec<String> {
        info!("Fetching unique FIGIs from tinkoff_shares collection");
        
        // Создаем агрегационный пайплайн для получения уникальных FIGI
        let pipeline = vec![
            doc! {
                "$group": {
                    "_id": "$figi"
                }
            }
        ];
        
        // Получаем коллекцию и выполняем агрегацию
        let result = self
            .database(DbNames::MARKET_DATA)
            .collection::<Document>(Collections::TINKOFF_SHARES)
            .aggregate(pipeline)
            .await;
            
        // Обрабатываем результат агрегации
        match result {
            Ok(cursor) => {
                match cursor.try_collect::<Vec<Document>>().await {
                    Ok(documents) => {
                        // Извлекаем FIGI прямо из _id
                        let figis: Vec<String> = documents
                            .into_iter()
                            .filter_map(|doc| doc.get_str("_id").ok().map(String::from))
                            .collect();
                            
                        info!("Found {} unique FIGIs", figis.len());
                        figis
                    }
                    Err(e) => {
                        error!("Failed to collect FIGI documents: {}", e);
                        Vec::new()
                    }
                }
            }
            Err(e) => {
                error!("Failed to execute aggregation for FIGIs: {}", e);
                Vec::new()
            }
        }
    }
}
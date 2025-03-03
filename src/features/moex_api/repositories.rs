use crate::db::mongo_db::{Collections, DbNames, MongoDb};
use crate::features::moex_api::models::CurrencyRatesResponse;
use futures::stream::TryStreamExt;
use mongodb::bson::{doc, Document};
use std::sync::Arc;
use tracing::info;

pub struct CurrencyRatesRepository {
    mongo_db: Arc<MongoDb>,
}

impl CurrencyRatesRepository {
    pub fn new(mongo_db: Arc<MongoDb>) -> Self {
        Self { mongo_db }
    }

    /// Сохраняет данные о курсах валют в MongoDB
    pub async fn save_currency_rates(
        &self,
        rates: &CurrencyRatesResponse,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        info!("Saving currency rates to MongoDB");

        // Получаем коллекцию для курсов валют
        let collection = self
            .mongo_db
            .database(DbNames::MARKET_REFERENCE)
            .collection::<Document>(Collections::CURRENCY_RATES);

        // Преобразуем rates в BSON Document
        let rates_doc = mongodb::bson::to_document(&rates)?;

        // Clear existing data
        collection.delete_many(doc! {}).await?;
        info!("Previous records deleted from bonds collection");

        let result = collection.insert_one(rates_doc).await?;


        info!(
            "Currency rates successfully saved to MongoDB. Inserted ID: {:?}",
            result.inserted_id
        );

        Ok(true)
    }
}

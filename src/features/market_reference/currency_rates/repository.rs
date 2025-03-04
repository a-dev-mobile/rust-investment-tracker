
use crate::features::db::mongo_db::{Collections, DbNames};
use crate::features::{db::MongoDb, moex_api::models::MoexRatesResponse};
use crate::features::market_reference::currency_rates::models::CurrencyRatesResponse;
use crate::features::market_reference::currency_rates::mappers::MoexRatesMapper;

use mongodb::bson::{doc, Document};
use std::sync::Arc;
use tracing::{info};

pub struct CurrencyRatesRepository {
    mongo_db: Arc<MongoDb>,
}

impl CurrencyRatesRepository {
    pub fn new(mongo_db: Arc<MongoDb>) -> Self {
        Self { mongo_db }
    }

    // Метод принимает MoexRatesResponse и сохраняет его в базу данных
    pub async fn save_moex_rates(
        &self,
        moex_rates: &MoexRatesResponse,
    ) -> Result<CurrencyRatesResponse, Box<dyn std::error::Error + Send + Sync>>  {
        info!("Processing and saving currency rates to MongoDB");

        // Преобразуем MoexRatesResponse в CurrencyRatesResponse
        let currency_rates = MoexRatesMapper::map_to_currency_rates(moex_rates)?;

        // Получаем коллекцию для курсов валют
        let collection = self
            .mongo_db
            .database(DbNames::MARKET_REFERENCE)
            .collection::<Document>(Collections::CURRENCY_RATES);

        // Преобразуем currency_rates в BSON Document
        let rates_doc = mongodb::bson::to_document(&currency_rates)?;

        // Очищаем существующие данные
        collection.delete_many(doc! {}).await?;
        info!("Previous records deleted from currency rates collection");

        // Сохраняем новые данные
        let result = collection.insert_one(rates_doc).await?;
        info!(
            "Currency rates successfully saved to MongoDB. Inserted ID: {:?}",
            result.inserted_id
        );

        // Возвращаем преобразованные данные
        Ok(currency_rates)
    }


}
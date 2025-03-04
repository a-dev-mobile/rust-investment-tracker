

use crate::features::{
    db::{
        mongo_db::{Collections, DbNames},
        mongo_extensions::currency_rates::mappers::MoexRatesMapper,
        MongoDb,
    },

    moex_api::models::MoexRatesResponse,
};

use mongodb::bson::{doc, Document};
use tracing::info;

use super::models::CurrencyRatesResponse;

impl MongoDb {
    pub async fn save_currency_rates(
        &self,
        moex_rates: &MoexRatesResponse,
    ) -> Result<CurrencyRatesResponse, Box<dyn std::error::Error + Send + Sync>> {
        info!("Processing and saving currency rates to MongoDB");

        // Map MoexRatesResponse to CurrencyRatesResponse using the mapper from mongo_extensions
        let currency_rates = MoexRatesMapper::map_to_currency_rates(moex_rates)?;

        // Get the collection for currency rates
        let collection = self
            .database(DbNames::MARKET_REFERENCE)
            .collection::<Document>(Collections::CURRENCY_RATES);

        // Convert currency_rates to BSON Document
        let rates_doc = mongodb::bson::to_document(&currency_rates)?;

        // Clear existing data
        collection.delete_many(doc! {}).await?;
        info!("Previous records deleted from currency rates collection");

        // Save new data
        let result = collection.insert_one(rates_doc).await?;
        info!(
            "Currency rates successfully saved to MongoDB. Inserted ID: {:?}",
            result.inserted_id
        );

        // Return the converted data
        Ok(currency_rates)
    }

    pub async fn get_currency_rates(&self) -> Option<CurrencyRatesResponse> {
        info!("Fetching currency rates from MongoDB");

        let collection = self
            .database(DbNames::MARKET_REFERENCE)
            .collection::<Document>(Collections::CURRENCY_RATES);

        match collection.find_one(doc! {}).await {
            Ok(Some(doc)) => {
                match bson::from_document::<CurrencyRatesResponse>(doc) {
                    Ok(rates) => {
                        info!("Successfully retrieved currency rates");
                        Some(rates)
                    }
                    Err(e) => {
                        tracing::error!("Failed to deserialize currency rates: {}", e);
                        None
                    }
                }
            }
            Ok(None) => {
                info!("No currency rates found in the database");
                None
            }
            Err(e) => {
                tracing::error!("Error fetching currency rates: {}", e);
                None
            }
        }
    }
}
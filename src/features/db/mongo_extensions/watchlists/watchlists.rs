// src/db/mongo_extensions/watchlists.rs

use crate::features::{
    db::{
        mongo_db::{Collections, DbNames},
        MongoDb,
    },

};
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use tracing::{error, info};

use super::models::DbUserConfigWatchlist;

impl MongoDb {
    /// Get all watchlists from the database, regardless of enabled status
    /// Returns an empty vector in case of an error
    pub async fn get_watchlists(&self) -> Vec<DbUserConfigWatchlist> {
        info!("Fetching all watchlists");

        let collection = match self
            .database(DbNames::USER_CONFIG)
            .collection::<DbUserConfigWatchlist>(Collections::WATCHLISTS)
            .find(doc! {})
            .await
        {
            Ok(cursor) => cursor,
            Err(e) => {
                error!("Failed to fetch watchlists: {}", e);
                return Vec::new();
            }
        };

        // Convert cursor to Vec
        match collection.try_collect::<Vec<DbUserConfigWatchlist>>().await {
            Ok(watchlists) => {
                let count = watchlists.len();
                info!("Found {} watchlists", count);
                watchlists
            }
            Err(e) => {
                error!("Failed to collect watchlists: {}", e);
                Vec::new()
            }
        }
    }

    /// Get only enabled watchlists from the database
    /// Returns an empty vector in case of an error
    pub async fn get_enabled_watchlists(&self) -> Vec<DbUserConfigWatchlist> {
        info!("Fetching enabled watchlists");

        let collection = match self
            .database(DbNames::USER_CONFIG)
            .collection::<DbUserConfigWatchlist>(Collections::WATCHLISTS)
            .find(doc! { "enabled": true })
            .await
        {
            Ok(cursor) => cursor,
            Err(e) => {
                error!("Failed to fetch enabled watchlists: {}", e);
                return Vec::new();
            }
        };

        // Convert cursor to Vec
        let result: Vec<DbUserConfigWatchlist> = match collection.try_collect::<Vec<DbUserConfigWatchlist>>().await {
            Ok(watchlists) => {
                let count = watchlists.len();
                info!("Found {} enabled watchlists", count);
                watchlists
            }
            Err(e) => {
                error!("Failed to collect enabled watchlists: {}", e);
                Vec::new()
            }
        };
        result
    }
}

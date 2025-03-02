use crate::db::mongo_db::{Collections, DbNames, MongoDb};
use crate::features::user_config::watchlists::models::DbUserConfigWatchlist;
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use std::sync::Arc;
use tracing::{ info};

pub struct WatchlistService {
    mongo_db: Arc<MongoDb>,
}

impl WatchlistService {
    pub fn new(mongo_db: Arc<MongoDb>) -> Self {
        Self { mongo_db }
    }

    /// Get all watchlists for a user
    pub async fn get_watchlists(&self) -> Result<Vec<DbUserConfigWatchlist>, Box<dyn std::error::Error>> {
        info!("Fetching watchlists");
        
        // Get the watchlists collection - assuming there's a collection for watchlists
        // You might need to define this collection in the MongoDb struct
        let collection = self
            .mongo_db
            .database(DbNames::USER_CONFIG)
            .collection::<DbUserConfigWatchlist>(Collections::WATCHLISTS); 
        
        // Find all watchlist documents
        let cursor = collection.find(doc! {}).await?;
        
        // Convert cursor to Vec
        let watchlists: Vec<DbUserConfigWatchlist> = cursor.try_collect().await?;
        
        info!("Found {} watchlists", watchlists.len());
        dbg!(&watchlists);
        Ok(watchlists)
    }
    
    /// Get watchlists filtered by enabled status
    pub async fn get_enabled_watchlists(&self) -> Result<Vec<DbUserConfigWatchlist>, Box<dyn std::error::Error>> {
        info!("Fetching enabled watchlists");
        
        let collection = self
        .mongo_db
        .database(DbNames::USER_CONFIG)
        .collection::<DbUserConfigWatchlist>(Collections::WATCHLISTS);
        
        // Only get enabled watchlists
        let cursor = collection.find(doc! { "enabled": true }).await?;
        
        let watchlists: Vec<DbUserConfigWatchlist> = cursor.try_collect().await?;
        
        info!("Found {} enabled watchlists", watchlists.len());
        
        Ok(watchlists)
    }
}

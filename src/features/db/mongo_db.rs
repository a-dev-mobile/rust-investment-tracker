use crate::env_config::models::app_setting::AppSettings;
use mongodb::bson::Document;
use mongodb::{options::ClientOptions, Client, Collection, Database as MongoDatabase};
use std::time::Duration;
use tracing::{error, info};

// Database names constant
pub struct DbNames;
impl DbNames {
    pub const MARKET_DATA: &'static str = "market_data";
    pub const MARKET_SERVICES: &'static str = "market_services";
    pub const USER_CONFIG: &'static str = "user_config";
    pub const MARKET_REFERENCE: &'static str = "market_reference";
    pub const MARKET_CANDLES: &'static str = "market_candles";
}

// Collection names constant
pub struct Collections;
impl Collections {
    // Market data collections
    pub const WATCHLISTS: &'static str = "watchlists";
    pub const TINKOFF_SHARES: &'static str = "tinkoff_shares";
    pub const TINKOFF_BONDS: &'static str = "tinkoff_bonds";
    pub const TINKOFF_ETFS: &'static str = "tinkoff_etfs";
    pub const TINKOFF_FUTURES: &'static str = "tinkoff_futures";
    pub const STATUS: &'static str = "_status";
    pub const CURRENCY_RATES: &'static str = "currency_rates";

    pub const CANDLES_TRACKING: &'static str = "candles_tracking";
    pub const TINKOFF_1M: &'static str = "tinkoff_1m";
    pub const TINKOFF_1M_HISTORICAL: &'static str = "tinkoff_1m_historical";
}

#[derive(Clone)]
pub struct MongoDb {
    pub client: Client,
    pub default_database: MongoDatabase,
}

impl MongoDb {
    pub async fn connect(settings: &AppSettings) -> Self {
        info!("Connecting to MongoDB...");

        // Configure MongoDB client options
        let mut client_options = ClientOptions::parse(&settings.app_env.mongo_url)
            .await
            .expect("Failed to parse MongoDB connection string");

        // Set a timeout for server selection
        client_options.connect_timeout = Some(Duration::from_secs(
            settings.app_config.mongo_db.timeout_seconds,
        ));

        // Set connection pool size
        client_options.max_pool_size = Some(settings.app_config.mongo_db.pool_size);

        // Set the app name if it exists
        client_options.app_name = Some("rust-market-api".to_string());

        // Get a handle to the deployment
        let client =
            Client::with_options(client_options).expect("Failed to initialize MongoDB client");

        // Default database selection (you can change this to market_data or another default)
        let default_database = client.database(DbNames::MARKET_DATA);

        // Ping the database to verify connection works
        match client
            .database("admin")
            .run_command(bson::doc! {"ping": 1})
            .await
        {
            Ok(_) => info!("Successfully connected to MongoDB"),
            Err(e) => error!("Failed to ping MongoDB server: {}", e),
        }

        MongoDb {
            client,
            default_database,
        }
    }

    // Helper methods to get specific databases or collections
    pub fn database(&self, name: &str) -> MongoDatabase {
        self.client.database(name)
    }
    // Convenience methods for commonly used collections
    pub fn shares_collection(&self) -> Collection<Document> {
        self.client
            .database(DbNames::MARKET_DATA)
            .collection(Collections::TINKOFF_SHARES)
    }

    pub fn bonds_collection(&self) -> Collection<Document> {
        self.client
            .database(DbNames::MARKET_DATA)
            .collection(Collections::TINKOFF_BONDS)
    }

    pub fn status_collection(&self) -> Collection<Document> {
        self.client
            .database(DbNames::MARKET_DATA)
            .collection(Collections::STATUS)
    }
    pub fn etfs_collection(&self) -> Collection<Document> {
        self.client
            .database(DbNames::MARKET_DATA)
            .collection::<Document>(Collections::TINKOFF_ETFS)
    }
    pub fn futures_collection(&self) -> Collection<Document> {
        self.client
            .database(DbNames::MARKET_DATA)
            .collection::<Document>(Collections::TINKOFF_FUTURES)
    }

    pub fn candles_tracking_collection(&self) -> Collection<Document> {
        self.client
            .database(DbNames::MARKET_SERVICES)
            .collection::<Document>(Collections::CANDLES_TRACKING)
    }
    pub fn get_historical_collection(&self) -> Collection<Document> {
        self.client
            .database(DbNames::MARKET_CANDLES)
            .collection::<Document>(Collections::TINKOFF_1M_HISTORICAL)
    }
}

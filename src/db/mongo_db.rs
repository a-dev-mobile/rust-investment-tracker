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
}

// Collection names constant
pub struct Collections;
impl Collections {
    // Market data collections
    pub const SHARES: &'static str = "shares";
    pub const BONDS: &'static str = "bonds";
    pub const ETFS: &'static str = "etfs";
    pub const FUTURES: &'static str = "futures";
    pub const STATUS: &'static str = "_status";

    pub const CANDLES_TRACKING: &'static str = "candles_tracking";
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

    pub fn market_data_db(&self) -> MongoDatabase {
        self.client.database(DbNames::MARKET_DATA)
    }
    pub fn market_service_db(&self) -> MongoDatabase {
        self.client.database(DbNames::MARKET_SERVICES)
    }
    // Convenience methods for commonly used collections
    pub fn shares_collection(&self) -> Collection<Document> {
        self.market_data_db().collection(Collections::SHARES)
    }

    pub fn bonds_collection(&self) -> Collection<Document> {
        self.market_data_db().collection(Collections::BONDS)
    }

    pub fn status_collection(&self) -> Collection<Document> {
        self.market_data_db().collection(Collections::STATUS)
    }
    pub fn etfs_collection(&self) -> Collection<Document> {
        self.market_data_db()
            .collection::<Document>(Collections::ETFS)
    }
    pub fn futures_collection(&self) -> Collection<Document> {
        self.market_data_db()
            .collection::<Document>(Collections::FUTURES)
    }

    pub fn candles_tracking_collection(&self) -> Collection<Document> {
        self.market_service_db()
            .collection::<Document>(Collections::CANDLES_TRACKING)
    }
}

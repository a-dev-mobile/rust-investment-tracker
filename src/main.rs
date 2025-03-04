use crate::{
    db::{mongo_db::MongoDb, PostgresDb},
    layers::{create_cors, create_trace},
    logger::init_logger,
};
use axum::{routing::get, Router};
use dotenv::dotenv;
use env_config::models::{
    app_config::AppConfig,
    app_env::{AppEnv, Env},
    app_setting::AppSettings,
};
use features::{
    market_candles::tinkoff_1m_historical::{start_historical_candle_service, HistoricalCandleDataService}, market_data::TinkoffInstrumentsUpdater, market_reference::currency_rates::CurrencyRatesRepository, tinkoff_market_data_stream::MarketDataStreamer, user_config::watchlists::models::DbUserConfigWatchlist
};
use features::{
    market_reference::currency_rates::CurrencyRatesUpdater, moex_api::MoexApiClient,
    user_config::watchlists::WatchlistService,
};

use services::tinkoff::client_grpc::TinkoffClient;
use sqlx::PgPool;

use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tracing::{debug, info};

mod api;
mod db;
mod enums;
mod env_config;
mod features;

mod gen;
mod layers;
mod logger;
mod middleware;
mod services;

mod utils;

/// Initialize application settings and logger
async fn initialize() -> AppSettings {
    let app_env = AppEnv::new();
    let app_config = AppConfig::new(&app_env.env);

    let app_settings = AppSettings {
        app_config,
        app_env: app_env.clone(),
    };

    // Initialize logger with settings
    init_logger(
        &app_settings.app_config.log.level,
        &app_settings.app_config.log.format,
    )
    .expect("Failed to initialize logger");

    // Log startup information
    tracing::info!("Starting application...");
    tracing::info!("Current environment: {}", app_env.env.to_string());

    if Env::is_dev(&app_env.env) {
        // Additional debug logging only in development
        tracing::debug!("Debug logging enabled");
        tracing::info!("Development mode active");
    }

    app_settings
}

/// Setup database connections
async fn setup_databases(settings: &AppSettings) -> (PostgresDb, MongoDb) {
    // Connect to PostgreSQL
    let postgres_db = PostgresDb::connect(settings).await;

    // Connect to MongoDB
    let mongo_db = MongoDb::connect(settings).await;

    (postgres_db, mongo_db)
}

/// Create and configure the application router
fn create_app(postgres_db: PostgresDb, mongo_db: MongoDb) -> Router {
    Router::new()
        .layer(create_cors())
        .route("/api-health", get(api::health_api))
        .route("/db-health", get(api::health_db))
        .layer(axum::Extension(postgres_db.clone()))
        .layer(axum::Extension(mongo_db.clone()))
        .layer(create_trace())
}

/// Start the share updater background service
pub async fn start_tinkoff_market_data_updater(
    postgres_db: Arc<PgPool>,
    mongo_db: Arc<MongoDb>,
    settings: Arc<AppSettings>,
    client: Arc<TinkoffClient>,
) {
    let updater = TinkoffInstrumentsUpdater::new(postgres_db, mongo_db, settings, client).await;
    tokio::spawn(async move {
        updater.start_update_loop().await;
    });
}

/// Start the HTTP server
async fn run_server(app: Router, addr: SocketAddr) {
    tracing::info!("Starting server on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    tracing::info!("Server started successfully");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

#[tokio::main]
async fn main() {
    // Load .env file at the beginning
    dotenv().ok();

    // Initialize application
    let settings = Arc::new(initialize().await);
    debug!("{:?}", settings);

    // Parse server address
    let http_addr: SocketAddr = format!(
        "{}:{}",
        settings.app_env.server_address, settings.app_env.server_port,
    )
    .parse()
    .expect("Invalid server address configuration - cannot start server");

    // Setup databases
    let (postgres_db, mongo_db) = setup_databases(&settings).await;
    let db_pool = Arc::new(postgres_db.pool);
    let mongodb_arc = Arc::new(mongo_db.clone());

    // Create application router
    let app = create_app(
        PostgresDb {
            pool: (*db_pool).clone(),
        },
        mongo_db,
    );

    // Initialize Tinkoff client
    let tinkoff_client = Arc::new(
        TinkoffClient::new(settings.clone())
            .await
            .expect("Failed to initialize Tinkoff client"),
    );

    // Start background services
    // start_candles_updater(db_pool.clone(), settings.clone(), tinkoff_client.clone()).await;
    start_tinkoff_market_data_updater(
        db_pool.clone(),
        mongodb_arc.clone(),
        settings.clone(),
        tinkoff_client.clone(),
    )
    .await;

    let watchlist_service = Arc::new(WatchlistService::new(mongodb_arc.clone()));

    let vec_watchlists = watchlist_service.get_watchlists().await.unwrap();

    // Start the market data stream with the watchlists
    start_market_data_stream(
        settings.clone(),
        tinkoff_client.clone(),
        mongodb_arc.clone(),
        vec_watchlists,
    )
    .await;

    start_currency_rates_updater(mongodb_arc.clone(), settings.clone()).await;

    // let client = MoexApiClient::new();
    // let security_info = client.get_security_info("GAZP").await;
    // dbg!(security_info);

    // In the main function, after initializing other services
    // Add this after initializing the watchlist_service
    let historical_candle_service = Arc::new(HistoricalCandleDataService::new(
        tinkoff_client.clone(),
        mongodb_arc.clone(),
        watchlist_service.clone(),
        settings.clone(),
    ));

    // Start the historical candle data service
    start_historical_candle_service(historical_candle_service).await;

    // Start HTTP server
    run_server(app, http_addr).await;
}

async fn start_currency_rates_updater(mongo_db: Arc<MongoDb>, settings: Arc<AppSettings>) {
    // Инициализация API клиента
    let api_client = MoexApiClient::new();

    // Инициализация репозитория
    let repository = Arc::new(CurrencyRatesRepository::new(mongo_db.clone()));

    // Создаем и запускаем планировщик обновлений с настройками
    let updater = CurrencyRatesUpdater::new(api_client, repository, settings);

    // Запускаем планировщик в отдельной задаче
    tokio::spawn(async move {
        updater.start_update_loop().await;
    });
}

/// Start the market data stream service
async fn start_market_data_stream(
    settings: Arc<AppSettings>,
    client: Arc<TinkoffClient>,
    mongo_db: Arc<MongoDb>,
    watchlists: Vec<DbUserConfigWatchlist>,
) {
    // Create a new MarketDataStreamer with watchlists data
    let streamer = MarketDataStreamer::new(settings, client, mongo_db, watchlists);

    // Start the streaming process in a separate task
    tokio::spawn(async move {
        streamer.start_streaming().await;
    });

    info!("Market data stream service started");
}

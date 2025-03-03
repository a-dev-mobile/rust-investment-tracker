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
    candles_tracking::CandlesTrackingUpdater, moex_api::MoexApiService,
    user_config::watchlists::WatchlistService,
};
use features::{
    market_data_updater::TinkoffInstrumentsUpdater,
    moex_api::{CurrencyRatesRepository, CurrencyRatesUpdater},
};

use services::tinkoff::client_grpc::TinkoffClient;
use sqlx::PgPool;

use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tracing::debug;

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
pub async fn start_candles_tracking_updater(
    postgres_db: Arc<PgPool>,
    mongo_db: Arc<MongoDb>,
    settings: Arc<AppSettings>,
    client: Arc<TinkoffClient>,
) {
    let updater = CandlesTrackingUpdater::new(postgres_db, mongo_db, settings, client).await;
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

    start_candles_tracking_updater(
        db_pool.clone(),
        mongodb_arc.clone(),
        settings.clone(),
        tinkoff_client.clone(),
    )
    .await;

    let watchlist_service = Arc::new(WatchlistService::new(mongodb_arc.clone()));

    let a = watchlist_service.get_watchlists().await.unwrap();

    // Инициализация сервисов для работы с курсами валют
    let moex_api_service = MoexApiService::new();
    let currency_repository = Arc::new(CurrencyRatesRepository::new(mongodb_arc.clone()));
    let currency_updater = CurrencyRatesUpdater::new(moex_api_service, currency_repository.clone());

    // Запуск фонового процесса для периодического обновления курсов валют
    tokio::spawn(async move {
        currency_updater.start_update_loop().await;
    });

      // Обновление курсов валют при запуске и вывод информации
    // match currency_updater.update_currency_rates().await {
    //     Ok(rates) => {
    //         println!("Курсы валют обновлены. Дата: {}", rates.date);
            
    //         // Вывод информации о курсах в консоль
    //         for (code, info) in rates.display_info {
    //             println!("{}: {} ({})", code, info.text, info.change_text);
    //         }
    //     }
    //     Err(e) => {
    //         eprintln!("Ошибка при обновлении курсов валют: {}", e);
    //     }
    // }
    
    // Тестовый пример получения курсов из базы данных
    // match currency_repository.get_latest_currency_rates().await {
    //     Ok(Some(rates)) => {
    //         println!("Последние курсы валют из БД: {}", rates.date);
    //     }
    //     Ok(None) => {
    //         println!("Курсы валют в БД пока отсутствуют");
    //     }
    //     Err(e) => {
    //         eprintln!("Ошибка при получении курсов валют из БД: {}", e);
    //     }
    // }


    // Start HTTP server
    run_server(app, http_addr).await;
}

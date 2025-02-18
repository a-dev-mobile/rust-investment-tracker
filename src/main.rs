use crate::{
    db::Database,
    layers::{create_cors, create_trace},
    logger::init_logger,
};
use axum::{routing::get, Router};
use dotenv::dotenv;
use env_config::models::{
    app_config::{self, AppConfig},
    app_env::{AppEnv, Env},
    app_setting::{self, AppSettings},
};
use sqlx::PgPool;
use features::share_updater::{ShareUpdater};
use std::io::Result;
use std::io::{Error, ErrorKind};
use std::{net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, sync::Mutex};
use tracing::debug;

mod api;
mod db;
mod enums;
mod env_config;
mod features;


mod services;
mod layers;
mod gen;
mod logger;
mod middleware;

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

/// Setup database connection
async fn setup_database(settings: &AppSettings) -> Database {
    let db = Database::connect(settings).await;

    db
}

/// Create and configure the application router
fn create_app(db: Database) -> Router {
    Router::new()
        .layer(create_cors())
        .route("/api-health", get(api::health_api))
        .route("/db-health", get(api::health_db))
        // TODO: Add v1 routes
        // .nest("/v1", routes::v1::router(db.pool.clone()))
        .layer(axum::Extension(db.pool.clone()))
        .layer(create_trace())
}

// Usage in main.rs:
pub async fn start_share_updater(db_pool: Arc<PgPool>, settings: Arc<AppSettings>) {
    let updater = ShareUpdater::new(db_pool, settings).await;
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
    // Загружаем .env файл в самом начале
    dotenv().ok();
    // Initialize application
    let settings = Arc::new(initialize().await);
    debug!("{:?}", settings);
    // Parse addresses
    let http_addr: SocketAddr = format!(
        "{}:{}",
        settings.app_env.server_address, settings.app_env.server_port,
    )
    .parse()
    .expect("Invalid server address configuration - cannot start server");

    let db = setup_database(&settings).await;
    let db_pool = Arc::new(db.pool);

    // Start candles updater
    // candles_updater::start_candles_updater(db_pool.clone(), settings.clone()).await;

    // Create app with database connection
    let app = create_app(Database {
        pool: (*db_pool).clone(),
    });

    // let mut tinkoffApiClient = TinkoffClient::new(settings)
    //     .await
    //     .expect("Failed to initialize Tinkoff client - cannot proceed without API access");

    // let a = tinkoffApiClient.create_request(InstrumentsRequest {
    //     instrument_status: InstrumentStatus::Base as i32,
    // });

    // println!("{:?}", a);

    // let shares = tinkoffApiClient.get_shares().await.unwrap();
    // pub async fn get_shares(&mut self) -> Result<SharesResponse>  {
    //     let request = self.create_request(InstrumentsRequest {
    //         instrument_status: InstrumentStatus::All as i32,
    //     }).expect("Failed to create request");

    //     self.instruments
    //         .shares(request)
    //         .await
    //         .map(|response| response.into_inner())
    //         .map_err(|e| AppError::ExternalServiceError(format!("Failed to get shares: {}", e)))
    // }

    // println!(
    //     "{:?}",
    //     shares.instruments.first().unwrap().country_of_risk_name
    // );
    start_share_updater(db_pool.clone(), settings.clone()).await;
    // Запуск сервера критичен, используем expect
    run_server(app, http_addr).await;
}

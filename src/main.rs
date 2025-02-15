use crate::config::get_settings;
use crate::{
    config::Settings,
    db::Database,
    layers::{create_cors, create_trace},
    logger::init_logger,
};
use axum::{routing::get, Router};

use gen::tinkoff_public_invest_api_contract_v1::{InstrumentStatus, InstrumentsRequest};
use services::TinkoffClient;
use std::io::Result;
use std::io::{Error, ErrorKind};
use std::{net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, sync::Mutex};

mod api;
mod config;
mod db;
mod enums;

mod gen;
mod layers;
mod logger;
mod middleware;
mod services;
mod utils;

/// Initialize application settings and logger
async fn initialize() -> Settings {
    // Создаем обычные настройки
    let settings = get_settings()
        .expect("Failed to initialize settings - application cannot start without configuration");

    // Initialize logger with settings
    init_logger(&settings.log.level, &settings.log.format).expect("Failed to initialize logger");

    // Log startup information
    tracing::info!("Starting application...");
    tracing::info!("Current environment: {}", settings.environment().as_str());

    if settings.is_dev() {
        // Additional debug logging only in development
        tracing::debug!("Debug logging enabled");
        tracing::info!("Development mode active");
    }

    settings
}

/// Setup database connection
async fn setup_database(settings: &Settings) -> Database {
    tracing::info!("Initializing database connection...");
    let db = Database::connect(settings)
        .await
        .expect("Failed to connect to database - application cannot function without database");
    tracing::info!("Database connection established");
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
    // Initialize application
    let settings = Arc::new(initialize().await);

    // Parse addresses
    let http_addr: SocketAddr = format!("{}:{}", settings.server.address, settings.server.port)
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

    let mut tinkoffApiClient = TinkoffClient::new(settings)
        .await
        .expect("Failed to initialize Tinkoff client - cannot proceed without API access");

    let a = tinkoffApiClient.create_request(InstrumentsRequest {
        instrument_status: InstrumentStatus::Base as i32,
    });

    println!("{:?}", a);

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

    // Запуск сервера критичен, используем expect
    run_server(app, http_addr).await;
}

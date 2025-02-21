use crate::{
    db::Database,
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


use features::share_updater::ShareUpdater;
use features::stream::MarketDataStreamer;
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
pub async fn start_share_updater(
    db_pool: Arc<PgPool>,
    settings: Arc<AppSettings>,
    client: Arc<TinkoffClient>,
) {
    let updater = ShareUpdater::new(db_pool, settings, client).await;
    tokio::spawn(async move {
        updater.start_update_loop().await;
    });
}
pub async fn start_market_streamer(
    db_pool: Arc<PgPool>,
    settings: Arc<AppSettings>,
    client: Arc<TinkoffClient>,
) {
    let streamer = MarketDataStreamer::new(db_pool, settings, client);
    tokio::spawn(async move {
        streamer.start_streaming().await;
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

    let app = create_app(Database {
        pool: (*db_pool).clone(),
    });

    let tinkoff_client = Arc::new(
        TinkoffClient::new(settings.clone())
            .await
            .expect("Failed to initialize Tinkoff client"),
    );
    // start_candles_updater(db_pool.clone(), settings.clone(), tinkoff_client.clone()).await;
    start_share_updater(db_pool.clone(), settings.clone(), tinkoff_client.clone()).await;
    start_market_streamer(db_pool.clone(), settings.clone(), tinkoff_client.clone()).await;

    run_server(app, http_addr).await;
}

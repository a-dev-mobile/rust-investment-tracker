use investment_tracker::config::Settings;
use sqlx::PgPool;

pub async fn setup_test_db() -> PgPool {
    let settings = Settings::new().unwrap();
    PgPool::connect(&settings.database.url).await.unwrap()
}

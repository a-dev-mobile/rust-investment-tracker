use crate::db::{mongo_db::MongoDb, PostgresDb};
use axum::{extract::Extension, http::StatusCode};
use mongodb::bson::doc;

pub async fn health_db(
    Extension(postgres_db): Extension<PostgresDb>,
    Extension(mongo_db): Extension<MongoDb>,
) -> Result<StatusCode, StatusCode> {
    // Check PostgreSQL connection
    let postgres_ok = sqlx::query("SELECT 1")
        .fetch_one(&postgres_db.pool)
        .await
        .is_ok();

    // Check MongoDB connection
    let mongo_ok = mongo_db
        .client
        .database("admin")
        .run_command(doc! {"ping": 1})
        .await
        .is_ok();

    // Return OK only if both databases are healthy
    if postgres_ok && mongo_ok {
        Ok(StatusCode::OK)
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

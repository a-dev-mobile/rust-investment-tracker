
use axum::{extract::Extension, http::StatusCode};
use mongodb::bson::doc;

use crate::features::db::MongoDb;

pub async fn health_db(
    Extension(mongo_db): Extension<MongoDb>,
) -> Result<StatusCode, StatusCode> {
    // Check PostgreSQL connection
   

    // Check MongoDB connection
    let mongo_ok = mongo_db
        .client
        .database("admin")
        .run_command(doc! {"ping": 1})
        .await
        .is_ok();

    // Return OK only if both databases are healthy
    if mongo_ok {
        Ok(StatusCode::OK)
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

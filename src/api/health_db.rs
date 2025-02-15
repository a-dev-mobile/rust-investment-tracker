use axum::{extract::Extension, http::StatusCode};

use sqlx::PgPool;

pub async fn health_db(Extension(pool): Extension<PgPool>) -> Result<StatusCode, StatusCode> {
    match sqlx::query("SELECT 1").fetch_one(&pool).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

use axum::async_trait;
use sqlx::SqlitePool;

use crate::error::AppError;

use super::DatabaseHandler;

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct Story {
    pub id: i64,
    pub language: String,
    pub pdf: Vec<u8>,
    pub epub: Vec<u8>,
    pub pid: i64,
}

#[async_trait]
impl DatabaseHandler for Story {
    async fn get_all(pool: SqlitePool) -> Result<Vec<Self>, AppError>
    where
        Self: Sized,
    {
        Ok(sqlx::query_as!(Story, "SELECT * FROM story")
            .fetch_all(&pool)
            .await?)
    }
}

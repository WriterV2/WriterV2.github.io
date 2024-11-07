use sqlx::SqlitePool;

use crate::error::AppError;
use async_trait::async_trait;

pub mod game;
pub mod product;
pub mod producttag;
pub mod story;
pub mod tag;
pub mod tool;

#[async_trait]
pub trait ProductDatabaseHandler {
    async fn get_all(pool: &SqlitePool) -> Result<Vec<Self>, AppError>
    where
        Self: Sized;

    async fn post(
        &self,
        pool: &SqlitePool,
        name: String,
        description: String,
        tags: Vec<String>,
    ) -> Result<Self, AppError>
    where
        Self: Sized;

    async fn delete(pool: &SqlitePool, id: i64) -> Result<(), AppError>
    where
        Self: Sized;
}

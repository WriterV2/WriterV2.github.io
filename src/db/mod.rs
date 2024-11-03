use sqlx::SqlitePool;

use crate::error::AppError;
use async_trait::async_trait;

pub mod game;
pub mod gametag;
pub mod product;
pub mod story;
pub mod storytag;
pub mod tag;
pub mod tool;
pub mod tooltag;

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
    ) -> Result<Self, AppError>
    where
        Self: Sized;
}

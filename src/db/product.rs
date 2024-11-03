use std::collections::HashMap;

use sqlx::SqlitePool;

use crate::error::AppError;

use super::ProductDatabaseHandler;

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub uploaddate: i64,
    pub updatedate: i64,
}

pub trait ProductMarker {
    fn product_id(&self) -> i64;
}

pub struct SpecificProduct<T: ProductMarker> {
    pub product: Product,
    pub detail: T,
}

async fn get_all_products(pool: &SqlitePool) -> Result<Vec<Product>, AppError> {
    Ok(sqlx::query_as!(Product, "SELECT * FROM product")
        .fetch_all(pool)
        .await?)
}

pub async fn get_all_specificproducts<T: ProductMarker + ProductDatabaseHandler>(
    pool: &SqlitePool,
) -> Result<Vec<SpecificProduct<T>>, AppError> {
    let product_details = T::get_all(pool).await?;
    let products = get_all_products(pool).await?;
    let mut details_map: HashMap<i64, T> = product_details
        .into_iter()
        .map(|detail| (detail.product_id(), detail))
        .collect();

    let specific_products = products
        .into_iter()
        .filter_map(|product| {
            details_map
                .remove(&product.id)
                .map(|detail| SpecificProduct { product, detail })
        })
        .collect();
    Ok(specific_products)
}

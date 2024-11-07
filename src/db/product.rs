use std::collections::HashMap;
use std::time::SystemTime;

use sqlx::{QueryBuilder, Sqlite, SqlitePool, Transaction};

use crate::error::AppError;

use super::producttag::ProductTag;
use super::tag::Tag;
use super::ProductDatabaseHandler;

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub uploaddate: i64,
    pub updatedate: i64,
}

impl Product {
    pub async fn post(tx: &mut Transaction<'static, Sqlite>, name: String, description: String, tags: Vec<String>) -> Result<Product, AppError> {
        let now = SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_millis() as i64;
        let product = sqlx::query_as!(
            Product, 
            "INSERT INTO product (name, description, uploaddate, updatedate) VALUES ($1, $2, $3, $4) RETURNING id, name, description, uploaddate, updatedate", 
            name, 
            description,
            now,
            now)
            .fetch_one(&mut **tx)
            .await?;

        if !tags.is_empty() {
            let mut tags_query_builder: QueryBuilder<Sqlite> = QueryBuilder::new("INSERT INTO tag (name) ");
            tags_query_builder.push_values(tags.iter(), | mut b, tag | { b.push_bind(tag); });
            tags_query_builder.push(" RETURNING id, name;");
            let added_tags: Vec<Tag> = tags_query_builder
                .build_query_as::<Tag>()
                .fetch_all(&mut **tx)
                .await?;

            let mut producttags_query_builder: QueryBuilder<Sqlite> = QueryBuilder::new("INSERT INTO producttag (pid, tid) ");
            producttags_query_builder.push_values(added_tags.iter(), | mut b, added_tag | {
                b.push_bind(product.id).push_bind(added_tag.id);
            });
            producttags_query_builder
                .build().execute(&mut **tx).await?;

        }
        Ok(product)
    }
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

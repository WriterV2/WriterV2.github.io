#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub uploaddate: i64,
    pub updatedate: i64,
}

pub trait ProductMarker {}

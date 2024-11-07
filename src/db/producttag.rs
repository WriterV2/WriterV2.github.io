#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct ProductTag {
    pub pid: i64,
    pub tid: i64,
}

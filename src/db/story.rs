#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct Story {
    pub id: i64,
    pub language: String,
    pub pdf: Vec<u8>,
    pub epub: Vec<u8>,
    pub pid: i64,
}

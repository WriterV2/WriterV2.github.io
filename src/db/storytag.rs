#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct Storytag {
    pub sid: i64,
    pub tgid: i64,
}

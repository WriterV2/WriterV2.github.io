#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct Tooltag {
    pub tid: i64,
    pub tgid: i64,
}

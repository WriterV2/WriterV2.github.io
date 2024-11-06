#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct Storytag {
    pub pid: i64,
    pub tid: i64,
}

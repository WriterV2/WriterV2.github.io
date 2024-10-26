#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct Tool {
    pub id: i64,
    pub repolink: String,
    pub pid: i64,
}

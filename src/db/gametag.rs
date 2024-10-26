#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct Gametag {
    pub gid: i64,
    pub tgid: i64,
}

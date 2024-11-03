use super::product::ProductMarker;

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct Game {
    pub id: i64,
    pub repolink: String,
    pub pid: i64,
}

impl ProductMarker for Game {
    fn product_id(&self) -> i64 {
        self.pid
    }
}

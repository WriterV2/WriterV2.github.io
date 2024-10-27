use axum::extract::DefaultBodyLimit;
use axum::middleware::{self};
use axum::routing::get;
use axum::Router;
use sqlx::SqlitePool;
use tower_http::services::ServeDir;

use admin::{admin_handler, admin_middleware};
use frontend::home;

mod admin;
mod frontend;

pub async fn router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/admin", get(admin_handler))
        .layer(middleware::from_fn(admin_middleware))
        .route("/", get(home))
        .nest_service("/static", ServeDir::new("src/static"))
        .layer(DefaultBodyLimit::max(10000))
}

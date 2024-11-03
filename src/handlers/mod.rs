use axum::extract::DefaultBodyLimit;
use axum::middleware::{self};
use axum::routing::{get, post};
use axum::Router;
use sqlx::SqlitePool;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::services::ServeDir;

use admin::{admin_healthcheck, admin_middleware, upload_story};
use frontend_builder::home;

use self::frontend_builder::{build_page, DummyProduct};

mod admin;
mod frontend_builder;

#[derive(Debug, Clone)]
struct ApiContext {
    pool: SqlitePool,
}

pub async fn router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/admin/story", post(upload_story))
        .route("/admin", get(admin_healthcheck))
        .layer(middleware::from_fn(admin_middleware))
        .route("/", get(home))
        .route("/stories", get(build_page::<DummyProduct>))
        .nest_service("/static", ServeDir::new("static"))
        .layer(DefaultBodyLimit::max(1074000000))
        .layer(AddExtensionLayer::new(ApiContext { pool }))
}

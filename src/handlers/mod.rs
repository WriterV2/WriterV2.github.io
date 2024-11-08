use std::sync::Arc;
use tokio::sync::Mutex;

use axum::extract::DefaultBodyLimit;
use axum::middleware::{self};
use axum::routing::{get, post};
use axum::Router;
use sqlx::SqlitePool;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::services::ServeDir;

use admin::{admin_healthcheck, admin_middleware, delete_story, generate_token, upload_story};
use frontend_builder::home;

use crate::db::story::Story;

use self::frontend_builder::build_page;

mod admin;
pub mod frontend_builder;

#[derive(Debug, Clone)]
pub struct ApiContext {
    pool: SqlitePool,
}

#[derive(Clone)]
struct AppState {
    admin_token: Arc<Mutex<String>>,
}

pub async fn router(pool: SqlitePool) -> Router {
    let state = AppState {
        admin_token: Arc::new(Mutex::new(generate_token())),
    };

    Router::new()
        .route("/admin/story", post(upload_story).delete(delete_story))
        .route("/admin", get(admin_healthcheck))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            admin_middleware,
        ))
        .with_state(state)
        .route("/", get(home))
        .route("/stories", get(build_page::<Story>))
        .nest_service("/static", ServeDir::new("static"))
        .layer(DefaultBodyLimit::max(1074000000))
        .layer(AddExtensionLayer::new(ApiContext { pool }))
}

use std::env;

use dotenvy::dotenv;
use sqlx::sqlite::SqlitePoolOptions;

use self::db::story;

pub mod db;
mod error;
mod handlers;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url =
        &env::var("DATABASE_URL").expect("Failed to get environment variable DATABASE_URL");

    let pool = SqlitePoolOptions::new()
        .connect(db_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    story::synchronize_story_files(&pool)
        .await
        .expect("Failed to synchronize files");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, handlers::router(pool).await.into_make_service())
        .await
        .unwrap();
}

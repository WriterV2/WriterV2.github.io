use std::env;

use dotenvy::dotenv;
use sqlx::sqlite::SqlitePoolOptions;

pub mod db;
mod error;
mod handlers;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let pool = SqlitePoolOptions::new()
        .connect(
            &env::var("DATABASE_URL").expect("Failed to get environment variable DATABASE_URL"),
        )
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, handlers::router(pool).await.into_make_service())
        .await
        .unwrap();
}

use std::env;

use axum::extract::{DefaultBodyLimit, Request};
use axum::http::{HeaderMap, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{middleware, Router};
use dotenvy::dotenv;
use maud::{html, Markup, DOCTYPE};
use sqlx::sqlite::SqlitePoolOptions;
use subtle::ConstantTimeEq;
use tower_http::services::ServeDir;

mod db;

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

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

    let app = Router::new()
        .route("/admin", get(admin_handler))
        .layer(middleware::from_fn(admin_middleware))
        .route("/", get(home))
        .nest_service("/static", ServeDir::new("src/static"))
        .layer(DefaultBodyLimit::max(10000));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn admin_middleware(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let expected_token = env::var("TOKEN").map_err(|_| {
        eprintln!("Error: TOKEN environment variable is not set");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let token = headers
        .get("X-Admin-Token")
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !expected_token
        .as_bytes()
        .ct_eq(token.as_bytes())
        .unwrap_u8()
        == 1
    {
        eprintln!("Warning: Unauthorized access attempt with invalid token");
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(request).await)
}

async fn admin_handler() -> &'static str {
    "Hello World"
}

async fn home() -> Result<impl IntoResponse, AppError> {
    Ok(html!({
        (head("Home"))
        body class="bg-[#eae3d2] text-[#333] dark:bg-[#333] dark:text-[#eae3d2] font-mono text-base tracking-wide" {
            (header_with_navbar())
        }
    }))
}

fn header_with_navbar() -> Markup {
    html!({
        header {
            navbar class="p-3 text-sm flex flex-row flex-wrap content-center justify-around decoration-purple-500 shadow" {
                a class="hover:underline" href="/" { "Home" }
                a class="hover:underline" href="/" { "Stories" }
                a class="hover:underline" href="/" { "Tools" }
                a class="hover:underline" href="/" { "Games" }
            }
        }
    })
}

fn head(page_title: &str) -> Markup {
    html! {
        head {
            (DOCTYPE)
            meta charset="utf-8";
            link rel="stylesheet" type="text/css" href="/static/styles.css";
            title { (page_title) }
        }
    }
}

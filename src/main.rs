use axum::extract::DefaultBodyLimit;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use maud::{html, Markup, DOCTYPE};
use tower_http::services::ServeDir;

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
    let app = Router::new()
        .route("/", get(home))
        .nest_service("/static", ServeDir::new("src/static"))
        .layer(DefaultBodyLimit::max(10000));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn home() -> Result<impl IntoResponse, AppError> {
    Ok(html!({
        (head("Home"))
        body class="bg-neutral-50 text-neutral-950 dark:bg-neutral-950 dark:text-neutral-50 font-sans text-base tracking-wide" {
            (header_with_navbar())
        }
    }))
}

fn header_with_navbar() -> Markup {
    html!({ header  { 
            navbar class="p-3 text-sm bg-neutral-100 dark:bg-neutral-900 flex flex-row flex-wrap content-center justify-around decoration-purple-500 border-b-2 border-neutral-200 dark:border-neutral-800" {
            a class="hover:underline" href="/" { "Home" }
            a class="hover:underline" href="/" { "Stories" } 
            a class="hover:underline" href="/" { "Tools" } 
            a class="hover:underline" href="/" { "Games" } 
        }
    } })
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

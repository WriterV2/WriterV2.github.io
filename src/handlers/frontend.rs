use axum::response::IntoResponse;
use maud::{html, Markup, DOCTYPE};

use crate::error::AppError;

pub async fn home() -> Result<impl IntoResponse, AppError> {
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

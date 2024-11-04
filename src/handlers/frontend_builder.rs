use axum::response::IntoResponse;
use axum::Extension;
use chrono::DateTime;
use maud::{html, Markup, DOCTYPE};

use crate::db::product::{get_all_specificproducts, ProductMarker, SpecificProduct};
use crate::db::ProductDatabaseHandler;
use crate::error::AppError;

pub async fn home() -> Result<impl IntoResponse, AppError> {
    Ok(html!((page("Home".to_string(), Markup::default()))))
}

fn page(page_title: String, content: Markup) -> Markup {
    html!(
        (head(page_title))
        body class="bg-[#eae3d2] text-[#333] dark:bg-[#333] dark:text-[#eae3d2] font-mono text-base" {
            (header())
            (navbar())
            (content)
        }
    )
}

fn header() -> Markup {
    html!(
        header {
            h1 class="underline decoration-[#601B69] dark:decoration-[#e1b1e7] text-center tracking-widest font-serif p-3 text-xl" {"Vithuran Vishnuthas"}
        }
    )
}

fn navbar() -> Markup {
    html!({
            navbar class="p-3 text-sm flex flex-row flex-wrap content-center justify-around decoration-[#eae3d2] tracking-wide" {
            a class="hover:underline" href="/" { "Home" }
            a class="hover:underline" href="/stories" { "Stories" }
            a class="hover:underline" href="/" { "Tools" }
            a class="hover:underline" href="/" { "Games" }
        }
    })
}

fn cards<T: PageBuilder + ProductMarker>(
    products: Vec<SpecificProduct<T>>,
    category: String,
) -> Markup {
    html!(
        div class="m-12" {
            h1 class="p-6 font-serif text-xl tracking-wider underline decoration-[#601B69] dark:decoration-[#E1B1E7]" { (category) }
            div class="grid grid-cols-2 gap-8" {
                @for product in products  {
                    (card(product))
                }
            }
        }
    )
}

fn card<T: PageBuilder + ProductMarker>(specific_product: SpecificProduct<T>) -> Markup {
    let date = |i: i64| -> String {
        if let Some(datetime) = DateTime::from_timestamp(i / 1_000, 0) {
            return format!("{}", datetime.format("%Y/%m/%d")).to_string();
        }
        "Unknown".to_string()
    };
    let upload_date = date(specific_product.product.uploaddate);
    let update_date = date(specific_product.product.updatedate);
    html!(
        div class="p-9 shadow" {
            h2 class="text-[#601B69] dark:text-[#E1B1E7]" {
                (specific_product.product.name)
            }
            div {
                (specific_product.product.description)
            }
            div {
                (specific_product.detail.product_specific_card_content())
            }
            div class="flex flex-row space-x-5 mt-9 text-xs" {
                span {"Upload: " (upload_date)}
                @if upload_date != update_date {
                    span {"Update: " (update_date)}
                }
            }
        }
    )
}

fn head(page_title: String) -> Markup {
    html! {
        head {
            (DOCTYPE)
            meta charset="utf-8";
            link rel="stylesheet" type="text/css" href="/static/styles.css";
            title { (page_title) }
        }
    }
}

pub async fn build_page<T: Sized + ProductMarker + PageBuilder + ProductDatabaseHandler>(
    ctx: Extension<super::ApiContext>,
) -> Result<impl IntoResponse, AppError> {
    Ok(html!(
        (page(
            T::page_title(),
            cards(
                get_all_specificproducts::<T>(&ctx.pool).await?,
                T::page_title()
            )
        ))
    ))
}

pub trait PageBuilder {
    fn page_title() -> String;
    fn product_specific_card_content(&self) -> Markup;
}

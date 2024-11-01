use axum::response::IntoResponse;
use maud::{html, Markup, DOCTYPE};

use crate::db::product::{Product, ProductMarker};
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
    html!(
        div class="p-9 shadow" {
            h2 class="text-[#601B69] dark:text-[#E1B1E7]" {
                (specific_product.product.name)
            }
            div {
                (specific_product.product.description)
            }
            div {
                (specific_product.specific.product_specific_card_content())
            }
            div class="flex flex-row space-x-5 mt-9 text-xs" {
                span {"Update: " (specific_product.product.updatedate)}
                span {"Upload: " (specific_product.product.uploaddate)}
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

pub async fn build_page<T: Sized + ProductMarker + PageBuilder>(
) -> Result<impl IntoResponse, AppError> {
    Ok(html!(
        (page(T::page_title(), cards(T::products(), T::page_title())))
    ))
}

pub trait PageBuilder {
    fn page_title() -> String;
    fn products() -> Vec<SpecificProduct<Self>>
    where
        Self: Sized + ProductMarker;
    fn product_specific_card_content(&self) -> Markup;
}

// TODO: Remove after first implementation
#[derive(Hash, PartialEq, Eq)]
pub struct DummyProduct {
    dummy: String,
}

impl PageBuilder for DummyProduct {
    fn page_title() -> String {
        "Dummy Products".to_string()
    }

    fn products() -> Vec<SpecificProduct<Self>>
    where
        Self: Sized,
    {
        let product1 = Product {
            id: 0,
            name: "Willensfreiheit in Ketten".to_string(),
            description:
                "Nach der Flucht seines Bruders, verzweifelt Fahim an der Existenz der Willensfreiheit."
                    .to_string(),
            uploaddate: 1,
            updatedate: 1,
        };

        let product2 = Product {
            id: 1,
            name: "Aufbruch und Abbruch".to_string(),
            description: "Ein frischer Absolvent muss sich vor der ersten Quest nur noch bei der Gilde für Heilkünstler anmelden.".to_string(),
            uploaddate: 1,
            updatedate: 1,
        };

        let product3 = Product {
            id: 2,
            name: "Mit gutem Gewissen".to_string(),
            description: "Ein frischer Absolvent muss sich vor der ersten Quest nur noch bei der Gilde für Heilkünstler anmelden.".to_string(),
            uploaddate: 1,
            updatedate: 1,
        };

        let test1 = DummyProduct {
            dummy: "/".to_string(),
        };

        let test2 = DummyProduct {
            dummy: "/".to_string(),
        };

        let test3 = DummyProduct {
            dummy: "/".to_string(),
        };

        let specific1 = SpecificProduct {
            product: product1,
            specific: test1,
        };

        let specific2 = SpecificProduct {
            product: product2,
            specific: test2,
        };

        let specific3 = SpecificProduct {
            product: product3,
            specific: test3,
        };

        vec![specific1, specific2, specific3]
    }

    fn product_specific_card_content(&self) -> Markup {
        html!(
            a class="mt-9 text-[#601B69] dark:text-[#E1B1E7]" href=(self.dummy) {
                "Link"
            }
        )
    }
}

impl ProductMarker for DummyProduct {}

// TODO: Refactoring
pub struct SpecificProduct<T: ProductMarker> {
    product: Product,
    specific: T,
}

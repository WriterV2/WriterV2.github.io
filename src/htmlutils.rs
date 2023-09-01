use chrono::Datelike;
use html::content::builders::{FooterBuilder, MainBuilder, NavigationBuilder};
use html::content::{Footer, Header, Heading1, Navigation};
use html::inline_text::{Anchor, Underline};
use html::metadata::builders::HeadBuilder;
use html::metadata::{Head, Link, Meta, Title};
use html::root::builders::HtmlBuilder;
use html::root::Html;

const STYLE_PATH: &str = "/style.css";
const ICON_PATH: &str = "data:,";
const PAGE_TITLE_BASE: &str = "Vithuran Vishnuthas";
// TODO: get routes dynamically
const ROUTES: Vec<Vec<&str>> = vec![vec!["Home", "/"]];

// Get HTML builder and add HTML components to it
fn build_html() -> HtmlBuilder {
    let mut html = Html::builder();
    html
}

// Build an HTML head section with the provided meta information
// (build and push to an HTML builder)
fn build_head(keywords: &Vec<&str>, title: &str) -> HeadBuilder {
    let page_title = if title.is_empty() {
        PAGE_TITLE_BASE
    } else {
        &format!("{} - {}", PAGE_TITLE_BASE, title)
    };

    let keywords_meta = Meta::builder().name("keywords");
    let mut content = String::new();
    for keyword in keywords.iter() {
        content += &format!("{},", keyword);
    }
    content.trim_end_matches(",");
    keywords_meta.content(content);

    *Head::builder()
        .push(Title::builder().title(page_title).build())
        .push(Meta::builder().charset("UTF-8").build())
        .push(
            Meta::builder()
                .name("viewport")
                .content("width=device-width, initial-scale=1")
                .build(),
        )
        .push(keywords_meta.build())
        .push(
            Meta::builder()
                .name("author")
                .content("Vithuran Vishnuthas")
                .build(),
        )
        .push(Link::builder().rel("stylesheet").href(STYLE_PATH).build())
        .push(Link::builder().rel("icon").href(ICON_PATH).build())
}

// Build an HTML navigation with the provided links
// (build and push to an HTML builder)
fn build_navigation() -> NavigationBuilder {
    let mut navigation = Navigation::builder();
    for pair in ROUTES.iter() {
        navigation.push(Anchor::builder().href(pair[1]).push(pair[0]).build());
    }
    navigation
}

// Build an HTML footer with an auto-updating copyright signature
// (build and push to an HTML builder)
fn build_footer() -> FooterBuilder {
    *Footer::builder().push(format!(
        "Vithuran Vishnuthas &copy; 2022 - {}",
        chrono::Utc::now().year()
    ))
}

trait GenerateHTMLGroup<T: GenerateHTMLPage> {
    fn get_pages() -> Vec<T>;
    fn build_overview_page() -> HtmlBuilder;
}

trait GenerateHTMLPage {
    fn build_single_page_content(&self) -> Vec<MainBuilder>;
    fn get_page_name(&self) -> String;
    fn get_tags(&self) -> Vec<String>;
    fn get_title(&self) -> String;
    fn get_description(&self) -> String;

    fn build_header() -> Header {
        Header::builder()
            .push(
                Underline::builder()
                    .push(
                        Anchor::builder()
                            .href(ROUTES[0][1])
                            .push(Heading1::builder().push("Writer V2").build())
                            .build(),
                    )
                    .build(),
            )
            .push("Vithuran Vishnuthas")
            .build()
    }
}

// fn build_grid(title: &str, works: Vec<Work>) -> MainBuilder {
//     let mut main = Main::builder();
//     let header = Heading2::builder().push(title);
//
//     let mut filters = Division::builder().id("filters");
//     for work in works.iter() {
//         for (i, tag) in work.filter_tags.iter().enumerate() {
//             filters
//                 .push(
//                     Input::builder()
//                         .type_("checkbox")
//                         .id(i.to_string())
//                         .checked("true")
//                         .hidden("true")
//                         .build(),
//                 )
//                 .push(Label::builder().for_(i.to_string()).push(*tag).build());
//         }
//     }
//
//     let mut works_overview = Division::builder().id("works");
//     for work in works.iter() {
//         works_overview.push(Anchor::builder().build());
//     }
//
//     main.push(header.build()).push(filters.build());
//     main
// }

use chrono::Datelike;
use html::content::builders::{FooterBuilder, MainBuilder, NavigationBuilder};
use html::content::{Footer, Header, Heading1, Main, Navigation};
use html::forms::{Input, Label};
use html::inline_text::{Anchor, Underline};
use html::metadata::builders::HeadBuilder;
use html::metadata::{Head, Link, Meta, Title};
use html::root::builders::HtmlBuilder;
use html::root::{Body, Html};
use html::text_content::Division;

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
fn build_head(keywords: Vec<&str>, title: &str) -> HeadBuilder {
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
    fn get_pages(&self) -> Vec<T>;
    fn get_title(&self) -> &str;
    fn get_keywords(&self) -> Vec<&str>;
    fn build_overview_page(&self) -> HtmlBuilder {
        // Main section
        let mut main = Main::builder().heading_2(|h| h.push(self.get_title()));
        let pages = self.get_pages();

        // Div for content filters
        let mut filters = Division::builder().id("filters");

        // Add label and invisible checkbox for every filter without duplicates
        let used_filters: Vec<&str> = Vec::new();
        for page in pages.iter() {
            for (i, f) in page.get_filters().iter().enumerate() {
                if !used_filters.contains(f) {
                    filters
                        .push(
                            Input::builder()
                                .type_("checkbox")
                                .id(i.to_string())
                                .checked("true")
                                .hidden("true")
                                .build(),
                        )
                        .push(Label::builder().for_(i.to_string()).push(*f).build());
                    used_filters.push(f);
                }
            }
        }

        // Add grid with an overview of the single pages
        let mut grid = Division::builder().id("grid");
        for page in pages.iter() {
            // TODO: Filters
            grid.push(Anchor::builder().href(page.get_page_name()).build())
                .heading_3(|h| h.push(page.get_title()))
                .paragraph(|p| p.push(page.get_description()));
        }

        main.push(filters.build()).push(grid.build());

        let html = build_html().push(build_head(self.get_keywords(), &self.get_title()).build());
        let body = Body::builder()
            .push(build_navigation().build())
            .push(main.build())
            .push(build_footer().build());
        html.push(body.build());
        *html
    }
}

trait GenerateHTMLPage {
    fn build_single_page_content(&self) -> Vec<MainBuilder>;
    fn get_page_name(&self) -> &str;
    fn get_filters(&self) -> Vec<&str>;
    fn get_title(&self) -> &str;
    fn get_description(&self) -> &str;
    fn get_keywords(&self) -> Vec<&str>;

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

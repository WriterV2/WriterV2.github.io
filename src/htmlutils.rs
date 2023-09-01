use chrono::Datelike;
use html::content::builders::{FooterBuilder, HeaderBuilder, MainBuilder, NavigationBuilder};
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

// Build HTML header
// (build and push to an HTML builder)
fn build_header() -> HeaderBuilder {
    *Header::builder()
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
        // Main section - root for structure built in this method
        let mut main = Main::builder().heading_2(|h| h.push(self.get_title()));
        let pages = self.get_pages();

        // labels for the grid filters
        let mut filters_div = Division::builder().id("filters");

        // hidden checkboxes for grid filters
        let mut filters_checkboxes = Division::builder().id("filters-checkboxes");

        // Add grid with an overview of the single pages
        let mut grid_div = Division::builder().id("grid");

        // Add label and invisible checkbox for every filter without duplicates
        let used_filters: Vec<&str> = Vec::new();
        for page in pages.iter() {
            // Add page card with the link to the page
            let anchor = Anchor::builder().href(page.get_page_name());
            for (i, f) in page.get_filters().iter().enumerate() {
                if !used_filters.contains(f) {
                    // Add checkbox for filter
                    filters_checkboxes.push(
                        Input::builder()
                            .type_("checkbox")
                            .id(i.to_string())
                            .checked("true")
                            .hidden("true")
                            .build(),
                    );
                    // Add label for filter
                    filters_div.push(Label::builder().for_(i.to_string()).push(*f).build());
                    // Add data indentifier to page card
                    anchor.data(i.to_string(), "");
                    used_filters.push(f);
                }
            }
            // Add title and description to page card
            anchor
                .heading_3(|h| h.push(page.get_title()))
                .paragraph(|p| p.push(page.get_description()));
            // Add page card to grid
            grid_div.push(anchor.build());
        }

        // Add filter checkboxes, filter labels and the page cards to the main
        // section
        main.push(filters_checkboxes.build())
            .push(filters_div.build())
            .push(grid_div.build());

        // Add the built main section to the body inbetween the navigation and footer
        let body = Body::builder()
            .push(build_header().build())
            .push(build_navigation().build())
            .push(main.build())
            .push(build_footer().build());

        // Add body to main HTML handler after head
        *build_html()
            .push(build_head(self.get_keywords(), &self.get_title()).build())
            .push(body.build())
    }
}

trait GenerateHTMLPage {
    fn build_single_page_content(&self) -> Vec<MainBuilder>;
    fn get_page_name(&self) -> &str;
    fn get_filters(&self) -> Vec<&str>;
    fn get_title(&self) -> &str;
    fn get_description(&self) -> &str;
    fn get_keywords(&self) -> Vec<&str>;
}

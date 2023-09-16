use chrono::Datelike;
use html::content::builders::MainBuilder;
use html::content::{Footer, Header, Heading1, Main, Navigation};
use html::forms::{Input, Label};
use html::inline_text::{Anchor, Underline};
use html::metadata::{Head, Link, Meta, Title};
use html::root::{Body, Html};
use html::text_content::Division;

const STYLE_PATH: &str = "/style.css";
const ICON_PATH: &str = "data:,";
const PAGE_TITLE_BASE: &str = "Vithuran Vishnuthas";
// TODO: get routes dynamically
const ROUTES: [[&str; 2]; 1] = [["Home", "/"]];

// Build an HTML page with the given main section, page SEO keywords, page title and the base
// elements (head, navigation, header, footer)
pub fn build_html_page(main: &mut MainBuilder, keywords: Vec<String>, title: &str) -> Html {
    Html::builder()
        .push(build_head(keywords, title))
        .push(
            Body::builder()
                .push(build_header())
                .push(build_navigation())
                .push(main.build())
                .push(build_footer())
                .build(),
        )
        .build()
}

// Build an HTML head section with the provided meta information
fn build_head(keywords: Vec<String>, title: &str) -> Head {
    let page_title = if title.is_empty() {
        String::from(PAGE_TITLE_BASE)
    } else {
        format!("{} - {}", PAGE_TITLE_BASE, &title)
    };

    let mut keywords_meta = Meta::builder();
    keywords_meta.name("keywords");
    let mut content = String::new();
    for keyword in keywords.iter() {
        content += &format!("{},", keyword);
    }
    let _ = content.trim_end_matches(',');
    keywords_meta.content(content);

    Head::builder()
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
        .build()
}

// Build an HTML navigation with the provided links
fn build_navigation() -> Navigation {
    let mut navigation = Navigation::builder();
    for pair in ROUTES.iter() {
        navigation.push(Anchor::builder().href(pair[1]).push(pair[0]).build());
    }
    navigation.build()
}

// Build HTML header
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

// Build an HTML footer with an auto-updating copyright signature
fn build_footer() -> Footer {
    Footer::builder()
        .push(format!(
            "Vithuran Vishnuthas &copy; 2022 - {}",
            chrono::Utc::now().year()
        ))
        .build()
}

pub trait GenerateHTMLGroup<T: GenerateHTMLPage> {
    fn get_pages(&self) -> Vec<T>;
    fn get_title(&self) -> String;
    fn get_keywords(&self) -> Vec<String>;

    // Build an overview page with all pages in this group in a grid and filters section
    fn build_overview_page(&self) -> Html {
        // Main section with the group title - root for structure built in this method
        let mut main = Main::builder();

        main.heading_2(|h| h.push(self.get_title().clone()));

        // Get all pages in this group
        let pages = self.get_pages();

        // Labels for the grid filters
        let mut filters_div = Division::builder();
        filters_div.id("filters");

        // Hidden checkboxes for grid filters
        let mut filters_checkboxes = Division::builder();
        filters_checkboxes.id("filters-checkboxes");

        // Add grid with an overview of the single pages
        let mut grid_div = Division::builder();
        grid_div.id("grid");

        // Add label and invisible checkbox for every filter without duplicates
        let mut used_filters: Vec<String> = Vec::new();
        for page in pages.iter() {
            // Add page card with the link to the page
            let mut anchor = Anchor::builder();
            anchor.href(page.get_page_name());
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
                    filters_div.push(
                        Label::builder()
                            .for_(i.to_string())
                            .push(f.to_string())
                            .build(),
                    );
                    // Add data indentifier to page card
                    anchor.data(i.to_string(), "");
                    used_filters.push(f.to_string());
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

        build_html_page(&mut main, self.get_keywords(), &self.get_title())
    }
}

pub trait GenerateHTMLPage {
    fn build_single_page_content(&self) -> Vec<MainBuilder>;
    fn get_page_name(&self) -> String;
    fn get_filters(&self) -> Vec<String>;
    fn get_title(&self) -> String;
    fn get_description(&self) -> String;
    fn get_keywords(&self) -> Vec<String>;
}

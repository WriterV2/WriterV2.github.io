use std::collections::HashMap;

use chrono::Datelike;
use html::content::builders::MainBuilder;
use html::content::{Footer, Header, Heading1, Navigation};
use html::inline_text::{Anchor, Underline};
use html::metadata::{Head, Link, Meta, Title};
use html::root::builders::HtmlBuilder;
use html::root::{Body, Html};

const STYLE_PATH: &str = "/style.css";
const ICON_PATH: &str = "data:,";

fn build_html(links: HashMap<&str, &str>, main: MainBuilder) -> HtmlBuilder {
    let mut html = Html::builder();
    let mut head = Head::builder()
        .push(Title::builder().title("Vithuran Vishnuthas").build())
        .push(Meta::builder().charset("UTF-8").build())
        .push(
            Meta::builder()
                .name("viewport")
                .content("width=device-width, initial-scale=1")
                .build(),
        )
        .push(Link::builder().rel("stylesheet").href(STYLE_PATH).build())
        .push(Link::builder().rel("icon").href(ICON_PATH).build());

    let mut header = Header::builder()
        .push(
            Underline::builder()
                .push(
                    Anchor::builder()
                        .href("/index.html")
                        .push(Heading1::builder().push("Writer V2").build())
                        .build(),
                )
                .build(),
        )
        .push("Vithuran Vishnuthas");

    let mut navigation = Navigation::builder();

    for (path, title) in links {
        navigation.push(Anchor::builder().href(path).push(title).build());
    }

    let mut footer = Footer::builder().push(format!(
        "Vithuran Vishnuthas &copy; 2022 - {}",
        chrono::Utc::now().year()
    ));

    let mut body = Body::builder()
        .push(header.build())
        .push(navigation.build())
        .push(main.build())
        .push(footer.build());

    html.push(head.build()).push(body.build());
    html
}

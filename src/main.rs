use std::fs::write;

use anyhow::{Context, Ok, Result};
use tera::Tera;

use self::works::stories::Stories;
use self::works::Works;

mod works;

const STORY: (&str, &str) = ("stories", "story");

fn main() -> Result<()> {
    // new tera instance
    let tera = Tera::new("templates/**/*.html").context("Failed to create new Tera instance")?;

    // render index.html file
    let output = tera
        .render("index.html", &tera::Context::new())
        .context("Failed to render index.html")?;
    write("docs/index.html", output).context("Failed to write index.html")?;

    // render pages for stories
    Stories::new_from_file(STORY.0)?
        .render_overview_page(&tera, STORY.0)
        .context("Failed to render overview page for stories")?
        .render_single_pages(&tera, STORY.1)
        .context("Failed to render single pages for stories")?;

    Ok(())
}

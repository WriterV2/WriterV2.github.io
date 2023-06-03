use std::fs::write;

use anyhow::{Context, Ok, Result};
use tera::Tera;

use self::works::stories::Stories;
use self::works::Works;

mod works;

fn main() -> Result<()> {
    // new tera instance
    let tera = Tera::new("templates/**/*.html").context("Failed to create new Tera instance")?;

    // render index.html file
    let output = tera
        .render("index.html", &tera::Context::new())
        .context("Failed to render index.html")?;
    write("docs/index.html", output).context("Failed to write index.html")?;

    // render pages for stories
    Stories::new_from_file("stories")?
        .render_overview_page(&tera, "stories")
        .context("Failed to render overview page for stories")?
        .render_single_pages(&tera, "story")
        .context("Failed to render single pages for stories")?;

    Ok(())
}

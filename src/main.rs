use anyhow::{Context, Ok, Result};
use tera::Tera;

use self::works::stories::Stories;
use self::works::Works;

mod works;

fn main() -> Result<()> {
    // new tera instance
    let tera = Tera::new("templates/**/*.html").context("Failed to create new Tera instance")?;

    // render index.html file
    let context = tera::Context::new();
    let output = tera
        .render("index.html", &context)
        .context("Failed to render index.html")?;
    std::fs::write("docs/index.html", output).context("Failed to write index.html")?;

    // render stories.html file
    Stories::new_from_file(
        &std::fs::File::open(std::path::PathBuf::from("stories.json"))
            .context("Failed to open stories.json")?,
    )?
    .render_overview_page(&tera, "stories")
    .context("Failed to render overview page for stories")?
    .render_single_pages(&tera, "story")
    .context("Failed to render single pages for stories")?;

    Ok(())
}

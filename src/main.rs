use anyhow::{Context, Ok, Result};
use tera::Tera;

use crate::works::Works;

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
    let stories = std::rc::Rc::new(works::stories::Stories::new_from_file(
        &std::fs::File::open(std::path::PathBuf::from("stories.json"))
            .context("Failed to open stories.json")?,
    )?);
    std::rc::Rc::clone(&stories)
        .render_overview_page(&tera, "stories")
        .context("Failed to render overview page for stories")?;
    std::rc::Rc::clone(&stories)
        .render_single_pages(&tera, "story")
        .context("Failed to render single pages for stories")?;
    Ok(())
}

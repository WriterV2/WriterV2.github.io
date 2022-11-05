use anyhow::Result;
use tera::{Context, Tera};

use crate::works::Works;

mod images_processing;
mod works;

fn main() -> Result<()> {
    images_processing::resize_workcard_images()?;
    // new tera instance
    let tera = Tera::new("templates/**/*.html")?;

    // render index.html file
    let context = Context::new();
    let output = tera.render("index.html", &context)?;
    std::fs::write("docs/index.html", output)?;

    // render stories.html file
    let stories = works::stories::Stories::new_from_file(std::fs::File::open(
        std::path::PathBuf::from("stories.json"),
    )?);
    stories?.render_overview_page(&tera, "stories")?;
    Ok(())
}

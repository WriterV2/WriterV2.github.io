use anyhow::Result;
use tera::{Context, Tera};

use crate::works::Works;

mod works;

fn main() -> Result<()> {
    let tera = Tera::new("templates/**/*.html")?;

    let context = Context::new();

    let output = tera.render("index.html", &context)?;

    std::fs::write("docs/index.html", output)?;

    let stories = works::stories::Stories::new_from_file(std::fs::File::open(
        std::path::PathBuf::from("stories.json"),
    )?);
    stories?.render_overview_page(&tera, "stories")?;
    Ok(())
}

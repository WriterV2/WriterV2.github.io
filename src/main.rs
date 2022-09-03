use anyhow::Result;
use tera::{Context, Tera};

use crate::works::Works;

mod works;

fn main() -> Result<()> {
    let tera = Tera::new("templates/**/*.html")?;

    let context = Context::new();

    let output = tera.render("index.html", &context)?;

    std::fs::write("docs/index.html", output)?;

    let file = std::fs::File::open(std::path::PathBuf::from("stories.json"))?;
    let stories = works::stories::Stories::new_from_file(file);
    let context = stories?.create_tera_context();

    let stories_html_output = tera.render("stories.html", &context?)?;
    std::fs::write("docs/stories.html", stories_html_output)?;
    Ok(())
}

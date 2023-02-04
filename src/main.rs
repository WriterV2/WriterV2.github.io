use anyhow::Result;
use tera::{Context, Tera};

use crate::works::Works;

mod works;

fn main() -> Result<()> {
    // new tera instance
    let tera = Tera::new("templates/**/*.html")?;

    // render index.html file
    let context = Context::new();
    let output = tera.render("index.html", &context)?;
    std::fs::write("docs/index.html", output)?;

    // render stories.html file
    let stories = std::rc::Rc::new(works::stories::Stories::new_from_file(
        std::fs::File::open(std::path::PathBuf::from("stories.json"))?,
    )?);
    std::rc::Rc::clone(&stories).render_overview_page(&tera, "stories")?;
    std::rc::Rc::clone(&stories).render_single_pages(&tera, "story")?;

    Ok(())
}

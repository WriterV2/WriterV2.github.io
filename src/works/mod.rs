use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

pub mod stories;

// functionality for a group of work, e.g. stories or games
pub trait Works: for<'a> Deserialize<'a> + Serialize {
    // create works from the corresponding JSON file,
    // e.g. stories from stories.json
    fn new_from_file(file: std::fs::File) -> Result<Self> {
        serde_json::from_reader(std::io::BufReader::new(&file))
            // add error context
            .with_context(|| format!("Failed to create works from {:?}", &file))
    }

    // crate tera context from the group of work
    // tera context can be accessed in HTML tera templating
    fn create_tera_context(&self) -> Result<tera::Context> {
        let mut context = tera::Context::new();
        context.insert("works", &self);
        Ok(context)
    }

    // render corresponding overview HTML template
    // with its tera context
    fn render_overview_page(
        &self,
        tera_instance: &tera::Tera,
        filename_without_extension: &str,
    ) -> Result<()> {
        std::fs::write(
            format!("docs/{}.html", filename_without_extension).as_str(),
            tera_instance.render(
                format!("{}.html", filename_without_extension).as_str(),
                &self.create_tera_context()?,
            )?,
        )
        // add error context
        .with_context(|| {
            format!(
                "Failed to render overview page for {:?}",
                filename_without_extension
            )
        })
    }
}

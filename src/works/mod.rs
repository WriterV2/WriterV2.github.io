use std::fs::{write, File};
use std::io::BufReader;
use std::path::PathBuf;

use anyhow::{Context, Ok, Result};
use serde::{Deserialize, Serialize};

pub mod stories;

// functionality for a group of work, e.g. stories or games
pub trait Works: for<'a> Deserialize<'a> + Serialize {
    // create works from the corresponding JSON file,
    // e.g. stories from stories.json
    fn new_from_file(workname: &str) -> Result<Self> {
        let work = serde_json::from_reader(BufReader::new(
            &File::open(PathBuf::from(format!("{}.json", workname)))
                .with_context(|| format!("Failed to open {}.json", workname))?,
        ))
        .with_context(|| format!("Failed to create works from {}.json", workname))?;
        Ok(work)
    }

    // crate tera context from the group of work
    // tera context can be accessed in HTML tera templating
    fn create_tera_context(&self) -> tera::Context {
        let mut context = tera::Context::new();
        context.insert("works", &self);
        context
    }

    // render page for every work
    fn render_single_pages(self, tera_instance: &tera::Tera, template_name: &str) -> Result<Self>;

    // render corresponding overview HTML template
    // with its tera context
    fn render_overview_page(
        self,
        tera_instance: &tera::Tera,
        filename_without_extension: &str,
    ) -> Result<Self> {
        write(
            format!("docs/{}.html", filename_without_extension).as_str(),
            tera_instance
                .render(
                    format!("{}.html", filename_without_extension).as_str(),
                    &self.create_tera_context(),
                )
                .with_context(|| {
                    format!(
                        "Failed rendering overview page for {}",
                        filename_without_extension
                    )
                })?,
        )
        .with_context(|| {
            format!(
                "Failed to write overview page for {}",
                filename_without_extension
            )
        })?;
        Ok(self)
    }
}

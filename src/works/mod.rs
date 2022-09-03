use anyhow::Result;
use serde::{Deserialize, Serialize};

pub mod stories;

pub trait Works: for<'a> Deserialize<'a> + Serialize {
    fn new_from_file(file: std::fs::File) -> Result<Self> {
        Ok(serde_json::from_reader(std::io::BufReader::new(file))?)
    }

    fn create_tera_context(&self) -> Result<tera::Context> {
        let mut context = tera::Context::new();
        context.insert("works", &self);
        Ok(context)
    }

    fn render_overview_page(
        &self,
        tera_instance: &tera::Tera,
        filename_without_extension: &str,
    ) -> Result<()> {
        Ok(std::fs::write(
            format!("docs/{}.html", filename_without_extension).as_str(),
            tera_instance.render(
                format!("{}.html", filename_without_extension).as_str(),
                &self.create_tera_context()?,
            )?,
        )?)
    }
}

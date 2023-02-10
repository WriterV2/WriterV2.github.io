use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

// stories and their information
// derive from JSON
// process to context for HTML
#[derive(Debug, Deserialize, Serialize)]
pub struct Stories(Vec<Story>);

#[derive(Debug, Deserialize, Serialize)]
struct Story {
    title: String,
    description: String,
    path_to_document: String,
    number_of_pages: u16,
    language: Language,
    last_update: NaiveDate,
}

// see src/works/mod.rs for information of Works trait
impl super::Works for Stories {
    // render single pages for every story with the extracted text and a download button
    fn render_single_pages(
        &self,
        tera_instance: &tera::Tera,
        template_name: &str,
    ) -> anyhow::Result<()> {
        // TODO: Extract story pdf text
        let mut result = Vec::new();
        for story in self.0.iter() {
            let mut context = tera::Context::new();
            context.insert("story", story);
            result.push(std::fs::write(
                story.get_html_path()?.as_str(),
                tera_instance.render(format!("{}.html", template_name).as_str(), &context)?,
            ));
        }
        Ok(())
    }
}

impl Story {
    // check if story document was modified
    fn document_modified(&self) -> anyhow::Result<bool> {
        Ok(
            std::fs::metadata(format!("docs/stories/{}", &self.path_to_document))?.modified()?
                > std::fs::metadata(self.get_html_path()?)?.modified()?,
        )
    }

    // use story document name for the single page
    // e.g. mit_gutem_gewissen.pdf -> mit_gutem_gewissen.html
    fn get_html_path(&self) -> anyhow::Result<String> {
        Ok(format!(
            "docs/stories/{}.html",
            self.path_to_document.replace(".pdf", "")
        ))
    }
}

// language of story content
#[derive(Deserialize, Serialize, Debug)]
pub enum Language {
    German,
    English,
}

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
                format!(
                    // use story document name for the single page
                    // e.g. mit_gutem_gewissen.pdf -> mit_gutem_gewissen.html
                    "docs/stories/{}.html",
                    story.path_to_document.replace(".pdf", "")
                )
                .as_str(),
                tera_instance.render(format!("{}.html", template_name).as_str(), &context)?,
            ));
        }
        Ok(())
    }
}

// language of story content
#[derive(Deserialize, Serialize, Debug)]
pub enum Language {
    German,
    English,
}

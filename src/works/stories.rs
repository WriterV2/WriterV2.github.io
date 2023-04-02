use anyhow::Context;
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
        let mut result = Vec::new();
        for story in self.0.iter() {
            if story.update_available()? {
                let mut context = tera::Context::new();
                context.insert("story", story);
                context.insert("text", &story.get_text()?);
                result.push(std::fs::write(
                    story.get_html_path().as_str(),
                    tera_instance.render(format!("{}.html", template_name).as_str(), &context)?,
                ));
            }
        }
        anyhow::Ok(()).with_context(|| "Failed to render single story pages")
    }
}

impl Story {
    // check if it's necessary to re-compile the html single page
    fn update_available(&self) -> anyhow::Result<bool> {
        anyhow::Ok(
            // HTML file doesn't exist yet
            !std::path::Path::new(&self.get_html_path()).exists() 
            // pdf document changes
            || std::fs::metadata(format!("docs/stories/{}", &self.path_to_document))?.modified()?
                > std::fs::metadata(self.get_html_path())?.modified()?
            // base template changes
            || std::fs::metadata("templates/base.html")?.modified()?
                > std::fs::metadata(self.get_html_path())?.modified()?
            // story template changes
            ||std::fs::metadata("templates/story.html")?.modified()? > std::fs::metadata(self.get_html_path())?.modified()?, // add error context
        )
        .with_context(|| format!("Failed to check for update for {}", self.title))
    }

    // use story document name for the single page
    // e.g. mit_gutem_gewissen.pdf -> mit_gutem_gewissen.html
    fn get_html_path(&self) -> String {
        format!(
            "docs/stories/{}.html",
            self.path_to_document.replace(".pdf", "")
        )
    }

    // extract text from pdf document
    fn get_text(&self) -> anyhow::Result<String> {
        let text = std::fs::read_to_string(format!("stories_html/{}.html", self.path_to_document.replace(".pdf", "")))?;

        anyhow::Ok(text).with_context(|| format!("Failed to extract text for {}", self.title))
    }
}

// language of story content
#[derive(Deserialize, Serialize, Debug)]
pub enum Language {
    German,
    English,
}

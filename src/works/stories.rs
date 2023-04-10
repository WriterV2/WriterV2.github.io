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
    #[serde(default)]
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

    // create stories from stories.json and add last modification date
    fn new_from_file(file: std::fs::File) -> anyhow::Result<Self> {
        let mut stories: Stories = serde_json::from_reader(std::io::BufReader::new(&file))?;
        for story in stories.0.iter_mut() {
            story.get_last_modified()?;
        }
        // add error context
        Ok(stories)
    }
}

impl Story {
    // check if it's necessary to re-compile the html single page
    fn update_available(&self) -> anyhow::Result<bool> {
        anyhow::Ok(
            // HTML file doesn't exist yet
            !std::path::Path::new(&self.get_html_path()).exists() 
            // pdf document changes
            || std::fs::metadata(self.get_pdf_path())?.modified()?
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

    // get path to pdf document
    fn get_pdf_path(&self) -> String {
        format!("docs/stories/{}", &self.path_to_document)
    }

    // get path to html file with story content
    fn get_content_html_path(&self) -> String {
        format!(
            "stories_html/{}.html",
            self.path_to_document.replace(".pdf", "")
        )
    }

    // extract text from pdf document
    fn get_text(&self) -> anyhow::Result<String> {
        let text = std::fs::read_to_string(self.get_content_html_path())?;
        anyhow::Ok(text).with_context(|| format!("Failed to extract text for {}", self.title))
    }

    // get date of last modification of pdf file
    fn get_last_modified(&mut self) -> anyhow::Result<chrono::NaiveDate> {
        use std::os::linux::fs::MetadataExt;
        let pdf_last_modification = std::fs::metadata(self.get_pdf_path())?.st_mtime();
        let html_last_modification = std::fs::metadata(self.get_html_path())?.st_mtime();
        let date = chrono::NaiveDateTime::from_timestamp(std::cmp::max(pdf_last_modification, html_last_modification), 0).date();
        self.last_update = date;
        Ok(date)
    }
}

// language of story content
#[derive(Deserialize, Serialize, Debug)]
pub enum Language {
    German,
    English,
}

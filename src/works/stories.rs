use anyhow::{Result, Ok, Context};
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
    language: Language,
    #[serde(default)]
    number_of_pages: u32,
    #[serde(default)]
    last_update: NaiveDate,
}

// see src/works/mod.rs for information of Works trait
impl super::Works for Stories {
    // render single pages for every story with the extracted text and a download button
    fn render_single_pages(
        self,
        tera_instance: &tera::Tera,
        template_name: &str,
    ) -> Result<Self> {
        let mut result = Vec::new();
        for story in self.0.iter() {
            if story.update_available().with_context(|| format!("Failed to check for update for {}", story.title))? {
                let mut context = tera::Context::new();
                context.insert("story", story);
                context.insert("text", &story.get_text().with_context(|| format!("Failed to get text from {}", story.title))?);
                result.push(std::fs::write(
                    story.get_html_path().as_str(),
                    tera_instance.render(format!("{}.html", template_name).as_str(), &context).with_context(|| format!("Failed to render single story page for {}", story.title))?,
                ));
            }
        }
        Ok(self)
    }

    // create stories from stories.json and add last modification date
    fn new_from_file(file: &std::fs::File) -> Result<Self> {
        let mut stories: Stories = serde_json::from_reader(std::io::BufReader::new(file)).with_context(|| format!("Failed to read JSON from {:#?}", file))?;
        for story in stories.0.iter_mut() {
            story.get_last_modified().with_context(|| format!("Failed to get last modified date for {}", story.title))?;
            story.get_pages_num().with_context(|| format!("Failed to get number of pages from {}'s PDF", story.title))?;
        }
        Ok(stories)
    }
}

impl Story {
    // check if it's necessary to re-compile the html single page
    fn update_available(&self) -> Result<bool> {
        Ok(
            // HTML file doesn't exist yet
            !std::path::Path::new(&self.get_html_path()).exists() 
            // pdf document changes
            || std::fs::metadata(self.get_pdf_path()).with_context(|| format!("Failed to get PDF metadata for {}", self.title))?.modified().with_context(|| format!("Failed to get last modified date for {}'s PDF", self.title))?
                > std::fs::metadata(self.get_html_path()).with_context(|| format!("Failed to get HTML metadata for {}", self.title))?.modified().with_context(|| format!("Failed to get last modified date for {}'s HTML", self.title))?
            // base template changes
            || std::fs::metadata("templates/base.html").context("Failed to get metadata for base HTML template")?.modified().context("Failed to get last modified date for base HTML template")?
                > std::fs::metadata(self.get_html_path()).with_context(|| format!("Failed to get HTML metadata for {}", self.title))?.modified().with_context(|| format!("Failed to get last modified date for {}'s HTML", self.title))?
            // story template changes
            ||std::fs::metadata("templates/story.html").context("Failed to get metadata for story HTML template")?.modified().context("Failed to get last modified date for story HTML template")? > std::fs::metadata(self.get_html_path()).with_context(|| format!("Failed to get HTML metadata for {}", self.title))?.modified().with_context(|| format!("Failed to get last modified date for {}'s HTML", self.title))?, // add error context
        )
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
    fn get_text(&self) -> Result<String> {
        let text = std::fs::read_to_string(self.get_content_html_path()).with_context(|| format!("Failed to read {}'s content HTML file", self.title))?;
        Ok(text)
    }

    // get date of last modification 
    fn get_last_modified(&mut self) -> Result<chrono::NaiveDate> {
        use std::os::linux::fs::MetadataExt;
        let pdf_last_modification = std::fs::metadata(self.get_pdf_path()).with_context(|| format!("Failed to get {}'s PDF", self.title))?.st_mtime();
        let html_path = std::fs::metadata(self.get_html_path());
        let html_last_modification = if let core::result::Result::Ok(file) = html_path {
            file.st_mtime()
        } else {
            0
        };
        let date = chrono::NaiveDateTime::from_timestamp(std::cmp::max(pdf_last_modification, html_last_modification), 0).date();
        self.last_update = date;
        Ok(date)
    }

    // get number of pages of pdf file
    fn get_pages_num(&mut self) -> Result<u32> {
        let file = pdf::file::File::open(self.get_pdf_path()).with_context(|| format!("Failed to get {}'s PDF", self.title))?;
        self.number_of_pages = file.num_pages();
        Ok(file.num_pages())
    }
}

// language of story content
#[derive(Deserialize, Serialize, Debug)]
pub enum Language {
    German,
    English,
}

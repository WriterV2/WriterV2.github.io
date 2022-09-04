use serde::{Deserialize, Serialize};

// stories and their information
// derive from JSON
// process to context for HTML
#[derive(Debug, Deserialize, Serialize)]
pub struct Stories(pub Vec<Story>);

#[derive(Debug, Deserialize, Serialize)]
pub struct Story {
    pub id: u8,
    pub title: String,
    pub description: String,
    pub path_to_document: String,
    pub slug: String,
    pub language: Language,
}

// see src/works/mod.rs for information of Works trait
impl super::Works for Stories {}

// language of story content
#[derive(Deserialize, Serialize, Debug)]
pub enum Language {
    German,
    English,
}

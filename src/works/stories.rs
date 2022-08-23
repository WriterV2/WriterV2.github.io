use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Stories(pub Vec<Story>);

#[derive(Debug, Deserialize)]
pub struct Story {
    pub title: String,
    pub description: String,
    pub path_to_document: String,
    pub slug: String,
    pub language: Language,
}

impl super::Works for Stories {}

#[derive(Deserialize, Debug)]
pub enum Language {
    German,
    English,
}

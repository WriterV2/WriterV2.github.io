use serde::{Deserialize, Serialize};

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

impl super::Works for Stories {}

#[derive(Deserialize, Serialize, Debug)]
pub enum Language {
    German,
    English,
}
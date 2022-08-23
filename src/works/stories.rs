use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
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

impl super::Works for Stories {
    fn create_tera_context(&self) -> Vec<tera::Context> {
        let mut context = Vec::new();
        for story in self.0.iter() {
            context.push(
                tera::Context::from_serialize(&story).expect("Trying to serialize to tera context"),
            );
        }
        context
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Language {
    German,
    English,
}

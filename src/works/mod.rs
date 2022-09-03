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
}

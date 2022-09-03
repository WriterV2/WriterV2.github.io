use serde::{Deserialize, Serialize};

pub mod stories;

pub trait Works: for<'a> Deserialize<'a> + Serialize {
    fn new_from_file(file: std::fs::File) -> Self {
        match serde_json::from_reader(std::io::BufReader::new(file)) {
            Ok(work) => work,
            Err(err) => panic!("Trying to create work from JSON File - {:?}", err),
        }
    }

    fn create_tera_context(&self) -> tera::Context {
        let mut context = tera::Context::new();
        context.insert("works", &self);
        context
    }
}

use tera::{Context, Tera};

use crate::works::Works;

mod works;

fn main() {
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let context = Context::new();

    let output = tera
        .render("index.html", &context)
        .expect("Failed to render index.html");

    std::fs::write("docs/index.html", output)
        .expect("Failed to write rendered output to index.html");

    let mut file = match std::fs::File::open(std::path::PathBuf::from("stories.json")) {
        Ok(file) => file,
        Err(err) => panic!("Trying to open stories.json in readmode - {:?}", err),
    };

    let stories = works::stories::Stories::new_from_file(file);
    println!("Stories: {:?}", stories);

    let context = stories.create_tera_context();
    println!("{:?}", context);
}

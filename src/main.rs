use tera::{Context, Tera};

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
}

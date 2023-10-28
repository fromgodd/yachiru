use std::fs;
use std::path::Path;
use comrak::{markdown_to_html, ComrakOptions};
use tera::{Tera, Context};

fn main() {
    // Initialize Tera with the templates in the templates directory
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    // Create dist directory
    fs::create_dir_all("dist").unwrap();

    // Process index.md
    process_md_file(&tera, "content/index.md", "dist/index.html");

    // Process articles
    for entry in fs::read_dir("content/articles").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() && path.extension().unwrap() == "md" {
            let output_path = format!("dist/{}.html", path.file_stem().unwrap().to_str().unwrap());
            process_md_file(&tera, &path.to_str().unwrap(), &output_path);
        }
    }
}

fn process_md_file(tera: &Tera, input_path: &str, output_path: &str) {
    // Read markdown file
    let md = fs::read_to_string(input_path).unwrap();

    // Convert markdown to HTML
    let html = markdown_to_html(&md, &ComrakOptions::default());

    // Apply template
    let mut context = Context::new();
    context.insert("content", &html);
    let result = tera.render("template.html", &context).unwrap();

    // Write to file
    fs::write(output_path, result).unwrap();
}

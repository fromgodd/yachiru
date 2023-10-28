use tera::Tera;
use walkdir::WalkDir;
use std::fs::File;
use std::io::Write;
use frontmatter::Frontmatter;
use std::path::PathBuf;

fn render_template(tera: &Tera, template_name: &str, context: &tera::Context) -> String {
    tera.render(template_name, context).expect("Failed to render template")
}

fn main() {
    // Initialize Tera with your template files
    let tera = Tera::new("templates/**/*").expect("Failed to create Tera instance");

    // Define the output directory
    let output_dir = "dist";

    // Create the output directory if it doesn't exist
    std::fs::create_dir_all(output_dir).expect("Failed to create output directory");

    // Walk through the content directory and process each Markdown file
    for entry in WalkDir::new("content") {
        if let Ok(entry) = entry {
            if entry.path().extension().and_then(|e| e.to_str()) == Some("md") {
                // Read the file content and parse Front Matter
                let content = std::fs::read_to_string(entry.path()).unwrap();
                let parsed = Frontmatter::parse(&content).unwrap();

                // Access metadata and content
                let metadata = parsed.metadata;
                let content = parsed.document;

                // Create a Tera context
                let mut context = tera::Context::new();
                context.insert("content", &content);

                // Include metadata in the Tera context
                for (key, value) in metadata.iter() {
                    context.insert(key, value);
                }

                // Render the template with the context
                let rendered = render_template(&tera, "base.html", &context);

                // Create the output file path
                let relative_path = entry.path().strip_prefix("content").unwrap();
                let output_path = PathBuf::from(output_dir).join(relative_path);

                // Ensure the output directories exist
                std::fs::create_dir_all(output_path.parent().unwrap())
                    .expect("Failed to create output directories");

                // Write the rendered content to the output file
                let mut output_file = File::create(&output_path).expect("Failed to create output file");
                output_file
                    .write_all(rendered.as_bytes())
                    .expect("Failed to write to output file");
            }
        }
    }

    println!("Website generated at: {}", output_dir);
}

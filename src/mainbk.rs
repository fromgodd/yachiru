use std::fs;
use std::path::PathBuf;
use comrak::{markdown_to_html, ComrakOptions};
use tera::{Tera, Context};
use std::collections::HashMap;
use std::env;

fn main() {
    // Get current directory
    let current_dir = env::current_dir().unwrap();

    // Initialize Tera with the templates in the templates directory
    let tera = match Tera::new(&format!("{}/templates/**/*", current_dir.to_str().unwrap())) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    // Create dist directory
    fs::create_dir_all(current_dir.join("dist")).unwrap();

    // Copy static files to dist directory
    copy_dir(current_dir.join("static"), current_dir.join("dist/static")).unwrap();

    // Process index.md
    process_md_file(&tera, &current_dir.join("content/index.md"), &current_dir.join("dist/index.html"));

    // Create articles directory in dist
    fs::create_dir_all(current_dir.join("dist/articles")).unwrap();

    // Process articles
    for entry in fs::read_dir(current_dir.join("content/articles")).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() && path.extension().unwrap() == "md" {
            let output_path = format!("{}/dist/articles/{}.html", current_dir.to_str().unwrap(), path.file_stem().unwrap().to_str().unwrap());
            process_md_file(&tera, &PathBuf::from(path.to_str().unwrap()), &PathBuf::from(&output_path));
        }
    }
}

fn process_md_file(tera: &Tera, input_path: &PathBuf, output_path: &PathBuf) {
    // Read markdown file
    let md = fs::read_to_string(input_path).unwrap();

    // Parse front matter and content
    let (front_matter, content) = parse_front_matter(&md);

    // Convert markdown content to HTML
    let html_content = markdown_to_html(&content, &ComrakOptions::default());

    // Apply template
    let mut context = Context::new();
    context.insert("content", &html_content);
    context.insert("title", front_matter.get("title").unwrap_or(&"".into()));
    let result = tera.render("template.html", &context).unwrap();

    // Write to file
    fs::write(output_path, result).unwrap();
}

fn parse_front_matter(md: &str) -> (HashMap<String, String>, String) {
    if !md.starts_with("---") {
        return (HashMap::new(), md.to_string());
    }

    let mut lines = md.lines();
    
    lines.next();  // Skip first ---
    
    let mut front_matter = HashMap::new();
    
    loop {
        if let Some(line) = lines.next() {
            if line == "---" {
                break;
            }
            
            let mut parts = line.splitn(2, ':');
            if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                front_matter.insert(key.trim().to_string(), value.trim().to_string());
            }
        } else {
            break;
        }
    }

    let content: String = lines.collect::<Vec<&str>>().join("\n");
    
    (front_matter, content)
}

// Function to recursively copy a directory
fn copy_dir(src: PathBuf, dst: PathBuf) -> std::io::Result<()> {
    if !src.is_dir() {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Source is not a directory"));
    }

    fs::create_dir_all(&dst)?;

    for entry_result in src.read_dir()? {
        let entry = entry_result?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            copy_dir(entry.path(), dst.join(entry.file_name()))?;
        } else if file_type.is_file() {
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }

    Ok(())
}

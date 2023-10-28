// content.rs

use std::fs;
use pulldown_cmark::{html, Options, Parser};

pub struct BlogPost {
    pub title: String,
    pub content: String,
}

impl BlogPost {
    // Create a new BlogPost from a Markdown file
    pub fn from_file(file_path: &str) -> Result<BlogPost, std::io::Error> {
        let file_content = fs::read_to_string(file_path)?;

        // Parse the Markdown content into HTML
        let options = Options::empty();
        let parser = Parser::new_ext(&file_content, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        Ok(BlogPost {
            title: String::from("Title Placeholder"), // Implement title extraction
            content: html_output,
        })
    }
}

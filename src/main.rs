// src/main.rs

use clap::{App, Arg};
use chrono::prelude::*;
use std::fs::File;
use std::io::Write;

fn create_new_article(title: &str) {
    let timestamp = Local::now();
    let filename = timestamp.format("%Y-%m-%d-%H-%M-%S").to_string();
    let content = format!(
        "---\ntitle: {}\ndate: {}\n---\n\n# {}",
        title,
        timestamp.to_rfc3339(),
        title
    );

    let file_path = format!("content/articles/{}.md", filename);
    let mut file = File::create(&file_path).expect("Failed to create article file");
    file.write_all(content.as_bytes())
        .expect("Failed to write article content");

    println!("Created a new article: {}", filename);
}

fn main() {
    let matches = App::new("Yachiru")
        .version("1.0")
        .author("Your Name")
        .about("A static blog generator.")
        .subcommand(App::new("new")
            .about("Create a new article")
            .arg(Arg::with_name("title")
                .help("Sets the title of the article")
                .required(true)
                .takes_value(true))
        )
        .get_matches();

    if let Some(new_matches) = matches.subcommand_matches("new") {
        if let Some(title) = new_matches.value_of("title") {
            create_new_article(title);
        }
    }
}

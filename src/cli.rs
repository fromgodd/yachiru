use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use chrono::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 || args[1] != "new" || args[3] != "--type" || args[5] != "--date" {
        println!("Usage: yachiru new <title> --type article --date true|false");
        return;
    }

    let title = &args[2];
    let date = if args[6] == "true" { Utc::now().to_string() } else { "".to_string() };

    let path = Path::new("content/articles").join(format!("{}.md", title));

    let mut file = File::create(&path).expect("Unable to create file");
    write!(file, "---\ntitle: {}\ndate: {}\n---\n", title, date).expect("Unable to write data");

    println!("Created new article at {:?}", path);
}

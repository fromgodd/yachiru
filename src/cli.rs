use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 || args[1] != "article" {
        println!("Usage: yachiru article <article_name>");
        return;
    }

    let article_name = &args[2];
    let path = Path::new("content/articles").join(format!("{}.md", article_name));

    let mut file = File::create(&path).expect("Unable to create file");
    file.write_all(b"---\ntitle: \n---\n").expect("Unable to write data");
    
    println!("Created new article at {:?}", path);
}
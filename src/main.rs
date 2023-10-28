extern crate yachiru;

use yachiru::ssg; // Import the ssg module from your project
use clap::{App, Arg};

fn main() {
    let matches = App::new("Yachiru SSG")
        .version("1.0")
        .author("Your Name")
        .about("A simple Static Site Generator")
        .subcommand(
            App::new("new")
                .about("Create a new website")
                .arg(Arg::with_name("name").required(true).index(1)),
        )
        .get_matches();

    if let Some(subcommand) = matches.subcommand_name() {
        match subcommand {
            "new" => {
                let sub_matches = matches.subcommand_matches("new").unwrap();
                let project_name = sub_matches.value_of("name").unwrap();
                ssg::generate_site(project_name);
            }
            _ => println!("Invalid subcommand"),
        }
    }
}

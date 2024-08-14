use clap::Parser;
use std::collections::HashMap;
use std::fs;

#[derive(Parser)]
#[command(name = "grep-clap")]
#[command(version)]
#[command(about = "My first cli on clap", long_about = None)]
pub struct Cli {
    pub name_file: String,
    pub query: String,

    #[arg(short, long)]
    lov_case: bool,
}

impl Cli {
    pub fn run<'a>(self, name_file: &'a str, query: &'a str) {
        let contents = fs::read_to_string(name_file);

        let contents = match contents {
            Ok(s) => s,
            Err(e) => panic!("{}", e),
        };

        if contents.is_empty() {
            panic!("This is an empty file ");
        }

        if self.lov_case == true {
            let result = search_insensative(&query, &contents);
            for (line_number, line) in result {
                println!("{}: {}", line_number, line);
            }
        } else {
            let result = search(&query, &contents);

            for (line_number, line) in result {
                println!("{}: {}", line_number, line);
            }
        }
    }
}

fn search<'a>(query: &'a str, content: &'a str) -> HashMap<usize, String> {
    let mut result = HashMap::new();

    for (i, line) in content.lines().enumerate() {
        if line.contains(query) {
            result.insert(i, line.to_string());
        }
    }

    result
}

fn search_insensative<'a>(query: &'a str, content: &'a str) -> HashMap<usize, String> {
    let query = query.to_lowercase();
    let mut result = HashMap::new();

    for (i, line) in content.lines().enumerate() {
        if line.to_lowercase().contains(&query) {
            result.insert(i, line.to_string());
        }
    }

    result
}

pub fn parse_cli() -> Cli {
    Cli::parse()
}

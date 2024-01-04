mod parser;
mod utils;

use std::process::exit;
use utils::parse_args;
use parser::JSON;

fn main() {
    let files = match parse_args() {
        None => {
            eprintln!("json-parser: usage: json-parser [file ...]");
            exit(1);
        }
        Some(files) => files,
    };

    let mut status_code = 0;
    for file in files {
        match JSON::parse_from_file(&file) {
            Err(err) => {
                status_code = 1;
                eprintln!("{}", err);
            },
            Ok(json) => {
                json.display();
            }
        };
    }

    exit(status_code);
}

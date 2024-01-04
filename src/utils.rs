use std::{env};

pub fn parse_args() -> Option<Vec<String>> {
    let mut args = env::args();
    args.next();

    let files: Vec<String> = args.collect();
    if files.len() == 0 {
        None
    } else {
        Some(files)
    }
}

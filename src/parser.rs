use std::{collections::HashMap, fmt::Display, fs};

#[derive(Debug)]
enum JSONValue {
    String(String),
    Number(i32),
    Bool(bool),
    Array(Vec<JSONValue>),
    Object(HashMap<String, JSONValue>),
}

#[derive(Debug)]
pub struct JSON {
    keys: Vec<String>,
    values: Vec<JSONValue>,
}

pub struct ArgsParseError(String);
impl Display for ArgsParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct JSONParseError;
impl Display for JSONParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid JSON file!")
    }
}

impl JSON {
    pub fn parse_from_file(file_name: &String) -> Result<JSON, ArgsParseError> {
        if !file_name.ends_with(".json") {
            return Err(ArgsParseError(format!("{} is not a JSON file", file_name)));
        }

        match fs::read_to_string(file_name) {
            Ok(content) => match JSON::parse(content) {
                Ok(json) => Ok(json),
                Err(err) => Err(ArgsParseError(format!("{}", err)))
            },
            Err(_) => Err(ArgsParseError(format!("{} does not exist!", file_name))),
        }
    }

    fn parse(content: String) -> Result<JSON, JSONParseError> {
        if !content.starts_with('{') || !content.ends_with('}') {
            Err(JSONParseError)
        } else {
            Ok(JSON {
                keys: vec![],
                values: vec![],
            })
        }
    }

    pub fn display(&self) {
        println!("{:?}", self);
    }
}

use std::{collections::HashMap, fmt::Display, fs, vec::IntoIter};

#[derive(Debug)]
enum JSONValue {
    String(String),
    Number(i32),
    Bool(bool),
    Array(Vec<JSONValue>),
    Object(HashMap<String, JSONValue>),
}

impl Display for JSONValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            JSONValue::String(val) => write!(f, "{}", val),
            JSONValue::Number(val) => write!(f, "{}", val),
            JSONValue::Bool(val) => write!(f, "{}", val),
            JSONValue::Array(vals) => {
                let mut str_val = String::new();
                for val in vals {
                    str_val.push_str(&format!("{}, ", val));
                }
                write!(f, "[{}]", str_val)
            }
            JSONValue::Object(map) => {
                let mut str_val = String::new();
                for key in map.keys() {
                    str_val.push_str(&format!("{}: {}", key, map.get(key).unwrap()));
                }
                write!(f, "[{}]", str_val)
            }
        }
    }
}

#[derive(Debug)]
pub struct JSON {
    object: HashMap<String, JSONValue>,
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
            Ok(content) => match JSON::parse(format!("{}", &content.trim())) {
                Ok(json) => Ok(json),
                Err(err) => Err(ArgsParseError(format!("{}", err))),
            },
            Err(_) => Err(ArgsParseError(format!("{} does not exist!", file_name))),
        }
    }

    fn parse(content: String) -> Result<JSON, JSONParseError> {
        if !content.starts_with('{') || !content.ends_with('}') {
            Err(JSONParseError)
        } else {
            let mut json = JSON {
                object: HashMap::new(),
            };

            let mut tokens = content.chars().skip(1).collect::<Vec<char>>().into_iter();
            while tokens.len() > 0 {
                match JSON::get_pair(&mut tokens) {
                    Ok((key, value)) => {
                        json.object.insert(key, value);
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            }

            Ok(json)
        }
    }

    fn get_pair(tokens: &mut IntoIter<char>) -> Result<(String, JSONValue), JSONParseError> {
        let key = match JSON::parse_key(tokens) {
            Ok(key) => key,
            Err(err) => return Err(err),
        };

        if let Some(err) = JSON::skip_colons(tokens) {
            return Err(err);
        }
        
        let value = match JSON::parse_value(tokens) {
            Ok(value) => value,
            Err(err) => return Err(err),
        };

        Ok((key, value))
    }

    fn parse_key(tokens: &mut IntoIter<char>) -> Result<String, JSONParseError> {

        let mut start = tokens.next().unwrap();
        while start.is_whitespace() {
            start = match tokens.next() {
                None => return Err(JSONParseError),
                Some(ch) => ch,
            }
        }

        if start != '"' {
            return Err(JSONParseError);
        }

        let mut key = String::new();
        let mut escaped = false;

        while let Some(ch) = tokens.next() {
            if escaped {
                key.push(ch);
                escaped = false;
            } else {
                match ch {
                    '"' => return Ok(key),
                    '\\' => {
                        escaped = true;
                    }
                    _ => key.push(ch),
                }
            }
        }

        Ok(String::new())
    }

    fn skip_colons(tokens: &mut IntoIter<char>) -> Option<JSONParseError> {
        todo!()
    }

    fn parse_value(tokens: &mut IntoIter<char>) -> Result<JSONValue, JSONParseError> {
        todo!()
    }
}

impl Display for JSON {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut json_str = String::from("{\n");
        for key in self.object.keys() {
            json_str.push_str(&format!("{}: {}\n", key, self.object.get(key).unwrap()));
        }
        json_str.push('}');
        write!(f, "{}", json_str)
    }
}

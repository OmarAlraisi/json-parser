use std::{collections::HashMap, fmt::Display, fs, iter::Peekable};

#[derive(Debug)]
enum JSONValue {
    String(String),
    Number(i32),
    Bool(bool),
    Null,
    Array(Vec<JSONValue>),
    Object(JSON),
}

impl Display for JSONValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            JSONValue::String(val) => write!(f, "\"{}\"", val),
            JSONValue::Number(val) => write!(f, "{}", val),
            JSONValue::Bool(val) => write!(f, "{}", val),
            JSONValue::Null => write!(f, "null"),
            JSONValue::Array(vals) => {
                let mut str_val = String::new();
                for (idx, val) in vals.iter().enumerate() {
                    str_val.push_str(&format!(
                        "{}{}",
                        val,
                        if idx < vals.len() - 1 { "," } else { "" }
                    ));
                }
                write!(f, "[{}]", str_val)
            }
            JSONValue::Object(json) => write!(f, "{}", json),
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
            Ok(content) => JSON::parse_from_string(content),
            Err(_) => Err(ArgsParseError(format!("{} does not exist!", file_name))),
        }
    }

    fn parse_from_string(content: String) -> Result<JSON, ArgsParseError> {
        match JSON::parse(format!("{}", content.trim())) {
            Ok(json) => Ok(json),
            Err(err) => Err(ArgsParseError(format!("{}", err))),
        }
    }

    fn parse(content: String) -> Result<JSON, JSONParseError> {
        if !content.starts_with('{') || !content.ends_with('}') {
            Err(JSONParseError)
        } else {
            let mut json = JSON {
                object: HashMap::new(),
            };

            let mut tokens = content
                .chars()
                .skip(1)
                .collect::<Vec<char>>()
                .into_iter()
                .peekable();

            while tokens.len() > 1 {
                match JSON::get_pair(&mut tokens) {
                    Ok((key, value)) => {
                        json.object.insert(key, value);
                        match JSON::skip_whitspace(&mut tokens) {
                            Some(ch) => match ch {
                                '}' => {
                                    return Ok(json);
                                }
                                ',' => {
                                    while let Some(token) = tokens.peek() {
                                        if *token == '}' {
                                            return Err(JSONParseError);
                                        }
                                        if token.is_whitespace() {
                                            tokens.next().unwrap();
                                        } else {
                                            break;
                                        }
                                    }
                                }
                                _ => return Err(JSONParseError),
                            },
                            None => return Err(JSONParseError),
                        }
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            }

            Ok(json)
        }
    }

    fn get_pair<I: Iterator<Item = char>>(
        tokens: &mut Peekable<I>,
    ) -> Result<(String, JSONValue), JSONParseError> {
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

    fn skip_whitspace<I: Iterator<Item = char>>(tokens: &mut Peekable<I>) -> Option<char> {
        while let Some(ch) = tokens.next() {
            if !ch.is_whitespace() {
                return Some(ch);
            }
        }

        None
    }

    fn parse_key<I: Iterator<Item = char>>(
        tokens: &mut Peekable<I>,
    ) -> Result<String, JSONParseError> {
        let start = match JSON::skip_whitspace(tokens) {
            None => return Err(JSONParseError),
            Some(ch) => ch,
        };

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

        Err(JSONParseError)
    }

    fn skip_colons<I: Iterator<Item = char>>(tokens: &mut Peekable<I>) -> Option<JSONParseError> {
        match JSON::skip_whitspace(tokens) {
            Some(ch) => match ch {
                ':' => None,
                _ => Some(JSONParseError),
            },
            None => Some(JSONParseError),
        }
    }

    fn parse_value<I: Iterator<Item = char>>(
        tokens: &mut Peekable<I>,
    ) -> Result<JSONValue, JSONParseError> {
        let token = match JSON::skip_whitspace(tokens) {
            Some(ch) => ch,
            None => return Err(JSONParseError),
        };

        match token {
            '"' => match JSON::parse_string_value(tokens) {
                Ok(val) => Ok(JSONValue::String(val)),
                Err(err) => Err(err),
            },
            'n' => {
                let mut str = String::from("n");
                while let Some(ch) = tokens.next() {
                    str.push(ch);
                    if ch.is_whitespace() || (ch == 'l' && str.len() == 4) {
                        break;
                    }
                }

                match str.as_str() {
                    "null" => Ok(JSONValue::Null),
                    _ => Err(JSONParseError),
                }
            }
            't' => {
                let mut str = String::from("t");
                while let Some(ch) = tokens.next() {
                    str.push(ch);
                    if ch.is_whitespace() || ch == 'e' {
                        break;
                    }
                }

                match str.as_str() {
                    "true" => Ok(JSONValue::Bool(true)),
                    _ => Err(JSONParseError),
                }
            }
            'f' => {
                let mut str = String::from("f");
                while let Some(ch) = tokens.next() {
                    str.push(ch);
                    if ch.is_whitespace() || ch == 'e' {
                        break;
                    }
                }
                match str.as_str() {
                    "false" => Ok(JSONValue::Bool(false)),
                    _ => Err(JSONParseError),
                }
            }
            '{' => match JSON::parse_object_value(tokens) {
                Ok(json) => Ok(JSONValue::Object(json)),
                Err(err) => Err(err),
            },
            '[' => match JSON::parse_array_value(tokens) {
                Ok(array) => Ok(JSONValue::Array(array)),
                Err(err) => Err(err),
            },
            _ => {
                if token.is_numeric() || token == '-' {
                    match JSON::parse_numeric_value(token, tokens) {
                        Ok(num) => Ok(JSONValue::Number(num)),
                        Err(err) => Err(err),
                    }
                } else {
                    Err(JSONParseError)
                }
            }
        }
    }

    fn parse_array_value<I: Iterator<Item = char>>(
        tokens: &mut Peekable<I>,
    ) -> Result<Vec<JSONValue>, JSONParseError> {
        let mut array: Vec<JSONValue> = vec![];

        while let Some(token) = tokens.peek() {
            if *token == ']' {
                tokens.next().unwrap();
                return Ok(array);
            }
            match JSON::parse_value(tokens) {
                Ok(val) => array.push(val),
                Err(err) => return Err(err),
            }
            match JSON::skip_whitspace(tokens) {
                None => return Err(JSONParseError),
                Some(token) => match token {
                    ',' => {}
                    ']' => return Ok(array),
                    _ => return Err(JSONParseError),
                },
            }
        }

        Err(JSONParseError)
    }

    fn parse_object_value<I: Iterator<Item = char>>(
        tokens: &mut Peekable<I>,
    ) -> Result<JSON, JSONParseError> {
        let mut object_str = String::from('{');
        let mut in_string = false;
        let mut opened = 0;

        while let Some(token) = tokens.next() {
            object_str.push(token);
            match token {
                '"' => {
                    in_string = !in_string;
                }
                '{' => {
                    if !in_string {
                        opened += 1
                    }
                }
                '}' => {
                    if !in_string {
                        if opened == 0 {
                            break;
                        }
                        opened -= 1;
                    }
                }
                _ => {}
            }
        }

        match JSON::parse_from_string(object_str) {
            Ok(json) => Ok(json),
            Err(_) => Err(JSONParseError),
        }
    }

    fn parse_numeric_value<I: Iterator<Item = char>>(
        digit: char,
        tokens: &mut Peekable<I>,
    ) -> Result<i32, JSONParseError> {
        let mut value = String::from(digit);

        while let Some(ch) = tokens.peek() {
            if !ch.is_numeric() {
                break;
            }
            value.push(tokens.next().unwrap());
        }

        return match value.parse::<i32>() {
            Ok(num) => Ok(num),
            Err(_) => Err(JSONParseError),
        };
    }

    fn parse_string_value<I: Iterator<Item = char>>(
        tokens: &mut Peekable<I>,
    ) -> Result<String, JSONParseError> {
        let mut value = String::new();
        let mut escaped = false;

        while let Some(ch) = tokens.next() {
            if escaped {
                value.push(ch);
                escaped = false;
            } else {
                match ch {
                    '"' => return Ok(value),
                    '\\' => {
                        escaped = true;
                    }
                    _ => value.push(ch),
                }
            }
        }

        Err(JSONParseError)
    }
}

fn get_padded_string(str: String) -> String {
    let mut output = String::new();
    for line in str.lines() {
        output.push_str("  ");
        output.push_str(line);
        output.push('\n');
    }
    output
}
impl Display for JSON {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.object.len() == 0 {
            write!(f, "{}{}", '{', '}')
        } else {
            let mut json_str = String::new();
            for (idx, key) in self.object.keys().enumerate() {
                if key.contains(' ') {
                    json_str.push_str(&format!("\"{}\"", key));
                } else {
                    json_str.push_str(key);
                }
                json_str.push_str(": ");
                json_str.push_str(&format!("{}", self.object.get(key).unwrap()));
                if idx < self.object.len() - 1 {
                    json_str.push(',');
                }
                json_str.push('\n');
            }
            write!(f, "{}\n{}{}", '{', get_padded_string(json_str), '}')
        }
    }
}

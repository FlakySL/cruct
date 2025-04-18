use std::collections::HashMap;
use std::fs::read_to_string;

use jzon::{JsonValue, parse};

use super::{ConfigValue, Parser, ParserError};

#[derive(Clone)]
pub struct JsonParser;

impl Parser for JsonParser {
    fn extensions(&self) -> &'static [&'static str] {
        &["json"]
    }

    fn load(&self, path: &str) -> Result<ConfigValue, ParserError> {
        let content = read_to_string(path)?;
        let json = parse(&content)?;
        parse_json_value(json)
    }
}

fn parse_json_value(value: JsonValue) -> Result<ConfigValue, ParserError> {
    match value {
        JsonValue::Object(obj) => {
            let mut map = HashMap::new();
            for (k, v) in obj {
                map.insert(k, parse_json_value(v)?);
            }
            Ok(ConfigValue::Section(map))
        },
        JsonValue::Array(arr) => {
            let items = arr
                .into_iter()
                .map(parse_json_value)
                .collect::<Result<Vec<_>, _>>()?;
            Ok(ConfigValue::Array(items))
        },
        _ => Ok(ConfigValue::Value(value.to_string())),
    }
}

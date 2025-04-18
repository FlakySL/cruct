use std::collections::HashMap;
use std::fs;

use toml::Value;

use super::{ConfigValue, Parser, ParserError};

#[derive(Clone)]
pub struct TomlParser;

impl Parser for TomlParser {
    fn extensions(&self) -> &'static [&'static str] {
        &["toml"]
    }

    fn load(&self, path: &str) -> Result<ConfigValue, ParserError> {
        let content = fs::read_to_string(path)?;
        let value = content.parse::<toml::Value>()?;
        parse_toml_value(value)
    }
}

fn parse_toml_value(value: toml::Value) -> Result<ConfigValue, ParserError> {
    match value {
        Value::Table(table) => {
            let mut map = HashMap::new();
            for (k, v) in table {
                map.insert(k, parse_toml_value(v)?);
            }
            Ok(ConfigValue::Section(map))
        },
        Value::Array(arr) => {
            let items = arr
                .into_iter()
                .map(parse_toml_value)
                .collect::<Result<Vec<_>, _>>()?;
            Ok(ConfigValue::Array(items))
        },
        Value::String(s) => Ok(ConfigValue::Value(s)),
        Value::Integer(i) => Ok(ConfigValue::Value(i.to_string())),
        Value::Float(f) => Ok(ConfigValue::Value(f.to_string())),
        Value::Boolean(b) => Ok(ConfigValue::Value(b.to_string())),
        Value::Datetime(dt) => Ok(ConfigValue::Value(dt.to_string())),
    }
}

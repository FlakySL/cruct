use super::{Parser, ParserError};
use std::collections::HashMap;
use std::fs;
use toml::Value;

#[derive(Clone)]
pub struct TomlParser;

impl Parser for TomlParser {
    fn extensions(&self) -> &'static [&'static str] {
        &["toml"]
    }

    fn load(&self, path: &str) -> Result<HashMap<String, String>, ParserError> {
        let content = fs::read_to_string(path)?;
        let value = content.parse::<Value>()?;
        let mut map = HashMap::new();

        if let Value::Table(table) = value {
            for (k, v) in table {
                let value_str = match v {
                    Value::String(s) => s,
                    Value::Integer(i) => i.to_string(),
                    Value::Float(f) => f.to_string(),
                    Value::Boolean(b) => b.to_string(),
                    _ => v.to_string(),
                };

                map.insert(k, value_str);
            }
        }

        Ok(map)
    }
}

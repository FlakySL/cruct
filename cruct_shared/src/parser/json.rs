use super::{Parser, ParserError};
use jzon::JsonValue;
use std::collections::HashMap;
use std::fs;

#[derive(Clone)]
pub struct JsonParser;

impl Parser for JsonParser {
    fn extensions(&self) -> &'static [&'static str] {
        &["json"]
    }

    fn load(&self, path: &str) -> Result<HashMap<String, String>, ParserError> {
        let content = fs::read_to_string(path)?;
        let json = jzon::parse(&content)?;
        let mut map = HashMap::new();

        if let JsonValue::Object(obj) = json {
            for (k, v) in obj {
                map.insert(k, v.to_string()); // Convert all values to strings
            }
        }

        Ok(map)
    }
}

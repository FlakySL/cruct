use std::collections::HashMap;
use std::fs;

use yaml_rust2::{Yaml, YamlLoader};

use super::{Parser, ParserError};

#[derive(Clone)]
pub struct YmlParser;

impl Parser for YmlParser {
    fn extensions(&self) -> &'static [&'static str] {
        &["yml", "yaml"]
    }

    fn load(&self, path: &str) -> Result<HashMap<String, String>, ParserError> {
        let content = fs::read_to_string(path)?;
        let docs = YamlLoader::load_from_str(&content)?;

        let doc = docs
            .first()
            .ok_or(ParserError::TypeMismatch {
                field: "document".to_string(),
                expected: "non-empty YAML document".to_string(),
            })?;

        let mut map = HashMap::new();

        if let Some(hash) = doc.as_hash() {
            for (k, v) in hash {
                if let Some(k_str) = k.as_str() {
                    let value = parse_yaml_value(v).map_err(|_| ParserError::TypeMismatch {
                        field: k_str.to_string(),
                        expected: "string".to_string(),
                    })?;

                    map.insert(k_str.to_string(), value);
                }
            }
        }

        Ok(map)
    }
}

fn parse_yaml_value(value: &Yaml) -> Result<String, ParserError> {
    match value {
        Yaml::String(s) => Ok(s.clone()),
        Yaml::Integer(i) => Ok(i.to_string()),
        Yaml::Boolean(b) => Ok(b.to_string()),
        Yaml::Real(s) => Ok(s.clone()),
        Yaml::Null => Ok("null".to_string()),
        _ => Err(ParserError::TypeMismatch {
            field: "value".to_string(),
            expected: "string".to_string(),
        }),
    }
}

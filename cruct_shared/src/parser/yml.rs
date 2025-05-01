use std::collections::HashMap;
use std::fs::read_to_string;

use yaml_rust2::{Yaml, YamlLoader};

use super::{ConfigValue, Parser, ParserError};

#[derive(Clone)]
pub struct YmlParser;

impl Parser for YmlParser {
    fn extensions(&self) -> &'static [&'static str] {
        &["yml", "yaml"]
    }

    fn load(&self, path: &str) -> Result<ConfigValue, ParserError> {
        let content = read_to_string(path)?;
        let docs = YamlLoader::load_from_str(&content)?;

        let doc = docs
            .first()
            .ok_or(ParserError::TypeMismatch {
                field: "document".to_string(),
                expected: "non-empty YAML document".to_string(),
            })?;

        parse_yaml_value(doc.clone())
    }
}

fn parse_yaml_value(value: Yaml) -> Result<ConfigValue, ParserError> {
    match value {
        Yaml::Hash(hash) => {
            let mut map = HashMap::new();
            for (k, v) in hash {
                if let Yaml::String(k_str) = k {
                    map.insert(k_str, parse_yaml_value(v)?);
                }
            }
            Ok(ConfigValue::Section(map))
        },
        Yaml::Array(arr) => {
            let items = arr
                .into_iter()
                .map(parse_yaml_value)
                .collect::<Result<Vec<_>, _>>()?;
            Ok(ConfigValue::Array(items))
        },
        Yaml::String(s) => Ok(ConfigValue::Value(s)),
        Yaml::Integer(i) => Ok(ConfigValue::Value(i.to_string())),
        Yaml::Boolean(b) => Ok(ConfigValue::Value(b.to_string())),
        Yaml::Real(s) => Ok(ConfigValue::Value(s)),
        Yaml::Null => Ok(ConfigValue::Value("null".to_string())),
        _ => Err(ParserError::TypeMismatch {
            field: "value".to_string(),
            expected: "supported YAML type".to_string(),
        }),
    }
}

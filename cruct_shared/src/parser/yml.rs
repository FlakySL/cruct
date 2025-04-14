use super::{Parser, ParserError};
use std::collections::HashMap;
use std::fs;
use yaml_rust2::{Yaml, YamlLoader};

#[derive(Clone)]
pub struct YmlParser;

impl Parser for YmlParser {
    fn extensions(&self) -> &'static [&'static str] {
        &["yml", "yaml"]
    }

    fn load(&self, path: &str) -> Result<HashMap<String, String>, ParserError> {
        let content = fs::read_to_string(path)?;
        let docs = YamlLoader::load_from_str(&content)?;
        let doc = &docs[0];
        let mut map = HashMap::new();

        if let Some(hash) = doc.as_hash() {
            for (k, v) in hash {
                if let Some(k_str) = k.as_str() {
                    let value = match v {
                        Yaml::String(s) => s.clone(),
                        Yaml::Integer(i) => i.to_string(),
                        Yaml::Real(f) => f.clone(),
                        Yaml::Boolean(b) => b.to_string(),
                        Yaml::Array(_) | Yaml::Hash(_) => {
                            return Err(ParserError::TypeMismatch {
                                field: k_str.to_string(),
                                expected: "primitive value".to_string(),
                            });
                        }
                        _ => "".to_string(),
                    };

                    map.insert(k_str.to_string(), value);
                }
            }
        }

        Ok(map)
    }
}

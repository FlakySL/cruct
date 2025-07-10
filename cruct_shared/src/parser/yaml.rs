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

/// Parses a YAML value into a corresponding `ConfigValue` type.
///
/// This function recursively converts YAML structures (e.g., hashes, arrays,
/// strings, etc.) into the application's internal configuration representation
/// (`ConfigValue`). Unsupported YAML types will result in a `ParserError`.
///
/// # Arguments
///
/// * `value` - A `Yaml` value representing the YAML element to parse.
///
/// # Returns
///
/// * `Result<ConfigValue, ParserError>` - On success, returns the parsed
///   `ConfigValue`. On failure, returns a `ParserError` indicating the type
///   mismatch.
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

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use yaml_rust2::YamlLoader;

    use super::*;

    #[test]
    fn test_parse_yaml_value_string() {
        let yaml_str = "test_string";
        let docs = YamlLoader::load_from_str(yaml_str).unwrap();
        let value = docs[0].clone();

        let result = parse_yaml_value(value).unwrap();
        assert_eq!(result, ConfigValue::Value("test_string".to_string()));
    }

    #[test]
    fn test_parse_yaml_value_integer() {
        let yaml_str = "123";
        let docs = YamlLoader::load_from_str(yaml_str).unwrap();
        let value = docs[0].clone();

        let result = parse_yaml_value(value).unwrap();
        assert_eq!(result, ConfigValue::Value("123".to_string()));
    }

    #[test]
    fn test_parse_yaml_value_float() {
        let yaml_str = "123.45";
        let docs = YamlLoader::load_from_str(yaml_str).unwrap();
        let value = docs[0].clone();

        let result = parse_yaml_value(value).unwrap();
        assert_eq!(result, ConfigValue::Value("123.45".to_string()));
    }

    #[test]
    fn test_parse_yaml_value_boolean() {
        let yaml_str = "true";
        let docs = YamlLoader::load_from_str(yaml_str).unwrap();
        let value = docs[0].clone();

        let result = parse_yaml_value(value).unwrap();
        assert_eq!(result, ConfigValue::Value("true".to_string()));
    }

    #[test]
    fn test_parse_yaml_value_object() {
        let yaml_str = "key: value";
        let docs = YamlLoader::load_from_str(yaml_str).unwrap();
        let value = docs[0].clone();

        let result = parse_yaml_value(value).unwrap();

        let mut expected_map = HashMap::new();
        expected_map.insert("key".to_string(), ConfigValue::Value("value".to_string()));

        assert_eq!(result, ConfigValue::Section(expected_map));
    }

    #[test]
    fn test_parse_yaml_value_array() {
        let yaml_str = "- item1\n- item2";
        let docs = YamlLoader::load_from_str(yaml_str).unwrap();
        let value = docs[0].clone();

        let result = parse_yaml_value(value).unwrap();

        assert_eq!(
            result,
            ConfigValue::Array(vec![
                ConfigValue::Value("item1".to_string()),
                ConfigValue::Value("item2".to_string())
            ])
        );
    }

    #[test]
    fn test_parse_yaml_value_null() {
        let yaml_str = "null";
        let docs = YamlLoader::load_from_str(yaml_str).unwrap();
        let value = docs[0].clone();

        let result = parse_yaml_value(value).unwrap();
        assert_eq!(result, ConfigValue::Value("null".to_string()));
    }

    #[test]
    fn test_parse_yaml_value_nested() {
        let yaml_str = "outer:\n  inner: value";
        let docs = YamlLoader::load_from_str(yaml_str).unwrap();
        let value = docs[0].clone();

        let result = parse_yaml_value(value).unwrap();

        let mut expected_map = HashMap::new();
        expected_map.insert(
            "outer".to_string(),
            ConfigValue::Section({
                let mut inner_map = HashMap::new();
                inner_map.insert("inner".to_string(), ConfigValue::Value("value".to_string()));
                inner_map
            }),
        );

        assert_eq!(result, ConfigValue::Section(expected_map));
    }

    #[test]
    fn test_parse_yaml_value_empty() {
        let yaml_str = "";
        let docs = YamlLoader::load_from_str(yaml_str).unwrap();
        assert!(docs.is_empty(), "Expected no documents for empty YAML");
    }

    #[test]
    fn test_parse_yaml_value_invalid() {
        let yaml_str = "invalid: yaml: syntax";
        let docs = YamlLoader::load_from_str(yaml_str);
        assert!(docs.is_err(), "Expected error for invalid YAML syntax");
    }
}

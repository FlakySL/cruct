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

/// Parses a JSON value into a configuration value.
///
/// * `value`: The `JsonValue` to be parsed.
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

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use jzon::object::Object;

    use super::*;

    #[test]
    fn test_parse_json_value_string() {
        let value = JsonValue::String("test_string".into());
        let result = parse_json_value(value).unwrap();

        assert_eq!(result, ConfigValue::Value("test_string".into()));
    }

    #[test]
    fn test_parse_json_value_integer() {
        let value = JsonValue::Number(123.into());
        let result = parse_json_value(value).unwrap();

        assert_eq!(result, ConfigValue::Value("123".into()));
    }

    #[test]
    fn test_parse_json_value_float() {
        let value = JsonValue::Number(123.45.into());
        let result = parse_json_value(value).unwrap();

        assert_eq!(result, ConfigValue::Value("123.45".into()));
    }

    #[test]
    fn test_parse_json_value_boolean() {
        let value = JsonValue::Boolean(true);
        let result = parse_json_value(value).unwrap();

        assert_eq!(result, ConfigValue::Value("true".into()));
    }

    #[test]
    fn test_parse_json_value_object() {
        let mut obj = Object::new();
        obj.insert("key", JsonValue::String("value".into()));

        let value = JsonValue::Object(obj);
        let result = parse_json_value(value).unwrap();

        let mut expected_map = HashMap::new();
        expected_map.insert("key".into(), ConfigValue::Value("value".into()));

        assert_eq!(result, ConfigValue::Section(expected_map));
    }

    #[test]
    fn test_parse_json_value_array() {
        let value = JsonValue::Array(vec![
            JsonValue::String("item1".into()),
            JsonValue::String("item2".into()),
        ]);

        let result = parse_json_value(value).unwrap();

        assert_eq!(
            result,
            ConfigValue::Array(vec![
                ConfigValue::Value("item1".into()),
                ConfigValue::Value("item2".into())
            ])
        );
    }
}

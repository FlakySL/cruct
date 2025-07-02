use std::collections::HashMap;
use std::fs;

use toml_edit::{DocumentMut, Item, Table, Value};

use super::{ConfigValue, Parser, ParserError};

#[derive(Clone)]
pub struct TomlParser;

impl Parser for TomlParser {
    fn extensions(&self) -> &'static [&'static str] {
        &["toml"]
    }

    fn load(&self, path: &str) -> Result<ConfigValue, ParserError> {
        let content = fs::read_to_string(path)?;
        let value = content.parse::<DocumentMut>()?;
        parse_toml(value.as_item())
    }
}

fn parse_toml(item: &Item) -> Result<ConfigValue, ParserError> {
    Ok(match item {
        Item::None => ConfigValue::Array(Vec::new()),
        Item::Value(value) => parse_toml_value(value)?,
        Item::Table(table) => parse_table(table)?,
        Item::ArrayOfTables(array_of_tables) => {
            let mut array = Vec::new();
            for table in array_of_tables {
                array.push(parse_table(table)?);
            }
            ConfigValue::Array(array)
        },
    })
}

fn parse_table(table: &Table) -> Result<ConfigValue, ParserError> {
    let mut map = HashMap::new();
    for (k, v) in table {
        map.insert(k.to_string(), parse_toml(v)?);
    }
    Ok(ConfigValue::Section(map))
}

fn parse_toml_value(value: &Value) -> Result<ConfigValue, ParserError> {
    Ok(match value {
        Value::InlineTable(table) => {
            let mut map = HashMap::new();
            for (k, v) in table {
                map.insert(k.to_string(), parse_toml_value(v)?);
            }
            ConfigValue::Section(map)
        },
        Value::Array(arr) => {
            let items = arr
                .into_iter()
                .map(parse_toml_value)
                .collect::<Result<Vec<_>, _>>()?;
            ConfigValue::Array(items)
        },
        Value::String(s) => ConfigValue::Value(
            s.clone()
                .into_value(),
        ),
        Value::Integer(i) => ConfigValue::Value(
            i.value()
                .to_string(),
        ),
        Value::Float(f) => ConfigValue::Value(
            f.value()
                .to_string(),
        ),
        Value::Boolean(b) => ConfigValue::Value(
            b.value()
                .to_string(),
        ),
        Value::Datetime(dt) => ConfigValue::Value(
            dt.value()
                .to_string(),
        ),
    })
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use toml_edit::{Array, ArrayOfTables, Date, Formatted, Item, Table, Value};

    use super::*;

    #[test]
    fn test_parse_toml_value_string() {
        let value = Value::String(Formatted::new("test_string".to_string()));
        let result = parse_toml_value(&value).unwrap();

        assert_eq!(result, ConfigValue::Value("test_string".to_string()));
    }

    #[test]
    fn test_parse_toml_value_integer() {
        let value = Value::Integer(Formatted::new(123));
        let result = parse_toml_value(&value).unwrap();

        assert_eq!(result, ConfigValue::Value("123".to_string()));
    }

    #[test]
    fn test_parse_toml_value_float() {
        let value = Value::Float(Formatted::new(123.45));
        let result = parse_toml_value(&value).unwrap();

        assert_eq!(result, ConfigValue::Value("123.45".to_string()));
    }

    #[test]
    fn test_parse_toml_value_boolean() {
        let value = Value::Boolean(Formatted::new(true));
        let result = parse_toml_value(&value).unwrap();

        assert_eq!(result, ConfigValue::Value("true".to_string()));
    }

    #[test]
    fn test_parse_toml_value_inline_table() {
        let mut table = toml_edit::InlineTable::default();
        table.insert("key", Value::String(Formatted::new("value".to_string())));

        let value = Value::InlineTable(table);
        let result = parse_toml_value(&value).unwrap();

        let mut expected_map = HashMap::new();
        expected_map.insert("key".to_string(), ConfigValue::Value("value".to_string()));

        assert_eq!(result, ConfigValue::Section(expected_map));
    }

    #[test]
    fn test_parse_toml_value_datetime() {
        let value = Value::Datetime(Formatted::new(Date { year: 2023, month: 10, day: 1 }.into()));
        let result = parse_toml_value(&value).unwrap();

        assert_eq!(result, ConfigValue::Value("2023-10-01".to_string()));
    }

    #[test]
    fn test_parse_toml_array() {
        let value = Value::Array(Array::from_iter(vec![
            Value::String(Formatted::new("item1".into())),
            Value::String(Formatted::new("item2".into())),
        ]));

        let result = parse_toml_value(&value).unwrap();

        assert_eq!(
            result,
            ConfigValue::Array(vec![
                ConfigValue::Value("item1".to_string()),
                ConfigValue::Value("item2".to_string())
            ])
        );
    }

    #[test]
    fn test_parse_toml_table() {
        let mut table = Table::new();

        table.insert("key1", Item::Value(Value::String(Formatted::new("value1".to_string()))));
        table.insert("key2", Item::Value(Value::Integer(Formatted::new(42.into()))));

        let result = parse_table(&table).unwrap();
        let mut expected_map = HashMap::new();

        expected_map.insert("key1".to_string(), ConfigValue::Value("value1".to_string()));
        expected_map.insert("key2".to_string(), ConfigValue::Value("42".to_string()));

        assert_eq!(result, ConfigValue::Section(expected_map));
    }

    #[test]
    fn test_parse_toml_item_table() {
        let mut table = Table::new();
        table.insert("key", Item::Value(Value::String(Formatted::new("value".to_string()))));

        let item = Item::Table(table);
        let result = parse_toml(&item).unwrap();

        let mut expected_map = HashMap::new();
        expected_map.insert("key".to_string(), ConfigValue::Value("value".to_string()));

        assert_eq!(result, ConfigValue::Section(expected_map));
    }

    #[test]
    fn test_parse_toml_item_array_of_tables() {
        let mut table1 = Table::new();
        let mut table2 = Table::new();

        table1.insert("key1", Item::Value(Value::String(Formatted::new("value1".to_string()))));
        table2.insert("key2", Item::Value(Value::String(Formatted::new("value2".to_string()))));

        let item = Item::ArrayOfTables(ArrayOfTables::from_iter(vec![table1, table2]));
        let result = parse_toml(&item).unwrap();

        let mut expected_map1 = HashMap::new();
        let mut expected_map2 = HashMap::new();

        expected_map1.insert("key1".to_string(), ConfigValue::Value("value1".to_string()));
        expected_map2.insert("key2".to_string(), ConfigValue::Value("value2".to_string()));

        assert_eq!(
            result,
            ConfigValue::Array(vec![
                ConfigValue::Section(expected_map1),
                ConfigValue::Section(expected_map2)
            ])
        );
    }

    #[test]
    fn test_parse_toml_item_none() {
        let item = Item::None;
        let result = parse_toml(&item).unwrap();

        assert_eq!(result, ConfigValue::Array(Vec::new()));
    }
}

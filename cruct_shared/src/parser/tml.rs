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
        let value = content.parse::<Value>()?;
        parse_toml_value(&value)
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

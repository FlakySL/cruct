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
    match item {
        Item::None => Ok(ConfigValue::Array(Vec::new())),
        Item::Value(value) => parse_toml_value(value),
        Item::Table(table) => parse_table(table),
        Item::ArrayOfTables(array_of_tables) => {
            let mut array = Vec::new();
            for table in array_of_tables {
                array.push(parse_table(table)?);
            }
            Ok(ConfigValue::Array(array))
        },
    }
}

fn parse_table(table: &Table) -> Result<ConfigValue, ParserError> {
    let mut map = HashMap::new();
    for (k, v) in table {
        map.insert(k.to_string(), parse_toml(v)?);
    }
    Ok(ConfigValue::Section(map))
}

fn parse_toml_value(value: &Value) -> Result<ConfigValue, ParserError> {
    match value {
        Value::InlineTable(table) => {
            let mut map = HashMap::new();
            for (k, v) in table {
                map.insert(k.to_string(), parse_toml_value(v)?);
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
        Value::String(s) => Ok(ConfigValue::Value(
            s.clone()
                .into_value(),
        )),
        Value::Integer(i) => Ok(ConfigValue::Value(
            i.value()
                .to_string(),
        )),
        Value::Float(f) => Ok(ConfigValue::Value(
            f.value()
                .to_string(),
        )),
        Value::Boolean(b) => Ok(ConfigValue::Value(
            b.value()
                .to_string(),
        )),
        Value::Datetime(dt) => Ok(ConfigValue::Value(
            dt.value()
                .to_string(),
        )),
    }
}

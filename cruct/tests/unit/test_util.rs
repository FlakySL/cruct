use std::collections::HashMap;

use cruct::ConfigValue;
use cruct::utils::{merge_maps, parse_numeric, validate_field_name};

#[test]
fn merge_overwrites_primitives_but_merges_sections() {
    let mut a = HashMap::new();
    a.insert("key".into(), ConfigValue::Value("one".into()));
    let mut b = HashMap::new();
    b.insert("key".into(), ConfigValue::Value("two".into()));
    let merged = merge_maps(a, b);
    assert_eq!(merged["key"], ConfigValue::Value("two".into()));
}

#[test]
fn merge_nested_sections_combines() {
    let mut a = HashMap::new();
    a.insert(
        "sec".into(),
        ConfigValue::Section({
            let mut m = HashMap::new();
            m.insert("x".into(), ConfigValue::Value("1".into()));
            m
        }),
    );
    let mut b = HashMap::new();
    b.insert(
        "sec".into(),
        ConfigValue::Section({
            let mut m = HashMap::new();
            m.insert("y".into(), ConfigValue::Value("2".into()));
            m
        }),
    );
    let merged = merge_maps(a, b);
    if let ConfigValue::Section(map) = &merged["sec"] {
        assert!(map.contains_key("x"));
        assert!(map.contains_key("y"));
    } else {
        panic!("Expected section");
    }
}

#[test]
fn parse_numeric_handles_valid_and_invalid() {
    assert_eq!(parse_numeric("42"), Ok(42));
    assert!(parse_numeric("foo").is_err());
}

#[test]
fn validate_field_name_rejects_invalid_chars() {
    assert!(validate_field_name("valid_name").is_ok());
    assert!(validate_field_name("invalid-name").is_err());
}

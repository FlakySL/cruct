use std::collections::HashMap;

use crate::ConfigValue;
use crate::source::{merge_configs, merge_sections};

#[test]
fn overrides_and_merges_nested_sections() {
    let mut base = HashMap::new();
    base.insert("a".into(), ConfigValue::Value("one".into()));
    base.insert(
        "nested".into(),
        ConfigValue::Section({
            let mut m = HashMap::new();
            m.insert("x".into(), ConfigValue::Value("10".into()));
            m
        }),
    );

    let mut high = HashMap::new();
    high.insert("a".into(), ConfigValue::Value("two".into()));
    high.insert(
        "nested".into(),
        ConfigValue::Section({
            let mut m = HashMap::new();
            m.insert("y".into(), ConfigValue::Value("20".into()));
            m
        }),
    );

    let merged = merge_sections(base, high);
    assert_eq!(merged["a"], ConfigValue::Value("two".into()));
    let nested = match &merged["nested"] {
        ConfigValue::Section(s) => s,
        _ => panic!("expected section"),
    };
    assert_eq!(nested["x"], ConfigValue::Value("10".into()));
    assert_eq!(nested["y"], ConfigValue::Value("20".into()));
}

#[test]
fn merge_configs_sections() {
    let mut base_section = HashMap::new();
    base_section.insert(
        "key1".to_string(),
        ConfigValue::Section(HashMap::from([(
            "subkey1".to_string(),
            ConfigValue::Value("high1".to_string()),
        )])),
    );
    base_section.insert("key2".to_string(), ConfigValue::Value("base2".to_string()));

    let mut high_section = HashMap::new();
    high_section.insert(
        "key1".to_string(),
        ConfigValue::Section(HashMap::from([
            ("subkey1".to_string(), ConfigValue::Value("high1".to_string())),
            ("subkey2".to_string(), ConfigValue::Value("high2".to_string())),
        ])),
    );
    high_section.insert("key3".to_string(), ConfigValue::Value("high3".to_string()));

    let base = ConfigValue::Section(base_section);
    let high = ConfigValue::Section(high_section);

    let result = merge_configs(base, high).unwrap();

    if let ConfigValue::Section(merged_section) = result {
        assert_eq!(merged_section.len(), 3);

        if let ConfigValue::Section(sub_section) = merged_section
            .get("key1")
            .unwrap()
        {
            assert_eq!(sub_section.len(), 2);
            assert_eq!(
                sub_section
                    .get("subkey1")
                    .unwrap(),
                &ConfigValue::Value("high1".to_string())
            );
            assert_eq!(
                sub_section
                    .get("subkey2")
                    .unwrap(),
                &ConfigValue::Value("high2".to_string())
            );
        } else {
            panic!("Expected key1 to be a section");
        }

        assert_eq!(
            merged_section
                .get("key2")
                .unwrap(),
            &ConfigValue::Value("base2".to_string())
        );
        assert_eq!(
            merged_section
                .get("key3")
                .unwrap(),
            &ConfigValue::Value("high3".to_string())
        );
    } else {
        panic!("Expected result to be a section");
    }
}

#[test]
fn merge_configs_non_sections() {
    let base = ConfigValue::Value("base_value".to_string());
    let high = ConfigValue::Value("high_value".to_string());

    let result = merge_configs(base, high).unwrap();

    assert_eq!(result, ConfigValue::Value("high_value".to_string()));
}

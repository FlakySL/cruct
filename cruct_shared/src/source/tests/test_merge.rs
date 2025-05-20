use std::collections::HashMap;

use crate::ConfigValue;
use crate::source::merge_sections;

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

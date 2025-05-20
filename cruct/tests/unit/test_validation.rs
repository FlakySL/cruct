use std::collections::HashMap;

use cruct::ConfigValue;
use cruct::validation::check_required_fields;

#[test]
fn detects_missing_required_field() {
    let mut map = HashMap::new();
    map.insert("present".into(), ConfigValue::Value("ok".into()));
    let missing = check_required_fields(&map, &vec!["present".into(), "absent".into()]);
    assert_eq!(missing, vec!["absent".to_string()]);
}

#[test]
fn all_required_fields_present_returns_empty() {
    let mut map = HashMap::new();
    map.insert("a".into(), ConfigValue::Value("1".into()));
    let missing = check_required_fields(&map, &vec!["a".into()]);
    assert!(missing.is_empty());
}

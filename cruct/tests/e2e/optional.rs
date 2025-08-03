use std::collections::HashMap;

use cruct::{ConfigValue, FromConfigValue, cruct};

#[cruct]
#[derive(Debug, PartialEq)]
struct OptionalConfig {
    #[field(optional = true)]
    optional_value: Option<String>,

    #[field(default = 123)]
    required_number: i32,
}

#[test]
fn optional_field_missing_is_none() {
    let config = ConfigValue::Section(Default::default());

    let parsed = OptionalConfig::from_config_value(&config).unwrap();

    assert_eq!(
        parsed,
        OptionalConfig {
            optional_value: None,
            required_number: 123,
        }
    );
}

#[test]
fn optional_field_present_is_some() {
    let mut section = HashMap::new();
    section.insert("optional_value".into(), ConfigValue::Value("hello".into()));
    let config = ConfigValue::Section(section);

    let parsed = OptionalConfig::from_config_value(&config).unwrap();

    assert_eq!(
        parsed,
        OptionalConfig {
            optional_value: Some("hello".into()),
            required_number: 123,
        }
    );
}

#[test]
fn optional_field_type_mismatch_is_error() {
    let mut section = HashMap::new();
    section.insert("optional_value".into(), ConfigValue::Array(vec![])); // not a string!
    let config = ConfigValue::Section(section);

    let err = OptionalConfig::from_config_value(&config).unwrap_err();

    assert_eq!(
        err.to_string(),
        "Type mismatch in field 'optional_value': expected \
         core::option::Option<alloc::string::String>, found '[]'"
    )
}

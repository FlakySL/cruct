use crate::{ConfigValue, FromConfigValue, ParserError};

#[test]
fn test_some_string() {
    let val = ConfigValue::Value("hello".to_string());
    let parsed: Option<String> = Option::from_config_value(&val).unwrap();
    assert_eq!(parsed, Some("hello".to_string()));
}

#[test]
fn test_none_from_missing_field_error() {
    #[derive(Debug, PartialEq)]
    struct FakeType;

    impl FromConfigValue for FakeType {
        fn from_config_value(_: &ConfigValue) -> Result<Self, ParserError> {
            Err(ParserError::MissingField("test".into()))
        }
    }

    let val = ConfigValue::Value("ignored".into());
    let result: Option<FakeType> = Option::from_config_value(&val).unwrap();
    assert_eq!(result, None);
}

#[test]
fn test_error_propagation_for_non_missing_error() {
    struct FailsWithTypeMismatch;
    impl FromConfigValue for FailsWithTypeMismatch {
        fn from_config_value(_: &ConfigValue) -> Result<Self, ParserError> {
            Err(ParserError::TypeMismatch {
                field: "x".into(),
                expected: "u32".into(),
                found: "wrong".into(),
            })
        }
    }

    let val = ConfigValue::Value("wrong".into());
    let result: Result<Option<FailsWithTypeMismatch>, _> = Option::from_config_value(&val);
    assert!(matches!(result, Err(ParserError::TypeMismatch { .. })));
}

#[test]
fn test_none_from_null_value() {
    let val = ConfigValue::Null;
    let parsed: Option<String> = Option::from_config_value(&val).unwrap();
    assert_eq!(parsed, None);
}

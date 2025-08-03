use syn::{Result, parse_str};

use crate::parse::FieldParams;

#[test]
fn name_invalid_value() {
    let src = r#"name = 1"#;
    let params: Result<FieldParams> = parse_str(src);

    if let Err(e) = params {
        assert_eq!(e.to_string(), "Invalid value type for 'name' expected '&str'".to_string());
    }
}

#[test]
fn insensitive_invalid_value() {
    let src = r#"insensitive = "true""#;
    let params: Result<FieldParams> = parse_str(src);

    if let Err(e) = params {
        assert_eq!(
            e.to_string(),
            "Invalid value type for 'insensitive' expected 'bool'".to_string()
        );
    }
}

#[test]
fn env_override_invalid_value() {
    let src = r#"env_override = 1"#;
    let params: Result<FieldParams> = parse_str(src);

    if let Err(e) = params {
        assert_eq!(
            e.to_string(),
            "Invalid value type for 'env_override' expected '&str'".to_string()
        );
    }
}

#[test]
fn arg_override_invalid_value() {
    let src = r#"arg_override = 1"#;
    let params: Result<FieldParams> = parse_str(src);

    if let Err(e) = params {
        assert_eq!(
            e.to_string(),
            "Invalid value type for 'arg_override' expected '&str'".to_string()
        );
    }
}

#[test]
fn unknown_key() {
    let src = r#"unknown = 1"#;
    let params: Result<FieldParams> = parse_str(src);

    if let Err(e) = params {
        assert_eq!(
            e.to_string(),
            "Unknown parameter 'unknown'. Known parameters include:\n- name: &str\n- insensitive: \
             bool\n- env_override: &str\n- arg_override: &str\n- optional: bool"
                .to_string()
        );
    }
}

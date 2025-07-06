use syn::{Result, parse_str};

use crate::parse::MacroParams;

#[test]
fn path_value_mismatch() {
    let src = r#"load_config(path = 123)"#;
    let params: Result<MacroParams> = parse_str(src);

    if let Err(e) = params {
        assert_eq!(
            e.to_string(),
            "Invalid parameter type for 'path', expected 'String', found '123'".to_string()
        );
    }
}

#[test]
fn format_value_mismatch() {
    let src = r#"load_config(format = 123)"#;
    let params: Result<MacroParams> = parse_str(src);

    if let Err(e) = params {
        assert_eq!(
            e.to_string(),
            "Invalid parameter type for 'format', expected 'String', found '123'".to_string()
        );
    }
}

#[test]
fn priority_value_mismatch() {
    let src = r#"load_config(priority = "high")"#;
    let params: Result<MacroParams> = parse_str(src);

    if let Err(e) = params {
        assert_eq!(
            e.to_string(),
            "Invalid parameter type for 'priority', expected 'Integer', found '\"high\"'"
                .to_string()
        );
    }
}

use syn::{Result, parse_str};

use crate::parse::MacroParams;

#[test]
fn parse_single_load_config() {
    let src = r#"load_config(path = "a.toml", format = "Toml", priority = 5)"#;
    let params: MacroParams = parse_str(src).unwrap();
    assert_eq!(
        params
            .configs
            .len(),
        1
    );

    let cfg = &params.configs[0];
    assert_eq!(cfg.path, "a.toml");
    assert_eq!(
        cfg.format
            .unwrap()
            .to_string(),
        "toml"
    );
    assert_eq!(
        cfg.priority
            .unwrap(),
        5
    );
}

#[test]
fn load_config_without_required_parameter() {
    let src = r#"load_config()"#;
    let params: Result<MacroParams> = parse_str(src);

    if let Err(e) = params {
        assert_eq!(e.to_string(), "Missing required parameter 'path'".to_string());
    }
}

#[test]
fn load_config_with_unknown_key() {
    let src = r#"load_config(unknown_key = "value")"#;
    let params: Result<MacroParams> = parse_str(src);

    if let Err(e) = params {
        assert_eq!(e.to_string(), "unknown key 'unknown_key' in load_config".to_string());
    }
}

#[test]
fn multiple_load_configs() {
    let src = r#"
        load_config(path = "a.toml", format = "Toml", priority = 5),
        load_config(path = "b.json", format = "Json")
    "#;
    let params: MacroParams = parse_str(src).unwrap();

    assert_eq!(
        params
            .configs
            .len(),
        2
    );

    let cfg1 = &params.configs[0];
    assert_eq!(cfg1.path, "a.toml");
    assert_eq!(
        cfg1.format
            .unwrap()
            .to_string(),
        "toml"
    );
    assert_eq!(
        cfg1.priority
            .unwrap(),
        5
    );

    let cfg2 = &params.configs[1];
    assert_eq!(cfg2.path, "b.json");
    assert_eq!(
        cfg2.format
            .unwrap()
            .to_string(),
        "json"
    );
}

#[test]
fn parse_invalid_macro_params() {
    let src = r#"asd"#;
    let params: Result<MacroParams> = parse_str(src);

    if let Err(e) = params {
        assert_eq!(
            e.to_string(),
            "expected `load_config(path = ..., format = ..., priority = ...)`".to_string()
        );
    }
}

#[test]
fn parse_invalid_file_format() {
    let src = r#"load_config(format = "xml")"#;
    let params: Result<MacroParams> = parse_str(src);

    if let Err(e) = params {
        assert_eq!(
            e.to_string(),
            "invalid file format: 'xml' is not a valid file format".to_string()
        );
    }
}

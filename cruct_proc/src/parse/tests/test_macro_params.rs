use syn::parse_str;

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

use cruct::cruct;

#[test]
fn missing_file_returns_error() {
    #[cruct(load_config(path = "tests/fixtures/does_not_exist.toml"))]
    #[derive(Debug)]
    struct E {
        a: String,
    }

    let err = E::loader()
        .with_config()
        .load()
        .unwrap_err();

    assert!(
        err.to_string()
            .contains("No such file")
    );
}

#[test]
fn invalid_toml_syntax_error() {
    #[cruct(load_config(path = "tests/fixtures/integration/invalid.toml"))]
    #[derive(Debug)]
    struct F {
        a: String,
    }

    let err = F::loader()
        .with_config()
        .load()
        .unwrap_err();

    assert!(
        err.to_string()
            .contains("TOML parse error")
    );
}

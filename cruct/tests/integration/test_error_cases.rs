use assay::assay;
use cruct::cruct;

#[cfg(unix)]
#[test]
fn missing_file_returns_error_unix() {
    #[cruct(load_config(path = "tests/fixtures/does_not_exist.toml"))]
    #[derive(Debug)]
    #[allow(dead_code)]
    struct E {
        a: String,
    }

    let err = E::loader()
        .with_config()
        .load()
        .unwrap_err();

    assert_eq!(err.to_string(), "No such file or directory (os error 2)");
}

#[cfg(windows)]
#[test]
fn missing_file_returns_error_windows() {
    #[cruct(load_config(path = "tests/fixtures/does_not_exist.toml"))]
    #[derive(Debug)]
    #[allow(dead_code)]
    struct E {
        a: String,
    }

    let err = E::loader()
        .with_config()
        .load()
        .unwrap_err();

    assert_eq!(err.to_string(), "The system cannot find the file specified. (os error 2)");
}

#[assay(
    include = ["tests/fixtures/integration/invalid.toml"],
)]
fn invalid_toml_syntax_error() {
    #[cruct(load_config(path = "tests/fixtures/integration/invalid.toml"))]
    #[derive(Debug)]
    #[allow(dead_code)]
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

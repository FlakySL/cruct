use assay::assay;
use cruct::cruct;

#[assay(
    include = ["tests/fixtures/integration/missing_flat_nested.toml"],
)]
fn missing_flat_nested_field() {
    #[cruct(load_config(path = "tests/fixtures/integration/missing_flat_nested.toml"))]
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Config {
        exists: String,
        nested: Nested,
    }

    #[cruct]
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Nested {
        missing: u32,
    }

    let result = Config::loader()
        .with_config()
        .load();

    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(
            e.to_string(),
            "Nested configuration error in nested: Missing required field: missing"
        );
    }
}

#[assay(
    include = ["tests/fixtures/integration/type_mismatch_flat.toml"],
)]
fn flat_nested_type_mismatch() {
    #[cruct(load_config(path = "tests/fixtures/integration/type_mismatch_flat.toml"))]
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Config {
        nested: Nested,
    }

    #[cruct]
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Nested {
        value: u32,
    }

    let result = Config::loader()
        .with_config()
        .load();

    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(
            e.to_string(),
            "Nested configuration error in nested: Type mismatch in field 'value': expected u32, \
             found 'not_a_number'"
        );
    }
}

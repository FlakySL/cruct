use assay::assay;
use cruct::cruct;

#[assay(
    include = ["tests/fixtures/e2e/flat_nested.toml"],
)]
fn flat_nested_structs_load_correctly() {
    #[cruct(load_config(path = "tests/fixtures/e2e/flat_nested.toml"))]
    #[derive(Debug, PartialEq)]
    struct Outer {
        top_level: String,
        nested: Nested,
    }

    #[cruct]
    #[derive(Debug, PartialEq)]
    struct Nested {
        value: u32,
        flag: bool,
    }

    let config = Outer::loader()
        .with_config()
        .load()
        .unwrap();

    assert_eq!(config.top_level, "root_value");
    assert_eq!(
        config
            .nested
            .value,
        42
    );
    assert_eq!(
        config
            .nested
            .flag,
        true
    );
}

#[assay(
    include = ["tests/fixtures/e2e/flat_nested_defaults.toml"],
)]
fn flat_nested_with_defaults() {
    #[cruct(load_config(path = "tests/fixtures/e2e/flat_nested_defaults.toml"))]
    #[derive(Debug, PartialEq)]
    struct Config {
        #[field(default = "default".into())]
        name: String,
        nested: NestedWithDefaults,
    }

    #[cruct]
    #[derive(Debug, PartialEq)]
    struct NestedWithDefaults {
        #[field(default = 100)]
        value: u32,
        #[field(default = false)]
        flag: bool,
    }

    let config = Config::loader()
        .with_config()
        .load()
        .unwrap();

    assert_eq!(config.name, "default");
    assert_eq!(
        config
            .nested
            .value,
        42
    ); // From config file
    assert_eq!(
        config
            .nested
            .flag,
        false
    ); // Default value
}

#[assay(
    include = ["tests/fixtures/e2e/mixed_nested.toml"],
)]
fn mixed_nested_and_flat() {
    #[cruct(load_config(path = "tests/fixtures/e2e/mixed_nested.toml"))]
    #[derive(Debug, PartialEq)]
    struct Config {
        top_value: u32,

        #[field(name = "nested_section")]
        nested: NestedSection,

        flat: FlatNested,
    }

    #[cruct]
    #[derive(Debug, PartialEq)]
    struct NestedSection {
        inner_value: String,
    }

    #[cruct]
    #[derive(Debug, PartialEq)]
    struct FlatNested {
        flat_value: bool,
    }

    let config = Config::loader()
        .with_config()
        .load()
        .unwrap();

    assert_eq!(config.top_value, 100);
    assert_eq!(
        config
            .nested
            .inner_value,
        "section_value"
    );
    assert_eq!(
        config
            .flat
            .flat_value,
        true
    );
}

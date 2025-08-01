use assay::assay;
use cruct::cruct;

#[assay(
    include = ["tests/fixtures/test_config.toml"],
)]
fn test_nested_struct_toml() {
    #[cruct(load_config(path = "tests/fixtures/test_config.toml"))]
    #[derive(Debug, PartialEq)]
    struct SomeStruct {
        items: Vec<String>,
        numbers: Vec<u16>,
    }

    #[cruct(load_config(path = "tests/fixtures/test_config.toml"))]
    #[derive(Debug, PartialEq)]
    struct NestedConfig {
        http_port: u16,
        nested: SomeStruct,
    }

    let cfg = NestedConfig::loader()
        .with_config()
        .load()
        .unwrap();

    assert_eq!(cfg.http_port, 8080);

    assert_eq!(
        cfg.nested,
        SomeStruct {
            items: vec!["foo".into(), "bar".into()],
            numbers: vec![42, 99],
        }
    );
}

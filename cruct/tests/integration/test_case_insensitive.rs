use assay::assay;
use cruct::cruct;

#[assay(
    include = ["tests/fixtures/test_config.toml"],
)]
fn test_case_insensitive() {
    #[cruct(load_config(path = "tests/fixtures/test_config.toml"))]
    #[derive(Debug, PartialEq)]
    struct TestInsensitive {
        #[field(name = "HTTP_PORT", insensitive = true)]
        http_port: u16,
    }

    let config = TestInsensitive::loader()
        .with_config()
        .load()
        .unwrap();

    assert_eq!(config.http_port, 8080);
}

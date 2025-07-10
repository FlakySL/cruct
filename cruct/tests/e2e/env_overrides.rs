use assay::assay;
use cruct::cruct;

#[assay(
    include = ["tests/fixtures/test_config.toml"],
    env = [
        ("TEST_HTTP_PORT", "9999")
    ],
)]
fn test_env_override() {
    #[cruct(load_config(path = "tests/fixtures/test_config.toml"))]
    #[derive(Debug, PartialEq)]
    struct TestEnv {
        #[field(env_override = "TEST_HTTP_PORT")]
        http_port: u16,
    }

    let config = TestEnv::loader()
        .with_config()
        .load()
        .unwrap();

    assert_eq!(config.http_port, 9999);
}

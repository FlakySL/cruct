use assay::assay;
use cruct::cruct;

#[assay(
    include = [
        "tests/fixtures/test_config.toml",
        "tests/fixtures/test_config.json",
    ]
)]
fn test_toml_loading() {
    #[cruct(
        load_config(path = "tests/fixtures/test_config.toml", format = "Toml", priority = 0),
        load_config(path = "tests/fixtures/test_config.json", format = "Json", priority = 1)
    )]
    #[derive(Debug, PartialEq)]
    struct TestToml {
        #[field(name = "else")]
        something: String,
        http_port: u16,
    }
    let config = TestToml::loader()
        .with_config()
        .load()
        .unwrap();

    assert_eq!(config.something, "toml value");
    assert_eq!(config.http_port, 8080);
}

#[assay(
    include = [
        "tests/fixtures/test_config.json",
    ]
)]
fn test_json_loading() {
    #[cruct(load_config(path = "./tests/fixtures/test_config.json", format = "Json"))]
    #[derive(Debug, PartialEq)]
    struct TestJson {
        #[field(name = "else")]
        something: String,
        http_port: u16,
    }

    let config = TestJson::loader()
        .with_config()
        .load()
        .unwrap();

    assert_eq!(config.something, "json value");
    assert_eq!(config.http_port, 3000);
}

#[assay(
    include = [
        "tests/fixtures/test_config.yml"
    ]
)]
fn test_yaml_loading() {
    #[cruct(load_config(path = "./tests/fixtures/test_config.yml", format = "Yml"))]
    #[derive(Debug, PartialEq)]
    struct TestYaml {
        #[field(name = "else")]
        something: String,
        http_port: u16,
    }

    let config = TestYaml::loader()
        .with_config()
        .load()
        .unwrap();

    assert_eq!(config.something, "yaml value");
    assert_eq!(config.http_port, 4000);
}

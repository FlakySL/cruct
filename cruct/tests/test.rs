use cruct::cruct;

#[cruct(path = "./tests/fixtures/test_config.toml", format = "Toml")]
#[derive(Debug, PartialEq)]
struct TestToml {
    #[field(name = "else")]
    something: String,
    http_port: u16,
}

#[cruct(path = "./tests/fixtures/test_config.json", format = "Json")]
#[derive(Debug, PartialEq)]
struct TestJson {
    #[field(name = "else")]
    something: String,
    http_port: u16,
}

#[cruct(path = "./tests/fixtures/test_config.yml", format = "Yml")]
#[derive(Debug, PartialEq)]
struct TestYaml {
    #[field(name = "else")]
    something: String,
    http_port: u16,
}

#[test]
fn test_toml_loading() {
    let config = TestToml::load().unwrap();

    assert_eq!(config.something, "toml value");
    assert_eq!(config.http_port, 8080);
}

#[test]
fn test_json_loading() {
    let config = TestJson::load().unwrap();
    assert_eq!(config.something, "json value");
    assert_eq!(config.http_port, 3000);
}

#[test]
fn test_yaml_loading() {
    let config = TestYaml::load().unwrap();
    assert_eq!(config.something, "yaml value");
    assert_eq!(config.http_port, 4000);
}

#[test]
fn test_default_values() {
    #[cruct(path = "./tests/fixtures/test_config.toml")]
    #[derive(Debug)]
    struct TestDefault {
        #[field(name = "missing_field", default = "default value")]
        field: String,
    }

    let config = TestDefault::load().unwrap();
    assert_eq!(config.field, "default value");
}

#[test]
fn test_missing_field() {
    #[cruct(path = "./tests/fixtures/test_config.toml")]
    #[derive(Debug)]
    #[allow(dead_code)]
    pub struct TestMissing {
        missing_field: String,
    }

    let result = TestMissing::load();
    match result {
        Err(cruct_shared::parser::ParserError::MissingField(field)) => {
            assert_eq!(field, "missing_field");
        },
        Ok(_) => panic!("Expected MissingField error"),
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}

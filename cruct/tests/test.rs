use cruct::{ConfigValue, FromConfigValue, ParserError, cruct};

#[test]
fn test_toml_loading() {
    #[cruct(path = "./tests/fixtures/test_config.toml", format = "Toml")]
    #[derive(Debug, PartialEq)]
    struct TestToml {
        #[field(name = "else")]
        something: String,
        http_port: u16,
    }
    let config = TestToml::load().unwrap();

    assert_eq!(config.something, "toml value");
    assert_eq!(config.http_port, 8080);
}

#[test]
fn test_json_loading() {
    #[cruct(path = "./tests/fixtures/test_config.json", format = "Json")]
    #[derive(Debug, PartialEq)]
    struct TestJson {
        #[field(name = "else")]
        something: String,
        http_port: u16,
    }

    let config = TestJson::load().unwrap();
    assert_eq!(config.something, "json value");
    assert_eq!(config.http_port, 3000);
}

#[test]
fn test_yaml_loading() {
    #[cruct(path = "./tests/fixtures/test_config.yml", format = "Yml")]
    #[derive(Debug, PartialEq)]
    struct TestYaml {
        #[field(name = "else")]
        something: String,
        http_port: u16,
    }

    let config = TestYaml::load().unwrap();
    assert_eq!(config.something, "yaml value");
    assert_eq!(config.http_port, 4000);
}

#[test]
fn test_default_values() {
    #[cruct(path = "./tests/fixtures/test_config.toml")]
    #[derive(Debug)]
    struct TestDefault {
        #[field(name = "missing_field", default = "default value".to_string())]
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

#[test]
fn test_env_override() {
    #[cruct(path = "./tests/fixtures/test_config.toml")]
    #[derive(Debug, PartialEq)]
    struct TestEnv {
        #[field(env_override = "TEST_HTTP_PORT")]
        http_port: u16,
    }

    unsafe {
        std::env::set_var("TEST_HTTP_PORT", "9999");
    }

    let config = TestEnv::load().unwrap();
    assert_eq!(config.http_port, 9999);

    unsafe {
        std::env::remove_var("TEST_HTTP_PORT");
    }
}

#[test]
fn test_case_insensitive() {
    #[cruct(path = "./tests/fixtures/test_config.toml")]
    #[derive(Debug, PartialEq)]
    struct TestInsensitive {
        #[field(name = "HTTP_PORT", insensitive = true)]
        http_port: u16,
    }

    let config = TestInsensitive::load().unwrap();
    assert_eq!(config.http_port, 8080);
}

fn get_default_from_fn() -> String {
    "function_default".to_string()
}

#[test]
fn test_enhanced_defaults() {
    #[cruct(path = "./tests/fixtures/test_config.toml")]
    #[derive(Debug, PartialEq)]
    struct TestStringDefault {
        #[field(default = "literal_default".to_string())]
        field: String,
    }

    #[cruct(path = "./tests/fixtures/test_config.toml")]
    #[derive(Debug, PartialEq)]
    struct TestFnDefault {
        #[field(default = get_default_from_fn())]
        field: String,
    }

    #[cruct(path = "./tests/fixtures/test_config.toml")]
    #[derive(Debug, PartialEq)]
    struct TestExprDefault {
        #[field(default = format!("{}-{}", "expr", 42))]
        field: String,
    }

    #[cruct(path = "./tests/fixtures/test_config.toml")]
    #[derive(Debug, PartialEq)]
    struct TestNumericDefault {
        #[field(default = 9000)]
        port: u16,
    }

    let string_config = TestStringDefault::load().unwrap();
    assert_eq!(string_config.field, "literal_default");

    let fn_config = TestFnDefault::load().unwrap();
    assert_eq!(fn_config.field, "function_default");

    let expr_config = TestExprDefault::load().unwrap();
    assert_eq!(expr_config.field, "expr-42");

    let numeric_config = TestNumericDefault::load().unwrap();
    assert_eq!(numeric_config.port, 9000);
}

#[test]
fn test_default_with_environment() {
    #[cruct(path = "./tests/fixtures/test_config.toml")]
    #[derive(Debug, PartialEq)]
    struct TestCombined {
        #[field(
            name = "missing_field",
            env_override = "TEST_DEFAULT_ENV",
            default = "base_default".to_string()
        )]
        field: String,
    }

    unsafe { std::env::set_var("TEST_DEFAULT_ENV", "env_value") };
    let config = TestCombined::load().unwrap();
    assert_eq!(config.field, "env_value");

    unsafe { std::env::remove_var("TEST_DEFAULT_ENV") };
    let config = TestCombined::load().unwrap();
    assert_eq!(config.field, "base_default");
}

#[test]
fn test_default_with_type_conversion() {
    #[cruct(path = "./tests/fixtures/test_config.toml")]
    #[derive(Debug, PartialEq)]
    struct TestTypeConversion {
        #[field(default = std::f64::consts::PI)]
        pi: f64,

        #[field(default = true)]
        enabled: bool,
    }

    let config = TestTypeConversion::load().unwrap();
    assert_eq!(config.pi, std::f64::consts::PI);
    assert_eq!(config.enabled, true);
}

#[test]
fn test_array_toml() {
    #[cruct(path = "./tests/fixtures/test_config.toml")]
    #[derive(Debug, PartialEq)]
    struct ArrayToml {
        items: Vec<String>,
        numbers: Vec<u16>,
    }

    let config = ArrayToml::load().unwrap();
    assert_eq!(config.items, vec!["a", "b", "c"]);
    assert_eq!(config.numbers, vec![1, 2, 3]);
}

#[test]
fn test_array_json() {
    #[cruct(path = "./tests/fixtures/test_config.json", format = "Json")]
    #[derive(Debug, PartialEq)]
    struct ArrayJson {
        items: Vec<String>,
        numbers: Vec<u16>,
    }

    let config = ArrayJson::load().unwrap();
    assert_eq!(config.items, vec!["x", "y", "z"]);
    assert_eq!(config.numbers, vec![10, 20, 30]);
}

#[test]
fn test_array_yaml() {
    #[cruct(path = "./tests/fixtures/test_config.yml", format = "Yml")]
    #[derive(Debug, PartialEq)]
    struct ArrayYaml {
        items: Vec<String>,
        numbers: Vec<u16>,
    }

    let config = ArrayYaml::load().unwrap();
    assert_eq!(config.items, vec!["alpha", "beta", "gamma"]);
    assert_eq!(config.numbers, vec![100, 200, 300]);
}

#[test]
fn test_nested_array_toml() {
    #[cruct(path = "./tests/fixtures/test_config.toml")]
    #[derive(Debug, PartialEq)]
    struct NestedArrayToml {
        matrix: Vec<Vec<u16>>,
    }

    let cfg = NestedArrayToml::load().unwrap();
    assert_eq!(cfg.matrix, vec![vec![1, 2], vec![3, 4]]);
}

#[test]
fn test_nested_array_json() {
    #[cruct(path = "./tests/fixtures/test_config.json", format = "Json")]
    #[derive(Debug, PartialEq)]
    struct NestedArrayJson {
        matrix: Vec<Vec<u16>>,
    }

    let cfg = NestedArrayJson::load().unwrap();
    assert_eq!(cfg.matrix, vec![vec![1, 2], vec![3, 4]]);
}

#[test]
fn test_nested_array_yaml() {
    #[cruct(path = "./tests/fixtures/test_config.yml", format = "Yml")]
    #[derive(Debug, PartialEq)]
    struct NestedArrayYaml {
        matrix: Vec<Vec<u16>>,
    }

    let cfg = NestedArrayYaml::load().unwrap();
    assert_eq!(cfg.matrix, vec![vec![1, 2], vec![3, 4]]);
}

#[cruct(path = "./tests/fixtures/test_config.toml")]
#[derive(Debug, PartialEq)]
struct SomeStruct {
    items: Vec<String>,
    numbers: Vec<u16>,
}

impl FromConfigValue for SomeStruct {
    fn from_config_value(value: &ConfigValue) -> Result<Self, ParserError> {
        SomeStruct::load_from(value)
    }
}

#[cruct(path = "./tests/fixtures/test_config.toml")]
#[derive(Debug, PartialEq)]
struct NestedConfig {
    http_port: u16,
    nested: SomeStruct,
}

#[test]
fn test_nested_struct_toml() {
    let cfg = NestedConfig::load().unwrap();

    assert_eq!(cfg.http_port, 8080);

    assert_eq!(
        cfg.nested,
        SomeStruct {
            items: vec!["foo".into(), "bar".into()],
            numbers: vec![42, 99],
        }
    );
}

#[cruct(path = "./tests/fixtures/test_config.toml", format = "Toml")]
#[derive(Debug, PartialEq)]
struct TestTomlLoad {
    #[field(name = "else")]
    something: String,
    http_port: u16,
}

#[test]
fn test_macro_load_without_cli() {
    let cfg = TestTomlLoad::load().unwrap();
    assert_eq!(cfg.something, "toml value");
    assert_eq!(cfg.http_port, 8080);
}

#[cfg(feature = "cli")]
#[test]
fn test_macro_load_with_cli_feature_enabled() {
    // Even with `cli` feature on, no flags means behavior is unchanged
    let cfg = TestTomlLoad::load().unwrap();
    assert_eq!(cfg.something, "toml value");
    assert_eq!(cfg.http_port, 8080);
}

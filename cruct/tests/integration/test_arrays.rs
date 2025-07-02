use assay::assay;
use cruct::cruct;

#[assay(
    include = [ "tests/fixtures/test_config.toml" ]
)]
fn test_array_toml() {
    #[cruct(load_config(path = "tests/fixtures/test_config.toml"))]
    #[derive(Debug, PartialEq)]
    struct ArrayToml {
        items: Vec<String>,
        numbers: Vec<u16>,
    }

    let config = ArrayToml::loader()
        .with_config()
        .load()
        .unwrap();

    assert_eq!(config.items, vec!["a", "b", "c"]);
    assert_eq!(config.numbers, vec![1, 2, 3]);
}

#[assay(
    include = [ "tests/fixtures/test_config.json" ]
)]
fn test_array_json() {
    #[cruct(load_config(path = "tests/fixtures/test_config.json"))]
    #[derive(Debug, PartialEq)]
    struct ArrayJson {
        items: Vec<String>,
        numbers: Vec<u16>,
    }

    let config = ArrayJson::loader()
        .with_config()
        .load()
        .unwrap();

    assert_eq!(config.items, vec!["x", "y", "z"]);
    assert_eq!(config.numbers, vec![10, 20, 30]);
}

#[assay(
    include = [ "tests/fixtures/test_config.yml" ]
)]
fn test_array_yaml() {
    #[cruct(load_config(path = "tests/fixtures/test_config.yml"))]
    #[derive(Debug, PartialEq)]
    struct ArrayYaml {
        items: Vec<String>,
        numbers: Vec<u16>,
    }

    let config = ArrayYaml::loader()
        .with_config()
        .load()
        .unwrap();

    assert_eq!(config.items, vec!["alpha", "beta", "gamma"]);
    assert_eq!(config.numbers, vec![100, 200, 300]);
}

#[assay(
    include = [ "tests/fixtures/test_config.toml" ]
)]
fn test_nested_array_toml() {
    #[cruct(load_config(path = "tests/fixtures/test_config.toml"))]
    #[derive(Debug, PartialEq)]
    struct NestedArrayToml {
        matrix: Vec<Vec<u16>>,
    }

    let cfg = NestedArrayToml::loader()
        .with_config()
        .load()
        .unwrap();

    assert_eq!(cfg.matrix, vec![vec![1, 2], vec![3, 4]]);
}

#[assay(
    include = [ "tests/fixtures/test_config.json" ]
)]
fn test_nested_array_json() {
    #[cruct(load_config(path = "tests/fixtures/test_config.json", format = "Json"))]
    #[derive(Debug, PartialEq)]
    struct NestedArrayJson {
        matrix: Vec<Vec<u16>>,
    }

    let cfg = NestedArrayJson::loader()
        .with_config()
        .load()
        .unwrap();

    assert_eq!(cfg.matrix, vec![vec![1, 2], vec![3, 4]]);
}

#[assay(
    include = [ "tests/fixtures/test_config.yml" ]
)]
fn test_nested_array_yaml() {
    #[cruct(load_config(path = "tests/fixtures/test_config.yml", format = "Yml"))]
    #[derive(Debug, PartialEq)]
    struct NestedArrayYaml {
        matrix: Vec<Vec<u16>>,
    }

    let cfg = NestedArrayYaml::loader()
        .with_config()
        .load()
        .unwrap();

    assert_eq!(cfg.matrix, vec![vec![1, 2], vec![3, 4]]);
}

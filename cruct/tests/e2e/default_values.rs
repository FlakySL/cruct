use assay::assay;
use cruct::cruct;

#[assay(
    include = ["tests/fixtures/e2e/basic.toml"],
    env = [
        ("MY_ENV", "world"),
    ]
)]
fn default_and_env_precedence() {
    #[cruct(load_config(path = "tests/fixtures/e2e/basic.toml"))]
    #[derive(Debug)]
    struct E2E {
        #[field(name = "missing", env_override = "MY_ENV", default = "hello".to_string())]
        v: String,
    }

    let v1 = E2E::loader()
        .with_config()
        .load()
        .unwrap();
    assert_eq!(v1.v, "world");

    unsafe {
        std::env::remove_var("MY_ENV");
    }

    let v2 = E2E::loader()
        .with_config()
        .load()
        .unwrap();
    assert_eq!(v2.v, "hello");
}

#[test]
fn test_string_default() {
    #[cruct(load_config(path = "tests/fixtures/e2e/defaults/value.toml"))]
    #[derive(Debug, PartialEq)]
    struct TestStringDefault {
        #[field(default = "literal_default".to_string())]
        field: String,
    }

    let string_config = TestStringDefault::loader()
        .with_config()
        .load()
        .unwrap();

    assert_eq!(string_config.field, "literal_default");
}

#[test]
fn test_fn_default() {
    fn get_default_from_fn() -> String {
        "function_default".to_string()
    }

    #[cruct(load_config(path = "tests/fixtures/e2e/defaults/value.toml"))]
    #[derive(Debug, PartialEq)]
    struct TestFnDefault {
        #[field(default = get_default_from_fn())]
        field: String,
    }

    let fn_config = TestFnDefault::loader()
        .with_config()
        .load()
        .unwrap();
    assert_eq!(fn_config.field, "function_default");
}

#[test]
fn test_expr_default() {
    #[cruct(load_config(path = "tests/fixtures/e2e/defaults/value.toml"))]
    #[derive(Debug, PartialEq)]
    struct TestExprDefault {
        #[field(default = format!("{}-{}", "expr", 42))]
        field: String,
    }

    let expr_config = TestExprDefault::loader()
        .with_config()
        .load()
        .unwrap();
    assert_eq!(expr_config.field, "expr-42");
}

#[test]
fn test_numeric_default() {
    #[cruct(load_config(path = "tests/fixtures/e2e/defaults/value.toml"))]
    #[derive(Debug, PartialEq)]
    struct TestNumericDefault {
        #[field(default = 9000)]
        port: u16,
    }

    let numeric_config = TestNumericDefault::loader()
        .with_config()
        .load()
        .unwrap();
    assert_eq!(numeric_config.port, 9000);
}

#[test]
fn test_float_default() {
    #[cruct(load_config(path = "tests/fixtures/e2e/defaults/value.toml"))]
    #[derive(Debug, PartialEq)]
    struct TestFloat {
        #[field(default = std::f64::consts::PI)]
        value: f64,
    }

    let config = TestFloat::loader()
        .with_config()
        .load()
        .unwrap();

    assert_eq!(config.value, std::f64::consts::PI);
}

#[test]
#[allow(clippy::bool_assert_comparison)]
fn test_boolean_default() {
    #[cruct(load_config(path = "tests/fixtures/e2e/defaults/value.toml"))]
    #[derive(Debug, PartialEq)]
    struct TestBoolean {
        #[field(default = false)]
        value: bool,
    }

    let config = TestBoolean::loader()
        .with_config()
        .load()
        .unwrap();

    assert_eq!(config.value, false);
}

#[test]
fn test_char_default() {
    #[cruct(load_config(path = "tests/fixtures/e2e/defaults/value.toml"))]
    #[derive(Debug, PartialEq)]
    struct TestChar {
        #[field(default = 'a')]
        value: char,
    }

    let config = TestChar::loader()
        .with_config()
        .load()
        .unwrap();

    assert_eq!(config.value, 'a');
}

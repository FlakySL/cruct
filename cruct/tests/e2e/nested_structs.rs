use cruct::{ConfigValue, FromConfigValue, ParserError, cruct};

#[test]
fn nested_structs_load_correctly() {
    #[cruct(load_config(path = "tests/fixtures/e2e/nested.toml"))]
    #[derive(Debug)]
    struct Outer {
        inner: Inner,
    }

    #[cruct]
    #[derive(Debug)]
    struct Inner {
        #[field(default = 100)]
        value: u32,
    }

    impl FromConfigValue for Inner {
        fn from_config_value(value: &ConfigValue) -> Result<Self, ParserError> {
            Inner::load_from(value)
        }
    }

    let o = Outer::loader()
        .with_config()
        .load()
        .unwrap();
    assert_eq!(
        o.inner
            .value,
        42
    );
}

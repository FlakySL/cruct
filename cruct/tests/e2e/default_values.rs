use cruct::cruct;

#[test]
fn default_and_env_precedence() {
    #[cruct(load_config(path = "tests/fixtures/e2e/basic.toml"))]
    #[derive(Debug)]
    struct E2E {
        #[field(name = "missing", env_override = "MY_ENV", default = "hello".to_string())]
        v: String,
    }

    unsafe { std::env::set_var("MY_ENV", "world") };
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

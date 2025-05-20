use cruct::cruct;

#[test]
fn simple_derive_generates_loader() {
    #[cruct(load_config(path = "tests/fixtures/basic.toml"))]
    #[derive(Debug)]
    struct S {
        a: String,
    }

    let _ = S::loader();
}

#[test]
fn derive_with_multiple_configs_collects_all() {
    #[cruct(
        load_config(path = "tests/fixtures/basic.toml"),
        load_config(path = "tests/fixtures/nested.toml", priority = 10)
    )]
    #[derive(Debug)]
    struct S2 {
        b: u32,
    }

    let loader = S2::loader();
    assert_eq!(
        loader
            .configs()
            .len(),
        2
    );
}

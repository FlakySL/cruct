use assay::assay;
use cruct::cruct;

#[assay(
    include = ["tests/fixtures/e2e/nested.toml"],
)]
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

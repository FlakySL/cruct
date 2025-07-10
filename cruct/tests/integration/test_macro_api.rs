use assay::assay;
use cruct::cruct;

#[assay(
    include = ["tests/fixtures/integration/basic.toml"],
)]
fn simple_derive_generates_loader() {
    #[cruct(load_config(path = "tests/fixtures/integration/basic.toml"))]
    #[derive(Debug)]
    #[allow(dead_code)]
    struct S {
        a: String,
    }

    let _ = S::loader();
}

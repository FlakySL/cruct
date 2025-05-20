use cruct::cruct;

#[test]
fn cli_override_takes_highest_priority() {
    #[cruct(load_config(path = "tests/fixtures/basic.toml"))]
    #[derive(Debug)]
    struct C {
        #[field(default = 1, shell_override = "")]
        n: u32,
    }

    let c = C::loader()
        .with_config()
        .with_cli(0)
        .load()
        .unwrap();

    assert_eq!(c.n, 999);
}

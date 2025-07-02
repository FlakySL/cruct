use assay::assay;
use cruct::cruct;

#[assay(
    include = ["tests/fixtures/integration/basic.toml"],
)]
fn loads_values_from_toml_and_env() {
    #[cruct(load_config(path = "tests/fixtures/integration/basic.toml"))]
    #[derive(Debug)]
    struct Cfg {
        #[field(env_override = "CFG_NAME", default = "fallback".to_string())]
        name: String,
        #[field(default = 100)]
        count: u32,
    }

    // Fixture TOML has `name = "from_file"` and no `count` key.
    unsafe {
        std::env::remove_var("CFG_NAME");
    }

    let cfg = Cfg::loader()
        .with_config()
        .load()
        .unwrap();

    assert_eq!(cfg.name, "from_file");
    assert_eq!(cfg.count, 100);

    // Now override via env
    unsafe {
        std::env::set_var("CFG_NAME", "env_name");
    }

    let cfg2 = Cfg::loader()
        .with_config()
        .load()
        .unwrap();

    assert_eq!(cfg2.name, "env_name");
}

#[assay(
    include = [
        "tests/fixtures/integration/nested.toml",
        "tests/fixtures/integration/basic.toml"
    ],
)]
fn loader_respects_priority_order() {
    #[cruct(
        load_config(path = "tests/fixtures/integration/basic.toml", priority = 5),
        load_config(path = "tests/fixtures/integration/nested.toml", priority = 1)
    )]
    #[derive(Debug)]
    struct P {
        #[field(default = 10)]
        x: u32,
    }

    // nested.toml has x=20, basic.toml has x not set => priority 1 applies first
    let p = P::loader()
        .with_config()
        .load()
        .unwrap();

    assert_eq!(p.x, 20);
}

use std::io::Write;
use std::sync::Arc;
use std::thread::spawn;

use assay::assay;
use cruct::parser::TomlParser;
use cruct::{Parser, cruct};
use tempfile::NamedTempFile;

#[assay(
    include = ["tests/fixtures/concurrency.toml"],
)]
fn config_loader_thread_safe() {
    #[allow(unused)]
    #[cruct(load_config(path = "tests/fixtures/concurrency.toml"))]
    struct Config {
        value: u32,
    }

    let loader = Config::loader();
    spawn(move || {
        let _ = loader
            .with_config()
            .load()
            .unwrap();
    })
    .join()
    .unwrap();
}

#[test]
fn parser_thread_safe() {
    let parser = Arc::new(TomlParser);
    let handles: Vec<_> = (0..10)
        .map(|i| {
            let parser = parser.clone();
            spawn(move || {
                let mut file = NamedTempFile::new().unwrap();
                writeln!(file, "value = {}", i).unwrap();
                let path = file
                    .path()
                    .to_str()
                    .unwrap();

                parser
                    .load(path)
                    .unwrap()
            })
        })
        .collect();

    for handle in handles {
        handle
            .join()
            .unwrap();
    }
}

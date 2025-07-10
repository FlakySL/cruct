use std::io::Write;

use tempfile::NamedTempFile;

use crate::{ConfigFileSource, ConfigSource, ConfigValue, FileFormat};

#[test]
fn parses_simple_yaml_map() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "foo: bar\nbaz: 42").unwrap();
    let path = file
        .path()
        .to_str()
        .unwrap();

    let src = ConfigFileSource::new(path, Some(FileFormat::Yml));
    let cfg = src
        .load()
        .unwrap();

    if let ConfigValue::Section(map) = cfg {
        assert_eq!(map["foo"], ConfigValue::Value("bar".to_string()));
        assert_eq!(map["baz"], ConfigValue::Value("42".to_string()));
    } else {
        panic!("expected section");
    }
}

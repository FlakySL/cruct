use std::collections::HashMap;
use std::io::Write;

use cruct_shared::source::merge_sections;
use cruct_shared::{
    ConfigBuilder,
    ConfigFileSource,
    ConfigSource,
    ConfigValue,
    FileFormat,
    ParserError,
};
use tempfile::NamedTempFile;

#[test]
fn merge_sections_overrides_existing_and_inserts_new() {
    let mut base = HashMap::new();
    base.insert("a".to_string(), ConfigValue::Value("1".to_string()));
    base.insert(
        "nested".to_string(),
        ConfigValue::Section({
            let mut m = HashMap::new();
            m.insert("x".to_string(), ConfigValue::Value("10".to_string()));
            m
        }),
    );

    let mut high = HashMap::new();
    // override top-level key
    high.insert("a".to_string(), ConfigValue::Value("2".to_string()));
    // override nested
    high.insert(
        "nested".to_string(),
        ConfigValue::Section({
            let mut m = HashMap::new();
            m.insert("x".to_string(), ConfigValue::Value("20".to_string()));
            m.insert("y".to_string(), ConfigValue::Value("30".to_string()));
            m
        }),
    );
    // insert new key
    high.insert("b".to_string(), ConfigValue::Value("3".to_string()));

    let merged = merge_sections(base, high);
    // a overridden
    assert_eq!(merged.get("a"), Some(&ConfigValue::Value("2".to_string())));
    // nested.x overridden, nested.y inserted
    if let Some(ConfigValue::Section(nmap)) = merged.get("nested") {
        assert_eq!(nmap.get("x"), Some(&ConfigValue::Value("20".to_string())));
        assert_eq!(nmap.get("y"), Some(&ConfigValue::Value("30".to_string())));
    } else {
        panic!("nested section missing");
    }

    // b inserted
    assert_eq!(merged.get("b"), Some(&ConfigValue::Value("3".to_string())));
}

#[test]
fn config_file_source_with_explicit_format() -> Result<(), ParserError> {
    // Create a TOML file with a simple key
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "foo = \"bar\"")?;
    let path = file
        .path()
        .to_str()
        .unwrap();

    let src = ConfigFileSource::new(path, Some(FileFormat::Toml));
    let val = src.load()?;
    if let ConfigValue::Section(map) = val {
        assert_eq!(map.get("foo"), Some(&ConfigValue::Value("bar".to_string())));
    } else {
        panic!("Expected section");
    }
    Ok(())
}

#[test]
fn config_file_source_auto_detect_extension() -> Result<(), ParserError> {
    // Create a JSON file with a simple key
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "{{ \"baz\": 123 }}").unwrap();

    // rename to have .json extension
    let path = file
        .into_temp_path()
        .with_extension("json");
    std::fs::write(&path, r#"{ "baz": 123 }"#)?;

    let src = ConfigFileSource::new(
        path.to_str()
            .unwrap(),
        None,
    );
    let val = src.load()?;

    if let ConfigValue::Section(map) = val {
        assert_eq!(map.get("baz"), Some(&ConfigValue::Value("123".to_string())));
    } else {
        panic!("Expected section");
    }

    Ok(())
}

#[test]
fn config_builder_merges_file_sources_in_order() -> Result<(), ParserError> {
    // Base TOML
    let mut file1 = NamedTempFile::new().unwrap();
    writeln!(file1, "port = 8000")?;
    let path1 = file1
        .path()
        .to_str()
        .unwrap();

    // Override TOML
    let mut file2 = NamedTempFile::new().unwrap();
    writeln!(file2, "port = 9000")?;
    let path2 = file2
        .path()
        .to_str()
        .unwrap();

    let val = ConfigBuilder::new()
        .add_source(ConfigFileSource::new(path1, Some(FileFormat::Toml)))
        .add_source(ConfigFileSource::new(path2, Some(FileFormat::Toml)))
        .load()?;

    if let ConfigValue::Section(map) = val {
        assert_eq!(map.get("port"), Some(&ConfigValue::Value("9000".to_string())));
    } else {
        panic!("Expected section");
    }

    Ok(())
}

#[test]
fn config_builder_applies_dummy_source_last() -> Result<(), ParserError> {
    // create a base
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "flag = \"off\"")?;
    let path = file
        .path()
        .to_str()
        .unwrap();

    struct Dummy;
    impl ConfigSource for Dummy {
        fn load(&self) -> Result<ConfigValue, ParserError> {
            let mut m = HashMap::new();
            m.insert("flag".to_string(), ConfigValue::Value("on".to_string()));
            Ok(ConfigValue::Section(m))
        }
    }

    let val = ConfigBuilder::new()
        .add_source(ConfigFileSource::new(path, Some(FileFormat::Toml)))
        .add_source(Dummy)
        .load()?;

    if let ConfigValue::Section(map) = val {
        assert_eq!(map.get("flag"), Some(&ConfigValue::Value("on".to_string())));
    } else {
        panic!("Expected section");
    }

    Ok(())
}

#[cfg(feature = "clap")]
mod cli_source_tests {
    use clap::{Arg, Command};
    use cruct_shared::ConfigValue;
    use cruct_shared::source::{ClapSource, ConfigSource};

    #[test]
    fn clap_source_parses_single_flag() {
        let cmd = Command::new("test_app").arg(
            Arg::new("port")
                    .long("port")
                    .num_args(1)  // Explicitly expect 1 value
                    .value_name("PORT"),
        );

        // Parse arguments first to get ArgMatches
        let matches = cmd.get_matches_from(["test_app", "--port", "4242"]);

        let clap_source = ClapSource::new(matches);
        let config = clap_source
            .load()
            .unwrap();

        if let ConfigValue::Section(map) = config {
            assert_eq!(map.get("port"), Some(&ConfigValue::Value("4242".to_string())));
        } else {
            panic!("Expected ConfigValue::Section");
        }
    }

    #[test]
    fn clap_source_merges_multiple_flags() {
        let cmd = Command::new("my_app")
            .arg(
                Arg::new("host")
                    .long("host")
                    .num_args(1)
                    .value_name("HOST"),
            )
            .arg(
                Arg::new("debug")
                    .long("debug")
                    .num_args(1)
                    .value_name("DEBUG"),
            );

        let matches = cmd.get_matches_from(["my_app", "--host", "localhost", "--debug", "true"]);

        let clap_source = ClapSource::new(matches);
        let config = clap_source
            .load()
            .unwrap();

        if let ConfigValue::Section(map) = config {
            assert_eq!(map.get("host"), Some(&ConfigValue::Value("localhost".to_string())));
            assert_eq!(map.get("debug"), Some(&ConfigValue::Value("true".to_string())));
        } else {
            panic!("Expected ConfigValue::Section");
        }
    }
}

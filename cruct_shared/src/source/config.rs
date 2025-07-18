use std::sync::Arc;

use super::ConfigSource;
use crate::parser::get_file_extension;
use crate::{ConfigValue, FileFormat, Parser, ParserError, get_parser};

pub struct ConfigFileSource {
    path: String,
    format: Option<FileFormat>,
}

impl ConfigFileSource {
    /// Creates a new `ConfigFileSource`.
    ///
    /// * `path`: The path to the configuration file.
    /// * `format`: Optional file format. If not provided, the format will be
    ///   inferred from the file extension.
    pub fn new(path: impl Into<String>, format: Option<FileFormat>) -> Self {
        ConfigFileSource { path: path.into(), format }
    }

    /// Retrieves the parser based on the file format or extension.
    ///
    /// If a format is provided, it uses that; otherwise, it infers the
    /// format from the file
    fn get_parser(&self) -> Result<Arc<dyn Parser>, ParserError> {
        let ext = if let Some(fmt) = &self.format {
            fmt.to_string()
        } else {
            get_file_extension(&self.path)?
        };

        get_parser(&ext)
    }
}

impl ConfigSource for ConfigFileSource {
    fn load(&self) -> Result<ConfigValue, ParserError> {
        let parser = self.get_parser()?;
        parser.load(&self.path)
    }
}

#[cfg(test)]
mod test {
    use crate::{ConfigFileSource, FileFormat};

    #[test]
    fn test_toml_parser_extensions() {
        let src = ConfigFileSource::new("test.toml", Some(FileFormat::Toml));
        let parser = src
            .get_parser()
            .expect("Failed to get parser");

        let extensions = parser.extensions();

        assert!(extensions.contains(&"toml"), "Expected 'toml' extension, found: {:?}", extensions);
    }

    #[test]
    fn test_json_parser_extensions() {
        let src = ConfigFileSource::new("test.json", Some(FileFormat::Json));
        let parser = src
            .get_parser()
            .expect("Failed to get parser");

        let extensions = parser.extensions();

        assert!(extensions.contains(&"json"), "Expected 'json' extension, found: {:?}", extensions);
    }

    #[test]
    fn test_yaml_parser_extensions() {
        let src = ConfigFileSource::new("test.yaml", Some(FileFormat::Yml));
        let parser = src
            .get_parser()
            .expect("Failed to get parser");

        let extensions = parser.extensions();

        assert!(extensions.contains(&"yaml"), "Expected 'yaml' extension, found: {:?}", extensions);
    }
}

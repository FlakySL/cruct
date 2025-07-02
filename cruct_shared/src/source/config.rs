use std::sync::Arc;

use super::ConfigSource;
use crate::parser::get_file_extension;
use crate::{ConfigValue, FileFormat, Parser, ParserError, get_parser};

pub struct ConfigFileSource {
    path: String,
    format: Option<FileFormat>,
}

impl ConfigFileSource {
    pub fn new(path: impl Into<String>, format: Option<FileFormat>) -> Self {
        ConfigFileSource { path: path.into(), format }
    }

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
    fn verify_extension_name() {
        let src = ConfigFileSource::new("test.toml", Some(FileFormat::Toml));
        let parser = src
            .get_parser()
            .expect("Failed to get parser");

        let extensions = parser.extensions();

        assert!(extensions.contains(&"toml"), "Expected 'toml' extension, found: {:?}", extensions);
    }
}

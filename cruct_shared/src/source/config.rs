use std::path::Path;

use super::ConfigSource;
use crate::{ConfigValue, ParserError, get_parser_by_extension};

pub struct ConfigFileSource {
    path: String,
    format: Option<crate::FileFormat>,
}

impl ConfigFileSource {
    pub fn new(path: impl Into<String>, format: Option<crate::FileFormat>) -> Self {
        ConfigFileSource { path: path.into(), format }
    }
}

impl ConfigSource for ConfigFileSource {
    fn load(&self) -> Result<ConfigValue, ParserError> {
        let parser = if let Some(fmt) = &self.format {
            let ext_str = fmt.to_string();

            get_parser_by_extension(&ext_str).ok_or(ParserError::InvalidFileFormat(ext_str))?
        } else {
            let ext = Path::new(&self.path)
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or_default();
            get_parser_by_extension(ext)
                .ok_or_else(|| ParserError::InvalidFileFormat(ext.into()))?
        };

        parser.load(&self.path)
    }
}

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

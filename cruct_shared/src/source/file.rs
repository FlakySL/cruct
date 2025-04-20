use std::path::Path;

use super::ConfigSource;
use crate::{ConfigValue, ParserError, get_parser_by_extension};

pub struct FileSource {
    path: String,
}

impl FileSource {
    pub fn new(path: impl Into<String>) -> Self {
        FileSource { path: path.into() }
    }
}

impl ConfigSource for FileSource {
    fn load(&self) -> Result<ConfigValue, ParserError> {
        let ext = Path::new(&self.path)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        let parser = get_parser_by_extension(ext)
            .ok_or_else(|| ParserError::InvalidFileFormat(ext.to_string()))?;

        parser.load(&self.path)
    }
}

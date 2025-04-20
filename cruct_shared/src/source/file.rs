use super::ConfigSource;
use crate::parser::get_file_extension;
use crate::{ConfigValue, ParserError, get_parser};

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
        let ext = get_file_extension(&self.path)?;
        let parser = get_parser(&ext)?;

        parser.load(&self.path)
    }
}

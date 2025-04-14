use jzon::Error as JsonError;
use std::collections::HashMap;
use std::fmt::Display;
use std::io::Error as StdError;
use std::sync::Arc;
use thiserror::Error as ThisError;
use toml::de::Error as TomlError;
use yaml_rust2::ScanError as YmlError;

mod json;
mod tml;
mod yml;

use json::JsonParser;
use tml::TomlParser;
use yml::YmlParser;

#[derive(Debug, ThisError)]
pub enum ParserError {
    #[error("'{0}' is not a valid file format")]
    InvalidFileFormat(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Type mismatch in field '{field}', expected {expected}")]
    TypeMismatch { field: String, expected: String },

    #[error("{0:#}")]
    Io(#[from] StdError),

    #[error("TOML parsing error: {0}")]
    TomlError(#[from] TomlError),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] JsonError),

    #[error("YAML parsing error: {0}")]
    YmlError(#[from] YmlError),

    #[error("Invalid YAML format")]
    InvalidYamlFormat,
}

/// This enum represents the available
/// file formats for configuration parsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileFormat {
    /// YML/YAML file format identifier.
    Yml,
    /// JSON file format identifier.
    Json,
    /// TOML file format identifier.
    Toml,
}

impl Display for FileFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileFormat::Yml => write!(f, "yaml"),
            FileFormat::Json => write!(f, "json"),
            FileFormat::Toml => write!(f, "toml"),
        }
    }
}

pub trait Parser: Send + Sync {
    /// Returns supported file extensions for this parser
    /// Example: ["yml", "yaml"] for YAML parser
    fn extensions(&self) -> &'static [&'static str];

    fn format(&self) -> FileFormat {
        match self.extensions()[0] {
            "yml" | "yaml" => FileFormat::Yml,
            "json" => FileFormat::Json,
            "toml" => FileFormat::Toml,
            _ => panic!("Unsupported file format"),
        }
    }

    /// Main parsing logic
    fn load(&self, path: &str) -> Result<HashMap<String, String>, ParserError>;
}

pub struct ParserRegistry {
    by_extension: HashMap<&'static str, Arc<dyn Parser>>,
}

impl ParserRegistry {
    pub fn new() -> Self {
        Self {
            by_extension: HashMap::new(),
        }
    }

    pub fn add_parser(&mut self, parser: Arc<dyn Parser>) {
        for ext in parser.extensions() {
            self.by_extension.insert(ext, Arc::clone(&parser));
        }
    }

    pub fn get_by_extension(&self, ext: &str) -> Option<&Arc<dyn Parser>> {
        self.by_extension.get(ext)
    }
}

impl Default for ParserRegistry {
    fn default() -> Self {
        Self::new()
    }
}

pub fn default_registry() -> ParserRegistry {
    let mut registry = ParserRegistry::new();
    registry.add_parser(Arc::new(TomlParser));
    registry.add_parser(Arc::new(JsonParser));
    registry.add_parser(Arc::new(YmlParser));

    registry
}

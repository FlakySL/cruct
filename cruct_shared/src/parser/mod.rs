use std::collections::HashMap;
use std::fmt::Display;
use std::io::Error as StdError;
use std::sync::Arc;

use jzon::Error as JsonError;
use thiserror::Error as ThisError;
use toml::de::Error as TomlError;
use yaml_rust2::ScanError as YmlError;

mod json;
mod tml;
mod yml;

use json::JsonParser;
use tml::TomlParser;
use yml::YmlParser;

/// Enum representing possible errors during parsing.
/// Utilizes the `thiserror` crate for error handling.
#[derive(Debug, ThisError)]
pub enum ParserError {
    /// Error indicating that the file format is not supported.
    #[error("'{0}' is not a valid file format")]
    InvalidFileFormat(String),

    /// Error indicating that the file format is not supported.
    #[error("Missing required field: {0}")]
    MissingField(String),

    /// Error indicating that the file format is not supported.
    #[error("Type mismatch in field '{field}', expected {expected}")]
    TypeMismatch { field: String, expected: String },

    /// Standard IO error.
    #[error("{0:#}")]
    Io(#[from] StdError),

    /// TOML parsing error.
    #[error("TOML parsing error: {0}")]
    TomlError(#[from] TomlError),

    /// JSON parsing error.
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] JsonError),

    /// YAML parsing error.
    #[error("YAML parsing error: {0}")]
    YmlError(#[from] YmlError),
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

/// Implement `Display` trait for `FileFormat` to allow easy conversion to
/// string.
impl Display for FileFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileFormat::Yml => write!(f, "yaml"),
            FileFormat::Json => write!(f, "json"),
            FileFormat::Toml => write!(f, "toml"),
        }
    }
}

/// Trait defining the interface for parsers.
/// Parsers must be thread-safe (`Send + Sync`).
pub trait Parser: Send + Sync {
    /// Returns supported file extensions for this parser
    /// Example: ["yml", "yaml"] for YAML parser
    fn extensions(&self) -> &'static [&'static str];

    /// Returns the file format associated with this parser.
    fn format(&self) -> FileFormat {
        match self.extensions()[0] {
            "yml" | "yaml" => FileFormat::Yml,
            "json" => FileFormat::Json,
            "toml" => FileFormat::Toml,
            _ => panic!("Unsupported file format"),
        }
    }

    /// Main parsing logic.
    /// Loads a file and returns a map of key-value pairs.
    /// Returns a `ParserError` if parsing fails.
    fn load(&self, path: &str) -> Result<HashMap<String, String>, ParserError>;
}

/// Function to get a parser based on file extension.
/// Returns an `Option` containing the parser if the extension is supported.
pub fn get_parser_by_extension(ext: &str) -> Option<Arc<dyn Parser>> {
    match ext {
        "yml" | "yaml" => Some(Arc::new(YmlParser)),
        "json" => Some(Arc::new(JsonParser)),
        "toml" => Some(Arc::new(TomlParser)),
        _ => None,
    }
}

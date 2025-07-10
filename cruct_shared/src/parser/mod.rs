use std::collections::HashMap;
use std::fmt::Display;
use std::io::Error as StdError;
use std::str::FromStr;
use std::sync::Arc;

use jzon::Error as JsonError;
use thiserror::Error as ThisError;
use toml_edit::TomlError;
use yaml_rust2::ScanError as YmlError;

#[cfg(feature = "json")]
mod json;

#[cfg(feature = "toml")]
mod toml;

#[cfg(feature = "yaml")]
mod yaml;

#[cfg(test)]
mod tests;

#[cfg(feature = "json")]
pub use json::JsonParser;
#[cfg(feature = "toml")]
pub use toml::TomlParser;
#[cfg(feature = "yaml")]
pub use yaml::YmlParser;

/// Represents various errors that can occur during the parsing process.
/// Leverages the `thiserror` crate for structured and user-friendly error
/// handling.
#[derive(Debug, ThisError)]
pub enum ParserError {
    /// Indicates that the provided file format is unsupported.
    /// Occurs when trying to parse a file with an invalid or unrecognized
    /// extension.
    #[error("'{0}' is not a valid file format")]
    InvalidFileFormat(String),

    /// Triggered when a required field is missing in the configuration file.
    /// This happens when an expected field is absent.
    #[error("Missing required field: {0}")]
    MissingField(String),

    /// Occurs when there is a type mismatch in a field within the configuration
    /// file. This happens when a field's value type does not match the
    /// expected type.
    #[error("Type mismatch in field '{field}', expected {expected}")]
    TypeMismatch { field: String, expected: String },

    /// Raised when a file path lacks an extension.
    /// Without an extension, determining the file format becomes impossible.
    #[error("This file has no file extension")]
    MissingFileExtension,

    /// Indicates a nested configuration error in a specific section.
    /// Provides details about the section and the root cause of the error.
    #[error("Nested configuration error in {section}: {source}")]
    NestedError {
        section: String,
        #[source]
        source: Box<ParserError>,
    },

    /// Reflects standard IO errors encountered during file operations.
    /// Arises when issues occur while reading or writing files.
    #[error("{0:#}")]
    Io(#[from] StdError),

    /// Represents a failure in parsing a TOML file.
    /// Occurs when the parser encounters invalid TOML syntax or structure.
    #[error("TOML parsing error: {0}")]
    TomlError(#[from] TomlError),

    /// Represents a failure in parsing a JSON file.
    /// Triggered by invalid JSON syntax or structure during parsing.
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] JsonError),

    /// Represents a failure in parsing a YAML file.
    /// Triggered by invalid YAML syntax or structure during parsing.
    #[error("YAML parsing error: {0}")]
    YmlError(#[from] YmlError),
}

/// Represents the supported file formats for configuration parsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileFormat {
    /// YML/YAML file format identifier.
    #[cfg(feature = "yaml")]
    Yml,
    /// JSON file format identifier.
    #[cfg(feature = "json")]
    Json,
    /// TOML file format identifier.
    #[cfg(feature = "toml")]
    Toml,
}

// Implement `FromStr` trait for `FileFormat` to allow easy conversion from
// string.
impl FromStr for FileFormat {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s
            .to_lowercase()
            .as_str()
        {
            #[cfg(feature = "yaml")]
            "yml" | "yaml" => Ok(FileFormat::Yml),

            #[cfg(feature = "json")]
            "json" => Ok(FileFormat::Json),

            #[cfg(feature = "toml")]
            "toml" => Ok(FileFormat::Toml),

            _ => Err(ParserError::InvalidFileFormat(s.into())),
        }
    }
}

/// Implement `Display` trait for `FileFormat` to allow easy conversion to
/// string.
impl Display for FileFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "yaml")]
            FileFormat::Yml => write!(f, "yaml"),

            #[cfg(feature = "json")]
            FileFormat::Json => write!(f, "json"),

            #[cfg(feature = "toml")]
            FileFormat::Toml => write!(f, "toml"),
        }
    }
}

/// This enum represents the possible values
/// that can be parsed from a configuration file.
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigValue {
    Value(String),
    Section(HashMap<String, ConfigValue>),
    Array(Vec<ConfigValue>),
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
            #[cfg(feature = "yaml")]
            "yml" | "yaml" => FileFormat::Yml,

            #[cfg(feature = "json")]
            "json" => FileFormat::Json,

            #[cfg(feature = "toml")]
            "toml" => FileFormat::Toml,

            _ => panic!("Unsupported file format"),
        }
    }

    /// Main parsing logic.
    /// Loads a file and returns a map of key-value pairs.
    /// Returns a `ParserError` if parsing fails.
    fn load(&self, path: &str) -> Result<ConfigValue, ParserError>;
}

/// Function to get a parser based on file extension.
/// Returns an `Option` containing the parser if the extension is supported.
pub fn get_parser(ext: &str) -> Result<Arc<dyn Parser>, ParserError> {
    match ext {
        #[cfg(feature = "yaml")]
        "yml" | "yaml" => Ok(Arc::new(YmlParser)),

        #[cfg(feature = "json")]
        "json" => Ok(Arc::new(JsonParser)),

        #[cfg(feature = "toml")]
        "toml" => Ok(Arc::new(TomlParser)),

        _ => Err(ParserError::InvalidFileFormat(ext.into())),
    }
}

/// Function to get the file extension from a path.
pub fn get_file_extension(path: &str) -> Result<String, ParserError> {
    let ext = std::path::Path::new(path)
        .extension()
        .and_then(|s| s.to_str())
        .ok_or(ParserError::MissingFileExtension)?;

    Ok(ext.into())
}

/// Trait to convert a `ConfigValue` to a specific type.
pub trait FromConfigValue {
    fn from_config_value(value: &ConfigValue) -> Result<Self, ParserError>
    where
        Self: Sized;
}

/// Macro to implement FromConfigValue for scalar types.
/// This macro generates implementations for types that can be parsed from a
/// string.
macro_rules! impl_from_config_value {
    ($($t:ty),* $(,)?) => {$(
        impl FromConfigValue for $t {
            fn from_config_value(value: &ConfigValue) -> Result<Self, ParserError> {
                match value {
                    ConfigValue::Value(s) => parse_value::<$t>(s),
                    _ => Err(ParserError::TypeMismatch {
                        field: "Expected a scalar value".into(),
                        expected: stringify!($t).into(),
                    }),
                }
            }
        }
    )*};
}

/// Helper function to parse a string into a specific type.
/// Returns a ParserError if parsing fails.
fn parse_value<T: FromStr>(s: &str) -> Result<T, ParserError> {
    s.parse::<T>()
        .map_err(|_| ParserError::TypeMismatch {
            field: "Failed to parse value".into(),
            expected: stringify!(T).into(),
        })
}

// Scalar types
impl_from_config_value!(
    String, bool, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize, isize, f32, f64, char
);

/// Helper trait to convert a `ConfigValue` to a `Vec<T>`.
impl<T> FromConfigValue for Vec<T>
where
    T: FromConfigValue,
{
    fn from_config_value(value: &ConfigValue) -> Result<Self, ParserError> {
        match value {
            ConfigValue::Array(items) => items
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    T::from_config_value(item).map_err(|e| ParserError::NestedError {
                        section: format!("[{}]", i),
                        source: Box::new(e),
                    })
                })
                .collect(),
            _ => Err(ParserError::TypeMismatch {
                field: "".into(),
                expected: "array".into(),
            }),
        }
    }
}

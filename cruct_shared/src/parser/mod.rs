use std::collections::HashMap;
use std::fmt::Display;
use std::io::Error as StdError;
use std::str::FromStr;
use std::sync::Arc;

use jzon::Error as JsonError;
use thiserror::Error as ThisError;
use toml_edit::TomlError;
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
    /// Error indicating that the provided file format is not supported by the
    /// parser. This error is returned when an attempt is made to parse a
    /// file with an unsupported extension.
    #[error("'{0}' is not a valid file format")]
    InvalidFileFormat(String),

    /// Error indicating that a required field is missing in the configuration
    /// file. This error is returned when a field that is expected to be
    /// present is not found.
    #[error("Missing required field: {0}")]
    MissingField(String),

    /// Error indicating a type mismatch in a field within the configuration
    /// file. This error is returned when the type of a field's value does
    /// not match the expected type.
    #[error("Type mismatch in field '{field}', expected {expected}")]
    TypeMismatch { field: String, expected: String },

    /// Error indicating that the file does not have an extension.
    /// This error is returned when a file path is provided without an
    /// extension, making it impossible to determine the file format.
    #[error("This file has no file extension")]
    MissingFileExtension,

    /// Error indicating a nested configuration error within a specific section.
    /// This error provides details about the section where the error occurred
    /// and the source of the error.
    #[error("Nested configuration error in {section}: {source}")]
    NestedError {
        section: String,
        #[source]
        source: Box<ParserError>,
    },

    /// Standard IO error that occurs during file operations.
    /// This error is returned when there is an issue with reading or writing
    /// files.
    #[error("{0:#}")]
    Io(#[from] StdError),

    /// Error indicating a failure in parsing a TOML file.
    /// This error is returned when the TOML parser encounters invalid syntax or
    /// structure.
    #[error("TOML parsing error: {0}")]
    TomlError(#[from] TomlError),

    /// Error indicating a failure in parsing a JSON file.
    /// This error is returned when the JSON parser encounters invalid syntax or
    /// structure.
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] JsonError),

    /// Error indicating a failure in parsing a YAML file.
    /// This error is returned when the YAML parser encounters invalid syntax or
    /// structure.
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

// Implement `FromStr` trait for `FileFormat` to allow easy conversion from
// string.
impl FromStr for FileFormat {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s
            .to_lowercase()
            .as_str()
        {
            "yml" | "yaml" => Ok(FileFormat::Yml),
            "json" => Ok(FileFormat::Json),
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
            FileFormat::Yml => write!(f, "yaml"),
            FileFormat::Json => write!(f, "json"),
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
            "yml" | "yaml" => FileFormat::Yml,
            "json" => FileFormat::Json,
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
        "yml" | "yaml" => Ok(Arc::new(YmlParser)),
        "json" => Ok(Arc::new(JsonParser)),
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
    ($t:ty) => {
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
    };
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
// TODO: This currently looks disgusting and inefficient. Consider finding a
// better way to implement this.
impl_from_config_value!(String);
impl_from_config_value!(bool);
impl_from_config_value!(i8);
impl_from_config_value!(i16);
impl_from_config_value!(i32);
impl_from_config_value!(i64);
impl_from_config_value!(i128);
impl_from_config_value!(u8);
impl_from_config_value!(u16);
impl_from_config_value!(u32);
impl_from_config_value!(u64);
impl_from_config_value!(u128);
impl_from_config_value!(usize);
impl_from_config_value!(isize);
impl_from_config_value!(f32);
impl_from_config_value!(f64);
impl_from_config_value!(char);

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

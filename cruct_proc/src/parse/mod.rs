use thiserror::Error as ThisError;

mod field_params;
mod field_struct;
mod macro_params;

#[cfg(test)]
mod tests;

pub use field_params::FieldParams;
pub use field_struct::{StructField, remove_field_attrs};
pub use macro_params::MacroParams;

/// This enum is an error representation for parameter parsing. It implements
/// Display for error descriptions.
#[derive(ThisError, Debug)]
pub enum ParameterError {
    /// This error is used when a required parameter is missing.
    #[error("Missing required parameter '{name}'")]
    MissingRequired {
        /// The name of which parameter is missing.
        name: String,
    },

    /// This error is used when a parameter
    /// has an unexpected type.
    #[error("Invalid parameter type for '{name}', expected '{expected}', found '{found}'")]
    InvalidType {
        /// The name of which parameter is missing.
        name: String,

        /// An identifier for the expected type. (not enforced)
        expected: String,

        /// An identifier for the found type. (not enforced)
        found: String,
    },
}

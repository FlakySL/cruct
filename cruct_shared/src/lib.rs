pub mod parser;
pub mod source;

/// re-export clap only if the feature is enabled
#[cfg(feature = "cli")]
pub use clap;
pub use parser::{ConfigValue, FileFormat, FromConfigValue, Parser, ParserError, get_parser};
#[cfg(feature = "cli")]
pub use source::ClapSource;
pub use source::{ConfigBuilder, ConfigFileSource, ConfigSource, FileSource};

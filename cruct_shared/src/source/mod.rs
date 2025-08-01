use std::cmp::Reverse;
use std::collections::HashMap;

use crate::{ConfigValue, ParserError};

mod cli;
mod config;

#[cfg(test)]
mod tests;

pub use cli::CliSource;
pub use config::ConfigFileSource;

/// Trait defining a configuration source.
///
/// A `ConfigSource` is an entity capable of yielding a `ConfigValue`,
/// which can represent configuration data from various sources, such as files,
/// environment variables, or CLI arguments.
pub trait ConfigSource {
    /// Load configuration from the source.
    ///
    /// Returns a `Result` containing either the configuration value
    /// (`ConfigValue`) or a parsing error (`ParserError`) if loading fails.
    fn load(&self) -> Result<ConfigValue, ParserError>;

    /// Defines the priority of this configuration source.
    ///
    /// Sources with higher priority override those with lower priority during
    /// merging.
    fn priority(&self) -> u8 {
        u8::MAX
    }
}

/// Merge two `ConfigValue::Section` maps, where `high` takes precedence.
///
/// This function recursively merges two `HashMap` instances containing
/// `ConfigValue::Section` entries. If both sections contain nested sections,
/// they are merged recursively. Otherwise, values from `high` override those
/// in `base`.
pub fn merge_sections(
    mut base: HashMap<String, ConfigValue>,
    high: HashMap<String, ConfigValue>,
) -> HashMap<String, ConfigValue> {
    for (k, v_high) in high {
        match (base.remove(&k), v_high) {
            (Some(ConfigValue::Section(bsub)), ConfigValue::Section(hsub)) => {
                base.insert(k, ConfigValue::Section(merge_sections(bsub, hsub)));
            },
            (_old, v_high) => {
                base.insert(k, v_high);
            },
        }
    }
    base
}

/// Merge two `ConfigValue` instances.
///
/// If both values are sections, their internal maps are merged using
/// `merge_sections`. Otherwise, it prioritizes the `high` value and returns it.
/// If both ConfigValue instances are sections, it merges their internal maps
/// using merge_sections. Otherwise, it prioritizes the high ConfigValue,
/// returning it directly.
pub fn merge_configs(base: ConfigValue, high: ConfigValue) -> Result<ConfigValue, ParserError> {
    match (base, high) {
        (ConfigValue::Section(mut base_map), ConfigValue::Section(high_map)) => {
            for (k, v_high) in high_map {
                if let Some(base_val) = base_map.remove(&k) {
                    let merged = merge_configs(base_val, v_high)?;
                    base_map.insert(k, merged);
                } else {
                    base_map.insert(k, v_high);
                }
            }

            Ok(ConfigValue::Section(base_map))
        },
        (_old, high_val) => Ok(high_val),
    }
}

/// Builder for creating a configuration from multiple sources.
///
/// `ConfigBuilder` allows users to specify multiple sources for configuration
/// (e.g., CLI, files, environment variables) and merges their data.
#[derive(Default)]
pub struct ConfigBuilder {
    /// A list of sources to load configuration from.
    sources: Vec<Box<dyn ConfigSource + Send + Sync>>,
}

impl ConfigBuilder {
    /// Create an empty configuration builder with no sources.
    pub fn new() -> Self {
        ConfigBuilder { sources: Vec::new() }
    }

    /// Add a configuration source.
    ///
    /// Accepts any entity implementing the `ConfigSource` trait and adds it to
    /// the list of sources to be loaded.
    pub fn add_source<S>(mut self, src: S) -> Self
    where
        S: ConfigSource + Send + Sync + 'static,
    {
        self.sources
            .push(Box::new(src));
        self
    }

    /// Load and merge all configuration sources.
    ///
    /// Sources are sorted by priority (highest first) and merged sequentially,
    /// ensuring that later sources override earlier ones.
    pub fn load(self) -> Result<ConfigValue, ParserError> {
        let mut sources = self.sources;

        sources.sort_by_key(|s| Reverse(s.priority()));

        let mut accumulated = ConfigValue::Section(HashMap::new());
        for src in sources {
            let next = src.load()?;
            accumulated = merge_configs(accumulated, next)?;
        }

        Ok(accumulated)
    }
}

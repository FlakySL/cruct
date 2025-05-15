use std::collections::HashMap;

use crate::{ConfigValue, ParserError};

mod config;
mod file;
#[cfg(feature = "clap")]
mod klap;

pub use config::ConfigFileSource;
pub use file::FileSource;
#[cfg(feature = "clap")]
pub use klap::ClapSource;

/// Anything that can yield a `ConfigValue` (file, env, CLI, etc).
pub trait ConfigSource {
    fn load(&self) -> Result<ConfigValue, ParserError>;
}

/// Merge two `ConfigValue::Section` maps, with `high` taking precedence.
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

pub struct ConfigBuilder {
    sources: Vec<Box<dyn ConfigSource + Send + Sync>>,
}

impl ConfigBuilder {
    /// Starts with no sources.
    pub fn new() -> Self {
        ConfigBuilder { sources: Vec::new() }
    }

    /// Add any source implementing `ConfigSource`.
    pub fn add_source<S>(mut self, src: S) -> Self
    where
        S: ConfigSource + Send + Sync + 'static,
    {
        self.sources
            .push(Box::new(src));
        self
    }

    /// Load and merge all sources in order: later ones override earlier.
    pub fn load(self) -> Result<ConfigValue, ParserError> {
        let mut iter = self
            .sources
            .into_iter();

        let mut accumulated = if let Some(first) = iter.next() {
            first.load()?
        } else {
            ConfigValue::Section(HashMap::new())
        };

        for src in iter {
            let next = src.load()?;
            accumulated = match (accumulated, next) {
                (ConfigValue::Section(base), ConfigValue::Section(high)) => {
                    ConfigValue::Section(merge_sections(base, high))
                },
                (_old, high_val) => high_val,
            };
        }

        Ok(accumulated)
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

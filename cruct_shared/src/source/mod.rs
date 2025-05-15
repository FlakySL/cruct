use std::cmp::Reverse;
use std::collections::HashMap;

use crate::{ConfigValue, ParserError};

mod cli;
mod config;
mod file;

pub use cli::CliSource;
pub use config::ConfigFileSource;
pub use file::FileSource;

/// Anything that can yield a `ConfigValue` (file, env, CLI, etc).
pub trait ConfigSource {
    fn load(&self) -> Result<ConfigValue, ParserError>;

    fn priority(&self) -> u8 {
        u8::MAX
    }
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

pub fn merge_configs(base: ConfigValue, high: ConfigValue) -> Result<ConfigValue, ParserError> {
    match (base, high) {
        (ConfigValue::Section(base), ConfigValue::Section(high)) => {
            Ok(ConfigValue::Section(merge_sections(base, high)))
        },
        (_old, high_val) => Ok(high_val),
    }
}

#[derive(Default)]
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

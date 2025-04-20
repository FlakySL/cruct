use std::collections::HashMap;

use clap::ArgMatches;

use super::ConfigSource;
use crate::{ConfigValue, ParserError};

/// Parses only flags passed on the command‑line.
#[derive(Clone)]
pub struct ClapSource {
    matches: ArgMatches,
}

impl ClapSource {
    /// Build a new `ClapSource` from the given `ArgMatches`.
    pub fn new(matches: ArgMatches) -> Self {
        ClapSource { matches }
    }
}

impl ConfigSource for ClapSource {
    fn load(&self) -> Result<ConfigValue, ParserError> {
        // Flatten into top‑level string → string
        let mut map = HashMap::new();
        for id in self
            .matches
            .ids()
        {
            if let Some(val) = self
                .matches
                .get_one::<String>(id.as_str())
            {
                map.insert(id.to_string(), ConfigValue::Value(val.clone()));
            }
        }
        Ok(ConfigValue::Section(map))
    }
}

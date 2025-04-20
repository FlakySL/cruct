use std::collections::HashMap;

use clap::{ArgMatches, Command};

use super::ConfigSource;
use crate::{ConfigValue, ParserError};

/// Parses only flags passed on the command‑line.
#[derive(Clone)]
pub struct ClapSource {
    matches: ArgMatches,
}

impl ClapSource {
    /// Build from a `clap::Command` that the user configures with all flags.
    pub fn new(cmd: Command) -> Self {
        let matches = cmd.get_matches();
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
